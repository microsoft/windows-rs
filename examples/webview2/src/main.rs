use std::{
    collections::HashMap,
    ffi::CString,
    mem,
    sync::{mpsc, Arc, Mutex},
};

use bindings::{
    Microsoft::Web::WebView2::Core::*,
    Windows::{
        Foundation::*,
        Win32::{
            self,
            DisplayDevices::{RECT, SIZE},
            Gdi,
            HiDpi::{self, PROCESS_DPI_AWARENESS},
            KeyboardAndMouseInput,
            MenusAndResources::HMENU,
            SystemServices::{self, HINSTANCE, LRESULT, PSTR},
            WindowsAndMessaging::{
                self, SetWindowPos_uFlags, HWND, LPARAM, MSG, SHOW_WINDOW_CMD, WINDOW_EX_STYLE,
                WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WNDCLASSA, WPARAM,
            },
        },
    },
};

use windows::*;

use serde::Deserialize;
use serde_json::{Number, Value};

fn main() -> Result<()> {
    initialize_sta()?;
    set_process_dpi_awareness()?;

    let webview = WebView::create(true, None)?;

    // Bind a quick and dirty calculator callback.
    let bound = webview.clone();
    webview.bind("hostCallback", move |id, request| {
        if request.len() == 3 {
            if let Some(Value::String(str)) = request.get(0) {
                if str == "Add" {
                    if let Some(Value::Number(a)) = request.get(1) {
                        if let Some(Value::Number(b)) = request.get(2) {
                            let result = a.as_f64().unwrap_or(0f64) + b.as_f64().unwrap_or(0f64);
                            let result = Number::from_f64(result);
                            if let Some(result) = result {
                                let result = Value::Number(result);
                                bound.resolve(id, 0, result).unwrap_or(&bound);
                            }
                        }
                    }
                }
            }
        }
    })?;

    // Configure the target URL and add an init script to trigger the calculator callback.
    webview
        .set_title("WebView2 Example (examples/webview2)")?
        .init(
            r#"window.hostCallback("Add", 2, 6).then(result => console.log(`Result: ${result}`));"#,
        )?
        .navigate("https://github.com/microsoft/windows-rs")?;

    // Off we go....
    webview.run()
}

#[derive(Clone)]
pub struct FrameWindow {
    window: Arc<HWND>,
    size: Arc<Mutex<SIZE>>,
}

impl Drop for FrameWindow {
    fn drop(&mut self) {
        if Arc::strong_count(&self.window) == 0 {
            unsafe {
                WindowsAndMessaging::DestroyWindow(*self.window);
                WindowsAndMessaging::PostQuitMessage(0);
            }
        }
    }
}

impl FrameWindow {
    fn new() -> Self {
        let hwnd = {
            let class_name = CString::new("WebView").expect("lpszClassName");
            let mut window_class = WNDCLASSA::default();
            window_class.lpfnWndProc = Some(window_proc);
            window_class.lpszClassName = PSTR(class_name.as_ptr() as *mut _);

            unsafe {
                WindowsAndMessaging::RegisterClassA(&window_class);

                WindowsAndMessaging::CreateWindowExA(
                    WINDOW_EX_STYLE(0),
                    PSTR(class_name.as_ptr() as *mut _),
                    PSTR(class_name.as_ptr() as *mut _),
                    WINDOW_STYLE::WS_OVERLAPPEDWINDOW,
                    WindowsAndMessaging::CW_USEDEFAULT,
                    WindowsAndMessaging::CW_USEDEFAULT,
                    640,
                    480,
                    HWND(0),
                    HMENU(0),
                    HINSTANCE(SystemServices::GetModuleHandleA(PSTR(0 as *mut _))),
                    0 as *mut _,
                )
            }
        };

        FrameWindow {
            window: Arc::new(hwnd),
            size: Arc::new(Mutex::new(SIZE { cx: 0, cy: 0 })),
        }
    }
}

#[derive(Clone)]
pub struct WebView {
    controller: Arc<CoreWebView2Controller>,
    webview: Arc<CoreWebView2>,
    tx: mpsc::Sender<Box<dyn FnOnce(WebView) + Send>>,
    rx: Arc<mpsc::Receiver<Box<dyn FnOnce(WebView) + Send>>>,
    thread_id: u32,
    token: EventRegistrationToken,
    bindings: Arc<Mutex<HashMap<String, Box<dyn FnMut(u64, Vec<Value>)>>>>,
    frame: Option<FrameWindow>,
    parent: Arc<HWND>,
    url: Arc<Mutex<HString>>,
}

unsafe impl Send for WebView {}
unsafe impl Sync for WebView {}

impl Drop for WebView {
    fn drop(&mut self) {
        match Arc::strong_count(&self.controller) {
            0 => {
                self.controller.Close().expect("call Close");
            }
            _ => (),
        }
    }
}

#[derive(Debug, Deserialize)]
struct InvokeMessage {
    id: u64,
    method: String,
    params: Vec<Value>,
}

impl WebView {
    pub fn create(debug: bool, parent: Option<HWND>) -> Result<WebView> {
        let (parent, frame) = match parent {
            Some(hwnd) => (hwnd, None),
            None => {
                let frame = FrameWindow::new();
                (*frame.window, Some(frame))
            }
        };

        let (tx, rx) = mpsc::channel();
        let environment = {
            CoreWebView2Environment::CreateAsync()?.SetCompleted(
                AsyncOperationCompletedHandler::new(move |op, _status| {
                    if let Some(op) = op {
                        tx.send(op.GetResults()?).expect("send over mpsc channel");
                    }
                    Ok(())
                }),
            )?;

            wait_with_pump(rx)
        }?;

        let (tx, rx) = mpsc::channel();
        let controller = {
            environment
                .CreateCoreWebView2ControllerAsync(
                    CoreWebView2ControllerWindowReference::CreateFromWindowHandle(parent.0 as u64)?,
                )?
                .SetCompleted(AsyncOperationCompletedHandler::new(move |op, _status| {
                    if let Some(op) = op.as_ref() {
                        tx.send(op.GetResults()?).expect("send over mpsc channel");
                    }
                    Ok(())
                }))?;

            wait_with_pump(rx)
        }?;

        let size = get_window_size(parent);
        let mut client_rect = RECT::default();
        unsafe { WindowsAndMessaging::GetClientRect(parent, &mut client_rect) };
        controller.SetBounds(Rect {
            X: 0f32,
            Y: 0f32,
            Width: size.cx as f32,
            Height: size.cy as f32,
        })?;
        controller.SetIsVisible(true)?;

        let webview = controller.CoreWebView2()?;

        if !debug {
            let settings = webview.Settings()?;
            settings.SetAreDevToolsEnabled(false)?;
            settings.SetAreDefaultContextMenusEnabled(false)?;
        }

        let bindings: Arc<Mutex<HashMap<String, Box<dyn FnMut(u64, Vec<Value>)>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let bindings_ref = bindings.clone();
        let token = webview.WebMessageReceived(TypedEventHandler::<
            CoreWebView2,
            CoreWebView2WebMessageReceivedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args {
                if let Ok(message) = String::from_utf16(args.WebMessageAsJson()?.as_wide()) {
                    if let Ok(value) = serde_json::from_str::<InvokeMessage>(&message) {
                        let mut bindings = bindings_ref.lock().expect("lock bindings");
                        if let Some(f) = bindings.get_mut(&value.method) {
                            (*f)(value.id, value.params);
                        }
                    }
                }
            }
            Ok(())
        }))?;

        if let Some(frame) = frame.as_ref() {
            *frame.size.lock().expect("lock size") = size;
        }

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(rx);
        let thread_id = unsafe { SystemServices::GetCurrentThreadId() };

        let webview = WebView {
            controller: Arc::new(controller),
            webview: Arc::new(webview),
            tx,
            rx,
            thread_id,
            token,
            bindings,
            frame,
            parent: Arc::new(parent),
            url: Arc::new(Mutex::new(HString::new())),
        };

        // Inject the invoke handler.
        webview
            .init(r#"window.external = { invoke: s => window.chrome.webview.postMessage(s) };"#)?;

        if webview.frame.is_some() {
            WebView::set_window_webview(parent, Box::new(webview.clone()));
        }

        Ok(webview)
    }

    pub fn run(self) -> Result<()> {
        let webview = self.webview.as_ref();
        let url = self.url.lock().expect("lock url").clone();
        let (tx, rx) = mpsc::channel();

        if url.len() > 0 {
            let token =
                webview.NavigationCompleted(TypedEventHandler::<
                    CoreWebView2,
                    CoreWebView2NavigationCompletedEventArgs,
                >::new(move |_sender, _args| {
                    tx.send(()).expect("send over mpsc channel");
                    Ok(())
                }))?;

            webview.Navigate(&url)?;

            let result = wait_with_pump(rx);
            webview.RemoveNavigationCompleted(token)?;
            result?;
        }

        if let Some(frame) = self.frame.as_ref() {
            let hwnd = *frame.window;
            unsafe {
                WindowsAndMessaging::ShowWindow(hwnd, SHOW_WINDOW_CMD::SW_SHOW);
                Gdi::UpdateWindow(hwnd);
                KeyboardAndMouseInput::SetFocus(hwnd);
            }
        }

        let mut msg = MSG::default();
        let h_wnd = HWND::default();

        loop {
            while let Ok(f) = self.rx.try_recv() {
                (f)(self.clone());
            }

            unsafe {
                let result = WindowsAndMessaging::GetMessageA(&mut msg, h_wnd, 0, 0).0;

                match result {
                    -1 => {
                        break {
                            let error =
                                format!("GetMessageW failed: {}", Win32::Debug::GetLastError());
                            Err(Error::new(ErrorCode::E_NOINTERFACE, &error))
                        }
                    }
                    0 => break Ok(()),
                    _ => match msg.message {
                        WindowsAndMessaging::WM_APP => (),
                        _ => {
                            WindowsAndMessaging::TranslateMessage(&msg);
                            WindowsAndMessaging::DispatchMessageA(&msg);
                        }
                    },
                }
            }
        }
    }

    pub fn terminate(self) -> Result<()> {
        self.dispatch(|_webview| unsafe {
            WindowsAndMessaging::PostQuitMessage(0);
        })?;
        Ok(())
    }

    pub fn set_title(&self, title: &str) -> Result<&Self> {
        if let Some(frame) = self.frame.as_ref() {
            if let Ok(title) = CString::new(title) {
                unsafe {
                    WindowsAndMessaging::SetWindowTextA(
                        *frame.window,
                        PSTR(title.as_ptr() as *mut _),
                    );
                }
            }
        }
        Ok(self)
    }

    pub fn set_size(&self, width: i32, height: i32) -> Result<&Self> {
        if let Some(frame) = self.frame.as_ref() {
            *frame.size.lock().expect("lock size") = SIZE {
                cx: width,
                cy: height,
            };
            self.controller.SetBounds(Rect {
                X: 0f32,
                Y: 0f32,
                Width: width as f32,
                Height: height as f32,
            })?;

            unsafe {
                WindowsAndMessaging::SetWindowPos(
                    *frame.window,
                    HWND(0),
                    0,
                    0,
                    width,
                    height,
                    SetWindowPos_uFlags::SWP_NOACTIVATE
                        | SetWindowPos_uFlags::SWP_NOZORDER
                        | SetWindowPos_uFlags::SWP_NOMOVE,
                );
            }
        }
        Ok(self)
    }

    pub fn get_window(&self) -> HWND {
        *self.parent
    }

    pub fn navigate(&self, url: &str) -> Result<&Self> {
        let url: Vec<u16> = url.encode_utf16().collect();
        let url = HString::from_wide(&url);
        *self.url.lock().expect("lock url") = url;
        Ok(self)
    }

    pub fn init(&self, js: &str) -> Result<&Self> {
        let js: Vec<u16> = js.encode_utf16().collect();
        let js = HString::from_wide(&js);

        let (tx, rx) = mpsc::channel();
        let webview = self.webview.as_ref();
        webview
            .AddScriptToExecuteOnDocumentCreatedAsync(js)?
            .SetCompleted(AsyncOperationCompletedHandler::new(move |op, _status| {
                if let Some(op) = op {
                    tx.send(op.GetResults()?).expect("send over mpsc channel");
                }
                Ok(())
            }))?;

        wait_with_pump(rx)?;
        Ok(self)
    }

    pub fn eval(&self, js: &str) -> Result<&Self> {
        let js: Vec<u16> = js.encode_utf16().collect();
        let js = HString::from_wide(&js);

        let webview = self.webview.as_ref();
        let (tx, rx) = mpsc::channel();
        webview
            .ExecuteScriptAsync(js)?
            .SetCompleted(AsyncOperationCompletedHandler::new(move |op, _status| {
                if let Some(op) = op {
                    tx.send(op.GetResults()?).expect("send over mpsc channel");
                }
                Ok(())
            }))?;

        wait_with_pump(rx)?;
        Ok(self)
    }

    pub fn dispatch<F>(&self, f: F) -> Result<&Self>
    where
        F: FnOnce(WebView) + Send + 'static,
    {
        self.tx.send(Box::new(f)).expect("send the fn");

        unsafe {
            WindowsAndMessaging::PostThreadMessageA(
                self.thread_id,
                WindowsAndMessaging::WM_APP,
                WPARAM(0),
                LPARAM(0),
            );
        }
        Ok(self)
    }

    pub fn bind<F>(&self, name: &str, f: F) -> Result<&Self>
    where
        F: FnMut(u64, Vec<Value>) + 'static,
    {
        self.bindings
            .lock()
            .unwrap()
            .insert(String::from(name), Box::new(f));

        let js = String::from(
            r#"
            (function() {
                var name = '"#,
        ) + name
            + r#"';
                var RPC = window._rpc = (window._rpc || {nextSeq: 1});
                window[name] = function() {
                    var seq = RPC.nextSeq++;
                    var promise = new Promise(function(resolve, reject) {
                        RPC[seq] = {
                            resolve: resolve,
                            reject: reject,
                        };
                    });
                    window.external.invoke({
                        id: seq,
                        method: name,
                        params: Array.prototype.slice.call(arguments),
                    });
                    return promise;
                }
            })()"#;

        self.init(&js)
    }

    pub fn resolve(&self, id: u64, status: i32, result: Value) -> Result<&Self> {
        let result = result.to_string();

        self.dispatch(move |webview| {
            let method = match status {
                0 => "resolve",
                _ => "reject",
            };
            let js = format!(
                r#"
                window._rpc[{}].{}({});
                window._rpc[{}] = undefined;"#,
                id, method, result, id
            );

            webview.eval(&js).expect("eval return script");
        })
    }

    fn set_window_webview(hwnd: HWND, webview: Box<WebView>) {
        unsafe {
            WindowsAndMessaging::SetWindowLongPtrA(
                hwnd,
                WINDOW_LONG_PTR_INDEX::GWLP_USERDATA,
                Box::into_raw(webview) as _,
            );
        }
    }

    fn get_window_webview(hwnd: HWND) -> Option<Box<WebView>> {
        unsafe {
            let data =
                WindowsAndMessaging::GetWindowLongPtrA(hwnd, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA);

            match data {
                0 => None,
                _ => {
                    let webview_ptr = data as *mut WebView;
                    let raw = Box::from_raw(webview_ptr);
                    let webview = raw.clone();
                    mem::forget(raw);

                    Some(webview)
                }
            }
        }
    }
}

fn set_process_dpi_awareness() -> Result<()> {
    let code = unsafe {
        HiDpi::SetProcessDpiAwareness(PROCESS_DPI_AWARENESS::PROCESS_PER_MONITOR_DPI_AWARE)
    };

    if code.is_err() {
        Err(Error::fast_error(code))
    } else {
        Ok(())
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let webview = match WebView::get_window_webview(hwnd) {
        Some(webview) => webview,
        None => return unsafe { WindowsAndMessaging::DefWindowProcA(hwnd, msg, w_param, l_param) },
    };

    let frame = webview
        .frame
        .as_ref()
        .expect("should only be called for owned windows");

    match msg {
        WindowsAndMessaging::WM_SIZE => {
            let size = get_window_size(hwnd);
            webview
                .controller
                .SetBounds(Rect {
                    X: 0f32,
                    Y: 0f32,
                    Width: size.cx as f32,
                    Height: size.cy as f32,
                })
                .expect("call SetBounds");
            *frame.size.lock().expect("lock size") = size;
            LRESULT(0)
        }

        WindowsAndMessaging::WM_CLOSE => {
            unsafe {
                WindowsAndMessaging::DestroyWindow(hwnd);
            }
            LRESULT(0)
        }

        WindowsAndMessaging::WM_DESTROY => {
            webview.terminate().expect("window is gone");
            LRESULT(0)
        }

        _ => unsafe { WindowsAndMessaging::DefWindowProcA(hwnd, msg, w_param, l_param) },
    }
}

fn wait_with_pump<T>(rx: mpsc::Receiver<T>) -> Result<T> {
    let mut msg = MSG::default();
    let hwnd = HWND::default();

    loop {
        if let Ok(result) = rx.try_recv() {
            return Ok(result);
        }

        unsafe {
            match WindowsAndMessaging::GetMessageA(&mut msg, hwnd, 0, 0).0 {
                -1 => {
                    let error = format!("GetMessageW failed: {}", Win32::Debug::GetLastError());
                    return Err(Error::new(ErrorCode::E_NOINTERFACE, &error));
                }
                0 => return Err(Error::new(ErrorCode::E_NOINTERFACE, "task canceled")),
                _ => {
                    WindowsAndMessaging::TranslateMessage(&msg);
                    WindowsAndMessaging::DispatchMessageA(&msg);
                }
            }
        }
    }
}

fn get_window_size(hwnd: HWND) -> SIZE {
    let mut client_rect = RECT::default();
    unsafe { WindowsAndMessaging::GetClientRect(hwnd, &mut client_rect) };
    SIZE {
        cx: client_rect.right - client_rect.left,
        cy: client_rect.bottom - client_rect.top,
    }
}

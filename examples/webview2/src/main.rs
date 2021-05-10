use std::{
    collections::HashMap,
    ffi::CString,
    mem,
    sync::{mpsc, Arc, Mutex},
};

use serde::Deserialize;
use serde_json::{Number, Value};
use windows::*;

use bindings::{
    Microsoft::Web::WebView2::Core::*,
    Windows::Foundation::*,
    Windows::Win32::{
        Graphics::Gdi::*,
        System::SystemServices::*,
        System::Threading::*,
        UI::{DisplayDevices::*, HiDpi::*, KeyboardAndMouseInput::*, WindowsAndMessaging::*},
    },
};

fn main() -> Result<()> {
    initialize_sta()?;
    set_process_dpi_awareness()?;

    let webview = WebView::create(None, true)?;

    // Bind a quick and dirty calculator callback.
    webview.bind("hostCallback", move |request| {
        if let [Value::String(str), Value::Number(a), Value::Number(b)] = &request[..] {
            if str == "Add" {
                let result = a.as_f64().unwrap_or(0f64) + b.as_f64().unwrap_or(0f64);
                let result = Number::from_f64(result);
                if let Some(result) = result {
                    return Ok(Value::Number(result));
                }
            }
        }

        Err(Error::CallbackError(String::from(
            r#"Usage: window.hostCallback("Add", a, b)"#,
        )))
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

#[derive(Debug)]
pub enum Error {
    WindowsError(windows::Error),
    JsonError(serde_json::Error),
    CallbackError(String),
    TaskCanceled,
    LockError,
    SendError,
}

impl From<windows::Error> for Error {
    fn from(err: windows::Error) -> Self {
        Self::WindowsError(err)
    }
}

impl From<HRESULT> for Error {
    fn from(err: HRESULT) -> Self {
        Self::WindowsError(windows::Error::fast_error(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl<'a, T: 'a> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::LockError
    }
}

impl<'a, T: 'a> From<std::sync::TryLockError<T>> for Error {
    fn from(_: std::sync::TryLockError<T>) -> Self {
        Self::LockError
    }
}

impl<'a, T: 'a> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(_: std::sync::mpsc::SendError<T>) -> Self {
        Self::SendError
    }
}

type Result<T> = std::result::Result<T, Error>;

struct Window(HWND);

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.0);
        }
    }
}

#[derive(Clone)]
pub struct FrameWindow {
    window: Arc<HWND>,
    size: Arc<Mutex<SIZE>>,
}

impl FrameWindow {
    fn new() -> Self {
        let hwnd = {
            let class_name = "WebView";
            let mut window_class = WNDCLASSA::default();
            window_class.lpfnWndProc = Some(window_proc);
            let c_class_name = CString::new(class_name).expect("lpszClassName");
            window_class.lpszClassName = PSTR(c_class_name.as_ptr() as *mut _);

            unsafe {
                RegisterClassA(&window_class);

                CreateWindowExA(
                    Default::default(),
                    class_name,
                    class_name,
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    None,
                    None,
                    GetModuleHandleA(None),
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

struct WebViewController(CoreWebView2Controller);

#[derive(Clone)]
pub struct WebView {
    controller: Arc<WebViewController>,
    webview: Arc<CoreWebView2>,
    tx: mpsc::Sender<Box<dyn FnOnce(WebView) + Send>>,
    rx: Arc<mpsc::Receiver<Box<dyn FnOnce(WebView) + Send>>>,
    thread_id: u32,
    bindings: Arc<Mutex<HashMap<String, Box<dyn FnMut(Vec<Value>) -> Result<Value>>>>>,
    frame: Option<FrameWindow>,
    parent: Arc<HWND>,
    url: Arc<Mutex<HSTRING>>,
}

impl Drop for WebViewController {
    fn drop(&mut self) {
        self.0.Close().expect("call Close");
    }
}

#[derive(Debug, Deserialize)]
struct InvokeMessage {
    id: u64,
    method: String,
    params: Vec<Value>,
}

impl WebView {
    pub fn create(parent: Option<HWND>, debug: bool) -> Result<WebView> {
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
        unsafe { GetClientRect(parent, &mut client_rect) };
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

        if let Some(frame) = frame.as_ref() {
            *frame.size.lock()? = size;
        }

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(rx);
        let thread_id = unsafe { GetCurrentThreadId() };

        let webview = WebView {
            controller: Arc::new(WebViewController(controller)),
            webview: Arc::new(webview),
            tx,
            rx,
            thread_id,
            bindings: Arc::new(Mutex::new(HashMap::new())),
            frame,
            parent: Arc::new(parent),
            url: Arc::new(Mutex::new(HSTRING::new())),
        };

        // Inject the invoke handler.
        webview
            .init(r#"window.external = { invoke: s => window.chrome.webview.postMessage(s) };"#)?;

        let bindings = webview.bindings.clone();
        let bound = webview.clone();
        let _token = webview.webview.WebMessageReceived(TypedEventHandler::<
            CoreWebView2,
            CoreWebView2WebMessageReceivedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args {
                if let Ok(message) = String::from_utf16(args.WebMessageAsJson()?.as_wide()) {
                    if let Ok(value) = serde_json::from_str::<InvokeMessage>(&message) {
                        if let Ok(mut bindings) = bindings.try_lock() {
                            if let Some(f) = bindings.get_mut(&value.method) {
                                match (*f)(value.params) {
                                    Ok(result) => bound.resolve(value.id, 0, result),
                                    Err(err) => bound.resolve(
                                        value.id,
                                        1,
                                        Value::String(format!("{:#?}", err)),
                                    ),
                                }
                                .unwrap();
                            }
                        }
                    }
                }
            }
            Ok(())
        }))?;

        if webview.frame.is_some() {
            WebView::set_window_webview(parent, Some(Box::new(webview.clone())));
        }

        Ok(webview)
    }

    pub fn run(self) -> Result<()> {
        let webview = self.webview.as_ref();
        let url = self.url.try_lock()?.clone();
        let (tx, rx) = mpsc::channel();

        if !url.is_empty() {
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
                ShowWindow(hwnd, SW_SHOW);
                UpdateWindow(hwnd);
                SetFocus(hwnd);
            }
        }

        let mut msg = MSG::default();
        let h_wnd = HWND::default();

        loop {
            while let Ok(f) = self.rx.try_recv() {
                (f)(self.clone());
            }

            unsafe {
                let result = GetMessageA(&mut msg, h_wnd, 0, 0).0;

                match result {
                    -1 => break Err(HRESULT::from_thread().into()),
                    0 => break Ok(()),
                    _ => match msg.message {
                        WM_APP => (),
                        _ => {
                            TranslateMessage(&msg);
                            DispatchMessageA(&msg);
                        }
                    },
                }
            }
        }
    }

    pub fn terminate(self) -> Result<()> {
        self.dispatch(|_webview| unsafe {
            PostQuitMessage(0);
        })?;

        if self.frame.is_some() {
            WebView::set_window_webview(self.get_window(), None);
        }

        Ok(())
    }

    pub fn set_title(&self, title: &str) -> Result<&Self> {
        if let Some(frame) = self.frame.as_ref() {
            unsafe {
                SetWindowTextA(*frame.window, title);
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
            self.controller.0.SetBounds(Rect {
                X: 0f32,
                Y: 0f32,
                Width: width as f32,
                Height: height as f32,
            })?;

            unsafe {
                SetWindowPos(
                    *frame.window,
                    None,
                    0,
                    0,
                    width,
                    height,
                    SWP_NOACTIVATE | SWP_NOZORDER | SWP_NOMOVE,
                );
            }
        }
        Ok(self)
    }

    pub fn get_window(&self) -> HWND {
        *self.parent
    }

    pub fn navigate(&self, url: &str) -> Result<&Self> {
        let url = url.into();
        *self.url.lock().expect("lock url") = url;
        Ok(self)
    }

    pub fn init(&self, js: &str) -> Result<&Self> {
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
            PostThreadMessageA(self.thread_id, WM_APP, WPARAM(0), LPARAM(0));
        }
        Ok(self)
    }

    pub fn bind<F>(&self, name: &str, f: F) -> Result<&Self>
    where
        F: FnMut(Vec<Value>) -> Result<Value> + 'static,
    {
        self.bindings
            .lock()?
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

    fn set_window_webview(hwnd: HWND, webview: Option<Box<WebView>>) -> Option<Box<WebView>> {
        unsafe {
            match SetWindowLong(
                hwnd,
                GWLP_USERDATA,
                match webview {
                    Some(webview) => Box::into_raw(webview) as _,
                    None => 0 as _,
                },
            ) {
                0 => None,
                ptr => Some(Box::from_raw(ptr as *mut _)),
            }
        }
    }

    fn get_window_webview(hwnd: HWND) -> Option<Box<WebView>> {
        unsafe {
            let data = GetWindowLong(hwnd, GWLP_USERDATA);

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
    unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE).ok()? };
    Ok(())
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let webview = match WebView::get_window_webview(hwnd) {
        Some(webview) => webview,
        None => return unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) },
    };

    let frame = webview
        .frame
        .as_ref()
        .expect("should only be called for owned windows");

    match msg {
        WM_SIZE => {
            let size = get_window_size(hwnd);
            webview
                .controller
                .0
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

        WM_CLOSE => {
            unsafe {
                DestroyWindow(hwnd);
            }
            LRESULT(0)
        }

        WM_DESTROY => {
            webview.terminate().expect("window is gone");
            LRESULT(0)
        }

        _ => unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) },
    }
}

/// The WebView2 threading model runs everything on the UI thread, including callbacks which it triggers
/// with `PostMessage`, and we're using this here because it's waiting for some async operations in WebView2
/// to finish before starting the main message loop in `WebView::run`. As long as there are no pending
/// results in `rx`, it will pump Window messages and check for a result after each message is dispatched.
///
/// `GetMessage` is a blocking call, so if we want to send results from another thread, senders from other
/// threads should "kick" the message loop after sending the result by calling `PostThreadMessage` with an
/// ignorable/unhandled message such as `WM_APP`.
fn wait_with_pump<T>(rx: mpsc::Receiver<T>) -> Result<T> {
    let mut msg = MSG::default();
    let hwnd = HWND::default();

    loop {
        if let Ok(result) = rx.try_recv() {
            return Ok(result);
        }

        unsafe {
            match GetMessageA(&mut msg, hwnd, 0, 0).0 {
                -1 => {
                    return Err(HRESULT::from_thread().into());
                }
                0 => return Err(Error::TaskCanceled),
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        }
    }
}

fn get_window_size(hwnd: HWND) -> SIZE {
    let mut client_rect = RECT::default();
    unsafe { GetClientRect(hwnd, &mut client_rect) };
    SIZE {
        cx: client_rect.right - client_rect.left,
        cy: client_rect.bottom - client_rect.top,
    }
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}

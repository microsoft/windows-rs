use crate::bindings::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::OnceLock;
use windows_core::*;

/// Message handler: receives the raw window handle, message code, and
/// `wparam`/`lparam`. Return `Some(result)` to handle the message, or `None` to
/// fall through to default processing.
type MessageHandler = Box<dyn FnMut(*mut core::ffi::c_void, u32, usize, isize) -> Option<isize>>;

/// Resize handler: receives the new client-area width and height in pixels.
type ResizeHandler = Box<dyn FnMut(i32, i32)>;

/// Move handler: runs when the parent window position changes.
type MoveHandler = Box<dyn FnMut()>;

/// Close handler: runs before the native window is destroyed.
type CloseHandler = Box<dyn FnMut()>;

struct State {
    live: Rc<Cell<bool>>,
    message: Option<MessageHandler>,
    resize: Option<ResizeHandler>,
    moved: Option<MoveHandler>,
    close: Option<CloseHandler>,
    quit_on_close: bool,
    closing: bool,
    dispatching: bool,
    deferred_close: bool,
}

/// A top-level window.
///
/// The window lives until it is dropped or closed by the user. Its raw `HWND`
/// is available via [`Window::hwnd`] for interop with other Windows APIs.
pub struct Window {
    hwnd: HWND,
    live: Rc<Cell<bool>>,
}

impl Window {
    /// Begins configuring a new window with the given title.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(title: &str) -> WindowBuilder {
        WindowBuilder {
            title: title.to_string(),
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            client_size: false,
            style: WS_OVERLAPPEDWINDOW as u32,
            ex_style: 0,
            message: None,
            resize: None,
            moved: None,
            close: None,
            quit_on_close: true,
        }
    }

    /// Returns the raw window handle for interop with other Windows APIs, or null after the native
    /// window is destroyed.
    pub fn hwnd(&self) -> *mut core::ffi::c_void {
        if self.live.get() {
            self.hwnd
        } else {
            core::ptr::null_mut()
        }
    }

    /// Returns the current client-area size in pixels as `(width, height)`.
    pub fn client_size(&self) -> (i32, i32) {
        if !self.live.get() {
            return (0, 0);
        }
        let mut rect = RECT::default();
        unsafe {
            if GetClientRect(self.hwnd, &mut rect).as_bool() {
                (rect.right - rect.left, rect.bottom - rect.top)
            } else {
                (0, 0)
            }
        }
    }

    /// Requests normal window close processing.
    pub fn close(&self) {
        if self.live.get() {
            unsafe {
                SendMessageW(self.hwnd, WM_CLOSE as u32, 0, 0);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.live.get() {
            unsafe {
                _ = DestroyWindow(self.hwnd);
            }
        }
    }
}

/// Builder for a [`Window`].
pub struct WindowBuilder {
    title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    client_size: bool,
    style: u32,
    ex_style: u32,
    message: Option<MessageHandler>,
    resize: Option<ResizeHandler>,
    moved: Option<MoveHandler>,
    close: Option<CloseHandler>,
    quit_on_close: bool,
}

impl WindowBuilder {
    /// Sets the initial screen position of the window, in pixels.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the initial outer window size, including non-client borders, in pixels.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self.client_size = false;
        self
    }

    /// Sets the initial client-area size, excluding non-client borders, in pixels.
    pub fn client_size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self.client_size = true;
        self
    }

    /// Sets the window style (`WS_*`). Defaults to `WS_OVERLAPPEDWINDOW`.
    pub fn style(mut self, style: u32) -> Self {
        self.style = style;
        self
    }

    /// Sets the extended window style (`WS_EX_*`). Defaults to none.
    pub fn ex_style(mut self, ex_style: u32) -> Self {
        self.ex_style = ex_style;
        self
    }

    /// Disables the DWM redirection surface for content supplied by composition.
    pub fn no_redirection_bitmap(mut self) -> Self {
        self.ex_style |= WS_EX_NOREDIRECTIONBITMAP as u32;
        self
    }

    /// Sets a handler called for every window message. Return `Some(result)` to
    /// handle the message, or `None` to fall through to default processing.
    pub fn on_message<F>(mut self, handler: F) -> Self
    where
        F: FnMut(*mut core::ffi::c_void, u32, usize, isize) -> Option<isize> + 'static,
    {
        self.message = Some(Box::new(handler));
        self
    }

    /// Sets a handler called when the client area is resized, with the new
    /// width and height in pixels.
    pub fn on_resize<F>(mut self, handler: F) -> Self
    where
        F: FnMut(i32, i32) + 'static,
    {
        self.resize = Some(Box::new(handler));
        self
    }

    /// Sets a handler called when the window position changes.
    pub fn on_move<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.moved = Some(Box::new(handler));
        self
    }

    /// Sets a handler called before the window is destroyed in response to `WM_CLOSE`.
    ///
    /// Use this to release hosted resources that require a live parent window. The window
    /// continues through default close processing after the handler returns.
    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.close = Some(Box::new(handler));
        self
    }

    /// Controls whether closing this window posts `WM_QUIT`.
    ///
    /// The default is `true`, which ends this thread's message loop when the window closes. Set
    /// this to `false` for multi-window, tray, or background applications whose lifetime is
    /// controlled separately, then call [`quit`] when the application should exit.
    pub fn quit_on_close(mut self, value: bool) -> Self {
        self.quit_on_close = value;
        self
    }

    /// Creates and shows the window.
    ///
    /// The first call attempts to set process DPI awareness to per-monitor v2.
    /// Set any different process DPI policy before calling this method.
    pub fn create(self) -> Result<Window> {
        unsafe {
            register_class();

            let mut title: Vec<u16> = self.title.encode_utf16().collect();
            title.push(0);

            let (width, height) = if self.client_size {
                let mut rect = RECT {
                    right: self.width,
                    bottom: self.height,
                    ..Default::default()
                };
                if !AdjustWindowRectExForDpi(
                    &mut rect,
                    self.style,
                    false.into(),
                    self.ex_style,
                    GetDpiForSystem(),
                )
                .as_bool()
                {
                    return Err(Error::from_thread());
                }
                (rect.right - rect.left, rect.bottom - rect.top)
            } else {
                (self.width, self.height)
            };

            let hwnd = CreateWindowExW(
                self.ex_style,
                class_name(),
                PCWSTR(title.as_ptr()),
                self.style,
                self.x,
                self.y,
                width,
                height,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null(),
            );

            if hwnd.is_null() {
                return Err(Error::from_thread());
            }

            let live = Rc::new(Cell::new(true));
            let state = Box::new(State {
                live: Rc::clone(&live),
                message: self.message,
                resize: self.resize,
                moved: self.moved,
                close: self.close,
                quit_on_close: self.quit_on_close,
                closing: false,
                dispatching: false,
                deferred_close: false,
            });
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as _);

            _ = ShowWindow(hwnd, SW_SHOWNORMAL);
            Ok(Window { hwnd, live })
        }
    }
}

/// Runs a blocking, event-driven message loop until the window is closed.
pub fn run() {
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, core::ptr::null_mut(), 0, 0).as_bool() {
            _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

/// Runs a message loop driven by `render`. `render` is called whenever no
/// messages are pending; return `Ok(true)` to keep rendering immediately (for
/// continuous animation) or `Ok(false)` to wait for the next message before
/// rendering again (event-driven, e.g. when occluded or idle). Returns when the
/// window is closed, or early if `render` returns an error.
pub fn run_with<F>(mut render: F) -> Result<()>
where
    F: FnMut() -> Result<bool>,
{
    unsafe {
        let mut message = MSG::default();
        let mut animating = true;
        loop {
            if animating {
                while PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE as u32)
                    .as_bool()
                {
                    if message.message == WM_QUIT as u32 {
                        return Ok(());
                    }
                    _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            } else if GetMessageW(&mut message, core::ptr::null_mut(), 0, 0).as_bool() {
                if message.message == WM_QUIT as u32 {
                    return Ok(());
                }
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            } else {
                return Ok(());
            }
            animating = render()?;
        }
    }
}

/// Posts a quit message, causing the message loop to exit.
pub fn quit() {
    unsafe { PostQuitMessage(0) };
}

/// Dispatches all currently-pending messages without blocking, then returns.
///
/// Returns `false` if a quit message was received (the caller should stop
/// pumping) or `true` otherwise. Unlike [`run`], this never blocks waiting for
/// the next message, so callers can drive the message loop while waiting on an
/// external condition - for example pumping until an asynchronous callback
/// completes.
pub fn pump() -> bool {
    unsafe {
        let mut message = MSG::default();
        while PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE as u32).as_bool() {
            if message.message == WM_QUIT as u32 {
                return false;
            }
            _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        true
    }
}

fn class_name() -> PCWSTR {
    static NAME: OnceLock<Vec<u16>> = OnceLock::new();
    let name = NAME.get_or_init(|| "windows-window.Window\0".encode_utf16().collect());
    PCWSTR(name.as_ptr())
}

unsafe fn register_class() {
    static REGISTER: OnceLock<()> = OnceLock::new();
    REGISTER.get_or_init(|| unsafe {
        _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        let wc = WNDCLASSW {
            style: (CS_HREDRAW | CS_VREDRAW) as u32,
            lpfnWndProc: Some(wndproc),
            hCursor: LoadCursorW(core::ptr::null_mut(), IDC_ARROW),
            lpszClassName: class_name(),
            ..Default::default()
        };
        RegisterClassW(&wc);
    });
}

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        let mut state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
        let mut handled = None;
        let mut replay_close = false;

        if !state.is_null() {
            if (*state).dispatching {
                return match message as i32 {
                    WM_CLOSE => {
                        (*state).deferred_close = true;
                        0
                    }
                    WM_DESTROY => {
                        if (*state).quit_on_close && (*state).closing {
                            PostQuitMessage(0);
                        }
                        0
                    }
                    WM_NCDESTROY => {
                        (*state).live.set(false);
                        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                        drop(Box::from_raw(state));
                        DefWindowProcW(hwnd, message, wparam, lparam)
                    }
                    _ => DefWindowProcW(hwnd, message, wparam, lparam),
                };
            }

            let mut message_handler = (*state).message.take();
            let mut resize_handler = (*state).resize.take();
            let mut move_handler = (*state).moved.take();
            let mut close_handler = (*state).close.take();
            (*state).dispatching = true;

            // Handlers are invoked directly, without catch_unwind: a panic that
            // escapes one unwinds to this extern "system" boundary and aborts the
            // process rather than crossing into the OS frames that called wndproc.
            // This is intentional.
            if let Some(handler) = message_handler.as_mut() {
                handled = handler(hwnd, message, wparam, lparam);
            }

            state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if state.is_null() {
                return handled.unwrap_or(0);
            }

            if handled.is_none() && message == WM_CLOSE as u32 {
                (*state).closing = true;
            }

            if handled.is_none()
                && message == WM_SIZE as u32
                && let Some(handler) = resize_handler.as_mut()
            {
                let width = (lparam & 0xffff) as i32;
                let height = ((lparam >> 16) & 0xffff) as i32;
                handler(width, height);
                handled = Some(0);
            }

            if handled.is_none()
                && message == WM_MOVE as u32
                && let Some(handler) = move_handler.as_mut()
            {
                handler();
            }

            if handled.is_none()
                && message == WM_CLOSE as u32
                && let Some(handler) = close_handler.as_mut()
            {
                handler();
            }

            // A handler may synchronously destroy the window. Re-read the state
            // before restoring callbacks or replaying a nested close request.
            state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if state.is_null() {
                return handled.unwrap_or(0);
            }

            (*state).dispatching = false;
            replay_close =
                std::mem::take(&mut (*state).deferred_close) && message != WM_CLOSE as u32;
            (*state).message = message_handler;
            (*state).resize = resize_handler;
            (*state).moved = move_handler;
            (*state).close = close_handler;
        }

        if message == WM_NCDESTROY as u32 {
            let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
            if !state.is_null() {
                (*state).live.set(false);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(state));
            }
        }

        let result = if let Some(result) = handled {
            result
        } else {
            match message as i32 {
                WM_DESTROY => {
                    let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
                    if !state.is_null() && (*state).quit_on_close && (*state).closing {
                        PostQuitMessage(0);
                    }
                    0
                }
                _ => DefWindowProcW(hwnd, message, wparam, lparam),
            }
        };

        if replay_close && GetWindowLongPtrW(hwnd, GWLP_USERDATA) != 0 {
            SendMessageW(hwnd, WM_CLOSE as u32, 0, 0);
        }

        result
    }
}

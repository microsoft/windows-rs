use crate::bindings::*;
use std::sync::OnceLock;
use windows_core::*;

/// Message handler: receives the raw window handle, message code, and
/// `wparam`/`lparam`. Return `Some(result)` to handle the message, or `None` to
/// fall through to default processing.
type MessageHandler = Box<dyn FnMut(*mut core::ffi::c_void, u32, usize, isize) -> Option<isize>>;

/// Resize handler: receives the new client-area width and height in pixels.
type ResizeHandler = Box<dyn FnMut(i32, i32)>;

struct State {
    message: Option<MessageHandler>,
    resize: Option<ResizeHandler>,
}

/// A top-level window.
///
/// The window lives until it is dropped or closed by the user. Its raw `HWND`
/// is available via [`Window::hwnd`] for interop with other Windows APIs.
pub struct Window(HWND);

impl Window {
    /// Begins configuring a new window with the given title.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(title: &str) -> WindowBuilder {
        WindowBuilder {
            title: title.to_string(),
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            style: WS_OVERLAPPEDWINDOW,
            ex_style: 0,
            state: State {
                message: None,
                resize: None,
            },
        }
    }

    /// Returns the raw window handle for interop with other Windows APIs.
    pub fn hwnd(&self) -> *mut core::ffi::c_void {
        self.0
    }

    /// Returns the current client-area size in pixels as `(width, height)`.
    pub fn client_size(&self) -> (i32, i32) {
        let mut rect = RECT::default();
        unsafe {
            if GetClientRect(self.0, &mut rect).as_bool() {
                (rect.right - rect.left, rect.bottom - rect.top)
            } else {
                (0, 0)
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if IsWindow(self.0).as_bool() {
                _ = DestroyWindow(self.0);
            }
        }
    }
}

/// Builder for a [`Window`].
pub struct WindowBuilder {
    title: String,
    width: i32,
    height: i32,
    style: u32,
    ex_style: u32,
    state: State,
}

impl WindowBuilder {
    /// Sets the initial window size in pixels.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
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

    /// Sets a handler called for every window message. Return `Some(result)` to
    /// handle the message, or `None` to fall through to default processing.
    pub fn on_message<F>(mut self, handler: F) -> Self
    where
        F: FnMut(*mut core::ffi::c_void, u32, usize, isize) -> Option<isize> + 'static,
    {
        self.state.message = Some(Box::new(handler));
        self
    }

    /// Sets a handler called when the client area is resized, with the new
    /// width and height in pixels.
    pub fn on_resize<F>(mut self, handler: F) -> Self
    where
        F: FnMut(i32, i32) + 'static,
    {
        self.state.resize = Some(Box::new(handler));
        self
    }

    /// Creates and shows the window.
    pub fn create(self) -> Result<Window> {
        unsafe {
            register_class();

            let mut title: Vec<u16> = self.title.encode_utf16().collect();
            title.push(0);

            let hwnd = CreateWindowExW(
                self.ex_style,
                class_name(),
                PCWSTR(title.as_ptr()),
                self.style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                self.width,
                self.height,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null(),
            );

            if hwnd.is_null() {
                return Err(Error::from_thread());
            }

            let state = Box::new(self.state);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as _);

            _ = ShowWindow(hwnd, SW_SHOWNORMAL);
            Ok(Window(hwnd))
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
                while PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE).as_bool() {
                    if message.message == WM_QUIT {
                        return Ok(());
                    }
                    _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            } else if GetMessageW(&mut message, core::ptr::null_mut(), 0, 0).as_bool() {
                if message.message == WM_QUIT {
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
/// external condition — for example pumping until an asynchronous callback
/// completes.
pub fn pump() -> bool {
    unsafe {
        let mut message = MSG::default();
        while PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE).as_bool() {
            if message.message == WM_QUIT {
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
            style: CS_HREDRAW | CS_VREDRAW,
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
        let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut State;
        let mut handled = None;

        if !state.is_null() {
            // Detach the handlers before invoking them so that any reentrant
            // dispatch (e.g. a handler that calls SetWindowPos) sees empty slots
            // and falls through to default processing rather than aliasing a
            // handler that is already running.
            let mut message_handler = (*state).message.take();
            let mut resize_handler = (*state).resize.take();

            if let Some(handler) = message_handler.as_mut() {
                handled = handler(hwnd, message, wparam, lparam);
            }

            if handled.is_none()
                && message == WM_SIZE
                && let Some(handler) = resize_handler.as_mut()
            {
                let width = (lparam & 0xffff) as i32;
                let height = ((lparam >> 16) & 0xffff) as i32;
                handler(width, height);
                handled = Some(0);
            }

            (*state).message = message_handler;
            (*state).resize = resize_handler;
        }

        if message == WM_NCDESTROY && !state.is_null() {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            drop(Box::from_raw(state));
        }

        if let Some(result) = handled {
            return result;
        }

        match message {
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, message, wparam, lparam),
        }
    }
}

use super::*;

/// The WebView2 environment. Owns the user data folder and browser process and
/// creates [`Controller`] instances that host the browser in a window.
pub struct Environment(pub(crate) ICoreWebView2Environment);

impl Environment {
    /// Creates the default WebView2 environment, pumping the calling thread's
    /// message loop until it is ready.
    ///
    /// This requires an installed WebView2 runtime and must be called on a UI
    /// thread with a message loop. The calling thread is initialized as a COM
    /// single-threaded apartment (STA) if it is not already; an error is
    /// returned if the thread is already a multi-threaded apartment (MTA).
    pub fn new() -> Result<Self> {
        init_com()?;
        let slot = pump::slot();
        create_environment(pump::slot_handler(&slot))?;
        pump::wait(&slot)
    }

    /// Creates a WebView2 environment configured by `options`, pumping the
    /// calling thread's message loop until it is ready.
    ///
    /// This requires an installed WebView2 runtime and must be called on a UI
    /// thread with a message loop.
    pub fn with_options(options: &EnvironmentOptions) -> Result<Self> {
        init_com()?;
        let slot = pump::slot();
        options.create_environment(pump::slot_handler(&slot))?;
        pump::wait(&slot)
    }

    /// Creates a [`Controller`] that hosts a WebView2 browser in the given
    /// window, pumping the calling thread's message loop until it is ready.
    ///
    /// This requires an installed WebView2 runtime and must be called on a UI
    /// thread with a message loop.
    pub fn create_controller(&self, window: &windows_window::Window) -> Result<Controller> {
        // SAFETY: `window` owns a live window handle for as long as the borrow
        // lasts.
        unsafe { self.create_controller_for_hwnd(window.hwnd()) }
    }

    /// Creates a [`Controller`] that hosts a WebView2 browser in the given raw
    /// window handle, pumping the calling thread's message loop until it is
    /// ready.
    ///
    /// This is the escape hatch for callers that own an `HWND` from a source
    /// other than [`windows_window`]; most callers should prefer the safe
    /// [`create_controller`](Self::create_controller).
    ///
    /// # Safety
    ///
    /// `parent` must be a valid window handle that outlives the controller.
    pub unsafe fn create_controller_for_hwnd(
        &self,
        parent: *mut core::ffi::c_void,
    ) -> Result<Controller> {
        let slot = pump::slot();
        unsafe { self.create_controller_async(parent, pump::slot_handler(&slot))? };
        pump::wait(&slot)
    }

    unsafe fn create_controller_async<F: FnOnce(Result<Controller>) + 'static>(
        &self,
        parent: HWND,
        handler: F,
    ) -> Result<()> {
        let handler = handler::ControllerCompleted::create(handler);
        unsafe { self.0.CreateCoreWebView2Controller(parent, &handler) }.ok()
    }

    /// Creates a [`Controller`] configured by `options` (profile, private mode,
    /// background colour) that hosts a WebView2 browser in the given window,
    /// pumping the calling thread's message loop until it is ready.
    pub fn create_controller_with_options(
        &self,
        window: &windows_window::Window,
        options: &ControllerOptions,
    ) -> Result<Controller> {
        // SAFETY: `window` owns a live window handle for as long as the borrow
        // lasts.
        unsafe { self.create_controller_with_options_for_hwnd(window.hwnd(), options) }
    }

    /// Creates a [`Controller`] configured by `options` in the given raw window
    /// handle. Most callers should prefer the safe
    /// [`create_controller_with_options`](Self::create_controller_with_options).
    ///
    /// # Safety
    ///
    /// `parent` must be a valid window handle that outlives the controller.
    pub unsafe fn create_controller_with_options_for_hwnd(
        &self,
        parent: *mut core::ffi::c_void,
        options: &ControllerOptions,
    ) -> Result<Controller> {
        let slot = pump::slot();
        options.create_controller(&self.0, parent, pump::slot_handler(&slot))?;
        pump::wait(&slot)
    }
}

fn create_environment<F: FnOnce(Result<Environment>) + 'static>(handler: F) -> Result<()> {
    let handler = handler::EnvironmentCompleted::create(handler);
    unsafe { CreateCoreWebView2Environment(Interface::as_raw(&handler)).ok() }
}

fn init_com() -> Result<()> {
    // SAFETY: FFI call into ole32; the reserved argument must be null.
    let hr = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
    if hr == RPC_E_CHANGED_MODE {
        return Err(Error::new(
            RPC_E_CHANGED_MODE,
            "windows_webview::Environment requires the calling thread to be a COM \
             single-threaded apartment (STA), but it is already initialized as a \
             multi-threaded apartment (MTA). Create the environment from a UI thread \
             that has not called CoInitializeEx(COINIT_MULTITHREADED).",
        ));
    }
    hr.ok()
}

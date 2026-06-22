use super::*;

/// The WebView2 environment. Owns the user data folder and browser process and
/// creates [`Controller`] instances that host the browser in a window.
pub struct Environment(pub(crate) ICoreWebView2Environment);

impl Environment {
    /// Creates the default WebView2 environment, pumping the calling thread's
    /// message loop until it is ready.
    ///
    /// This requires an installed WebView2 runtime and must be called on a UI
    /// thread with a message loop.
    pub fn new() -> Result<Environment> {
        let slot = pump::slot();
        create_environment(pump::slot_handler(&slot))?;
        pump::wait(&slot)
    }

    /// Creates a WebView2 environment configured by `options`, pumping the
    /// calling thread's message loop until it is ready.
    ///
    /// This requires an installed WebView2 runtime and must be called on a UI
    /// thread with a message loop.
    pub fn with_options(options: &EnvironmentOptions) -> Result<Environment> {
        let slot = pump::slot();
        options.create_environment(pump::slot_handler(&slot))?;
        pump::wait(&slot)
    }

    /// Creates a [`Controller`] that hosts a WebView2 browser in the given
    /// parent window, pumping the calling thread's message loop until it is
    /// ready.
    ///
    /// # Safety
    ///
    /// `parent` must be a valid window handle that outlives the controller.
    pub unsafe fn create_controller(&self, parent: HWND) -> Result<Controller> {
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
        unsafe { self.0.CreateCoreWebView2Controller(parent, &handler) }
    }
}

fn create_environment<F: FnOnce(Result<Environment>) + 'static>(handler: F) -> Result<()> {
    let handler = handler::EnvironmentCompleted::create(handler);
    unsafe { CreateCoreWebView2Environment(Interface::as_raw(&handler)).ok() }
}

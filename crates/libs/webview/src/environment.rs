use super::*;

/// The WebView2 environment. Owns the user data folder and browser process and
/// creates [`Controller`] instances that host the browser in a window.
pub struct Environment(pub(crate) ICoreWebView2Environment);

impl Environment {
    /// Asynchronously creates a [`Controller`] that hosts a WebView2 browser in
    /// the given parent window. The `handler` closure runs on the UI thread once
    /// the controller is ready.
    ///
    /// # Safety
    ///
    /// `parent` must be a valid window handle that outlives the controller.
    pub unsafe fn create_controller<F: FnOnce(Result<Controller>) + 'static>(
        &self,
        parent: HWND,
        handler: F,
    ) -> Result<()> {
        let handler = handler::ControllerCompleted::create(handler);
        unsafe { self.0.CreateCoreWebView2Controller(parent, &handler) }
    }
}

/// Asynchronously creates the default WebView2 [`Environment`]. The `handler`
/// closure runs on the UI thread once the environment is ready.
///
/// This requires a running message loop and an installed WebView2 runtime.
pub fn create_environment<F: FnOnce(Result<Environment>) + 'static>(handler: F) -> Result<()> {
    let handler = handler::EnvironmentCompleted::create(handler);
    unsafe { CreateCoreWebView2Environment(Interface::as_raw(&handler)).ok() }
}

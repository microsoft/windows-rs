use super::*;

/// The WebView2 environment. Owns the user data folder and browser process and
/// creates [`Controller`] instances that host the browser in a window.
pub struct Environment(pub(crate) ICoreWebView2Environment);

impl Environment {
    /// Starts creating the default WebView2 environment.
    ///
    /// The calling thread must be a COM STA with a running message loop. `handler` runs at most
    /// once on that thread after creation completes.
    pub fn create<F: FnOnce(Result<Self>) + 'static>(handler: F) -> Result<()> {
        create_environment(handler)
    }

    /// Starts creating a WebView2 environment configured by `options`.
    ///
    /// The calling thread must be a COM STA with a running message loop. `handler` runs at most
    /// once on that thread after creation completes.
    pub fn create_with_options<F: FnOnce(Result<Self>) + 'static>(
        options: &EnvironmentOptions,
        handler: F,
    ) -> Result<()> {
        options.create_environment(handler)
    }

    /// Starts creating a [`Controller`] hosted in a raw window handle.
    ///
    /// `parent` must be a valid window handle owned by the calling thread. Keep it valid through
    /// completion and for the lifetime of the controller.
    // HWND is an opaque handle; Rust does not dereference it.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn create_controller_for_hwnd<F: FnOnce(Result<Controller>) + 'static>(
        &self,
        parent: *mut core::ffi::c_void,
        handler: F,
    ) -> Result<()> {
        validate_parent(parent)?;
        let handler = handler::ControllerCompleted::create(handler);
        unsafe { self.0.CreateCoreWebView2Controller(parent, &handler) }.ok()
    }

    /// Starts creating an option-configured [`Controller`] hosted in a raw window handle.
    ///
    /// `parent` must be a valid window handle owned by the calling thread. Keep it valid through
    /// completion and for the lifetime of the controller.
    pub fn create_controller_with_options_for_hwnd<F: FnOnce(Result<Controller>) + 'static>(
        &self,
        parent: *mut core::ffi::c_void,
        options: &ControllerOptions,
        handler: F,
    ) -> Result<()> {
        validate_parent(parent)?;
        options.create_controller(&self.0, parent, handler)
    }
}

fn create_environment<F: FnOnce(Result<Environment>) + 'static>(handler: F) -> Result<()> {
    let handler = handler::EnvironmentCompleted::create(handler);
    unsafe { CreateCoreWebView2Environment(Interface::as_raw(&handler)).ok() }
}

pub(crate) fn validate_parent(parent: HWND) -> Result<()> {
    if parent.is_null() {
        Err(Error::new(
            E_INVALIDARG,
            "WebView2 parent HWND must not be null",
        ))
    } else {
        Ok(())
    }
}

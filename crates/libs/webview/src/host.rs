use super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

type HostCompletion = Rc<RefCell<Option<Box<dyn FnOnce(Result<WebViewHost>)>>>>;

pub(crate) struct HostState {
    registrations: RefCell<Vec<EventRegistration>>,
    pub(crate) controller: Rc<Controller>,
    close_state: RefCell<CloseState>,
}

enum CloseState {
    Open,
    Closing,
    Closed(Option<Error>),
}

impl HostState {
    pub(crate) fn close(&self) -> Result<()> {
        {
            let mut state = self.close_state.borrow_mut();
            match &*state {
                CloseState::Open => *state = CloseState::Closing,
                CloseState::Closing => {
                    return Err(Error::new(
                        E_PENDING,
                        "WebView2 controller close is already in progress",
                    ));
                }
                CloseState::Closed(Some(error)) => return Err(error.clone()),
                CloseState::Closed(None) => return Ok(()),
            }
        }

        let registrations = std::mem::take(&mut *self.registrations.borrow_mut());
        drop(registrations);
        let result = self.controller.close();
        *self.close_state.borrow_mut() = CloseState::Closed(result.as_ref().err().cloned());
        result
    }

    #[cfg(feature = "system")]
    pub(crate) fn is_closing(&self) -> bool {
        matches!(&*self.close_state.borrow(), CloseState::Closing)
    }

    fn is_closing_or_closed(&self) -> bool {
        !matches!(&*self.close_state.borrow(), CloseState::Open)
    }
}

impl Drop for HostState {
    fn drop(&mut self) {
        _ = self.close();
    }
}

/// A framework-neutral WebView2 host.
///
/// This type owns the environment, controller, browser, and retained event registrations, but not
/// the parent window or its message loop. The parent HWND must remain valid until this host drops.
pub struct WebViewHost {
    webview: WebView,
    pub(crate) state: Rc<HostState>,
    _environment: Rc<Environment>,
}

impl WebViewHost {
    /// Begins configuring a WebView2 host.
    pub fn builder() -> WebViewHostBuilder {
        WebViewHostBuilder::new()
    }

    /// Returns the hosted browser.
    pub fn webview(&self) -> &WebView {
        &self.webview
    }

    /// Returns the browser controller.
    pub fn controller(&self) -> &Controller {
        &self.state.controller
    }

    /// Retains an event registration for the lifetime of this host.
    pub fn retain(&self, registration: EventRegistration) {
        if self.state.is_closing_or_closed() {
            drop(registration);
        } else {
            self.state.registrations.borrow_mut().push(registration);
        }
    }

    /// Retains event registrations for the lifetime of this host.
    pub fn retain_all(&self, registrations: impl IntoIterator<Item = EventRegistration>) {
        for registration in registrations {
            self.retain(registration);
        }
    }

    /// Closes the browser controller.
    pub fn close(&self) -> Result<()> {
        self.state.close()
    }
}

impl Drop for WebViewHost {
    fn drop(&mut self) {
        _ = self.state.close();
    }
}

/// Configures framework-neutral WebView2 creation.
#[derive(Default)]
pub struct WebViewHostBuilder {
    environment_options: Option<EnvironmentOptions>,
    controller_options: Option<ControllerOptions>,
}

impl WebViewHostBuilder {
    /// Creates a builder using WebView2 defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets options for WebView2 environment creation.
    pub fn environment_options(mut self, options: EnvironmentOptions) -> Self {
        self.environment_options = Some(options);
        self
    }

    /// Sets options for WebView2 controller creation.
    pub fn controller_options(mut self, options: ControllerOptions) -> Self {
        self.controller_options = Some(options);
        self
    }

    /// Starts creating a WebView2 host in a raw parent HWND.
    ///
    /// `parent` must be a valid window owned by the calling UI STA. The caller owns window
    /// lifetime, layout, position notifications, and message-loop integration. `completed` runs at
    /// most once on the calling thread.
    // HWND is an opaque handle; Rust does not dereference it.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn create_for_hwnd<F: FnOnce(Result<WebViewHost>) + 'static>(
        self,
        parent: *mut core::ffi::c_void,
        completed: F,
    ) -> Result<()> {
        self.create_for_hwnd_with_cancel(parent, Rc::new(Cell::new(false)), completed)
    }

    pub(crate) fn create_for_hwnd_with_cancel<F: FnOnce(Result<WebViewHost>) + 'static>(
        self,
        parent: *mut core::ffi::c_void,
        cancelled: Rc<Cell<bool>>,
        completed: F,
    ) -> Result<()> {
        environment::validate_parent(parent)?;
        let completion: HostCompletion = Rc::new(RefCell::new(Some(Box::new(completed))));
        let controller_options = self.controller_options;
        let environment_cancelled = Rc::clone(&cancelled);
        let environment_completion = Rc::clone(&completion);
        let environment_created = move |result: Result<Environment>| {
            if environment_cancelled.get() {
                return complete(
                    &environment_completion,
                    Err(Error::new(E_ABORT, "WebView2 host creation cancelled")),
                );
            }
            let environment = match result {
                Ok(environment) => Rc::new(environment),
                Err(error) => return complete(&environment_completion, Err(error)),
            };
            let retained_environment = Rc::clone(&environment);
            let controller_cancelled = Rc::clone(&environment_cancelled);
            let controller_completion = Rc::clone(&environment_completion);
            let controller_created = move |result: Result<Controller>| {
                let controller = match result {
                    Ok(controller) => Rc::new(controller),
                    Err(error) => return complete(&controller_completion, Err(error)),
                };
                if controller_cancelled.get() {
                    _ = controller.close();
                    return complete(
                        &controller_completion,
                        Err(Error::new(E_ABORT, "WebView2 host creation cancelled")),
                    );
                }
                let webview = match controller.webview() {
                    Ok(webview) => webview,
                    Err(error) => {
                        _ = controller.close();
                        return complete(&controller_completion, Err(error));
                    }
                };
                if controller_cancelled.get() {
                    _ = controller.close();
                    return complete(
                        &controller_completion,
                        Err(Error::new(E_ABORT, "WebView2 host creation cancelled")),
                    );
                }
                complete(
                    &controller_completion,
                    Ok(WebViewHost {
                        webview,
                        state: Rc::new(HostState {
                            registrations: RefCell::new(Vec::new()),
                            controller,
                            close_state: RefCell::new(CloseState::Open),
                        }),
                        _environment: retained_environment,
                    }),
                );
            };

            let result = match controller_options.as_ref() {
                Some(options) => environment.create_controller_with_options_for_hwnd(
                    parent,
                    options,
                    controller_created,
                ),
                None => environment.create_controller_for_hwnd(parent, controller_created),
            };
            if let Err(error) = result {
                complete(&environment_completion, Err(error));
            }
        };

        match self.environment_options.as_ref() {
            Some(options) => Environment::create_with_options(options, environment_created),
            None => Environment::create(environment_created),
        }
    }
}

fn complete(completion: &HostCompletion, result: Result<WebViewHost>) {
    let completion = completion.borrow_mut().take();
    if let Some(completion) = completion {
        completion(result);
    }
}

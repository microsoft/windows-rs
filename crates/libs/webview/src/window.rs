use super::*;
use crate::host::HostState;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

type HostSlot = Rc<RefCell<Option<Weak<HostState>>>>;
type WindowCompletion = Rc<RefCell<Option<Box<dyn FnOnce(Result<WebViewWindow>, bool)>>>>;

const WM_CLOSE: u32 = 0x0010;

/// A WebView2 browser hosted by a [`windows_window::Window`].
///
/// This convenience adapter owns the parent window and a framework-neutral [`WebViewHost`]. Use
/// [`WebViewHostBuilder`] directly when another framework owns the window and message loop.
pub struct WebViewWindow {
    host: WebViewHost,
    window: Rc<windows_window::Window>,
    closed: Rc<Cell<bool>>,
    close_requested: Rc<Cell<bool>>,
}

impl WebViewWindow {
    /// Begins configuring a system-hosted WebView2 window.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(title: &str) -> WebViewWindowBuilder {
        WebViewWindowBuilder {
            window: windows_window::Window::new(title),
            environment_options: None,
            controller_options: None,
            close: None,
        }
    }

    /// Returns the framework-neutral WebView2 host.
    pub fn host(&self) -> &WebViewHost {
        &self.host
    }

    /// Returns the hosted browser.
    pub fn webview(&self) -> &WebView {
        self.host.webview()
    }

    /// Returns the browser controller.
    pub fn controller(&self) -> &Controller {
        self.host.controller()
    }

    /// Returns the parent window.
    pub fn window(&self) -> &windows_window::Window {
        &self.window
    }

    /// Retains an event registration for the lifetime of this host.
    pub fn retain(&self, registration: EventRegistration) {
        self.host.retain(registration);
    }

    /// Retains event registrations for the lifetime of this host.
    pub fn retain_all(&self, registrations: impl IntoIterator<Item = EventRegistration>) {
        self.host.retain_all(registrations);
    }

    /// Closes the browser while its parent window is still alive.
    pub fn close(&self) -> Result<()> {
        self.closed.set(true);
        let result = self.host.close();
        if self.close_requested.replace(false) {
            self.window.close();
        }
        result
    }
}

impl Drop for WebViewWindow {
    fn drop(&mut self) {
        _ = self.close();
    }
}

/// Configures a system-hosted [`WebViewWindow`].
pub struct WebViewWindowBuilder {
    window: windows_window::WindowBuilder,
    environment_options: Option<EnvironmentOptions>,
    controller_options: Option<ControllerOptions>,
    close: Option<Box<dyn FnMut()>>,
}

impl WebViewWindowBuilder {
    /// Sets the initial screen position of the parent window, in pixels.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.window = self.window.position(x, y);
        self
    }

    /// Sets the initial outer window size, including non-client borders, in pixels.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.window = self.window.size(width, height);
        self
    }

    /// Sets the initial client-area size, excluding non-client borders, in pixels.
    pub fn client_size(mut self, width: i32, height: i32) -> Self {
        self.window = self.window.client_size(width, height);
        self
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

    /// Controls whether closing this window exits the `windows-window` message loop.
    ///
    /// The default is `true`. Set this to `false` when application lifetime is controlled
    /// separately, then call [`windows_window::quit`] to exit.
    pub fn quit_on_close(mut self, value: bool) -> Self {
        self.window = self.window.quit_on_close(value);
        self
    }

    /// Sets a handler called during normal close processing, before the parent window is
    /// destroyed.
    ///
    /// Dropping a live `WebViewWindow` destroys its parent directly and does not call this handler.
    pub fn on_close<F: FnMut() + 'static>(mut self, handler: F) -> Self {
        self.close = Some(Box::new(handler));
        self
    }

    /// Starts creating the window and WebView2 host.
    ///
    /// The calling thread must be a COM STA. The application must run its UI message loop after
    /// this method returns. `completed` runs at most once on that thread. The callback must retain
    /// a successful `WebViewWindow`; dropping it closes the browser and destroys its parent.
    pub fn create<F: FnOnce(Result<WebViewWindow>) + 'static>(self, completed: F) -> Result<()> {
        self.create_with_status(move |result, _cancelled| completed(result))
    }

    fn create_with_status<F: FnOnce(Result<WebViewWindow>, bool) + 'static>(
        self,
        completed: F,
    ) -> Result<()> {
        let completion: WindowCompletion = Rc::new(RefCell::new(Some(Box::new(completed))));
        let host_slot = HostSlot::default();
        let closed = Rc::new(Cell::new(false));
        let pending = Rc::new(Cell::new(true));
        let cancelled = Rc::new(Cell::new(false));
        let close_requested = Rc::new(Cell::new(false));

        let message_pending = Rc::clone(&pending);
        let message_cancelled = Rc::clone(&cancelled);
        let message_close_requested = Rc::clone(&close_requested);
        let message_host_slot = Rc::clone(&host_slot);
        let resize_host = Rc::clone(&host_slot);
        let resize_closed = Rc::clone(&closed);
        let move_host = Rc::clone(&host_slot);
        let move_closed = Rc::clone(&closed);
        let close_host_slot = Rc::clone(&host_slot);
        let close_closed = Rc::clone(&closed);
        let mut close = self.close;
        let window = Rc::new(
            self.window
                .on_message(move |_hwnd, message, _wparam, _lparam| {
                    if message == WM_CLOSE {
                        if message_pending.get() {
                            message_cancelled.set(true);
                            message_close_requested.set(true);
                            return Some(0);
                        }
                        if message_host_slot
                            .borrow()
                            .as_ref()
                            .and_then(Weak::upgrade)
                            .is_some_and(|host| host.is_closing())
                        {
                            message_close_requested.set(true);
                            return Some(0);
                        }
                    }
                    None
                })
                .on_resize(move |width, height| {
                    if !resize_closed.get()
                        && let Some(host) = resize_host.borrow().as_ref().and_then(Weak::upgrade)
                    {
                        _ = host.controller.set_bounds(0, 0, width, height);
                    }
                })
                .on_move(move || {
                    if !move_closed.get()
                        && let Some(host) = move_host.borrow().as_ref().and_then(Weak::upgrade)
                    {
                        _ = host.controller.notify_parent_window_position_changed();
                    }
                })
                .on_close(move || {
                    close_closed.set(true);
                    if let Some(host) = close_host_slot.borrow().as_ref().and_then(Weak::upgrade) {
                        _ = host.close();
                    }
                    if let Some(close) = close.as_mut() {
                        close();
                    }
                })
                .create()?,
        );

        let mut builder = WebViewHost::builder();
        if let Some(options) = self.environment_options {
            builder = builder.environment_options(options);
        }
        if let Some(options) = self.controller_options {
            builder = builder.controller_options(options);
        }

        let create_window = Rc::clone(&window);
        let create_closed = Rc::clone(&closed);
        let create_pending = Rc::clone(&pending);
        let create_close_requested = Rc::clone(&close_requested);
        let create_host_slot = Rc::clone(&host_slot);
        let create_completion = Rc::clone(&completion);
        builder.create_for_hwnd_with_cancel(window.hwnd(), cancelled, move |result| {
            create_pending.set(false);
            if create_close_requested.get() {
                if let Ok(host) = result {
                    _ = host.close();
                }
                create_window.close();
                complete(
                    &create_completion,
                    Err(Error::new(
                        E_ABORT,
                        "WebView2 parent window closed during controller creation",
                    )),
                    true,
                );
                return;
            }
            let host = match result {
                Ok(host) => host,
                Err(error) => {
                    create_window.close();
                    return complete(&create_completion, Err(error), false);
                }
            };
            if create_closed.get() {
                _ = host.close();
                return complete(
                    &create_completion,
                    Err(Error::new(
                        E_ABORT,
                        "WebView2 parent window closed during controller creation",
                    )),
                    true,
                );
            }
            let (width, height) = create_window.client_size();
            if let Err(error) = host.controller().set_bounds(0, 0, width, height) {
                _ = host.close();
                create_window.close();
                return complete(&create_completion, Err(error), false);
            }
            if create_closed.get() {
                _ = host.close();
                return complete(
                    &create_completion,
                    Err(Error::new(
                        E_ABORT,
                        "WebView2 parent window closed during controller creation",
                    )),
                    true,
                );
            }
            *create_host_slot.borrow_mut() = Some(Rc::downgrade(&host.state));
            complete(
                &create_completion,
                Ok(WebViewWindow {
                    host,
                    window: create_window,
                    closed: create_closed,
                    close_requested: create_close_requested,
                }),
                false,
            );
        })
    }

    /// Creates a WebView2 host and runs its `windows-window` message loop.
    ///
    /// `setup` runs after WebView2 creation. Use [`create`](Self::create) instead when the
    /// application owns its message loop or must remain active after this window closes.
    pub fn run<F>(mut self, setup: F) -> Result<()>
    where
        F: FnOnce(&WebViewWindow) -> Result<()> + 'static,
    {
        self.window = self.window.quit_on_close(true);
        let _apartment = init_sta()?;
        let host = Rc::new(RefCell::new(None::<WebViewWindow>));
        let failure = Rc::new(RefCell::new(None::<Error>));
        let callback_host = Rc::clone(&host);
        let callback_failure = Rc::clone(&failure);
        self.create_with_status(move |result, cancelled| {
            if cancelled {
                windows_window::quit();
                return;
            }
            let value = match result {
                Ok(value) => value,
                Err(error) => return stop(&callback_failure, error),
            };
            if let Err(error) = setup(&value) {
                return stop(&callback_failure, error);
            }
            *callback_host.borrow_mut() = Some(value);
        })?;

        windows_window::run();
        let host = host.borrow_mut().take();
        let close_result = host.as_ref().map_or(Ok(()), WebViewWindow::close);
        drop(host);
        if let Some(error) = failure.borrow_mut().take() {
            return Err(error);
        }
        close_result
    }
}

fn complete(completion: &WindowCompletion, result: Result<WebViewWindow>, cancelled: bool) {
    let completion = completion.borrow_mut().take();
    if let Some(completion) = completion {
        completion(result, cancelled);
    }
}

fn stop(failure: &RefCell<Option<Error>>, error: Error) {
    let mut failure = failure.borrow_mut();
    if failure.is_none() {
        *failure = Some(error);
    }
    windows_window::quit();
}

use super::*;
use std::cell::{Cell, RefCell};

/// Adapts a Rust closure to the `ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler`
/// COM interface.
pub(crate) struct EnvironmentCompleted;

impl EnvironmentCompleted {
    pub(crate) fn create<F: FnOnce(Result<Environment>) + 'static>(
        handler: F,
    ) -> ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler::new(
            move |errorcode: HRESULT, result: Ref<ICoreWebView2Environment>| {
                let outcome = errorcode
                    .ok()
                    .and_then(|()| Ok(Environment(result.ok()?.clone())));

                if let Some(handler) = handler.take() {
                    handler(outcome);
                }
            },
        )
    }
}

/// Adapts a Rust closure to the `ICoreWebView2CreateCoreWebView2ControllerCompletedHandler`
/// COM interface.
pub(crate) struct ControllerCompleted;

impl ControllerCompleted {
    pub(crate) fn create<F: FnOnce(Result<Controller>) + 'static>(
        handler: F,
    ) -> ICoreWebView2CreateCoreWebView2ControllerCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2CreateCoreWebView2ControllerCompletedHandler::new(
            move |errorcode: HRESULT, result: Ref<ICoreWebView2Controller>| {
                let outcome = errorcode
                    .ok()
                    .and_then(|()| Ok(Controller(result.ok()?.clone())));

                if let Some(handler) = handler.take() {
                    handler(outcome);
                }
            },
        )
    }
}

/// Adapts a Rust closure to the `ICoreWebView2ExecuteScriptCompletedHandler`
/// COM interface.
pub(crate) struct ExecuteScriptCompleted;

impl ExecuteScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> ICoreWebView2ExecuteScriptCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2ExecuteScriptCompletedHandler::new(
            move |errorcode: HRESULT, result: LPCWSTR| {
                let outcome = errorcode.ok().map(|()| unsafe { string::decode(result) });

                if let Some(handler) = handler.take() {
                    handler(outcome);
                }
            },
        )
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler` COM
/// interface. The completion result is the registered script's id.
pub(crate) struct AddScriptCompleted;

impl AddScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler::new(
            move |errorcode: HRESULT, id: LPCWSTR| {
                let outcome = errorcode.ok().map(|()| unsafe { string::decode(id) });

                if let Some(handler) = handler.take() {
                    handler(outcome);
                }
            },
        )
    }
}

/// Adapts a Rust closure to the `ICoreWebView2GetCookiesCompletedHandler` COM
/// interface. The completion result is the retrieved cookies, converted from the
/// COM cookie list so the list type never reaches the public surface.
pub(crate) struct GetCookiesCompleted;

impl GetCookiesCompleted {
    pub(crate) fn create<F: FnOnce(Result<Vec<Cookie>>) + 'static>(
        handler: F,
    ) -> ICoreWebView2GetCookiesCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2GetCookiesCompletedHandler::new(
            move |errorcode: HRESULT, result: Ref<ICoreWebView2CookieList>| {
                let outcome = errorcode.ok().and_then(|()| cookie::collect(result.ok()?));

                if let Some(handler) = handler.take() {
                    handler(outcome);
                }
            },
        )
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2ClearBrowsingDataCompletedHandler` COM interface.
pub(crate) struct ClearBrowsingDataCompleted;

impl ClearBrowsingDataCompleted {
    pub(crate) fn create<F: FnOnce(Result<()>) + 'static>(
        handler: F,
    ) -> ICoreWebView2ClearBrowsingDataCompletedHandler {
        let handler = Cell::new(Some(handler));
        ICoreWebView2ClearBrowsingDataCompletedHandler::new(move |errorcode: HRESULT| {
            if let Some(handler) = handler.take() {
                handler(errorcode.ok());
            }
        })
    }
}

/// Defines an event-handler adapter that forwards a WebView2 event to a Rust
/// `FnMut` closure, bridging it onto the bindgen-generated `IXHandler::new`
/// closure constructor. The default form wraps the event's args interface with
/// an `ICoreWebView2` sender; passing a sender type handles events raised on a
/// different sender (such as the controller); the `sender` form (for events with
/// no args interface) wraps the sender instead.
macro_rules! event_handler {
    ($name:ident, $handler:ident, $args:ident => $wrapper:ident) => {
        event_handler!($name, $handler, ICoreWebView2, $args => $wrapper);
    };
    ($name:ident, $handler:ident, $sender:ident, $args:ident => $wrapper:ident) => {
        pub(crate) struct $name;

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> $handler {
                let handler = RefCell::new(handler);
                $handler::new(move |_sender: Ref<$sender>, args: Ref<$args>| {
                    if let Some(args) = args.as_ref() {
                        (*handler.borrow_mut())($wrapper(args.clone()));
                    }
                })
            }
        }
    };
    ($name:ident, $handler:ident, sender $sender:ident => $wrapper:ident) => {
        pub(crate) struct $name;

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> $handler {
                let handler = RefCell::new(handler);
                $handler::new(move |sender: Ref<$sender>, _args: Ref<IUnknown>| {
                    if let Some(sender) = sender.as_ref() {
                        (*handler.borrow_mut())($wrapper(sender.clone()));
                    }
                })
            }
        }
    };
}

event_handler!(NavigationCompleted, ICoreWebView2NavigationCompletedEventHandler, ICoreWebView2NavigationCompletedEventArgs => NavigationCompletedArgs);
event_handler!(NavigationStarting, ICoreWebView2NavigationStartingEventHandler, ICoreWebView2NavigationStartingEventArgs => NavigationStartingArgs);
event_handler!(WebMessageReceived, ICoreWebView2WebMessageReceivedEventHandler, ICoreWebView2WebMessageReceivedEventArgs => WebMessageReceivedArgs);
event_handler!(ContentLoading, ICoreWebView2ContentLoadingEventHandler, ICoreWebView2ContentLoadingEventArgs => ContentLoadingArgs);
event_handler!(NewWindowRequested, ICoreWebView2NewWindowRequestedEventHandler, ICoreWebView2NewWindowRequestedEventArgs => NewWindowRequestedArgs);
event_handler!(PermissionRequested, ICoreWebView2PermissionRequestedEventHandler, ICoreWebView2PermissionRequestedEventArgs => PermissionRequestedArgs);
event_handler!(DownloadStarting, ICoreWebView2DownloadStartingEventHandler, ICoreWebView2DownloadStartingEventArgs => DownloadStartingArgs);
event_handler!(DownloadStateChanged, ICoreWebView2StateChangedEventHandler, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(BytesReceivedChanged, ICoreWebView2BytesReceivedChangedEventHandler, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(ProcessFailed, ICoreWebView2ProcessFailedEventHandler, ICoreWebView2ProcessFailedEventArgs => ProcessFailedArgs);
event_handler!(MoveFocusRequested, ICoreWebView2MoveFocusRequestedEventHandler, ICoreWebView2Controller, ICoreWebView2MoveFocusRequestedEventArgs => MoveFocusRequestedArgs);
event_handler!(AcceleratorKeyPressed, ICoreWebView2AcceleratorKeyPressedEventHandler, ICoreWebView2Controller, ICoreWebView2AcceleratorKeyPressedEventArgs => AcceleratorKeyPressedArgs);

/// Adapts a Rust closure to the `ICoreWebView2FocusChangedEventHandler` COM
/// interface, shared by the controller's got-focus and lost-focus events. The
/// event carries no args, so the closure takes none.
pub(crate) struct FocusChanged;

impl FocusChanged {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> ICoreWebView2FocusChangedEventHandler {
        let handler = RefCell::new(handler);
        ICoreWebView2FocusChangedEventHandler::new(
            move |_sender: Ref<ICoreWebView2Controller>, _args: Ref<IUnknown>| {
                (*handler.borrow_mut())();
            },
        )
    }
}

/// Adapts a Rust closure to the `ICoreWebView2DocumentTitleChangedEventHandler`
/// COM interface. The event carries no args, so the new document title is read
/// from the sender and handed to the closure.
pub(crate) struct DocumentTitleChanged;

impl DocumentTitleChanged {
    pub(crate) fn create<F: FnMut(String) + 'static>(
        handler: F,
    ) -> ICoreWebView2DocumentTitleChangedEventHandler {
        let handler = RefCell::new(handler);
        ICoreWebView2DocumentTitleChangedEventHandler::new(
            move |sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>| {
                let title = unsafe {
                    string::take_result(sender.ok().and_then(|sender| sender.DocumentTitle()))
                };
                (*handler.borrow_mut())(title);
            },
        )
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2ContainsFullScreenElementChangedEventHandler` COM interface.
/// The event carries no args, so the new fullscreen state is read from the
/// sender and handed to the closure.
pub(crate) struct ContainsFullScreenElementChanged;

impl ContainsFullScreenElementChanged {
    pub(crate) fn create<F: FnMut(bool) + 'static>(
        handler: F,
    ) -> ICoreWebView2ContainsFullScreenElementChangedEventHandler {
        let handler = RefCell::new(handler);
        ICoreWebView2ContainsFullScreenElementChangedEventHandler::new(
            move |sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>| {
                let contains = sender
                    .ok()
                    .and_then(|sender| unsafe { sender.ContainsFullScreenElement() })
                    .is_ok_and(|value| value.as_bool());
                (*handler.borrow_mut())(contains);
            },
        )
    }
}

/// Adapts a Rust closure to the `ICoreWebView2WindowCloseRequestedEventHandler`
/// COM interface. The event carries no args, so the closure takes none.
pub(crate) struct WindowCloseRequested;

impl WindowCloseRequested {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> ICoreWebView2WindowCloseRequestedEventHandler {
        let handler = RefCell::new(handler);
        ICoreWebView2WindowCloseRequestedEventHandler::new(
            move |_sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>| {
                (*handler.borrow_mut())();
            },
        )
    }
}

/// Defines an event-subscription method on a wrapper that holds a WebView2
/// interface in its first field. It registers the generated [`crate::handler`]
/// adapter, then returns an [`EventRevoker`] that removes it on drop.
macro_rules! subscription {
    (
        $(#[$doc:meta])*
        $method:ident($arg:ty) => $handler:ident, $event:ident
    ) => {
        $(#[$doc])*
        pub fn $method<F: FnMut($arg) + 'static>(&self, handler: F) -> Result<EventRevoker> {
            let handler = crate::handler::$handler::create(handler);
            unsafe { self.0.$event(&handler) }
        }
    };
}

pub(crate) use subscription;

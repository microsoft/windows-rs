use super::*;
use std::cell::{Cell, RefCell};
use windows_core::implement_decl;

/// Adapts a Rust closure to the `ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler`
/// COM interface.
pub(crate) struct EnvironmentCompleted(Cell<Option<Box<dyn FnOnce(Result<Environment>)>>>);

implement_decl! {
    impl EnvironmentCompleted as pub(crate) EnvironmentCompleted_Impl:
        [ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler]
}

impl EnvironmentCompleted {
    pub(crate) fn create<F: FnOnce(Result<Environment>) + 'static>(
        handler: F,
    ) -> ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl for EnvironmentCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2Environment>) -> Result<()> {
        let outcome = errorcode
            .ok()
            .and_then(|()| Ok(Environment(result.ok()?.clone())));

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2CreateCoreWebView2ControllerCompletedHandler`
/// COM interface.
pub(crate) struct ControllerCompleted(Cell<Option<Box<dyn FnOnce(Result<Controller>)>>>);

implement_decl! {
    impl ControllerCompleted as pub(crate) ControllerCompleted_Impl:
        [ICoreWebView2CreateCoreWebView2ControllerCompletedHandler]
}

impl ControllerCompleted {
    pub(crate) fn create<F: FnOnce(Result<Controller>) + 'static>(
        handler: F,
    ) -> ICoreWebView2CreateCoreWebView2ControllerCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl for ControllerCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2Controller>) -> Result<()> {
        let outcome = errorcode
            .ok()
            .and_then(|()| Ok(Controller(result.ok()?.clone())));

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2ExecuteScriptCompletedHandler`
/// COM interface.
pub(crate) struct ExecuteScriptCompleted(Cell<Option<Box<dyn FnOnce(Result<String>)>>>);

implement_decl! {
    impl ExecuteScriptCompleted as pub(crate) ExecuteScriptCompleted_Impl:
        [ICoreWebView2ExecuteScriptCompletedHandler]
}

impl ExecuteScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> ICoreWebView2ExecuteScriptCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2ExecuteScriptCompletedHandler_Impl for ExecuteScriptCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: &PCWSTR) -> Result<()> {
        let outcome = errorcode
            .ok()
            .map(|()| unsafe { result.to_string().unwrap_or_default() });

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler` COM
/// interface. The completion result is the registered script's id.
pub(crate) struct AddScriptCompleted(Cell<Option<Box<dyn FnOnce(Result<String>)>>>);

implement_decl! {
    impl AddScriptCompleted as pub(crate) AddScriptCompleted_Impl:
        [ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler]
}

impl AddScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl
    for AddScriptCompleted_Impl
{
    fn Invoke(&self, errorcode: HRESULT, id: &PCWSTR) -> Result<()> {
        let outcome = errorcode
            .ok()
            .map(|()| unsafe { id.to_string().unwrap_or_default() });

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2GetCookiesCompletedHandler` COM
/// interface. The completion result is the retrieved cookies, converted from the
/// COM cookie list so the list type never reaches the public surface.
pub(crate) struct GetCookiesCompleted(Cell<Option<Box<dyn FnOnce(Result<Vec<Cookie>>)>>>);

implement_decl! {
    impl GetCookiesCompleted as pub(crate) GetCookiesCompleted_Impl:
        [ICoreWebView2GetCookiesCompletedHandler]
}

impl GetCookiesCompleted {
    pub(crate) fn create<F: FnOnce(Result<Vec<Cookie>>) + 'static>(
        handler: F,
    ) -> ICoreWebView2GetCookiesCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2GetCookiesCompletedHandler_Impl for GetCookiesCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2CookieList>) -> Result<()> {
        let outcome = errorcode.ok().and_then(|()| cookie::collect(result.ok()?));

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2ClearBrowsingDataCompletedHandler` COM interface.
pub(crate) struct ClearBrowsingDataCompleted(Cell<Option<Box<dyn FnOnce(Result<()>)>>>);

implement_decl! {
    impl ClearBrowsingDataCompleted as pub(crate) ClearBrowsingDataCompleted_Impl:
        [ICoreWebView2ClearBrowsingDataCompletedHandler]
}

impl ClearBrowsingDataCompleted {
    pub(crate) fn create<F: FnOnce(Result<()>) + 'static>(
        handler: F,
    ) -> ICoreWebView2ClearBrowsingDataCompletedHandler {
        Self(Cell::new(Some(Box::new(handler)))).into()
    }
}

impl ICoreWebView2ClearBrowsingDataCompletedHandler_Impl for ClearBrowsingDataCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT) -> Result<()> {
        if let Some(handler) = self.0.take() {
            handler(errorcode.ok());
        }

        Ok(())
    }
}

/// Defines an event-handler adapter that forwards a WebView2 event to a Rust
/// `FnMut` closure. The default form wraps the event's args interface with an
/// `ICoreWebView2` sender; passing a sender type handles events raised on a
/// different sender (such as the controller); the `sender` form (for events with
/// no args interface) wraps the sender instead.
macro_rules! event_handler {
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, $args:ident => $wrapper:ident) => {
        event_handler!($name / $impl, $handler / $handler_trait, ICoreWebView2, $args => $wrapper);
    };
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, $sender:ident, $args:ident => $wrapper:ident) => {
        pub(crate) struct $name(RefCell<Box<dyn FnMut($wrapper)>>);

        implement_decl! {
            impl $name as pub(crate) $impl: [$handler]
        }

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> $handler {
                Self(RefCell::new(Box::new(handler))).into()
            }
        }

        impl $handler_trait for $impl {
            fn Invoke(&self, _sender: Ref<$sender>, args: Ref<$args>) -> Result<()> {
                let args = $wrapper(args.ok()?.clone());
                (*self.0.borrow_mut())(args);
                Ok(())
            }
        }
    };
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, sender $sender:ident => $wrapper:ident) => {
        pub(crate) struct $name(RefCell<Box<dyn FnMut($wrapper)>>);

        implement_decl! {
            impl $name as pub(crate) $impl: [$handler]
        }

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> $handler {
                Self(RefCell::new(Box::new(handler))).into()
            }
        }

        impl $handler_trait for $impl {
            fn Invoke(&self, sender: Ref<$sender>, _args: Ref<IUnknown>) -> Result<()> {
                let value = $wrapper(sender.ok()?.clone());
                (*self.0.borrow_mut())(value);
                Ok(())
            }
        }
    };
}

event_handler!(NavigationCompleted / NavigationCompleted_Impl, ICoreWebView2NavigationCompletedEventHandler / ICoreWebView2NavigationCompletedEventHandler_Impl, ICoreWebView2NavigationCompletedEventArgs => NavigationCompletedArgs);
event_handler!(NavigationStarting / NavigationStarting_Impl, ICoreWebView2NavigationStartingEventHandler / ICoreWebView2NavigationStartingEventHandler_Impl, ICoreWebView2NavigationStartingEventArgs => NavigationStartingArgs);
event_handler!(WebMessageReceived / WebMessageReceived_Impl, ICoreWebView2WebMessageReceivedEventHandler / ICoreWebView2WebMessageReceivedEventHandler_Impl, ICoreWebView2WebMessageReceivedEventArgs => WebMessageReceivedArgs);
event_handler!(ContentLoading / ContentLoading_Impl, ICoreWebView2ContentLoadingEventHandler / ICoreWebView2ContentLoadingEventHandler_Impl, ICoreWebView2ContentLoadingEventArgs => ContentLoadingArgs);
event_handler!(NewWindowRequested / NewWindowRequested_Impl, ICoreWebView2NewWindowRequestedEventHandler / ICoreWebView2NewWindowRequestedEventHandler_Impl, ICoreWebView2NewWindowRequestedEventArgs => NewWindowRequestedArgs);
event_handler!(PermissionRequested / PermissionRequested_Impl, ICoreWebView2PermissionRequestedEventHandler / ICoreWebView2PermissionRequestedEventHandler_Impl, ICoreWebView2PermissionRequestedEventArgs => PermissionRequestedArgs);
event_handler!(DownloadStarting / DownloadStarting_Impl, ICoreWebView2DownloadStartingEventHandler / ICoreWebView2DownloadStartingEventHandler_Impl, ICoreWebView2DownloadStartingEventArgs => DownloadStartingArgs);
event_handler!(DownloadStateChanged / DownloadStateChanged_Impl, ICoreWebView2StateChangedEventHandler / ICoreWebView2StateChangedEventHandler_Impl, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(BytesReceivedChanged / BytesReceivedChanged_Impl, ICoreWebView2BytesReceivedChangedEventHandler / ICoreWebView2BytesReceivedChangedEventHandler_Impl, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(ProcessFailed / ProcessFailed_Impl, ICoreWebView2ProcessFailedEventHandler / ICoreWebView2ProcessFailedEventHandler_Impl, ICoreWebView2ProcessFailedEventArgs => ProcessFailedArgs);
event_handler!(MoveFocusRequested / MoveFocusRequested_Impl, ICoreWebView2MoveFocusRequestedEventHandler / ICoreWebView2MoveFocusRequestedEventHandler_Impl, ICoreWebView2Controller, ICoreWebView2MoveFocusRequestedEventArgs => MoveFocusRequestedArgs);
event_handler!(AcceleratorKeyPressed / AcceleratorKeyPressed_Impl, ICoreWebView2AcceleratorKeyPressedEventHandler / ICoreWebView2AcceleratorKeyPressedEventHandler_Impl, ICoreWebView2Controller, ICoreWebView2AcceleratorKeyPressedEventArgs => AcceleratorKeyPressedArgs);

/// Adapts a Rust closure to the `ICoreWebView2FocusChangedEventHandler` COM
/// interface, shared by the controller's got-focus and lost-focus events. The
/// event carries no args, so the closure takes none.
pub(crate) struct FocusChanged(RefCell<Box<dyn FnMut()>>);

implement_decl! {
    impl FocusChanged as pub(crate) FocusChanged_Impl:
        [ICoreWebView2FocusChangedEventHandler]
}

impl FocusChanged {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> ICoreWebView2FocusChangedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2FocusChangedEventHandler_Impl for FocusChanged_Impl {
    fn Invoke(&self, _sender: Ref<ICoreWebView2Controller>, _args: Ref<IUnknown>) -> Result<()> {
        (*self.0.borrow_mut())();
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2DocumentTitleChangedEventHandler`
/// COM interface. The event carries no args, so the new document title is read
/// from the sender and handed to the closure.
pub(crate) struct DocumentTitleChanged(RefCell<Box<dyn FnMut(String)>>);

implement_decl! {
    impl DocumentTitleChanged as pub(crate) DocumentTitleChanged_Impl:
        [ICoreWebView2DocumentTitleChangedEventHandler]
}

impl DocumentTitleChanged {
    pub(crate) fn create<F: FnMut(String) + 'static>(
        handler: F,
    ) -> ICoreWebView2DocumentTitleChangedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2DocumentTitleChangedEventHandler_Impl for DocumentTitleChanged_Impl {
    fn Invoke(&self, sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> Result<()> {
        let title =
            unsafe { string::take_result(sender.ok().and_then(|sender| sender.DocumentTitle())) };
        (*self.0.borrow_mut())(title);
        Ok(())
    }
}

/// Adapts a Rust closure to the
/// `ICoreWebView2ContainsFullScreenElementChangedEventHandler` COM interface.
/// The event carries no args, so the new fullscreen state is read from the
/// sender and handed to the closure.
pub(crate) struct ContainsFullScreenElementChanged(RefCell<Box<dyn FnMut(bool)>>);

implement_decl! {
    impl ContainsFullScreenElementChanged as pub(crate) ContainsFullScreenElementChanged_Impl:
        [ICoreWebView2ContainsFullScreenElementChangedEventHandler]
}

impl ContainsFullScreenElementChanged {
    pub(crate) fn create<F: FnMut(bool) + 'static>(
        handler: F,
    ) -> ICoreWebView2ContainsFullScreenElementChangedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl
    for ContainsFullScreenElementChanged_Impl
{
    fn Invoke(&self, sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> Result<()> {
        let contains = sender
            .ok()
            .and_then(|sender| unsafe { sender.ContainsFullScreenElement() })
            .is_ok_and(|value| value.as_bool());
        (*self.0.borrow_mut())(contains);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2WindowCloseRequestedEventHandler`
/// COM interface. The event carries no args, so the closure takes none.
pub(crate) struct WindowCloseRequested(RefCell<Box<dyn FnMut()>>);

implement_decl! {
    impl WindowCloseRequested as pub(crate) WindowCloseRequested_Impl:
        [ICoreWebView2WindowCloseRequestedEventHandler]
}

impl WindowCloseRequested {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> ICoreWebView2WindowCloseRequestedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2WindowCloseRequestedEventHandler_Impl for WindowCloseRequested_Impl {
    fn Invoke(&self, _sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> Result<()> {
        (*self.0.borrow_mut())();
        Ok(())
    }
}

/// Defines an event-subscription method on a wrapper that holds a WebView2
/// interface in its first field. It registers the generated [`crate::handler`]
/// adapter, then returns an [`EventRegistration`] that removes it on drop.
macro_rules! subscription {
    (
        $(#[$doc:meta])*
        $method:ident($arg:ty) => $handler:ident, $add:ident / $remove:ident
    ) => {
        $(#[$doc])*
        pub fn $method<F: FnMut($arg) + 'static>(&self, handler: F) -> Result<EventRegistration> {
            let handler = crate::handler::$handler::create(handler);
            let token = unsafe { self.0.$add(&handler)? };
            let source = self.0.clone();
            Ok(EventRegistration::new(move || {
                let _ = unsafe { source.$remove(token) };
            }))
        }
    };
}

pub(crate) use subscription;

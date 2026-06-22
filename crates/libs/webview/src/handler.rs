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
    fn Invoke(&self, errorcode: HRESULT, result: LPCWSTR) -> Result<()> {
        let outcome = errorcode.ok().map(|()| unsafe { string::decode(result) });

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
    fn Invoke(&self, errorcode: HRESULT, id: LPCWSTR) -> Result<()> {
        let outcome = errorcode.ok().map(|()| unsafe { string::decode(id) });

        if let Some(handler) = self.0.take() {
            handler(outcome);
        }

        Ok(())
    }
}

/// Defines an event-handler adapter that forwards a WebView2 event to a Rust
/// `FnMut` closure. The `args` form wraps the event's args interface; the
/// `sender` form (for events with no args interface) wraps the sender instead.
macro_rules! event_handler {
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, $args:ident => $wrapper:ident) => {
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
            fn Invoke(&self, _sender: Ref<ICoreWebView2>, args: Ref<$args>) -> Result<()> {
                let args = $wrapper(args.ok()?.clone());
                (*self.0.borrow_mut())(args);
                Ok(())
            }
        }
    };
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, controller $args:ident => $wrapper:ident) => {
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
            fn Invoke(
                &self,
                _sender: Ref<ICoreWebView2Controller>,
                args: Ref<$args>,
            ) -> Result<()> {
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
event_handler!(MoveFocusRequested / MoveFocusRequested_Impl, ICoreWebView2MoveFocusRequestedEventHandler / ICoreWebView2MoveFocusRequestedEventHandler_Impl, controller ICoreWebView2MoveFocusRequestedEventArgs => MoveFocusRequestedArgs);
event_handler!(AcceleratorKeyPressed / AcceleratorKeyPressed_Impl, ICoreWebView2AcceleratorKeyPressedEventHandler / ICoreWebView2AcceleratorKeyPressedEventHandler_Impl, controller ICoreWebView2AcceleratorKeyPressedEventArgs => AcceleratorKeyPressedArgs);

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
        let title = sender
            .ok()
            .and_then(|sender| unsafe { sender.DocumentTitle() })
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default();
        (*self.0.borrow_mut())(title);
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

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

/// Adapts a Rust closure to the `ICoreWebView2NavigationCompletedEventHandler`
/// COM interface.
pub(crate) struct NavigationCompleted(RefCell<Box<dyn FnMut(NavigationCompletedArgs)>>);

implement_decl! {
    impl NavigationCompleted as pub(crate) NavigationCompleted_Impl:
        [ICoreWebView2NavigationCompletedEventHandler]
}

impl NavigationCompleted {
    pub(crate) fn create<F: FnMut(NavigationCompletedArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2NavigationCompletedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2NavigationCompletedEventHandler_Impl for NavigationCompleted_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2NavigationCompletedEventArgs>,
    ) -> Result<()> {
        let args = NavigationCompletedArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2WebMessageReceivedEventHandler`
/// COM interface.
pub(crate) struct WebMessageReceived(RefCell<Box<dyn FnMut(WebMessageReceivedArgs)>>);

implement_decl! {
    impl WebMessageReceived as pub(crate) WebMessageReceived_Impl:
        [ICoreWebView2WebMessageReceivedEventHandler]
}

impl WebMessageReceived {
    pub(crate) fn create<F: FnMut(WebMessageReceivedArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2WebMessageReceivedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageReceived_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2WebMessageReceivedEventArgs>,
    ) -> Result<()> {
        let args = WebMessageReceivedArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2NavigationStartingEventHandler`
/// COM interface.
pub(crate) struct NavigationStarting(RefCell<Box<dyn FnMut(NavigationStartingArgs)>>);

implement_decl! {
    impl NavigationStarting as pub(crate) NavigationStarting_Impl:
        [ICoreWebView2NavigationStartingEventHandler]
}

impl NavigationStarting {
    pub(crate) fn create<F: FnMut(NavigationStartingArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2NavigationStartingEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2NavigationStartingEventHandler_Impl for NavigationStarting_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2NavigationStartingEventArgs>,
    ) -> Result<()> {
        let args = NavigationStartingArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2ContentLoadingEventHandler`
/// COM interface.
pub(crate) struct ContentLoading(RefCell<Box<dyn FnMut(ContentLoadingArgs)>>);

implement_decl! {
    impl ContentLoading as pub(crate) ContentLoading_Impl:
        [ICoreWebView2ContentLoadingEventHandler]
}

impl ContentLoading {
    pub(crate) fn create<F: FnMut(ContentLoadingArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2ContentLoadingEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2ContentLoadingEventHandler_Impl for ContentLoading_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2ContentLoadingEventArgs>,
    ) -> Result<()> {
        let args = ContentLoadingArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
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

/// Adapts a Rust closure to the `ICoreWebView2NewWindowRequestedEventHandler`
/// COM interface.
pub(crate) struct NewWindowRequested(RefCell<Box<dyn FnMut(NewWindowRequestedArgs)>>);

implement_decl! {
    impl NewWindowRequested as pub(crate) NewWindowRequested_Impl:
        [ICoreWebView2NewWindowRequestedEventHandler]
}

impl NewWindowRequested {
    pub(crate) fn create<F: FnMut(NewWindowRequestedArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2NewWindowRequestedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2NewWindowRequestedEventHandler_Impl for NewWindowRequested_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2NewWindowRequestedEventArgs>,
    ) -> Result<()> {
        let args = NewWindowRequestedArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2PermissionRequestedEventHandler`
/// COM interface.
pub(crate) struct PermissionRequested(RefCell<Box<dyn FnMut(PermissionRequestedArgs)>>);

implement_decl! {
    impl PermissionRequested as pub(crate) PermissionRequested_Impl:
        [ICoreWebView2PermissionRequestedEventHandler]
}

impl PermissionRequested {
    pub(crate) fn create<F: FnMut(PermissionRequestedArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2PermissionRequestedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2PermissionRequestedEventHandler_Impl for PermissionRequested_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2PermissionRequestedEventArgs>,
    ) -> Result<()> {
        let args = PermissionRequestedArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2DownloadStartingEventHandler`
/// COM interface.
pub(crate) struct DownloadStarting(RefCell<Box<dyn FnMut(DownloadStartingArgs)>>);

implement_decl! {
    impl DownloadStarting as pub(crate) DownloadStarting_Impl:
        [ICoreWebView2DownloadStartingEventHandler]
}

impl DownloadStarting {
    pub(crate) fn create<F: FnMut(DownloadStartingArgs) + 'static>(
        handler: F,
    ) -> ICoreWebView2DownloadStartingEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2DownloadStartingEventHandler_Impl for DownloadStarting_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2DownloadStartingEventArgs>,
    ) -> Result<()> {
        let args = DownloadStartingArgs(args.ok()?.clone());
        (*self.0.borrow_mut())(args);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2StateChangedEventHandler` COM
/// interface. The event carries no args, so the [`DownloadOperation`] that
/// changed is read from the sender and handed to the closure.
pub(crate) struct DownloadStateChanged(RefCell<Box<dyn FnMut(DownloadOperation)>>);

implement_decl! {
    impl DownloadStateChanged as pub(crate) DownloadStateChanged_Impl:
        [ICoreWebView2StateChangedEventHandler]
}

impl DownloadStateChanged {
    pub(crate) fn create<F: FnMut(DownloadOperation) + 'static>(
        handler: F,
    ) -> ICoreWebView2StateChangedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2StateChangedEventHandler_Impl for DownloadStateChanged_Impl {
    fn Invoke(
        &self,
        sender: Ref<ICoreWebView2DownloadOperation>,
        _args: Ref<IUnknown>,
    ) -> Result<()> {
        let operation = DownloadOperation(sender.ok()?.clone());
        (*self.0.borrow_mut())(operation);
        Ok(())
    }
}

/// Adapts a Rust closure to the `ICoreWebView2BytesReceivedChangedEventHandler`
/// COM interface. The event carries no args, so the [`DownloadOperation`] that
/// changed is read from the sender and handed to the closure.
pub(crate) struct BytesReceivedChanged(RefCell<Box<dyn FnMut(DownloadOperation)>>);

implement_decl! {
    impl BytesReceivedChanged as pub(crate) BytesReceivedChanged_Impl:
        [ICoreWebView2BytesReceivedChangedEventHandler]
}

impl BytesReceivedChanged {
    pub(crate) fn create<F: FnMut(DownloadOperation) + 'static>(
        handler: F,
    ) -> ICoreWebView2BytesReceivedChangedEventHandler {
        Self(RefCell::new(Box::new(handler))).into()
    }
}

impl ICoreWebView2BytesReceivedChangedEventHandler_Impl for BytesReceivedChanged_Impl {
    fn Invoke(
        &self,
        sender: Ref<ICoreWebView2DownloadOperation>,
        _args: Ref<IUnknown>,
    ) -> Result<()> {
        let operation = DownloadOperation(sender.ok()?.clone());
        (*self.0.borrow_mut())(operation);
        Ok(())
    }
}

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

use super::*;
use std::cell::Cell;
use windows_core::implement;

/// Adapts a Rust closure to the `ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler`
/// COM interface.
#[implement(ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler)]
pub(crate) struct EnvironmentCompleted(Cell<Option<Box<dyn FnOnce(Result<Environment>)>>>);

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
#[implement(ICoreWebView2CreateCoreWebView2ControllerCompletedHandler)]
pub(crate) struct ControllerCompleted(Cell<Option<Box<dyn FnOnce(Result<Controller>)>>>);

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
#[implement(ICoreWebView2ExecuteScriptCompletedHandler)]
pub(crate) struct ExecuteScriptCompleted(Cell<Option<Box<dyn FnOnce(Result<String>)>>>);

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

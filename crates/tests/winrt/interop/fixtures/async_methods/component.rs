use super::*;
use AsyncMethods::*;
use windows_future::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "AsyncMethods.Widget").then(activatable::<Widget, AsyncMethods::Widget>)
}

#[implement(AsyncMethods::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Action(&self) -> Result<IAsyncAction> {
        Ok(IAsyncAction::ready(Ok(())))
    }

    fn Operation(&self, value: i32) -> Result<IAsyncOperation<i32>> {
        Ok(IAsyncOperation::ready(Ok(value)))
    }

    fn StringOperation(&self, value: &HSTRING) -> Result<IAsyncOperation<HSTRING>> {
        Ok(IAsyncOperation::ready(Ok(value.clone())))
    }

    fn Fail(&self) -> Result<IAsyncOperation<i32>> {
        Ok(IAsyncOperation::ready(Err(E_BOUNDS.into())))
    }

    fn Increment(&self, operation: Ref<IAsyncOperation<i32>>) -> Result<IAsyncOperation<i32>> {
        let operation = operation.ok()?.clone();
        Ok(IAsyncOperation::spawn(move || Ok(operation.join()? + 1)))
    }
}

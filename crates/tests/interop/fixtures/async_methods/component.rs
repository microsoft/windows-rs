use super::*;
use AsyncMethods::*;
use windows_future::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "AsyncMethods.Widget").then(activatable::<Widget, AsyncMethods::Widget>)
}

#[implement(AsyncMethods::Widget)]
#[derive(Default)]
struct Widget;

fn check_value(value: i32) -> Result<i32> {
    if value < 0 {
        Err(E_BOUNDS.into())
    } else {
        Ok(value)
    }
}

impl IWidget_Impl for Widget_Impl {
    fn Action(&self, value: i32) -> Result<IAsyncAction> {
        Ok(match check_value(value) {
            Ok(_) => IAsyncAction::spawn(|| Ok(())),
            Err(error) => IAsyncAction::ready(Err(error)),
        })
    }

    fn ActionWithProgress(&self, value: i32) -> Result<IAsyncActionWithProgress<u32>> {
        Ok(match check_value(value) {
            Ok(_) => IAsyncActionWithProgress::spawn(|| Ok(())),
            Err(error) => IAsyncActionWithProgress::ready(Err(error)),
        })
    }

    fn Operation(&self, value: i32) -> Result<IAsyncOperation<i32>> {
        Ok(match check_value(value) {
            Ok(value) => IAsyncOperation::spawn(move || Ok(value)),
            Err(error) => IAsyncOperation::ready(Err(error)),
        })
    }

    fn OperationWithProgress(&self, value: i32) -> Result<IAsyncOperationWithProgress<i32, u32>> {
        Ok(match check_value(value) {
            Ok(value) => IAsyncOperationWithProgress::spawn(move || Ok(value)),
            Err(error) => IAsyncOperationWithProgress::ready(Err(error)),
        })
    }

    fn RejectOperation(&self) -> Result<IAsyncOperation<i32>> {
        Err(Error::new(
            HRESULT(0x8004_a123_u32 as i32),
            "component-sync",
        ))
    }

    fn FailOperation(&self) -> Result<IAsyncOperation<i32>> {
        Ok(IAsyncOperation::spawn(|| {
            Err(Error::new(
                HRESULT(0x8004_a123_u32 as i32),
                "component-async",
            ))
        }))
    }

    fn AwaitAction(&self, action: Ref<IAsyncAction>) -> Result<IAsyncAction> {
        let action = action.ok()?.clone();
        Ok(IAsyncAction::spawn(move || action.join()))
    }

    fn AwaitActionWithProgress(
        &self,
        action: Ref<IAsyncActionWithProgress<u32>>,
    ) -> Result<IAsyncActionWithProgress<u32>> {
        let action = action.ok()?.clone();
        Ok(IAsyncActionWithProgress::spawn(move || action.join()))
    }

    fn AwaitOperation(&self, operation: Ref<IAsyncOperation<i32>>) -> Result<IAsyncOperation<i32>> {
        let operation = operation.ok()?.clone();
        Ok(IAsyncOperation::spawn(move || Ok(operation.join()? + 1)))
    }

    fn AwaitOperationWithProgress(
        &self,
        operation: Ref<IAsyncOperationWithProgress<i32, u32>>,
    ) -> Result<IAsyncOperationWithProgress<i32, u32>> {
        let operation = operation.ok()?.clone();
        Ok(IAsyncOperationWithProgress::spawn(move || {
            Ok(operation.join()? + 1)
        }))
    }
}

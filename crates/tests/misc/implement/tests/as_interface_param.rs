use windows::core::*;
use windows_future::*;

#[implement(IAsyncAction)]
struct Async;

impl IAsyncAction_Impl for Async_Impl {
    fn SetCompleted(&self, handler: Ref<AsyncActionCompletedHandler>) -> Result<()> {
        // This validates that `as_interface` may be used to call a bindgen-produced method expecting a `Param<T>` argument.
        handler
            .unwrap()
            .Invoke(self.as_interface(), AsyncStatus::Completed)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler> {
        todo!();
    }
    fn GetResults(&self) -> Result<()> {
        todo!();
    }
}

impl IAsyncInfo_Impl for Async_Impl {
    fn Id(&self) -> Result<u32> {
        todo!();
    }
    fn Status(&self) -> Result<AsyncStatus> {
        todo!();
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        todo!();
    }
    fn Cancel(&self) -> Result<()> {
        todo!();
    }
    fn Close(&self) -> Result<()> {
        todo!();
    }
}

#[test]
fn test() -> Result<()> {
    let a: IAsyncAction = Async.into();
    let a_clone = a.clone();
    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        assert_eq!(&a_clone, sender.unwrap());
        assert_eq!(status, AsyncStatus::Completed);
        Ok(())
    }))
}

use windows::core::*;
use windows_future::*;

#[implement(IAsyncAction)]
struct Async;

impl IAsyncAction_Impl for Async_Impl {
    fn SetCompleted(&self, handler: Ref<AsyncActionCompletedHandler>) -> Result<(), HRESULT> {
        // This validates that `as_interface` may be used to call a bindgen-produced method expecting a `Param<T>` argument.
        handler
            .unwrap()
            .Invoke(self.as_interface(), AsyncStatus::Completed)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler, HRESULT> {
        todo!();
    }
    fn GetResults(&self) -> Result<(), HRESULT> {
        todo!();
    }
}

impl IAsyncInfo_Impl for Async_Impl {
    fn Id(&self) -> Result<u32, HRESULT> {
        todo!();
    }
    fn Status(&self) -> Result<AsyncStatus, HRESULT> {
        todo!();
    }
    fn ErrorCode(&self) -> Result<HRESULT, HRESULT> {
        todo!();
    }
    fn Cancel(&self) -> Result<(), HRESULT> {
        todo!();
    }
    fn Close(&self) -> Result<(), HRESULT> {
        todo!();
    }
}

#[test]
fn test() -> Result<(), HRESULT> {
    let a: IAsyncAction = Async.into();
    let a_clone = a.clone();
    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        assert_eq!(&a_clone, sender.unwrap());
        assert_eq!(status, AsyncStatus::Completed);
        Ok(())
    }))
}

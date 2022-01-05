pub trait ICreateDeviceAccessAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn Close();
    fn GetResult();
}
pub trait IDeviceIoControlImpl: Sized {
    fn DeviceIoControlSync();
    fn DeviceIoControlAsync();
    fn CancelOperation();
}
pub trait IDeviceRequestCompletionCallbackImpl: Sized {
    fn Invoke();
}

pub trait IDisplayDeviceInteropImpl: Sized {
    fn CreateSharedHandle();
    fn OpenSharedHandle();
}
pub trait IDisplayPathInteropImpl: Sized {
    fn CreateSourcePresentationHandle();
    fn GetSourceId();
}

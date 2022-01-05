pub trait ILearningModelDeviceFactoryNativeImpl: Sized {
    fn CreateFromD3D12CommandQueue();
}
pub trait ILearningModelOperatorProviderNativeImpl: Sized {
    fn GetRegistry();
}
pub trait ILearningModelSessionOptionsNativeImpl: Sized {
    fn SetIntraOpNumThreadsOverride();
}
pub trait ITensorNativeImpl: Sized {
    fn GetBuffer();
    fn GetD3D12Resource();
}
pub trait ITensorStaticsNativeImpl: Sized {
    fn CreateFromD3D12Resource();
}

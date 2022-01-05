pub trait ICallFrameImpl: Sized {
    fn GetInfo();
    fn GetIIDAndMethod();
    fn GetNames();
    fn GetStackLocation();
    fn SetStackLocation();
    fn SetReturnValue();
    fn GetReturnValue();
    fn GetParamInfo();
    fn SetParam();
    fn GetParam();
    fn Copy();
    fn Free();
    fn FreeParam();
    fn WalkFrame();
    fn GetMarshalSizeMax();
    fn Marshal();
    fn Unmarshal();
    fn ReleaseMarshalData();
    fn Invoke();
}
pub trait ICallFrameEventsImpl: Sized {
    fn OnCall();
}
pub trait ICallFrameWalkerImpl: Sized {
    fn OnWalkInterface();
}
pub trait ICallIndirectImpl: Sized {
    fn CallIndirect();
    fn GetMethodInfo();
    fn GetStackSize();
    fn GetIID();
}
pub trait ICallInterceptorImpl: Sized + ICallIndirectImpl {
    fn RegisterSink();
    fn GetRegisteredSink();
}
pub trait ICallUnmarshalImpl: Sized {
    fn Unmarshal();
    fn ReleaseMarshalData();
}
pub trait IInterfaceRelatedImpl: Sized {
    fn SetIID();
    fn GetIID();
}

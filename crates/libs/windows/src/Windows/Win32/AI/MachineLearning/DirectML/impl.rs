pub trait IDMLBindingTableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn BindInputs();
    fn BindOutputs();
    fn BindTemporaryResource();
    fn BindPersistentResource();
    fn Reset();
}
pub trait IDMLCommandRecorderImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn RecordDispatch();
}
pub trait IDMLCompiledOperatorImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {}
pub trait IDMLDebugDeviceImpl: Sized {
    fn SetMuteDebugOutput();
}
pub trait IDMLDeviceImpl: Sized + IDMLObjectImpl {
    fn CheckFeatureSupport();
    fn CreateOperator();
    fn CompileOperator();
    fn CreateOperatorInitializer();
    fn CreateCommandRecorder();
    fn CreateBindingTable();
    fn Evict();
    fn MakeResident();
    fn GetDeviceRemovedReason();
    fn GetParentDevice();
}
pub trait IDMLDevice1Impl: Sized + IDMLDeviceImpl + IDMLObjectImpl {
    fn CompileGraph();
}
pub trait IDMLDeviceChildImpl: Sized + IDMLObjectImpl {
    fn GetDevice();
}
pub trait IDMLDispatchableImpl: Sized + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn GetBindingProperties();
}
pub trait IDMLObjectImpl: Sized {
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn SetName();
}
pub trait IDMLOperatorImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}
pub trait IDMLOperatorInitializerImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn Reset();
}
pub trait IDMLPageableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}

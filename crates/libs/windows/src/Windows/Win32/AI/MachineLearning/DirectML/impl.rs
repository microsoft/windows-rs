pub trait IDMLBindingTableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn BindInputs();
    fn BindOutputs();
    fn BindTemporaryResource();
    fn BindPersistentResource();
    fn Reset();
}
impl ::windows::core::RuntimeName for IDMLBindingTable {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLBindingTable";
}
impl IDMLBindingTableVtbl {
    pub const fn new<Impl: IDMLBindingTableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLBindingTableVtbl {
        unsafe extern "system" fn BindInputs<Impl: IDMLBindingTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BindInputs(bindingcount, &*(&bindings as *const <DML_BINDING_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_DESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindOutputs<Impl: IDMLBindingTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BindOutputs(bindingcount, &*(&bindings as *const <DML_BINDING_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_DESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindTemporaryResource<Impl: IDMLBindingTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BindTemporaryResource(&*(&binding as *const <DML_BINDING_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_DESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindPersistentResource<Impl: IDMLBindingTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BindPersistentResource(&*(&binding as *const <DML_BINDING_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_DESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Reset<Impl: IDMLBindingTableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset(&*(&desc as *const <DML_BINDING_TABLE_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_TABLE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLBindingTable>, base.5, BindInputs::<Impl, OFFSET>, BindOutputs::<Impl, OFFSET>, BindTemporaryResource::<Impl, OFFSET>, BindPersistentResource::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IDMLCommandRecorderImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn RecordDispatch();
}
impl ::windows::core::RuntimeName for IDMLCommandRecorder {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLCommandRecorder";
}
impl IDMLCommandRecorderVtbl {
    pub const fn new<Impl: IDMLCommandRecorderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLCommandRecorderVtbl {
        unsafe extern "system" fn RecordDispatch<Impl: IDMLCommandRecorderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, dispatchable: ::windows::core::RawPtr, bindings: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RecordDispatch(
                    &*(&commandlist as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandList as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandList as ::windows::core::DefaultType>::DefaultType),
                    &*(&dispatchable as *const <IDMLDispatchable as ::windows::core::Abi>::Abi as *const <IDMLDispatchable as ::windows::core::DefaultType>::DefaultType),
                    &*(&bindings as *const <IDMLBindingTable as ::windows::core::Abi>::Abi as *const <IDMLBindingTable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLCommandRecorder>, base.5, RecordDispatch::<Impl, OFFSET>)
    }
}
pub trait IDMLCompiledOperatorImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {}
impl ::windows::core::RuntimeName for IDMLCompiledOperator {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLCompiledOperator";
}
impl IDMLCompiledOperatorVtbl {
    pub const fn new<Impl: IDMLCompiledOperatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLCompiledOperatorVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLCompiledOperator>, base.5)
    }
}
pub trait IDMLDebugDeviceImpl: Sized {
    fn SetMuteDebugOutput();
}
impl ::windows::core::RuntimeName for IDMLDebugDevice {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLDebugDevice";
}
impl IDMLDebugDeviceVtbl {
    pub const fn new<Impl: IDMLDebugDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLDebugDeviceVtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDMLDebugDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(&*(&mute as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLDebugDevice>, base.5, SetMuteDebugOutput::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IDMLDevice {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLDevice";
}
impl IDMLDeviceVtbl {
    pub const fn new<Impl: IDMLDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLDeviceVtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, featurequerydatasize, &*(&featurequerydata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), featuresupportdatasize, ::core::mem::transmute_copy(&featuresupportdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperator<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOperator(&*(&desc as *const <DML_OPERATOR_DESC as ::windows::core::Abi>::Abi as *const <DML_OPERATOR_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompileOperator<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileOperator(&*(&op as *const <IDMLOperator as ::windows::core::Abi>::Abi as *const <IDMLOperator as ::windows::core::DefaultType>::DefaultType), flags, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperatorInitializer<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOperatorInitializer(operatorcount, &*(&operators as *const <IDMLCompiledOperator as ::windows::core::Abi>::Abi as *const <IDMLCompiledOperator as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandRecorder<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCommandRecorder(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBindingTable<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBindingTable(&*(&desc as *const <DML_BINDING_TABLE_DESC as ::windows::core::Abi>::Abi as *const <DML_BINDING_TABLE_DESC as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Evict<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Evict(count, &*(&ppobjects as *const <IDMLPageable as ::windows::core::Abi>::Abi as *const <IDMLPageable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeResident<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeResident(count, &*(&ppobjects as *const <IDMLPageable as ::windows::core::Abi>::Abi as *const <IDMLPageable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceRemovedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentDevice<Impl: IDMLDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParentDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLDevice>, base.5, CheckFeatureSupport::<Impl, OFFSET>, CreateOperator::<Impl, OFFSET>, CompileOperator::<Impl, OFFSET>, CreateOperatorInitializer::<Impl, OFFSET>, CreateCommandRecorder::<Impl, OFFSET>, CreateBindingTable::<Impl, OFFSET>, Evict::<Impl, OFFSET>, MakeResident::<Impl, OFFSET>, GetDeviceRemovedReason::<Impl, OFFSET>, GetParentDevice::<Impl, OFFSET>)
    }
}
pub trait IDMLDevice1Impl: Sized + IDMLDeviceImpl + IDMLObjectImpl {
    fn CompileGraph();
}
impl ::windows::core::RuntimeName for IDMLDevice1 {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLDevice1";
}
impl IDMLDevice1Vtbl {
    pub const fn new<Impl: IDMLDevice1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLDevice1Vtbl {
        unsafe extern "system" fn CompileGraph<Impl: IDMLDevice1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileGraph(&*(&desc as *const <DML_GRAPH_DESC as ::windows::core::Abi>::Abi as *const <DML_GRAPH_DESC as ::windows::core::DefaultType>::DefaultType), flags, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLDevice1>, base.5, CompileGraph::<Impl, OFFSET>)
    }
}
pub trait IDMLDeviceChildImpl: Sized + IDMLObjectImpl {
    fn GetDevice();
}
impl ::windows::core::RuntimeName for IDMLDeviceChild {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLDeviceChild";
}
impl IDMLDeviceChildVtbl {
    pub const fn new<Impl: IDMLDeviceChildImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLDeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDMLDeviceChildImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLDeviceChild>, base.5, GetDevice::<Impl, OFFSET>)
    }
}
pub trait IDMLDispatchableImpl: Sized + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn GetBindingProperties();
}
impl ::windows::core::RuntimeName for IDMLDispatchable {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLDispatchable";
}
impl IDMLDispatchableVtbl {
    pub const fn new<Impl: IDMLDispatchableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLDispatchableVtbl {
        unsafe extern "system" fn GetBindingProperties<Impl: IDMLDispatchableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBindingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLDispatchable>, base.5, GetBindingProperties::<Impl, OFFSET>)
    }
}
pub trait IDMLObjectImpl: Sized {
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn SetName();
}
impl ::windows::core::RuntimeName for IDMLObject {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLObject";
}
impl IDMLObjectVtbl {
    pub const fn new<Impl: IDMLObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLObjectVtbl {
        unsafe extern "system" fn GetPrivateData<Impl: IDMLObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, ::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDMLObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&data as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDMLObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IDMLObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLObject>, base.5, GetPrivateData::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, SetPrivateDataInterface::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
pub trait IDMLOperatorImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}
impl ::windows::core::RuntimeName for IDMLOperator {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLOperator";
}
impl IDMLOperatorVtbl {
    pub const fn new<Impl: IDMLOperatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLOperatorVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLOperator>, base.5)
    }
}
pub trait IDMLOperatorInitializerImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn Reset();
}
impl ::windows::core::RuntimeName for IDMLOperatorInitializer {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLOperatorInitializer";
}
impl IDMLOperatorInitializerVtbl {
    pub const fn new<Impl: IDMLOperatorInitializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLOperatorInitializerVtbl {
        unsafe extern "system" fn Reset<Impl: IDMLOperatorInitializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset(operatorcount, &*(&operators as *const <IDMLCompiledOperator as ::windows::core::Abi>::Abi as *const <IDMLCompiledOperator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLOperatorInitializer>, base.5, Reset::<Impl, OFFSET>)
    }
}
pub trait IDMLPageableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}
impl ::windows::core::RuntimeName for IDMLPageable {
    const NAME: &'static str = "Windows.Win32.AI.MachineLearning.DirectML.IDMLPageable";
}
impl IDMLPageableVtbl {
    pub const fn new<Impl: IDMLPageableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDMLPageableVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDMLPageable>, base.5)
    }
}

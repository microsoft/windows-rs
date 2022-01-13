#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLBindingTableImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl {
    fn BindInputs(&mut self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindOutputs(&mut self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindTemporaryResource(&mut self, binding: *const DML_BINDING_DESC);
    fn BindPersistentResource(&mut self, binding: *const DML_BINDING_DESC);
    fn Reset(&mut self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLBindingTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLBindingTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLBindingTableVtbl {
        unsafe extern "system" fn BindInputs<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindInputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindOutputs<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindOutputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindTemporaryResource<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindTemporaryResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn BindPersistentResource<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindPersistentResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn Reset<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&desc)).into()
        }
        Self {
            base: IDMLDeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BindInputs: BindInputs::<Impl, IMPL_OFFSET>,
            BindOutputs: BindOutputs::<Impl, IMPL_OFFSET>,
            BindTemporaryResource: BindTemporaryResource::<Impl, IMPL_OFFSET>,
            BindPersistentResource: BindPersistentResource::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLBindingTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLCommandRecorderImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl {
    fn RecordDispatch(&mut self, commandlist: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandList>, dispatchable: ::core::option::Option<IDMLDispatchable>, bindings: ::core::option::Option<IDMLBindingTable>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLCommandRecorderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCommandRecorderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCommandRecorderVtbl {
        unsafe extern "system" fn RecordDispatch<Impl: IDMLCommandRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, dispatchable: ::windows::core::RawPtr, bindings: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordDispatch(::core::mem::transmute(&commandlist), ::core::mem::transmute(&dispatchable), ::core::mem::transmute(&bindings))
        }
        Self { base: IDMLDeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RecordDispatch: RecordDispatch::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCommandRecorder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLCompiledOperatorImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl + IDMLPageableImpl + IDMLDispatchableImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLCompiledOperatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCompiledOperatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCompiledOperatorVtbl {
        Self { base: IDMLDispatchableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCompiledOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDebugDeviceImpl: Sized {
    fn SetMuteDebugOutput(&mut self, mute: super::super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDebugDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDebugDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDebugDeviceVtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDMLDebugDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&mute))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetMuteDebugOutput: SetMuteDebugOutput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDebugDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLDeviceImpl: Sized + IDMLObjectImpl {
    fn CheckFeatureSupport(&mut self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateOperator(&mut self, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CompileOperator(&mut self, op: ::core::option::Option<IDMLOperator>, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateOperatorInitializer(&mut self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCommandRecorder(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateBindingTable(&mut self, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Evict(&mut self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()>;
    fn MakeResident(&mut self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()>;
    fn GetDeviceRemovedReason(&mut self) -> ::windows::core::Result<()>;
    fn GetParentDevice(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDeviceVtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&featurequerydatasize), ::core::mem::transmute_copy(&featurequerydata), ::core::mem::transmute_copy(&featuresupportdatasize), ::core::mem::transmute_copy(&featuresupportdata)).into()
        }
        unsafe extern "system" fn CreateOperator<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateOperator(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CompileOperator<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileOperator(::core::mem::transmute(&op), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateOperatorInitializer<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateOperatorInitializer(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCommandRecorder<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandRecorder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBindingTable<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBindingTable(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Evict<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Evict(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn MakeResident<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeResident(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetParentDevice<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParentDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: IDMLObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            CreateOperator: CreateOperator::<Impl, IMPL_OFFSET>,
            CompileOperator: CompileOperator::<Impl, IMPL_OFFSET>,
            CreateOperatorInitializer: CreateOperatorInitializer::<Impl, IMPL_OFFSET>,
            CreateCommandRecorder: CreateCommandRecorder::<Impl, IMPL_OFFSET>,
            CreateBindingTable: CreateBindingTable::<Impl, IMPL_OFFSET>,
            Evict: Evict::<Impl, IMPL_OFFSET>,
            MakeResident: MakeResident::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetParentDevice: GetParentDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLDevice1Impl: Sized + IDMLObjectImpl + IDMLDeviceImpl {
    fn CompileGraph(&mut self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDevice1Vtbl {
        unsafe extern "system" fn CompileGraph<Impl: IDMLDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileGraph(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: IDMLDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CompileGraph: CompileGraph::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDeviceChildImpl: Sized + IDMLObjectImpl {
    fn GetDevice(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDeviceChildImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDMLDeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: IDMLObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDispatchableImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl + IDMLPageableImpl {
    fn GetBindingProperties(&mut self) -> DML_BINDING_PROPERTIES;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDispatchableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDispatchableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDispatchableVtbl {
        unsafe extern "system" fn GetBindingProperties<Impl: IDMLDispatchableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindingProperties()
        }
        Self { base: IDMLPageableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetBindingProperties: GetBindingProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDispatchable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLObjectImpl: Sized {
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, data: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetName(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLObjectVtbl {
        unsafe extern "system" fn GetPrivateData<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn SetName<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLOperatorImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperatorVtbl {
        Self { base: IDMLDeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLOperatorInitializerImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl + IDMLPageableImpl + IDMLDispatchableImpl {
    fn Reset(&mut self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperatorInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperatorInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperatorInitializerVtbl {
        unsafe extern "system" fn Reset<Impl: IDMLOperatorInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators)).into()
        }
        Self { base: IDMLDispatchableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Reset: Reset::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperatorInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLPageableImpl: Sized + IDMLObjectImpl + IDMLDeviceChildImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLPageableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLPageableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLPageableVtbl {
        Self { base: IDMLDeviceChildVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLPageable as ::windows::core::Interface>::IID
    }
}

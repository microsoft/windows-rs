#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLBindingTable_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl {
    fn BindInputs(&mut self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindOutputs(&mut self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindTemporaryResource(&mut self, binding: *const DML_BINDING_DESC);
    fn BindPersistentResource(&mut self, binding: *const DML_BINDING_DESC);
    fn Reset(&mut self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLBindingTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLBindingTable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLBindingTable_Vtbl {
        unsafe extern "system" fn BindInputs<Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindInputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindOutputs<Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindOutputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindTemporaryResource<Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindTemporaryResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn BindPersistentResource<Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindPersistentResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn Reset<Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&desc)).into()
        }
        Self {
            base: IDMLDeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDMLCommandRecorder_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl {
    fn RecordDispatch(&mut self, commandlist: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandList>, dispatchable: ::core::option::Option<IDMLDispatchable>, bindings: ::core::option::Option<IDMLBindingTable>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLCommandRecorder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCommandRecorder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCommandRecorder_Vtbl {
        unsafe extern "system" fn RecordDispatch<Impl: IDMLCommandRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, dispatchable: ::windows::core::RawPtr, bindings: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordDispatch(::core::mem::transmute(&commandlist), ::core::mem::transmute(&dispatchable), ::core::mem::transmute(&bindings))
        }
        Self { base: IDMLDeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RecordDispatch: RecordDispatch::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCommandRecorder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLCompiledOperator_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl + IDMLPageable_Impl + IDMLDispatchable_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLCompiledOperator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCompiledOperator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCompiledOperator_Vtbl {
        Self { base: IDMLDispatchable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCompiledOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDebugDevice_Impl: Sized {
    fn SetMuteDebugOutput(&mut self, mute: super::super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDebugDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDebugDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDebugDevice_Vtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDMLDebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
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
pub trait IDMLDevice_Impl: Sized + IDMLObject_Impl {
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
impl IDMLDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDevice_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&featurequerydatasize), ::core::mem::transmute_copy(&featurequerydata), ::core::mem::transmute_copy(&featuresupportdatasize), ::core::mem::transmute_copy(&featuresupportdata)).into()
        }
        unsafe extern "system" fn CreateOperator<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateOperator(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CompileOperator<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileOperator(::core::mem::transmute(&op), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateOperatorInitializer<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateOperatorInitializer(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCommandRecorder<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCommandRecorder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBindingTable<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBindingTable(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Evict<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Evict(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn MakeResident<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeResident(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetParentDevice<Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParentDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: IDMLObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDMLDevice1_Impl: Sized + IDMLObject_Impl + IDMLDevice_Impl {
    fn CompileGraph(&mut self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDevice1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDevice1_Vtbl {
        unsafe extern "system" fn CompileGraph<Impl: IDMLDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileGraph(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: IDMLDevice_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CompileGraph: CompileGraph::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDeviceChild_Impl: Sized + IDMLObject_Impl {
    fn GetDevice(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDeviceChild_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Impl: IDMLDeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: IDMLObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDispatchable_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl + IDMLPageable_Impl {
    fn GetBindingProperties(&mut self) -> DML_BINDING_PROPERTIES;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDispatchable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDispatchable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDispatchable_Vtbl {
        unsafe extern "system" fn GetBindingProperties<Impl: IDMLDispatchable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindingProperties()
        }
        Self { base: IDMLPageable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetBindingProperties: GetBindingProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDispatchable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLObject_Impl: Sized {
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, data: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetName(&mut self, name: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLObject_Vtbl {
        unsafe extern "system" fn GetPrivateData<Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn SetName<Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IDMLOperator_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperator_Vtbl {
        Self { base: IDMLDeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLOperatorInitializer_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl + IDMLPageable_Impl + IDMLDispatchable_Impl {
    fn Reset(&mut self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperatorInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperatorInitializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperatorInitializer_Vtbl {
        unsafe extern "system" fn Reset<Impl: IDMLOperatorInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators)).into()
        }
        Self { base: IDMLDispatchable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Reset: Reset::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperatorInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLPageable_Impl: Sized + IDMLObject_Impl + IDMLDeviceChild_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLPageable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLPageable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLPageable_Vtbl {
        Self { base: IDMLDeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLPageable as ::windows::core::Interface>::IID
    }
}

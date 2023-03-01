#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLBindingTable_Impl: Sized + IDMLDeviceChild_Impl {
    fn BindInputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindOutputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindTemporaryResource(&self, binding: *const DML_BINDING_DESC);
    fn BindPersistentResource(&self, binding: *const DML_BINDING_DESC);
    fn Reset(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IDMLBindingTable {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLBindingTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>() -> IDMLBindingTable_Vtbl {
        unsafe extern "system" fn BindInputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindInputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindOutputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindOutputs(::core::mem::transmute_copy(&bindingcount), ::core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindTemporaryResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindTemporaryResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn BindPersistentResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindPersistentResource(::core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&desc)).into()
        }
        Self {
            base__: IDMLDeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            BindInputs: BindInputs::<Identity, Impl, OFFSET>,
            BindOutputs: BindOutputs::<Identity, Impl, OFFSET>,
            BindTemporaryResource: BindTemporaryResource::<Identity, Impl, OFFSET>,
            BindPersistentResource: BindPersistentResource::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLBindingTable as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLCommandRecorder_Impl: Sized + IDMLDeviceChild_Impl {
    fn RecordDispatch(&self, commandlist: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandList>, dispatchable: ::core::option::Option<&IDMLDispatchable>, bindings: ::core::option::Option<&IDMLBindingTable>);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IDMLCommandRecorder {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLCommandRecorder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLCommandRecorder_Impl, const OFFSET: isize>() -> IDMLCommandRecorder_Vtbl {
        unsafe extern "system" fn RecordDispatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLCommandRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: *mut ::core::ffi::c_void, dispatchable: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordDispatch(::windows::core::from_raw_borrowed(&commandlist), ::windows::core::from_raw_borrowed(&dispatchable), ::windows::core::from_raw_borrowed(&bindings))
        }
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), RecordDispatch: RecordDispatch::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCommandRecorder as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLCompiledOperator_Impl: Sized + IDMLDispatchable_Impl {}
impl ::windows::core::RuntimeName for IDMLCompiledOperator {}
impl IDMLCompiledOperator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLCompiledOperator_Impl, const OFFSET: isize>() -> IDMLCompiledOperator_Vtbl {
        Self { base__: IDMLDispatchable_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCompiledOperator as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID || iid == &<IDMLPageable as ::windows::core::ComInterface>::IID || iid == &<IDMLDispatchable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDebugDevice_Impl: Sized {
    fn SetMuteDebugOutput(&self, mute: super::super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDMLDebugDevice {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDebugDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDebugDevice_Impl, const OFFSET: isize>() -> IDMLDebugDevice_Vtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDebugDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMuteDebugOutput(::core::mem::transmute_copy(&mute))
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDebugDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice_Impl: Sized + IDMLObject_Impl {
    fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateOperator(&self, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CompileOperator(&self, op: ::core::option::Option<&IDMLOperator>, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateOperatorInitializer(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateCommandRecorder(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateBindingTable(&self, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Evict(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()>;
    fn MakeResident(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()>;
    fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()>;
    fn GetParentDevice(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IDMLDevice {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>() -> IDMLDevice_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&featurequerydatasize), ::core::mem::transmute_copy(&featurequerydata), ::core::mem::transmute_copy(&featuresupportdatasize), ::core::mem::transmute_copy(&featuresupportdata)).into()
        }
        unsafe extern "system" fn CreateOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateOperator(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CompileOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, op: *mut ::core::ffi::c_void, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompileOperator(::windows::core::from_raw_borrowed(&op), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateOperatorInitializer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateOperatorInitializer(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCommandRecorder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateCommandRecorder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBindingTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateBindingTable(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Evict<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Evict(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn MakeResident<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeResident(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetParentDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParentDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: IDMLObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            CreateOperator: CreateOperator::<Identity, Impl, OFFSET>,
            CompileOperator: CompileOperator::<Identity, Impl, OFFSET>,
            CreateOperatorInitializer: CreateOperatorInitializer::<Identity, Impl, OFFSET>,
            CreateCommandRecorder: CreateCommandRecorder::<Identity, Impl, OFFSET>,
            CreateBindingTable: CreateBindingTable::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            MakeResident: MakeResident::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetParentDevice: GetParentDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice1_Impl: Sized + IDMLDevice_Impl {
    fn CompileGraph(&self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IDMLDevice1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice1_Impl, const OFFSET: isize>() -> IDMLDevice1_Vtbl {
        unsafe extern "system" fn CompileGraph<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompileGraph(::core::mem::transmute_copy(&desc), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IDMLDevice_Vtbl::new::<Identity, Impl, OFFSET>(), CompileGraph: CompileGraph::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice1 as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLDeviceChild_Impl: Sized + IDMLObject_Impl {
    fn GetDevice(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDMLDeviceChild {}
impl IDMLDeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDeviceChild_Impl, const OFFSET: isize>() -> IDMLDeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IDMLObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLDispatchable_Impl: Sized + IDMLPageable_Impl {
    fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES;
}
impl ::windows::core::RuntimeName for IDMLDispatchable {}
impl IDMLDispatchable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDispatchable_Impl, const OFFSET: isize>() -> IDMLDispatchable_Vtbl {
        unsafe extern "system" fn GetBindingProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLDispatchable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = this.GetBindingProperties()
        }
        Self { base__: IDMLPageable_Vtbl::new::<Identity, Impl, OFFSET>(), GetBindingProperties: GetBindingProperties::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDispatchable as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID || iid == &<IDMLPageable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLObject_Impl: Sized {
    fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, data: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetName(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDMLObject {}
impl IDMLObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: isize>() -> IDMLObject_Vtbl {
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::windows::core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLOperator_Impl: Sized + IDMLDeviceChild_Impl {}
impl ::windows::core::RuntimeName for IDMLOperator {}
impl IDMLOperator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLOperator_Impl, const OFFSET: isize>() -> IDMLOperator_Vtbl {
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperator as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLOperatorInitializer_Impl: Sized + IDMLDispatchable_Impl {
    fn Reset(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDMLOperatorInitializer {}
impl IDMLOperatorInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLOperatorInitializer_Impl, const OFFSET: isize>() -> IDMLOperatorInitializer_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLOperatorInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&operatorcount), ::core::mem::transmute_copy(&operators)).into()
        }
        Self { base__: IDMLDispatchable_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperatorInitializer as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID || iid == &<IDMLPageable as ::windows::core::ComInterface>::IID || iid == &<IDMLDispatchable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"implement\"`*"]
pub trait IDMLPageable_Impl: Sized + IDMLDeviceChild_Impl {}
impl ::windows::core::RuntimeName for IDMLPageable {}
impl IDMLPageable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMLPageable_Impl, const OFFSET: isize>() -> IDMLPageable_Vtbl {
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLPageable as ::windows::core::ComInterface>::IID || iid == &<IDMLObject as ::windows::core::ComInterface>::IID || iid == &<IDMLDeviceChild as ::windows::core::ComInterface>::IID
    }
}

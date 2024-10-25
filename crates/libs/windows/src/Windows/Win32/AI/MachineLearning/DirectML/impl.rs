#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLBindingTable_Impl: Sized + IDMLDeviceChild_Impl {
    fn BindInputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindOutputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC);
    fn BindTemporaryResource(&self, binding: *const DML_BINDING_DESC);
    fn BindPersistentResource(&self, binding: *const DML_BINDING_DESC);
    fn Reset(&self, desc: *const DML_BINDING_TABLE_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IDMLBindingTable {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLBindingTable_Vtbl {
    pub const fn new<Identity: IDMLBindingTable_Impl, const OFFSET: isize>() -> IDMLBindingTable_Vtbl {
        unsafe extern "system" fn BindInputs<Identity: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLBindingTable_Impl::BindInputs(this, core::mem::transmute_copy(&bindingcount), core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindOutputs<Identity: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLBindingTable_Impl::BindOutputs(this, core::mem::transmute_copy(&bindingcount), core::mem::transmute_copy(&bindings))
        }
        unsafe extern "system" fn BindTemporaryResource<Identity: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLBindingTable_Impl::BindTemporaryResource(this, core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn BindPersistentResource<Identity: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLBindingTable_Impl::BindPersistentResource(this, core::mem::transmute_copy(&binding))
        }
        unsafe extern "system" fn Reset<Identity: IDMLBindingTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLBindingTable_Impl::Reset(this, core::mem::transmute_copy(&desc)).into()
        }
        Self {
            base__: IDMLDeviceChild_Vtbl::new::<Identity, OFFSET>(),
            BindInputs: BindInputs::<Identity, OFFSET>,
            BindOutputs: BindOutputs::<Identity, OFFSET>,
            BindTemporaryResource: BindTemporaryResource::<Identity, OFFSET>,
            BindPersistentResource: BindPersistentResource::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLBindingTable as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLCommandRecorder_Impl: Sized + IDMLDeviceChild_Impl {
    fn RecordDispatch(&self, commandlist: Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandList>, dispatchable: Option<&IDMLDispatchable>, bindings: Option<&IDMLBindingTable>);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IDMLCommandRecorder {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLCommandRecorder_Vtbl {
    pub const fn new<Identity: IDMLCommandRecorder_Impl, const OFFSET: isize>() -> IDMLCommandRecorder_Vtbl {
        unsafe extern "system" fn RecordDispatch<Identity: IDMLCommandRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandlist: *mut core::ffi::c_void, dispatchable: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLCommandRecorder_Impl::RecordDispatch(this, windows_core::from_raw_borrowed(&commandlist), windows_core::from_raw_borrowed(&dispatchable), windows_core::from_raw_borrowed(&bindings))
        }
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, OFFSET>(), RecordDispatch: RecordDispatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLCommandRecorder as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID
    }
}
pub trait IDMLCompiledOperator_Impl: Sized + IDMLDispatchable_Impl {}
impl windows_core::RuntimeName for IDMLCompiledOperator {}
impl IDMLCompiledOperator_Vtbl {
    pub const fn new<Identity: IDMLCompiledOperator_Impl, const OFFSET: isize>() -> IDMLCompiledOperator_Vtbl {
        Self { base__: IDMLDispatchable_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLCompiledOperator as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID || iid == &<IDMLPageable as windows_core::Interface>::IID || iid == &<IDMLDispatchable as windows_core::Interface>::IID
    }
}
pub trait IDMLDebugDevice_Impl: Sized + windows_core::IUnknownImpl {
    fn SetMuteDebugOutput(&self, mute: super::super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for IDMLDebugDevice {}
impl IDMLDebugDevice_Vtbl {
    pub const fn new<Identity: IDMLDebugDevice_Impl, const OFFSET: isize>() -> IDMLDebugDevice_Vtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Identity: IDMLDebugDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDebugDevice_Impl::SetMuteDebugOutput(this, core::mem::transmute_copy(&mute))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLDebugDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice_Impl: Sized + IDMLObject_Impl {
    fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateOperator(&self, desc: *const DML_OPERATOR_DESC, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CompileOperator(&self, op: Option<&IDMLOperator>, flags: DML_EXECUTION_FLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateOperatorInitializer(&self, operatorcount: u32, operators: *const Option<IDMLCompiledOperator>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCommandRecorder(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateBindingTable(&self, desc: *const DML_BINDING_TABLE_DESC, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Evict(&self, count: u32, ppobjects: *const Option<IDMLPageable>) -> windows_core::Result<()>;
    fn MakeResident(&self, count: u32, ppobjects: *const Option<IDMLPageable>) -> windows_core::Result<()>;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetParentDevice(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IDMLDevice {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLDevice_Vtbl {
    pub const fn new<Identity: IDMLDevice_Impl, const OFFSET: isize>() -> IDMLDevice_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&featurequerydatasize), core::mem::transmute_copy(&featurequerydata), core::mem::transmute_copy(&featuresupportdatasize), core::mem::transmute_copy(&featuresupportdata)).into()
        }
        unsafe extern "system" fn CreateOperator<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CreateOperator(this, core::mem::transmute_copy(&desc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CompileOperator<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, op: *mut core::ffi::c_void, flags: DML_EXECUTION_FLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CompileOperator(this, windows_core::from_raw_borrowed(&op), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateOperatorInitializer<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorcount: u32, operators: *const *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CreateOperatorInitializer(this, core::mem::transmute_copy(&operatorcount), core::mem::transmute_copy(&operators), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCommandRecorder<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CreateCommandRecorder(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBindingTable<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::CreateBindingTable(this, core::mem::transmute_copy(&desc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Evict<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, ppobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::Evict(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn MakeResident<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, ppobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::MakeResident(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::GetDeviceRemovedReason(this).into()
        }
        unsafe extern "system" fn GetParentDevice<Identity: IDMLDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice_Impl::GetParentDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: IDMLObject_Vtbl::new::<Identity, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            CreateOperator: CreateOperator::<Identity, OFFSET>,
            CompileOperator: CompileOperator::<Identity, OFFSET>,
            CreateOperatorInitializer: CreateOperatorInitializer::<Identity, OFFSET>,
            CreateCommandRecorder: CreateCommandRecorder::<Identity, OFFSET>,
            CreateBindingTable: CreateBindingTable::<Identity, OFFSET>,
            Evict: Evict::<Identity, OFFSET>,
            MakeResident: MakeResident::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            GetParentDevice: GetParentDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLDevice as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IDMLDevice1_Impl: Sized + IDMLDevice_Impl {
    fn CompileGraph(&self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IDMLDevice1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IDMLDevice1_Vtbl {
    pub const fn new<Identity: IDMLDevice1_Impl, const OFFSET: isize>() -> IDMLDevice1_Vtbl {
        unsafe extern "system" fn CompileGraph<Identity: IDMLDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDevice1_Impl::CompileGraph(this, core::mem::transmute_copy(&desc), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IDMLDevice_Vtbl::new::<Identity, OFFSET>(), CompileGraph: CompileGraph::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLDevice1 as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDevice as windows_core::Interface>::IID
    }
}
pub trait IDMLDeviceChild_Impl: Sized + IDMLObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDMLDeviceChild {}
impl IDMLDeviceChild_Vtbl {
    pub const fn new<Identity: IDMLDeviceChild_Impl, const OFFSET: isize>() -> IDMLDeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: IDMLDeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLDeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IDMLObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLDeviceChild as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID
    }
}
pub trait IDMLDispatchable_Impl: Sized + IDMLPageable_Impl {
    fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES;
}
impl windows_core::RuntimeName for IDMLDispatchable {}
impl IDMLDispatchable_Vtbl {
    pub const fn new<Identity: IDMLDispatchable_Impl, const OFFSET: isize>() -> IDMLDispatchable_Vtbl {
        unsafe extern "system" fn GetBindingProperties<Identity: IDMLDispatchable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = IDMLDispatchable_Impl::GetBindingProperties(this)
        }
        Self { base__: IDMLPageable_Vtbl::new::<Identity, OFFSET>(), GetBindingProperties: GetBindingProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLDispatchable as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID || iid == &<IDMLPageable as windows_core::Interface>::IID
    }
}
pub trait IDMLObject_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPrivateData(&self, guid: *const windows_core::GUID, datasize: *mut u32, data: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, data: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, data: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDMLObject {}
impl IDMLObject_Vtbl {
    pub const fn new<Identity: IDMLObject_Impl, const OFFSET: isize>() -> IDMLObject_Vtbl {
        unsafe extern "system" fn GetPrivateData<Identity: IDMLObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: *mut u32, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDMLObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, data: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: IDMLObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn SetName<Identity: IDMLObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLObject_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLObject as windows_core::Interface>::IID
    }
}
pub trait IDMLOperator_Impl: Sized + IDMLDeviceChild_Impl {}
impl windows_core::RuntimeName for IDMLOperator {}
impl IDMLOperator_Vtbl {
    pub const fn new<Identity: IDMLOperator_Impl, const OFFSET: isize>() -> IDMLOperator_Vtbl {
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLOperator as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID
    }
}
pub trait IDMLOperatorInitializer_Impl: Sized + IDMLDispatchable_Impl {
    fn Reset(&self, operatorcount: u32, operators: *const Option<IDMLCompiledOperator>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDMLOperatorInitializer {}
impl IDMLOperatorInitializer_Vtbl {
    pub const fn new<Identity: IDMLOperatorInitializer_Impl, const OFFSET: isize>() -> IDMLOperatorInitializer_Vtbl {
        unsafe extern "system" fn Reset<Identity: IDMLOperatorInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorcount: u32, operators: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMLOperatorInitializer_Impl::Reset(this, core::mem::transmute_copy(&operatorcount), core::mem::transmute_copy(&operators)).into()
        }
        Self { base__: IDMLDispatchable_Vtbl::new::<Identity, OFFSET>(), Reset: Reset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLOperatorInitializer as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID || iid == &<IDMLPageable as windows_core::Interface>::IID || iid == &<IDMLDispatchable as windows_core::Interface>::IID
    }
}
pub trait IDMLPageable_Impl: Sized + IDMLDeviceChild_Impl {}
impl windows_core::RuntimeName for IDMLPageable {}
impl IDMLPageable_Vtbl {
    pub const fn new<Identity: IDMLPageable_Impl, const OFFSET: isize>() -> IDMLPageable_Vtbl {
        Self { base__: IDMLDeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMLPageable as windows_core::Interface>::IID || iid == &<IDMLObject as windows_core::Interface>::IID || iid == &<IDMLDeviceChild as windows_core::Interface>::IID
    }
}

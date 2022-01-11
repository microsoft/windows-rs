#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLBindingTableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn BindInputs();
    fn BindOutputs();
    fn BindTemporaryResource();
    fn BindPersistentResource();
    fn Reset();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLBindingTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLBindingTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLBindingTableVtbl {
        unsafe extern "system" fn BindInputs<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindOutputs<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindTemporaryResource<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindPersistentResource<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IDMLBindingTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            BindInputs::<Impl, IMPL_OFFSET>,
            BindOutputs::<Impl, IMPL_OFFSET>,
            BindTemporaryResource::<Impl, IMPL_OFFSET>,
            BindPersistentResource::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLBindingTable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLCommandRecorderImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn RecordDispatch();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLCommandRecorderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCommandRecorderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCommandRecorderVtbl {
        unsafe extern "system" fn RecordDispatch<Impl: IDMLCommandRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, dispatchable: ::windows::core::RawPtr, bindings: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, RecordDispatch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCommandRecorder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLCompiledOperatorImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLCompiledOperatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLCompiledOperatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLCompiledOperatorVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetBindingProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLCompiledOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDebugDeviceImpl: Sized {
    fn SetMuteDebugOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDebugDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDebugDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDebugDeviceVtbl {
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDMLDebugDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetMuteDebugOutput::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDebugDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDeviceVtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOperator<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompileOperator<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOperatorInitializer<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCommandRecorder<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBindingTable<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Evict<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeResident<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentDevice<Impl: IDMLDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            CreateOperator::<Impl, IMPL_OFFSET>,
            CompileOperator::<Impl, IMPL_OFFSET>,
            CreateOperatorInitializer::<Impl, IMPL_OFFSET>,
            CreateCommandRecorder::<Impl, IMPL_OFFSET>,
            CreateBindingTable::<Impl, IMPL_OFFSET>,
            Evict::<Impl, IMPL_OFFSET>,
            MakeResident::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetParentDevice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub trait IDMLDevice1Impl: Sized + IDMLDeviceImpl + IDMLObjectImpl {
    fn CompileGraph();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl IDMLDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDevice1Vtbl {
        unsafe extern "system" fn CompileGraph<Impl: IDMLDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            CreateOperator::<Impl, IMPL_OFFSET>,
            CompileOperator::<Impl, IMPL_OFFSET>,
            CreateOperatorInitializer::<Impl, IMPL_OFFSET>,
            CreateCommandRecorder::<Impl, IMPL_OFFSET>,
            CreateBindingTable::<Impl, IMPL_OFFSET>,
            Evict::<Impl, IMPL_OFFSET>,
            MakeResident::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetParentDevice::<Impl, IMPL_OFFSET>,
            CompileGraph::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDeviceChildImpl: Sized + IDMLObjectImpl {
    fn GetDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDeviceChildImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDMLDeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLDispatchableImpl: Sized + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn GetBindingProperties();
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLDispatchableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLDispatchableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLDispatchableVtbl {
        unsafe extern "system" fn GetBindingProperties<Impl: IDMLDispatchableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetBindingProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLDispatchable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLObjectImpl: Sized {
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn SetName();
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLObjectVtbl {
        unsafe extern "system" fn GetPrivateData<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IDMLObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLOperatorImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperatorVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLOperatorInitializerImpl: Sized + IDMLDispatchableImpl + IDMLPageableImpl + IDMLDeviceChildImpl + IDMLObjectImpl {
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IDMLOperatorInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLOperatorInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLOperatorInitializerVtbl {
        unsafe extern "system" fn Reset<Impl: IDMLOperatorInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetBindingProperties::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLOperatorInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDMLPageableImpl: Sized + IDMLDeviceChildImpl + IDMLObjectImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IDMLPageableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMLPageableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMLPageableVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMLPageable as ::windows::core::Interface>::IID
    }
}

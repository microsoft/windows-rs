#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ILearningModelDeviceFactoryNativeImpl: Sized {
    fn CreateFromD3D12CommandQueue();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ILearningModelDeviceFactoryNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDeviceFactoryNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelDeviceFactoryNativeVtbl {
        unsafe extern "system" fn CreateFromD3D12CommandQueue<Impl: ILearningModelDeviceFactoryNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateFromD3D12CommandQueue: CreateFromD3D12CommandQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDeviceFactoryNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub trait ILearningModelOperatorProviderNativeImpl: Sized {
    fn GetRegistry();
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl ILearningModelOperatorProviderNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelOperatorProviderNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelOperatorProviderNativeVtbl {
        unsafe extern "system" fn GetRegistry<Impl: ILearningModelOperatorProviderNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRegistry: GetRegistry::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelOperatorProviderNative as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelSessionOptionsNativeImpl: Sized {
    fn SetIntraOpNumThreadsOverride();
}
impl ILearningModelSessionOptionsNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionOptionsNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionOptionsNativeVtbl {
        unsafe extern "system" fn SetIntraOpNumThreadsOverride<Impl: ILearningModelSessionOptionsNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetIntraOpNumThreadsOverride: SetIntraOpNumThreadsOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionOptionsNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorNativeImpl: Sized {
    fn GetBuffer();
    fn GetD3D12Resource();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorNativeVtbl {
        unsafe extern "system" fn GetBuffer<Impl: ITensorNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetD3D12Resource<Impl: ITensorNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            GetD3D12Resource: GetD3D12Resource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorStaticsNativeImpl: Sized {
    fn CreateFromD3D12Resource();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorStaticsNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorStaticsNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorStaticsNativeVtbl {
        unsafe extern "system" fn CreateFromD3D12Resource<Impl: ITensorStaticsNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateFromD3D12Resource: CreateFromD3D12Resource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorStaticsNative as ::windows::core::Interface>::IID
    }
}

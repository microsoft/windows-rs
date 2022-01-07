pub trait ILearningModelDeviceFactoryNativeImpl: Sized {
    fn CreateFromD3D12CommandQueue();
}
impl ::windows::core::RuntimeName for ILearningModelDeviceFactoryNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ML.ILearningModelDeviceFactoryNative";
}
impl ILearningModelDeviceFactoryNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDeviceFactoryNativeImpl, const OFFSET: isize>() -> ILearningModelDeviceFactoryNativeVtbl {
        unsafe extern "system" fn CreateFromD3D12CommandQueue<Impl: ILearningModelDeviceFactoryNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromD3D12CommandQueue(&*(&value as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelDeviceFactoryNative>, ::windows::core::GetTrustLevel, CreateFromD3D12CommandQueue::<Impl, OFFSET>)
    }
}
pub trait ILearningModelOperatorProviderNativeImpl: Sized {
    fn GetRegistry();
}
impl ::windows::core::RuntimeName for ILearningModelOperatorProviderNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ML.ILearningModelOperatorProviderNative";
}
impl ILearningModelOperatorProviderNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelOperatorProviderNativeImpl, const OFFSET: isize>() -> ILearningModelOperatorProviderNativeVtbl {
        unsafe extern "system" fn GetRegistry<Impl: ILearningModelOperatorProviderNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistry(::core::mem::transmute_copy(&ppoperatorregistry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelOperatorProviderNative>, ::windows::core::GetTrustLevel, GetRegistry::<Impl, OFFSET>)
    }
}
pub trait ILearningModelSessionOptionsNativeImpl: Sized {
    fn SetIntraOpNumThreadsOverride();
}
impl ::windows::core::RuntimeName for ILearningModelSessionOptionsNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ML.ILearningModelSessionOptionsNative";
}
impl ILearningModelSessionOptionsNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionOptionsNativeImpl, const OFFSET: isize>() -> ILearningModelSessionOptionsNativeVtbl {
        unsafe extern "system" fn SetIntraOpNumThreadsOverride<Impl: ILearningModelSessionOptionsNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIntraOpNumThreadsOverride(intraopnumthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelSessionOptionsNative>, ::windows::core::GetTrustLevel, SetIntraOpNumThreadsOverride::<Impl, OFFSET>)
    }
}
pub trait ITensorNativeImpl: Sized {
    fn GetBuffer();
    fn GetD3D12Resource();
}
impl ::windows::core::RuntimeName for ITensorNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ML.ITensorNative";
}
impl ITensorNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorNativeImpl, const OFFSET: isize>() -> ITensorNativeVtbl {
        unsafe extern "system" fn GetBuffer<Impl: ITensorNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetD3D12Resource<Impl: ITensorNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetD3D12Resource(::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITensorNative>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, GetD3D12Resource::<Impl, OFFSET>)
    }
}
pub trait ITensorStaticsNativeImpl: Sized {
    fn CreateFromD3D12Resource();
}
impl ::windows::core::RuntimeName for ITensorStaticsNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ML.ITensorStaticsNative";
}
impl ITensorStaticsNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorStaticsNativeImpl, const OFFSET: isize>() -> ITensorStaticsNativeVtbl {
        unsafe extern "system" fn CreateFromD3D12Resource<Impl: ITensorStaticsNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromD3D12Resource(&*(&value as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType), shape, shapecount, ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITensorStaticsNative>, ::windows::core::GetTrustLevel, CreateFromD3D12Resource::<Impl, OFFSET>)
    }
}

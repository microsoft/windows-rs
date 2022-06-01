#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ILearningModelDeviceFactoryNative_Impl: Sized {
    fn CreateFromD3D12CommandQueue(&self, value: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for ILearningModelDeviceFactoryNative {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ILearningModelDeviceFactoryNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelDeviceFactoryNative_Impl, const OFFSET: isize>() -> ILearningModelDeviceFactoryNative_Vtbl {
        unsafe extern "system" fn CreateFromD3D12CommandQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelDeviceFactoryNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFromD3D12CommandQueue(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateFromD3D12CommandQueue: CreateFromD3D12CommandQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDeviceFactoryNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub trait ILearningModelOperatorProviderNative_Impl: Sized {
    fn GetRegistry(&self) -> ::windows::core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>;
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl ::windows::core::RuntimeName for ILearningModelOperatorProviderNative {}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl ILearningModelOperatorProviderNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelOperatorProviderNative_Impl, const OFFSET: isize>() -> ILearningModelOperatorProviderNative_Vtbl {
        unsafe extern "system" fn GetRegistry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelOperatorProviderNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegistry() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperatorregistry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRegistry: GetRegistry::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelOperatorProviderNative as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelSessionOptionsNative_Impl: Sized {
    fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILearningModelSessionOptionsNative {}
impl ILearningModelSessionOptionsNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelSessionOptionsNative_Impl, const OFFSET: isize>() -> ILearningModelSessionOptionsNative_Vtbl {
        unsafe extern "system" fn SetIntraOpNumThreadsOverride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelSessionOptionsNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntraOpNumThreadsOverride(::core::mem::transmute_copy(&intraopnumthreads)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetIntraOpNumThreadsOverride: SetIntraOpNumThreadsOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionOptionsNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorNative_Impl: Sized {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()>;
    fn GetD3D12Resource(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for ITensorNative {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: isize>() -> ITensorNative_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)).into()
        }
        unsafe extern "system" fn GetD3D12Resource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensorNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetD3D12Resource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetD3D12Resource: GetD3D12Resource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorStaticsNative_Impl: Sized {
    fn CreateFromD3D12Resource(&self, value: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for ITensorStaticsNative {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorStaticsNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensorStaticsNative_Impl, const OFFSET: isize>() -> ITensorStaticsNative_Vtbl {
        unsafe extern "system" fn CreateFromD3D12Resource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensorStaticsNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateFromD3D12Resource(::core::mem::transmute(&value), ::core::mem::transmute_copy(&shape), ::core::mem::transmute_copy(&shapecount), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateFromD3D12Resource: CreateFromD3D12Resource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorStaticsNative as ::windows::core::Interface>::IID
    }
}

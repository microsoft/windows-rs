pub trait IHolographicCameraInteropImpl: Sized {
    fn CreateDirect3D12BackBufferResource();
    fn CreateDirect3D12HardwareProtectedBackBufferResource();
    fn AcquireDirect3D12BufferResource();
    fn AcquireDirect3D12BufferResourceWithTimeout();
    fn UnacquireDirect3D12BufferResource();
}
impl ::windows::core::RuntimeName for IHolographicCameraInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Holographic.IHolographicCameraInterop";
}
impl IHolographicCameraInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInteropImpl, const OFFSET: isize>() -> IHolographicCameraInteropVtbl {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12BackBufferResource(
                &*(&pdevice as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::DefaultType>::DefaultType),
                &*(&ptexture2ddesc as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcreatedtexture2dresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedBackBufferResource(
                &*(&pdevice as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::DefaultType>::DefaultType),
                &*(&ptexture2ddesc as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedresourcesession as *const <super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcreatedtexture2dresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireDirect3D12BufferResource(&*(&presourcetoacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&pcommandqueue as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireDirect3D12BufferResourceWithTimeout(
                &*(&presourcetoacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcommandqueue as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType),
                duration,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnacquireDirect3D12BufferResource(&*(&presourcetounacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicCameraInterop>,
            ::windows::core::GetTrustLevel,
            CreateDirect3D12BackBufferResource::<Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource::<Impl, OFFSET>,
            AcquireDirect3D12BufferResource::<Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout::<Impl, OFFSET>,
            UnacquireDirect3D12BufferResource::<Impl, OFFSET>,
        )
    }
}
pub trait IHolographicCameraRenderingParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource();
    fn CommitDirect3D12ResourceWithDepthData();
}
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParametersInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Holographic.IHolographicCameraRenderingParametersInterop";
}
impl IHolographicCameraRenderingParametersInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersInteropImpl, const OFFSET: isize>() -> IHolographicCameraRenderingParametersInteropVtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Impl: IHolographicCameraRenderingParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitDirect3D12Resource(
                &*(&pcolorresourcetocommit as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcolorresourcefence as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::DefaultType>::DefaultType),
                colorresourcefencesignalvalue,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Impl: IHolographicCameraRenderingParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::core::RawPtr, pdepthresourcefence: ::windows::core::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitDirect3D12ResourceWithDepthData(
                &*(&pcolorresourcetocommit as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcolorresourcefence as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::DefaultType>::DefaultType),
                colorresourcefencesignalvalue,
                &*(&pdepthresourcetocommit as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pdepthresourcefence as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::DefaultType>::DefaultType),
                depthresourcefencesignalvalue,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParametersInterop>, ::windows::core::GetTrustLevel, CommitDirect3D12Resource::<Impl, OFFSET>, CommitDirect3D12ResourceWithDepthData::<Impl, OFFSET>)
    }
}
pub trait IHolographicQuadLayerInteropImpl: Sized {
    fn CreateDirect3D12ContentBufferResource();
    fn CreateDirect3D12HardwareProtectedContentBufferResource();
    fn AcquireDirect3D12BufferResource();
    fn AcquireDirect3D12BufferResourceWithTimeout();
    fn UnacquireDirect3D12BufferResource();
}
impl ::windows::core::RuntimeName for IHolographicQuadLayerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Holographic.IHolographicQuadLayerInterop";
}
impl IHolographicQuadLayerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>() -> IHolographicQuadLayerInteropVtbl {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12ContentBufferResource(
                &*(&pdevice as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::DefaultType>::DefaultType),
                &*(&ptexture2ddesc as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pptexture2dresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedContentBufferResource(
                &*(&pdevice as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Device as ::windows::core::DefaultType>::DefaultType),
                &*(&ptexture2ddesc as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&pprotectedresourcesession as *const <super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcreatedtexture2dresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireDirect3D12BufferResource(&*(&presourcetoacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType), &*(&pcommandqueue as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireDirect3D12BufferResourceWithTimeout(
                &*(&presourcetoacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcommandqueue as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType),
                duration,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnacquireDirect3D12BufferResource(&*(&presourcetounacquire as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerInterop>,
            ::windows::core::GetTrustLevel,
            CreateDirect3D12ContentBufferResource::<Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource::<Impl, OFFSET>,
            AcquireDirect3D12BufferResource::<Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout::<Impl, OFFSET>,
            UnacquireDirect3D12BufferResource::<Impl, OFFSET>,
        )
    }
}
pub trait IHolographicQuadLayerUpdateParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource();
}
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParametersInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Holographic.IHolographicQuadLayerUpdateParametersInterop";
}
impl IHolographicQuadLayerUpdateParametersInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParametersInteropImpl, const OFFSET: isize>() -> IHolographicQuadLayerUpdateParametersInteropVtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Impl: IHolographicQuadLayerUpdateParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitDirect3D12Resource(
                &*(&pcolorresourcetocommit as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcolorresourcefence as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct3D12::ID3D12Fence as ::windows::core::DefaultType>::DefaultType),
                colorresourcefencesignalvalue,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerUpdateParametersInterop>, ::windows::core::GetTrustLevel, CommitDirect3D12Resource::<Impl, OFFSET>)
    }
}

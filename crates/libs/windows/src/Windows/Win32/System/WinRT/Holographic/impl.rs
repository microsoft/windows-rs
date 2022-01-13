#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicCameraInteropImpl: Sized {
    fn CreateDirect3D12BackBufferResource(&mut self, pdevice: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedBackBufferResource(&mut self, pdevice: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&mut self, presourcetoacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows::core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&mut self, presourcetoacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows::core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&mut self, presourcetounacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IHolographicCameraInterop {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicCameraInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraInteropVtbl {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12BackBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedBackBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::core::mem::transmute(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireDirect3D12BufferResource(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireDirect3D12BufferResourceWithTimeout(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Impl: IHolographicCameraInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnacquireDirect3D12BufferResource(::core::mem::transmute(&presourcetounacquire)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicCameraInterop, BASE_OFFSET>(),
            CreateDirect3D12BackBufferResource: CreateDirect3D12BackBufferResource::<Impl, IMPL_OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource: CreateDirect3D12HardwareProtectedBackBufferResource::<Impl, IMPL_OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Impl, IMPL_OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Impl, IMPL_OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicCameraRenderingParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource(&mut self, pcolorresourcetocommit: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
    fn CommitDirect3D12ResourceWithDepthData(&mut self, pcolorresourcetocommit: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pdepthresourcefence: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, depthresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParametersInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicCameraRenderingParametersInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraRenderingParametersInteropVtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Impl: IHolographicCameraRenderingParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitDirect3D12Resource(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Impl: IHolographicCameraRenderingParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::core::RawPtr, pdepthresourcefence: ::windows::core::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitDirect3D12ResourceWithDepthData(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue), ::core::mem::transmute(&pdepthresourcetocommit), ::core::mem::transmute(&pdepthresourcefence), ::core::mem::transmute_copy(&depthresourcefencesignalvalue)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicCameraRenderingParametersInterop, BASE_OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Impl, IMPL_OFFSET>,
            CommitDirect3D12ResourceWithDepthData: CommitDirect3D12ResourceWithDepthData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParametersInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicQuadLayerInteropImpl: Sized {
    fn CreateDirect3D12ContentBufferResource(&mut self, pdevice: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedContentBufferResource(&mut self, pdevice: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&mut self, presourcetoacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows::core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&mut self, presourcetoacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows::core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&mut self, presourcetounacquire: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayerInterop {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicQuadLayerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerInteropVtbl {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12ContentBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedContentBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::core::mem::transmute(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireDirect3D12BufferResource(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireDirect3D12BufferResourceWithTimeout(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Impl: IHolographicQuadLayerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnacquireDirect3D12BufferResource(::core::mem::transmute(&presourcetounacquire)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicQuadLayerInterop, BASE_OFFSET>(),
            CreateDirect3D12ContentBufferResource: CreateDirect3D12ContentBufferResource::<Impl, IMPL_OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource: CreateDirect3D12HardwareProtectedContentBufferResource::<Impl, IMPL_OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Impl, IMPL_OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Impl, IMPL_OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicQuadLayerUpdateParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource(&mut self, pcolorresourcetocommit: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParametersInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicQuadLayerUpdateParametersInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParametersInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerUpdateParametersInteropVtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Impl: IHolographicQuadLayerUpdateParametersInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitDirect3D12Resource(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicQuadLayerUpdateParametersInterop, BASE_OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParametersInterop as ::windows::core::Interface>::IID
    }
}

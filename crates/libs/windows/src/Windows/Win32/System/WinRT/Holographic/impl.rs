#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicCameraInterop_Impl: Sized {
    fn CreateDirect3D12BackBufferResource(&self, pdevice: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedBackBufferResource(&self, pdevice: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows::core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows::core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IHolographicCameraInterop {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicCameraInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>() -> IHolographicCameraInterop_Vtbl {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirect3D12BackBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedBackBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::core::mem::transmute(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireDirect3D12BufferResource(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireDirect3D12BufferResourceWithTimeout(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnacquireDirect3D12BufferResource(::core::mem::transmute(&presourcetounacquire)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicCameraInterop, OFFSET>(),
            CreateDirect3D12BackBufferResource: CreateDirect3D12BackBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource: CreateDirect3D12HardwareProtectedBackBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicCameraRenderingParametersInterop_Impl: Sized {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
    fn CommitDirect3D12ResourceWithDepthData(&self, pcolorresourcetocommit: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pdepthresourcefence: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, depthresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParametersInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicCameraRenderingParametersInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>() -> IHolographicCameraRenderingParametersInterop_Vtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitDirect3D12Resource(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::core::RawPtr, pdepthresourcefence: ::windows::core::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitDirect3D12ResourceWithDepthData(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue), ::core::mem::transmute(&pdepthresourcetocommit), ::core::mem::transmute(&pdepthresourcefence), ::core::mem::transmute_copy(&depthresourcefencesignalvalue)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicCameraRenderingParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
            CommitDirect3D12ResourceWithDepthData: CommitDirect3D12ResourceWithDepthData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParametersInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicQuadLayerInterop_Impl: Sized {
    fn CreateDirect3D12ContentBufferResource(&self, pdevice: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedContentBufferResource(&self, pdevice: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows::core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows::core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayerInterop {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicQuadLayerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>() -> IHolographicQuadLayerInterop_Vtbl {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirect3D12ContentBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::core::RawPtr, ppcreatedtexture2dresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirect3D12HardwareProtectedContentBufferResource(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::core::mem::transmute(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcreatedtexture2dresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireDirect3D12BufferResource(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, duration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireDirect3D12BufferResourceWithTimeout(::core::mem::transmute(&presourcetoacquire), ::core::mem::transmute(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnacquireDirect3D12BufferResource(::core::mem::transmute(&presourcetounacquire)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicQuadLayerInterop, OFFSET>(),
            CreateDirect3D12ContentBufferResource: CreateDirect3D12ContentBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource: CreateDirect3D12HardwareProtectedContentBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicQuadLayerUpdateParametersInterop_Impl: Sized {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: &::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParametersInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>() -> IHolographicQuadLayerUpdateParametersInterop_Vtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows::core::RawPtr, pcolorresourcefence: ::windows::core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitDirect3D12Resource(::core::mem::transmute(&pcolorresourcetocommit), ::core::mem::transmute(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicQuadLayerUpdateParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParametersInterop as ::windows::core::Interface>::IID
    }
}

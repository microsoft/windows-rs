#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicCameraInterop_Impl: Sized {
    fn CreateDirect3D12BackBufferResource(&self, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedBackBufferResource(&self, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::RuntimeName for IHolographicCameraInterop {}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicCameraInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>() -> IHolographicCameraInterop_Vtbl {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDirect3D12BackBufferResource(::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDirect3D12HardwareProtectedBackBufferResource(::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::windows_core::from_raw_borrowed(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireDirect3D12BufferResource(::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireDirect3D12BufferResourceWithTimeout(::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnacquireDirect3D12BufferResource(::windows_core::from_raw_borrowed(&presourcetounacquire)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IHolographicCameraInterop, OFFSET>(),
            CreateDirect3D12BackBufferResource: CreateDirect3D12BackBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource: CreateDirect3D12HardwareProtectedBackBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHolographicCameraInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicCameraRenderingParametersInterop_Impl: Sized {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
    fn CommitDirect3D12ResourceWithDepthData(&self, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pdepthresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, depthresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::RuntimeName for IHolographicCameraRenderingParametersInterop {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicCameraRenderingParametersInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>() -> IHolographicCameraRenderingParametersInterop_Vtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitDirect3D12Resource(::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut ::core::ffi::c_void, pdepthresourcefence: *mut ::core::ffi::c_void, depthresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitDirect3D12ResourceWithDepthData(::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue), ::windows_core::from_raw_borrowed(&pdepthresourcetocommit), ::windows_core::from_raw_borrowed(&pdepthresourcefence), ::core::mem::transmute_copy(&depthresourcefencesignalvalue)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IHolographicCameraRenderingParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
            CommitDirect3D12ResourceWithDepthData: CommitDirect3D12ResourceWithDepthData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParametersInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicQuadLayerInterop_Impl: Sized {
    fn CreateDirect3D12ContentBufferResource(&self, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedContentBufferResource(&self, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::RuntimeName for IHolographicQuadLayerInterop {}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicQuadLayerInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>() -> IHolographicQuadLayerInterop_Vtbl {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDirect3D12ContentBufferResource(::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDirect3D12HardwareProtectedContentBufferResource(::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::windows_core::from_raw_borrowed(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireDirect3D12BufferResource(::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue)).into()
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireDirect3D12BufferResourceWithTimeout(::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnacquireDirect3D12BufferResource(::windows_core::from_raw_borrowed(&presourcetounacquire)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IHolographicQuadLayerInterop, OFFSET>(),
            CreateDirect3D12ContentBufferResource: CreateDirect3D12ContentBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource: CreateDirect3D12HardwareProtectedContentBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHolographicQuadLayerInterop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Holographic\"`, `\"Win32_Graphics_Direct3D12\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicQuadLayerUpdateParametersInterop_Impl: Sized {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::RuntimeName for IHolographicQuadLayerUpdateParametersInterop {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>() -> IHolographicQuadLayerUpdateParametersInterop_Vtbl {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitDirect3D12Resource(::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IHolographicQuadLayerUpdateParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParametersInterop as ::windows_core::ComInterface>::IID
    }
}

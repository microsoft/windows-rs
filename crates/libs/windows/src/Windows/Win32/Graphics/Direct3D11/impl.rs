pub trait ID3D11Asynchronous_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDataSize(&mut self) -> u32;
}
impl ID3D11Asynchronous_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Asynchronous_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Asynchronous_Vtbl {
        unsafe extern "system" fn GetDataSize<Impl: ID3D11Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataSize()
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDataSize: GetDataSize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11AuthenticatedChannel_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCertificateSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetCertificate(&mut self, certificatesize: u32) -> ::windows::core::Result<u8>;
    fn GetChannelHandle(&mut self, pchannelhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11AuthenticatedChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11AuthenticatedChannel_Vtbl {
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificatesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificate(::core::mem::transmute_copy(&certificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelHandle<Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelHandle(::core::mem::transmute_copy(&pchannelhandle))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Impl, IMPL_OFFSET>,
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
            GetChannelHandle: GetChannelHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11AuthenticatedChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11BlendState_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11BlendState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11BlendState_Impl {
    fn GetDesc1(&mut self, pdesc: *mut D3D11_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendState1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11BlendState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11BlendState1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11BlendState_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Buffer_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_BUFFER_DESC);
}
impl ID3D11Buffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Buffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Buffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Buffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Buffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassInstance_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassLinkage(&mut self, pplinkage: *mut ::core::option::Option<ID3D11ClassLinkage>);
    fn GetDesc(&mut self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC);
    fn GetInstanceName(&mut self, pinstancename: super::super::Foundation::PSTR, pbufferlength: *mut usize);
    fn GetTypeName(&mut self, ptypename: super::super::Foundation::PSTR, pbufferlength: *mut usize);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ClassInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ClassInstance_Vtbl {
        unsafe extern "system" fn GetClassLinkage<Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClassLinkage(::core::mem::transmute_copy(&pplinkage))
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetInstanceName<Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstancename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInstanceName(::core::mem::transmute_copy(&pinstancename), ::core::mem::transmute_copy(&pbufferlength))
        }
        unsafe extern "system" fn GetTypeName<Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeName(::core::mem::transmute_copy(&ptypename), ::core::mem::transmute_copy(&pbufferlength))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetClassLinkage: GetClassLinkage::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetInstanceName: GetInstanceName::<Impl, IMPL_OFFSET>,
            GetTypeName: GetTypeName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassLinkage_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassInstance(&mut self, pclassinstancename: super::super::Foundation::PSTR, instanceindex: u32) -> ::windows::core::Result<ID3D11ClassInstance>;
    fn CreateClassInstance(&mut self, pclasstypename: super::super::Foundation::PSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> ::windows::core::Result<ID3D11ClassInstance>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ClassLinkage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ClassLinkage_Vtbl {
        unsafe extern "system" fn GetClassInstance<Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassinstancename: super::super::Foundation::PSTR, instanceindex: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassInstance(::core::mem::transmute_copy(&pclassinstancename), ::core::mem::transmute_copy(&instanceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassInstance<Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclasstypename: super::super::Foundation::PSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClassInstance(::core::mem::transmute_copy(&pclasstypename), ::core::mem::transmute_copy(&constantbufferoffset), ::core::mem::transmute_copy(&constantvectoroffset), ::core::mem::transmute_copy(&textureoffset), ::core::mem::transmute_copy(&sampleroffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetClassInstance: GetClassInstance::<Impl, IMPL_OFFSET>,
            CreateClassInstance: CreateClassInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassLinkage as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11CommandList_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContextFlags(&mut self) -> u32;
}
impl ID3D11CommandList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CommandList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11CommandList_Vtbl {
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11CommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContextFlags()
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetContextFlags: GetContextFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CommandList as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ComputeShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11ComputeShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ComputeShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ComputeShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Counter_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_COUNTER_DESC);
}
impl ID3D11Counter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Counter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Counter_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Counter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Counter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11CryptoSession_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCryptoType(&mut self, pcryptotype: *mut ::windows::core::GUID);
    fn GetDecoderProfile(&mut self, pdecoderprofile: *mut ::windows::core::GUID);
    fn GetCertificateSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetCertificate(&mut self, certificatesize: u32) -> ::windows::core::Result<u8>;
    fn GetCryptoSessionHandle(&mut self, pcryptosessionhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11CryptoSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11CryptoSession_Vtbl {
        unsafe extern "system" fn GetCryptoType<Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCryptoType(::core::mem::transmute_copy(&pcryptotype))
        }
        unsafe extern "system" fn GetDecoderProfile<Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDecoderProfile(::core::mem::transmute_copy(&pdecoderprofile))
        }
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificatesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificate(::core::mem::transmute_copy(&certificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCryptoSessionHandle(::core::mem::transmute_copy(&pcryptosessionhandle))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCryptoType: GetCryptoType::<Impl, IMPL_OFFSET>,
            GetDecoderProfile: GetDecoderProfile::<Impl, IMPL_OFFSET>,
            GetCertificateSize: GetCertificateSize::<Impl, IMPL_OFFSET>,
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
            GetCryptoSessionHandle: GetCryptoSessionHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CryptoSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D11Debug_Impl: Sized {
    fn SetFeatureMask(&mut self, mask: u32) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&mut self) -> u32;
    fn SetPresentPerRenderOpDelay(&mut self, milliseconds: u32) -> ::windows::core::Result<()>;
    fn GetPresentPerRenderOpDelay(&mut self) -> u32;
    fn SetSwapChain(&mut self, pswapchain: &::core::option::Option<super::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&mut self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain>;
    fn ValidateContext(&mut self, pcontext: &::core::option::Option<ID3D11DeviceContext>) -> ::windows::core::Result<()>;
    fn ReportLiveDeviceObjects(&mut self, flags: D3D11_RLDO_FLAGS) -> ::windows::core::Result<()>;
    fn ValidateContextForDispatch(&mut self, pcontext: &::core::option::Option<ID3D11DeviceContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D11Debug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Debug_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureMask()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentPerRenderOpDelay(::core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentPerRenderOpDelay()
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwapChain() {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateContext<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateContext(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ValidateContextForDispatch<Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateContextForDispatch(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Impl, IMPL_OFFSET>,
            GetFeatureMask: GetFeatureMask::<Impl, IMPL_OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            SetSwapChain: SetSwapChain::<Impl, IMPL_OFFSET>,
            GetSwapChain: GetSwapChain::<Impl, IMPL_OFFSET>,
            ValidateContext: ValidateContext::<Impl, IMPL_OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Impl, IMPL_OFFSET>,
            ValidateContextForDispatch: ValidateContextForDispatch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11DepthStencilState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11DepthStencilState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DepthStencilState_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11DepthStencilView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11DepthStencilView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DepthStencilView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device_Impl: Sized {
    fn CreateBuffer(&mut self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Buffer>;
    fn CreateTexture1D(&mut self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture1D>;
    fn CreateTexture2D(&mut self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture2D>;
    fn CreateTexture3D(&mut self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture3D>;
    fn CreateShaderResourceView(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D11ShaderResourceView>;
    fn CreateUnorderedAccessView(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC) -> ::windows::core::Result<ID3D11UnorderedAccessView>;
    fn CreateRenderTargetView(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D11RenderTargetView>;
    fn CreateDepthStencilView(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D11DepthStencilView>;
    fn CreateInputLayout(&mut self, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D11InputLayout>;
    fn CreateVertexShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11VertexShader>;
    fn CreateGeometryShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11GeometryShader>;
    fn CreateGeometryShaderWithStreamOutput(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11GeometryShader>;
    fn CreatePixelShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11PixelShader>;
    fn CreateHullShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11HullShader>;
    fn CreateDomainShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11DomainShader>;
    fn CreateComputeShader(&mut self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11ComputeShader>;
    fn CreateClassLinkage(&mut self) -> ::windows::core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(&mut self, pblendstatedesc: *const D3D11_BLEND_DESC) -> ::windows::core::Result<ID3D11BlendState>;
    fn CreateDepthStencilState(&mut self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D11DepthStencilState>;
    fn CreateRasterizerState(&mut self, prasterizerdesc: *const D3D11_RASTERIZER_DESC) -> ::windows::core::Result<ID3D11RasterizerState>;
    fn CreateSamplerState(&mut self, psamplerdesc: *const D3D11_SAMPLER_DESC) -> ::windows::core::Result<ID3D11SamplerState>;
    fn CreateQuery(&mut self, pquerydesc: *const D3D11_QUERY_DESC) -> ::windows::core::Result<ID3D11Query>;
    fn CreatePredicate(&mut self, ppredicatedesc: *const D3D11_QUERY_DESC) -> ::windows::core::Result<ID3D11Predicate>;
    fn CreateCounter(&mut self, pcounterdesc: *const D3D11_COUNTER_DESC) -> ::windows::core::Result<ID3D11Counter>;
    fn CreateDeferredContext(&mut self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext>;
    fn OpenSharedResource(&mut self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CheckFormatSupport(&mut self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn CheckMultisampleQualityLevels(&mut self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32>;
    fn CheckCounterInfo(&mut self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(&mut self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()>;
    fn CheckFeatureSupport(&mut self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFeatureLevel(&mut self) -> super::Direct3D::D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&mut self) -> u32;
    fn GetDeviceRemovedReason(&mut self) -> ::windows::core::Result<()>;
    fn GetImmediateContext(&mut self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext>);
    fn SetExceptionMode(&mut self, raiseflags: u32) -> ::windows::core::Result<()>;
    fn GetExceptionMode(&mut self) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device_Vtbl {
        unsafe extern "system" fn CreateBuffer<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBuffer(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture1D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture1d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnorderedAccessView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuaview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprtview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInputLayout(::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputlayout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVertexShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvertexshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShaderWithStreamOutput(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&pbufferstrides), ::core::mem::transmute_copy(&numstrides), ::core::mem::transmute_copy(&rasterizedstream), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePixelShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppixelshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHullShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pphullshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHullShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pphullshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppdomainshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDomainShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdomainshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComputeShader<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppcomputeshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComputeShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomputeshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassLinkage<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClassLinkage() {
                ::core::result::Result::Ok(ok__) => {
                    *pplinkage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilState(::core::mem::transmute_copy(&pdepthstencildesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSamplerState(::core::mem::transmute_copy(&psamplerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsamplerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(::core::mem::transmute_copy(&pquerydesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePredicate(::core::mem::transmute_copy(&ppredicatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppredicate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCounter(::core::mem::transmute_copy(&pcounterdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeferredContext<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedResource(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFormatSupport(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformatsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnumqualitylevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounter(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureLevel()
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationFlags()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetImmediateContext<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExceptionMode(::core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExceptionMode()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateBuffer: CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D: CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D: CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D: CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout: CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader: CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader: CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader: CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader: CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader: CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState: CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState: CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery: CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate: CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter: CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource: OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter: CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags: GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext: GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode: SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode: GetExceptionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device1_Impl: Sized + ID3D11Device_Impl {
    fn GetImmediateContext1(&mut self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext1>);
    fn CreateDeferredContext1(&mut self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext1>;
    fn CreateBlendState1(&mut self, pblendstatedesc: *const D3D11_BLEND_DESC1) -> ::windows::core::Result<ID3D11BlendState1>;
    fn CreateRasterizerState1(&mut self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1) -> ::windows::core::Result<ID3D11RasterizerState1>;
    fn CreateDeviceContextState(&mut self, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::core::option::Option<ID3DDeviceContextState>) -> ::windows::core::Result<()>;
    fn OpenSharedResource1(&mut self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenSharedResourceByName(&mut self, lpname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device1_Vtbl {
        unsafe extern "system" fn GetImmediateContext1<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext1(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext1<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext1(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState1(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState1<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState1(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceContextState<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeviceContextState(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pfeaturelevels), ::core::mem::transmute_copy(&featurelevels), ::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute_copy(&emulatedinterface), ::core::mem::transmute_copy(&pchosenfeaturelevel), ::core::mem::transmute_copy(&ppcontextstate)).into()
        }
        unsafe extern "system" fn OpenSharedResource1<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedResource1(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn OpenSharedResourceByName<Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedResourceByName(::core::mem::transmute_copy(&lpname), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        Self {
            base: ID3D11Device_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetImmediateContext1: GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1: CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1: CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1: CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState: CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1: OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName: OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device2_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl {
    fn GetImmediateContext2(&mut self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext2>);
    fn CreateDeferredContext2(&mut self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext2>;
    fn GetResourceTiling(&mut self, ptiledresource: &::core::option::Option<ID3D11Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING);
    fn CheckMultisampleQualityLevels1(&mut self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device2_Vtbl {
        unsafe extern "system" fn GetImmediateContext2<Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext2(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext2<Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext2(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTiling<Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceTiling(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels1(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnumqualitylevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11Device1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetImmediateContext2: GetImmediateContext2::<Impl, IMPL_OFFSET>,
            CreateDeferredContext2: CreateDeferredContext2::<Impl, IMPL_OFFSET>,
            GetResourceTiling: GetResourceTiling::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels1: CheckMultisampleQualityLevels1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device3_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl {
    fn CreateTexture2D1(&mut self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture2D1>;
    fn CreateTexture3D1(&mut self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture3D1>;
    fn CreateRasterizerState2(&mut self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2) -> ::windows::core::Result<ID3D11RasterizerState2>;
    fn CreateShaderResourceView1(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D11ShaderResourceView1>;
    fn CreateUnorderedAccessView1(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1) -> ::windows::core::Result<ID3D11UnorderedAccessView1>;
    fn CreateRenderTargetView1(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1) -> ::windows::core::Result<ID3D11RenderTargetView1>;
    fn CreateQuery1(&mut self, pquerydesc1: *const D3D11_QUERY_DESC1) -> ::windows::core::Result<ID3D11Query1>;
    fn GetImmediateContext3(&mut self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext3>);
    fn CreateDeferredContext3(&mut self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext3>;
    fn WriteToSubresource(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ReadFromSubresource(&mut self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device3_Vtbl {
        unsafe extern "system" fn CreateTexture2D1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D1(::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D1(::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState2<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState2(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnorderedAccessView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuaview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprtview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery1<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery1(::core::mem::transmute_copy(&pquerydesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmediateContext3<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext3(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext3<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext3(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToSubresource<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ReadFromSubresource<Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        Self {
            base: ID3D11Device2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateTexture2D1: CreateTexture2D1::<Impl, IMPL_OFFSET>,
            CreateTexture3D1: CreateTexture3D1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState2: CreateRasterizerState2::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView1: CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView1: CreateUnorderedAccessView1::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView1: CreateRenderTargetView1::<Impl, IMPL_OFFSET>,
            CreateQuery1: CreateQuery1::<Impl, IMPL_OFFSET>,
            GetImmediateContext3: GetImmediateContext3::<Impl, IMPL_OFFSET>,
            CreateDeferredContext3: CreateDeferredContext3::<Impl, IMPL_OFFSET>,
            WriteToSubresource: WriteToSubresource::<Impl, IMPL_OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device4_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl + ID3D11Device3_Impl {
    fn RegisterDeviceRemovedEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterDeviceRemoved(&mut self, dwcookie: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device4_Vtbl {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDeviceRemovedEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDeviceRemoved(::core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base: ID3D11Device3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterDeviceRemovedEvent: RegisterDeviceRemovedEvent::<Impl, IMPL_OFFSET>,
            UnregisterDeviceRemoved: UnregisterDeviceRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device5_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl + ID3D11Device3_Impl + ID3D11Device4_Impl {
    fn OpenSharedFence(&mut self, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateFence(&mut self, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device5_Vtbl {
        unsafe extern "system" fn OpenSharedFence<Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedFence(::core::mem::transmute_copy(&hfence), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn CreateFence<Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateFence(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into()
        }
        Self {
            base: ID3D11Device4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OpenSharedFence: OpenSharedFence::<Impl, IMPL_OFFSET>,
            CreateFence: CreateFence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device5 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DeviceChild_Impl: Sized {
    fn GetDevice(&mut self, ppdevice: *mut ::core::option::Option<ID3D11Device>);
    fn GetPrivateData(&mut self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn PSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSSetShader(&mut self, ppixelshader: &::core::option::Option<ID3D11PixelShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn PSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn VSSetShader(&mut self, pvertexshader: &::core::option::Option<ID3D11VertexShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DrawIndexed(&mut self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&mut self, vertexcount: u32, startvertexlocation: u32);
    fn Map(&mut self, presource: &::core::option::Option<ID3D11Resource>, subresource: u32, maptype: D3D11_MAP, mapflags: u32) -> ::windows::core::Result<D3D11_MAPPED_SUBRESOURCE>;
    fn Unmap(&mut self, presource: &::core::option::Option<ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn IASetInputLayout(&mut self, pinputlayout: &::core::option::Option<ID3D11InputLayout>);
    fn IASetVertexBuffers(&mut self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D11Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&mut self, pindexbuffer: &::core::option::Option<ID3D11Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&mut self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&mut self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn GSSetShader(&mut self, pshader: &::core::option::Option<ID3D11GeometryShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn IASetPrimitiveTopology(&mut self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn Begin(&mut self, pasync: &::core::option::Option<ID3D11Asynchronous>);
    fn End(&mut self, pasync: &::core::option::Option<ID3D11Asynchronous>);
    fn GetData(&mut self, pasync: &::core::option::Option<ID3D11Asynchronous>, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>;
    fn SetPredication(&mut self, ppredicate: &::core::option::Option<ID3D11Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn OMSetRenderTargets(&mut self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>);
    fn OMSetRenderTargetsAndUnorderedAccessViews(&mut self, numrtvs: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn OMSetBlendState(&mut self, pblendstate: &::core::option::Option<ID3D11BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&mut self, pdepthstencilstate: &::core::option::Option<ID3D11DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&mut self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D11Buffer>, poffsets: *const u32);
    fn DrawAuto(&mut self);
    fn DrawIndexedInstancedIndirect(&mut self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn DrawInstancedIndirect(&mut self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn Dispatch(&mut self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(&mut self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn RSSetState(&mut self, prasterizerstate: &::core::option::Option<ID3D11RasterizerState>);
    fn RSSetViewports(&mut self, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(&mut self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
    fn CopyResource(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, psrcresource: &::core::option::Option<ID3D11Resource>);
    fn UpdateSubresource(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn CopyStructureCount(&mut self, pdstbuffer: &::core::option::Option<ID3D11Buffer>, dstalignedbyteoffset: u32, psrcview: &::core::option::Option<ID3D11UnorderedAccessView>);
    fn ClearRenderTargetView(&mut self, prendertargetview: &::core::option::Option<ID3D11RenderTargetView>, colorrgba: *const f32);
    fn ClearUnorderedAccessViewUint(&mut self, punorderedaccessview: &::core::option::Option<ID3D11UnorderedAccessView>, values: *const u32);
    fn ClearUnorderedAccessViewFloat(&mut self, punorderedaccessview: &::core::option::Option<ID3D11UnorderedAccessView>, values: *const f32);
    fn ClearDepthStencilView(&mut self, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&mut self, pshaderresourceview: &::core::option::Option<ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(&mut self, presource: &::core::option::Option<ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(&mut self, presource: &::core::option::Option<ID3D11Resource>) -> f32;
    fn ResolveSubresource(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn ExecuteCommandList(&mut self, pcommandlist: &::core::option::Option<ID3D11CommandList>, restorecontextstate: super::super::Foundation::BOOL);
    fn HSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSSetShader(&mut self, phullshader: &::core::option::Option<ID3D11HullShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn HSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn HSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn DSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSSetShader(&mut self, pdomainshader: &::core::option::Option<ID3D11DomainShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn DSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn CSSetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSSetUnorderedAccessViews(&mut self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn CSSetShader(&mut self, pcomputeshader: &::core::option::Option<ID3D11ComputeShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn CSSetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn CSSetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn VSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn PSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSGetShader(&mut self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn VSGetShader(&mut self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn IAGetInputLayout(&mut self, ppinputlayout: *mut ::core::option::Option<ID3D11InputLayout>);
    fn IAGetVertexBuffers(&mut self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D11Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&mut self, pindexbuffer: *mut ::core::option::Option<ID3D11Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn GSGetShader(&mut self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn IAGetPrimitiveTopology(&mut self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn GetPredication(&mut self, pppredicate: *mut ::core::option::Option<ID3D11Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn OMGetRenderTargets(&mut self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>);
    fn OMGetRenderTargetsAndUnorderedAccessViews(&mut self, numrtvs: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn OMGetBlendState(&mut self, ppblendstate: *mut ::core::option::Option<ID3D11BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&mut self, ppdepthstencilstate: *mut ::core::option::Option<ID3D11DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&mut self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D11Buffer>);
    fn RSGetState(&mut self, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState>);
    fn RSGetViewports(&mut self, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(&mut self, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn HSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSGetShader(&mut self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn HSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn HSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn DSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSGetShader(&mut self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn DSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn DSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn CSGetShaderResources(&mut self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSGetUnorderedAccessViews(&mut self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn CSGetShader(&mut self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn CSGetSamplers(&mut self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn CSGetConstantBuffers(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn ClearState(&mut self);
    fn Flush(&mut self);
    fn GetType(&mut self) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(&mut self) -> u32;
    fn FinishCommandList(&mut self, restoredeferredcontextstate: super::super::Foundation::BOOL) -> ::windows::core::Result<ID3D11CommandList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext_Vtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShader(::core::mem::transmute(&ppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShader(::core::mem::transmute(&pvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexed(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn Map<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmappedresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetInputLayout(::core::mem::transmute(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(::core::mem::transmute(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShader(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(::core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn Begin<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(::core::mem::transmute(&pasync))
        }
        unsafe extern "system" fn End<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End(::core::mem::transmute(&pasync))
        }
        unsafe extern "system" fn GetData<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData(::core::mem::transmute(&pasync), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(::core::mem::transmute(&ppredicate), ::core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargetsAndUnorderedAccessViews(::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendState(::core::mem::transmute(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthStencilState(::core::mem::transmute(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawAuto()
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstancedIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn DrawInstancedIndirect<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstancedIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn Dispatch<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dispatch(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn DispatchIndirect<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetState(::core::mem::transmute(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn CopyStructureCount<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstalignedbyteoffset: u32, psrcview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyStructureCount(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstalignedbyteoffset), ::core::mem::transmute(&psrcview))
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(::core::mem::transmute(&prendertargetview), ::core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewUint(::core::mem::transmute(&punorderedaccessview), ::core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewFloat(::core::mem::transmute(&punorderedaccessview), ::core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateMips(::core::mem::transmute(&pshaderresourceview))
        }
        unsafe extern "system" fn SetResourceMinLOD<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, minlod: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceMinLOD(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&minlod))
        }
        unsafe extern "system" fn GetResourceMinLOD<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceMinLOD(::core::mem::transmute(&presource))
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn ExecuteCommandList<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr, restorecontextstate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteCommandList(::core::mem::transmute(&pcommandlist), ::core::mem::transmute_copy(&restorecontextstate))
        }
        unsafe extern "system" fn HSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phullshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetShader(::core::mem::transmute(&phullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn HSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetShader(::core::mem::transmute(&pdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSSetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetUnorderedAccessViews(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn CSSetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomputeshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetShader(::core::mem::transmute(&pcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn CSSetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSSetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShader(::core::mem::transmute_copy(&pppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShader(::core::mem::transmute_copy(&ppvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShader(::core::mem::transmute_copy(&ppgeometryshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargetsAndUnorderedAccessViews(::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOGetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets))
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetState(::core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetViewports(::core::mem::transmute_copy(&pnumviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetScissorRects(::core::mem::transmute_copy(&pnumrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn HSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphullshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetShader(::core::mem::transmute_copy(&pphullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn HSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomainshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetShader(::core::mem::transmute_copy(&ppdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn DSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSGetShaderResources<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetUnorderedAccessViews(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn CSGetShader<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomputeshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetShader(::core::mem::transmute_copy(&ppcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn CSGetSamplers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSGetConstantBuffers<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn ClearState<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState()
        }
        unsafe extern "system" fn Flush<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContextFlags()
        }
        unsafe extern "system" fn FinishCommandList<Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishCommandList(::core::mem::transmute_copy(&restoredeferredcontextstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader: PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers: PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader: VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed: DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw: Draw::<Impl, IMPL_OFFSET>,
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout: IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced: DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader: GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers: VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin: Begin::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            SetPredication: SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers: GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState: OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets: SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto: DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch: Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect: DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState: RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports: RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource: CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource: UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount: CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips: GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource: ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader: HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers: HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader: DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers: DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader: CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers: CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader: PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers: PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader: VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader: GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers: VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication: GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers: GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState: OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets: SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState: RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports: RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader: HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers: HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader: DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers: DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader: CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers: CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState: ClearState::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags: GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList: FinishCommandList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl {
    fn CopySubresourceRegion1(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32);
    fn UpdateSubresource1(&mut self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32);
    fn DiscardResource(&mut self, presource: &::core::option::Option<ID3D11Resource>);
    fn DiscardView(&mut self, presourceview: &::core::option::Option<ID3D11View>);
    fn VSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn HSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn DSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn GSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn PSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn CSSetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn VSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn HSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn DSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn GSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn PSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn CSGetConstantBuffers1(&mut self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn SwapDeviceContextState(&mut self, pstate: &::core::option::Option<ID3DDeviceContextState>, pppreviousstate: *mut ::core::option::Option<ID3DDeviceContextState>);
    fn ClearView(&mut self, pview: &::core::option::Option<ID3D11View>, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32);
    fn DiscardView1(&mut self, presourceview: &::core::option::Option<ID3D11View>, prects: *const super::super::Foundation::RECT, numrects: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext1_Vtbl {
        unsafe extern "system" fn CopySubresourceRegion1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopySubresourceRegion1(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox), ::core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn UpdateSubresource1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateSubresource1(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch), ::core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResource(::core::mem::transmute(&presource))
        }
        unsafe extern "system" fn DiscardView<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardView(::core::mem::transmute(&presourceview))
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn SwapDeviceContextState<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: ::windows::core::RawPtr, pppreviousstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwapDeviceContextState(::core::mem::transmute(&pstate), ::core::mem::transmute_copy(&pppreviousstate))
        }
        unsafe extern "system" fn ClearView<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: ::windows::core::RawPtr, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearView(::core::mem::transmute(&pview), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&numrects))
        }
        unsafe extern "system" fn DiscardView1<Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr, prects: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardView1(::core::mem::transmute(&presourceview), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&numrects))
        }
        Self {
            base: ID3D11DeviceContext_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CopySubresourceRegion1: CopySubresourceRegion1::<Impl, IMPL_OFFSET>,
            UpdateSubresource1: UpdateSubresource1::<Impl, IMPL_OFFSET>,
            DiscardResource: DiscardResource::<Impl, IMPL_OFFSET>,
            DiscardView: DiscardView::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers1: VSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers1: HSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers1: DSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers1: GSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers1: PSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers1: CSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers1: VSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers1: HSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers1: DSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers1: GSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers1: PSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers1: CSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            SwapDeviceContextState: SwapDeviceContextState::<Impl, IMPL_OFFSET>,
            ClearView: ClearView::<Impl, IMPL_OFFSET>,
            DiscardView1: DiscardView1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl {
    fn UpdateTileMappings(&mut self, ptiledresource: &::core::option::Option<ID3D11Resource>, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: &::core::option::Option<ID3D11Buffer>, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::Result<()>;
    fn CopyTileMappings(&mut self, pdesttiledresource: &::core::option::Option<ID3D11Resource>, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: &::core::option::Option<ID3D11Resource>, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::Result<()>;
    fn CopyTiles(&mut self, ptiledresource: &::core::option::Option<ID3D11Resource>, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: &::core::option::Option<ID3D11Buffer>, bufferstartoffsetinbytes: u64, flags: u32);
    fn UpdateTiles(&mut self, pdesttiledresource: &::core::option::Option<ID3D11Resource>, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32);
    fn ResizeTilePool(&mut self, ptilepool: &::core::option::Option<ID3D11Buffer>, newsizeinbytes: u64) -> ::windows::core::Result<()>;
    fn TiledResourceBarrier(&mut self, ptiledresourceorviewaccessbeforebarrier: &::core::option::Option<ID3D11DeviceChild>, ptiledresourceorviewaccessafterbarrier: &::core::option::Option<ID3D11DeviceChild>);
    fn IsAnnotationEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn SetMarkerInt(&mut self, plabel: super::super::Foundation::PWSTR, data: i32);
    fn BeginEventInt(&mut self, plabel: super::super::Foundation::PWSTR, data: i32);
    fn EndEvent(&mut self);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext2_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: ::windows::core::RawPtr, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateTileMappings(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&numtiledresourceregions), ::core::mem::transmute_copy(&ptiledresourceregionstartcoordinates), ::core::mem::transmute_copy(&ptiledresourceregionsizes), ::core::mem::transmute(&ptilepool), ::core::mem::transmute_copy(&numranges), ::core::mem::transmute_copy(&prangeflags), ::core::mem::transmute_copy(&ptilepoolstartoffsets), ::core::mem::transmute_copy(&prangetilecounts), ::core::mem::transmute_copy(&flags))
                .into()
        }
        unsafe extern "system" fn CopyTileMappings<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: ::windows::core::RawPtr, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTileMappings(::core::mem::transmute(&pdesttiledresource), ::core::mem::transmute_copy(&pdestregionstartcoordinate), ::core::mem::transmute(&psourcetiledresource), ::core::mem::transmute_copy(&psourceregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CopyTiles<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiles(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn UpdateTiles<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTiles(::core::mem::transmute(&pdesttiledresource), ::core::mem::transmute_copy(&pdesttileregionstartcoordinate), ::core::mem::transmute_copy(&pdesttileregionsize), ::core::mem::transmute_copy(&psourcetiledata), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResizeTilePool<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilepool: ::windows::core::RawPtr, newsizeinbytes: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResizeTilePool(::core::mem::transmute(&ptilepool), ::core::mem::transmute_copy(&newsizeinbytes)).into()
        }
        unsafe extern "system" fn TiledResourceBarrier<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: ::windows::core::RawPtr, ptiledresourceorviewaccessafterbarrier: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TiledResourceBarrier(::core::mem::transmute(&ptiledresourceorviewaccessbeforebarrier), ::core::mem::transmute(&ptiledresourceorviewaccessafterbarrier))
        }
        unsafe extern "system" fn IsAnnotationEnabled<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsAnnotationEnabled()
        }
        unsafe extern "system" fn SetMarkerInt<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarkerInt(::core::mem::transmute_copy(&plabel), ::core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn BeginEventInt<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEventInt(::core::mem::transmute_copy(&plabel), ::core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent()
        }
        Self {
            base: ID3D11DeviceContext1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Impl, IMPL_OFFSET>,
            CopyTileMappings: CopyTileMappings::<Impl, IMPL_OFFSET>,
            CopyTiles: CopyTiles::<Impl, IMPL_OFFSET>,
            UpdateTiles: UpdateTiles::<Impl, IMPL_OFFSET>,
            ResizeTilePool: ResizeTilePool::<Impl, IMPL_OFFSET>,
            TiledResourceBarrier: TiledResourceBarrier::<Impl, IMPL_OFFSET>,
            IsAnnotationEnabled: IsAnnotationEnabled::<Impl, IMPL_OFFSET>,
            SetMarkerInt: SetMarkerInt::<Impl, IMPL_OFFSET>,
            BeginEventInt: BeginEventInt::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext3_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl + ID3D11DeviceContext2_Impl {
    fn Flush1(&mut self, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE);
    fn SetHardwareProtectionState(&mut self, hwprotectionenable: super::super::Foundation::BOOL);
    fn GetHardwareProtectionState(&mut self, phwprotectionenable: *mut super::super::Foundation::BOOL);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext3_Vtbl {
        unsafe extern "system" fn Flush1<Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush1(::core::mem::transmute_copy(&contexttype), ::core::mem::transmute_copy(&hevent))
        }
        unsafe extern "system" fn SetHardwareProtectionState<Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardwareProtectionState(::core::mem::transmute_copy(&hwprotectionenable))
        }
        unsafe extern "system" fn GetHardwareProtectionState<Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHardwareProtectionState(::core::mem::transmute_copy(&phwprotectionenable))
        }
        Self {
            base: ID3D11DeviceContext2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Flush1: Flush1::<Impl, IMPL_OFFSET>,
            SetHardwareProtectionState: SetHardwareProtectionState::<Impl, IMPL_OFFSET>,
            GetHardwareProtectionState: GetHardwareProtectionState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext4_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl + ID3D11DeviceContext2_Impl + ID3D11DeviceContext3_Impl {
    fn Signal(&mut self, pfence: &::core::option::Option<ID3D11Fence>, value: u64) -> ::windows::core::Result<()>;
    fn Wait(&mut self, pfence: &::core::option::Option<ID3D11Fence>, value: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext4_Vtbl {
        unsafe extern "system" fn Signal<Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Signal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ID3D11DeviceContext3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Signal: Signal::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext4 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DomainShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11DomainShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DomainShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DomainShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DomainShader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ID3D11Fence_Impl: Sized + ID3D11DeviceChild_Impl {
    fn CreateSharedHandle(&mut self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetCompletedValue(&mut self) -> u64;
    fn SetEventOnCompletion(&mut self, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ID3D11Fence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Fence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Fence_Vtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute_copy(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompletedValue<Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCompletedValue()
        }
        unsafe extern "system" fn SetEventOnCompletion<Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventOnCompletion(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Impl, IMPL_OFFSET>,
            GetCompletedValue: GetCompletedValue::<Impl, IMPL_OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Fence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionLinkingGraph_Impl: Sized {
    fn CreateModuleInstance(&mut self, ppmoduleinstance: *mut ::core::option::Option<ID3D11ModuleInstance>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>;
    fn SetInputSignature(&mut self, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn SetOutputSignature(&mut self, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn CallFunction(&mut self, pmoduleinstancenamespace: super::super::Foundation::PSTR, pmodulewithfunctionprototype: &::core::option::Option<ID3D11Module>, pfunctionname: super::super::Foundation::PSTR) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn PassValue(&mut self, psrcnode: &::core::option::Option<ID3D11LinkingNode>, srcparameterindex: i32, pdstnode: &::core::option::Option<ID3D11LinkingNode>, dstparameterindex: i32) -> ::windows::core::Result<()>;
    fn PassValueWithSwizzle(&mut self, psrcnode: &::core::option::Option<ID3D11LinkingNode>, srcparameterindex: i32, psrcswizzle: super::super::Foundation::PSTR, pdstnode: &::core::option::Option<ID3D11LinkingNode>, dstparameterindex: i32, pdstswizzle: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn GetLastError(&mut self) -> ::windows::core::Result<super::Direct3D::ID3DBlob>;
    fn GenerateHlsl(&mut self, uflags: u32) -> ::windows::core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionLinkingGraph_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionLinkingGraph_Vtbl {
        unsafe extern "system" fn CreateModuleInstance<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmoduleinstance: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateModuleInstance(::core::mem::transmute_copy(&ppmoduleinstance), ::core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn SetInputSignature<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInputSignature(::core::mem::transmute_copy(&pinputparameters), ::core::mem::transmute_copy(&cinputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSignature<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputSignature(::core::mem::transmute_copy(&poutputparameters), ::core::mem::transmute_copy(&coutputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallFunction<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmoduleinstancenamespace: super::super::Foundation::PSTR, pmodulewithfunctionprototype: ::windows::core::RawPtr, pfunctionname: super::super::Foundation::PSTR, ppcallnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallFunction(::core::mem::transmute_copy(&pmoduleinstancenamespace), ::core::mem::transmute(&pmodulewithfunctionprototype), ::core::mem::transmute_copy(&pfunctionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PassValue<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PassValue(::core::mem::transmute(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::core::mem::transmute(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex)).into()
        }
        unsafe extern "system" fn PassValueWithSwizzle<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, psrcswizzle: super::super::Foundation::PSTR, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32, pdstswizzle: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PassValueWithSwizzle(::core::mem::transmute(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::core::mem::transmute_copy(&psrcswizzle), ::core::mem::transmute(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex), ::core::mem::transmute_copy(&pdstswizzle)).into()
        }
        unsafe extern "system" fn GetLastError<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    *pperrorbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateHlsl<Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateHlsl(::core::mem::transmute_copy(&uflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateModuleInstance: CreateModuleInstance::<Impl, IMPL_OFFSET>,
            SetInputSignature: SetInputSignature::<Impl, IMPL_OFFSET>,
            SetOutputSignature: SetOutputSignature::<Impl, IMPL_OFFSET>,
            CallFunction: CallFunction::<Impl, IMPL_OFFSET>,
            PassValue: PassValue::<Impl, IMPL_OFFSET>,
            PassValueWithSwizzle: PassValueWithSwizzle::<Impl, IMPL_OFFSET>,
            GetLastError: GetLastError::<Impl, IMPL_OFFSET>,
            GenerateHlsl: GenerateHlsl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionLinkingGraph as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_PARAMETER_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionParameterReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionParameterReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionParameterReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionParameterReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_FUNCTION_DESC>;
    fn GetConstantBufferByIndex(&mut self, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&mut self, name: super::super::Foundation::PSTR) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetFunctionParameter(&mut self, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFunctionParameter(::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetFunctionParameter: GetFunctionParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionReflection as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11GeometryShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11GeometryShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11GeometryShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11GeometryShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11HullShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11HullShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11HullShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11HullShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11HullShader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&mut self, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&mut self);
    fn GetMessage(&mut self, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&mut self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&mut self) -> u64;
    fn GetNumStoredMessages(&mut self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&mut self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&mut self) -> u64;
    fn GetMessageCountLimit(&mut self) -> u64;
    fn AddStorageFilterEntries(&mut self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&mut self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&mut self);
    fn PushEmptyStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&mut self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&mut self);
    fn GetStorageFilterStackSize(&mut self) -> u32;
    fn AddRetrievalFilterEntries(&mut self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&mut self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&mut self);
    fn PushEmptyRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&mut self) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&mut self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&mut self);
    fn GetRetrievalFilterStackSize(&mut self) -> u32;
    fn AddMessage(&mut self, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&mut self, severity: D3D11_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&mut self, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&mut self, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&mut self, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&mut self, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&mut self, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&mut self, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&mut self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11InfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMuteDebugOutput()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter: GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter: PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter: PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID: SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID: GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11InputLayout_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11InputLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InputLayout_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11InputLayout_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InputLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11LibraryReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_LIBRARY_DESC>;
    fn GetFunctionByIndex(&mut self, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11LibraryReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFunctionByIndex(::core::mem::transmute_copy(&functionindex))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LibraryReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11Linker_Impl: Sized {
    fn Link(&mut self, pentry: &::core::option::Option<ID3D11ModuleInstance>, pentryname: super::super::Foundation::PSTR, ptargetname: super::super::Foundation::PSTR, uflags: u32, ppshaderblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>;
    fn UseLibrary(&mut self, plibrarymi: &::core::option::Option<ID3D11ModuleInstance>) -> ::windows::core::Result<()>;
    fn AddClipPlaneFromCBuffer(&mut self, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11Linker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Linker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Linker_Vtbl {
        unsafe extern "system" fn Link<Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentry: ::windows::core::RawPtr, pentryname: super::super::Foundation::PSTR, ptargetname: super::super::Foundation::PSTR, uflags: u32, ppshaderblob: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Link(::core::mem::transmute(&pentry), ::core::mem::transmute_copy(&pentryname), ::core::mem::transmute_copy(&ptargetname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ppshaderblob), ::core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn UseLibrary<Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrarymi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseLibrary(::core::mem::transmute(&plibrarymi)).into()
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddClipPlaneFromCBuffer(::core::mem::transmute_copy(&ucbufferslot), ::core::mem::transmute_copy(&ucbufferentry)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Link: Link::<Impl, IMPL_OFFSET>,
            UseLibrary: UseLibrary::<Impl, IMPL_OFFSET>,
            AddClipPlaneFromCBuffer: AddClipPlaneFromCBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Linker as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11LinkingNode_Impl: Sized {}
impl ID3D11LinkingNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkingNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11LinkingNode_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LinkingNode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11Module_Impl: Sized {
    fn CreateInstance(&mut self, pnamespace: super::super::Foundation::PSTR) -> ::windows::core::Result<ID3D11ModuleInstance>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11Module_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Module_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Module_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ID3D11Module_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: super::super::Foundation::PSTR, ppmoduleinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&pnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmoduleinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Module as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ModuleInstance_Impl: Sized {
    fn BindConstantBuffer(&mut self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::Result<()>;
    fn BindConstantBufferByName(&mut self, pname: super::super::Foundation::PSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::Result<()>;
    fn BindResource(&mut self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceByName(&mut self, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindSampler(&mut self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindSamplerByName(&mut self, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindUnorderedAccessView(&mut self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindUnorderedAccessViewByName(&mut self, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceAsUnorderedAccessView(&mut self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceAsUnorderedAccessViewByName(&mut self, psrvname: super::super::Foundation::PSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ModuleInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ModuleInstance_Vtbl {
        unsafe extern "system" fn BindConstantBuffer<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindConstantBuffer(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindConstantBufferByName<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindConstantBufferByName(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindResource<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindResource(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceByName<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindResourceByName(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSampler<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindSampler(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSamplerByName<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindSamplerByName(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessView<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindUnorderedAccessView(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindUnorderedAccessViewByName(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindResourceAsUnorderedAccessView(::core::mem::transmute_copy(&usrcsrvslot), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrvname: super::super::Foundation::PSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindResourceAsUnorderedAccessViewByName(::core::mem::transmute_copy(&psrvname), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BindConstantBuffer: BindConstantBuffer::<Impl, IMPL_OFFSET>,
            BindConstantBufferByName: BindConstantBufferByName::<Impl, IMPL_OFFSET>,
            BindResource: BindResource::<Impl, IMPL_OFFSET>,
            BindResourceByName: BindResourceByName::<Impl, IMPL_OFFSET>,
            BindSampler: BindSampler::<Impl, IMPL_OFFSET>,
            BindSamplerByName: BindSamplerByName::<Impl, IMPL_OFFSET>,
            BindUnorderedAccessView: BindUnorderedAccessView::<Impl, IMPL_OFFSET>,
            BindUnorderedAccessViewByName: BindUnorderedAccessViewByName::<Impl, IMPL_OFFSET>,
            BindResourceAsUnorderedAccessView: BindResourceAsUnorderedAccessView::<Impl, IMPL_OFFSET>,
            BindResourceAsUnorderedAccessViewByName: BindResourceAsUnorderedAccessViewByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ModuleInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11Multithread_Impl: Sized {
    fn Enter(&mut self);
    fn Leave(&mut self);
    fn SetMultithreadProtected(&mut self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11Multithread_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Multithread_Vtbl {
        unsafe extern "system" fn Enter<Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enter()
        }
        unsafe extern "system" fn Leave<Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Leave()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultithreadProtected(::core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMultithreadProtected()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Enter: Enter::<Impl, IMPL_OFFSET>,
            Leave: Leave::<Impl, IMPL_OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Impl, IMPL_OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11PixelShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11PixelShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PixelShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11PixelShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11PixelShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Predicate_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl + ID3D11Query_Impl {}
impl ID3D11Predicate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Predicate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Predicate_Vtbl {
        Self { base: ID3D11Query_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Predicate as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Query_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_QUERY_DESC);
}
impl ID3D11Query_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Query_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Query_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Query1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl + ID3D11Query_Impl {
    fn GetDesc1(&mut self, pdesc1: *mut D3D11_QUERY_DESC1);
}
impl ID3D11Query1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Query1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Query1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11Query_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerState_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RasterizerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11RasterizerState_Impl {
    fn GetDesc1(&mut self, pdesc: *mut D3D11_RASTERIZER_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RasterizerState1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11RasterizerState_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11RasterizerState_Impl + ID3D11RasterizerState1_Impl {
    fn GetDesc2(&mut self, pdesc: *mut D3D11_RASTERIZER_DESC2);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerState2_Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: ID3D11RasterizerState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc2(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11RasterizerState1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc2: GetDesc2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState2 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefDefaultTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&mut self, resourcetypeflags: u32, options: u32) -> ::windows::core::Result<()>;
}
impl ID3D11RefDefaultTrackingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefDefaultTrackingOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RefDefaultTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackingOptions(::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefDefaultTrackingOptions as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&mut self, uoptions: u32) -> ::windows::core::Result<()>;
}
impl ID3D11RefTrackingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefTrackingOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RefTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackingOptions(::core::mem::transmute_copy(&uoptions)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefTrackingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RenderTargetView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RenderTargetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11RenderTargetView_Impl {
    fn GetDesc1(&mut self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RenderTargetView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RenderTargetView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11RenderTargetView_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Resource_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetType(&mut self, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&mut self, evictionpriority: u32);
    fn GetEvictionPriority(&mut self) -> u32;
}
impl ID3D11Resource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Resource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Resource_Vtbl {
        unsafe extern "system" fn GetType<Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&presourcedimension))
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEvictionPriority()
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11SamplerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_SAMPLER_DESC);
}
impl ID3D11SamplerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SamplerState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11SamplerState_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11SamplerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SamplerState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflection_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_SHADER_DESC>;
    fn GetConstantBufferByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&mut self, resourceindex: u32) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetPatchConstantParameterDesc(&mut self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&mut self, name: super::super::Foundation::PSTR) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetMovInstructionCount(&mut self) -> u32;
    fn GetMovcInstructionCount(&mut self) -> u32;
    fn GetConversionInstructionCount(&mut self) -> u32;
    fn GetBitwiseInstructionCount(&mut self) -> u32;
    fn GetGSInputPrimitive(&mut self) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&mut self) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(&mut self) -> u32;
    fn GetMinFeatureLevel(&mut self) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&mut self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&mut self) -> u64;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMovInstructionCount()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMovcInstructionCount()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConversionInstructionCount()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBitwiseInstructionCount()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGSInputPrimitive()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSampleFrequencyShader()
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumInterfaceSlots()
        }
        unsafe extern "system" fn GetMinFeatureLevel<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequiresFlags()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Impl, IMPL_OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Impl, IMPL_OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Impl, IMPL_OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Impl, IMPL_OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Impl, IMPL_OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Impl, IMPL_OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Impl, IMPL_OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Impl, IMPL_OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Impl, IMPL_OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&mut self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::Result<()>;
    fn GetVariableByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetVariableByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute_copy(&name))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Impl, IMPL_OFFSET>,
            GetVariableByName: GetVariableByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflectionType_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_SHADER_TYPE_DESC>;
    fn GetMemberTypeByIndex(&mut self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeByName(&mut self, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeName(&mut self, index: u32) -> super::super::Foundation::PSTR;
    fn IsEqual(&mut self, ptype: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn GetSubType(&mut self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBaseClass(&mut self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetNumInterfaces(&mut self) -> u32;
    fn GetInterfaceByIndex(&mut self, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn IsOfType(&mut self, ptype: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn ImplementsInterface(&mut self, pbase: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSubType()
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBaseClass()
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumInterfaces()
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceByIndex(::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOfType(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImplementsInterface(::core::mem::transmute(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl, IMPL_OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl, IMPL_OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            GetSubType: GetSubType::<Impl, IMPL_OFFSET>,
            GetBaseClass: GetBaseClass::<Impl, IMPL_OFFSET>,
            GetNumInterfaces: GetNumInterfaces::<Impl, IMPL_OFFSET>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl, IMPL_OFFSET>,
            IsOfType: IsOfType::<Impl, IMPL_OFFSET>,
            ImplementsInterface: ImplementsInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&mut self) -> ::windows::core::Result<D3D11_SHADER_VARIABLE_DESC>;
    fn GetType(&mut self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBuffer(&mut self) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&mut self, uarrayindex: u32) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer()
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceSlot(::core::mem::transmute_copy(&uarrayindex))
        }
        Self {
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            GetInterfaceSlot: GetInterfaceSlot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderResourceView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderResourceView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11ShaderResourceView_Impl {
    fn GetDesc1(&mut self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderResourceView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11ShaderResourceView_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderTrace_Impl: Sized {
    fn TraceReady(&mut self) -> ::windows::core::Result<u64>;
    fn ResetTrace(&mut self);
    fn GetTraceStats(&mut self) -> ::windows::core::Result<D3D11_TRACE_STATS>;
    fn PSSelectStamp(&mut self, stampindex: u32) -> ::windows::core::Result<()>;
    fn GetInitialRegisterContents(&mut self, pregister: *const D3D11_TRACE_REGISTER) -> ::windows::core::Result<D3D11_TRACE_VALUE>;
    fn GetStep(&mut self, stepindex: u32) -> ::windows::core::Result<D3D11_TRACE_STEP>;
    fn GetWrittenRegister(&mut self, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::Result<()>;
    fn GetReadRegister(&mut self, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ShaderTrace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderTrace_Vtbl {
        unsafe extern "system" fn TraceReady<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptestcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TraceReady() {
                ::core::result::Result::Ok(ok__) => {
                    *ptestcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetTrace<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTrace()
        }
        unsafe extern "system" fn GetTraceStats<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTraceStats() {
                ::core::result::Result::Ok(ok__) => {
                    *ptracestats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PSSelectStamp<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stampindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSelectStamp(::core::mem::transmute_copy(&stampindex)).into()
        }
        unsafe extern "system" fn GetInitialRegisterContents<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInitialRegisterContents(::core::mem::transmute_copy(&pregister)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStep<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStep(::core::mem::transmute_copy(&stepindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptracestep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWrittenRegister<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWrittenRegister(::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&writtenregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadRegister<Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReadRegister(::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&readregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TraceReady: TraceReady::<Impl, IMPL_OFFSET>,
            ResetTrace: ResetTrace::<Impl, IMPL_OFFSET>,
            GetTraceStats: GetTraceStats::<Impl, IMPL_OFFSET>,
            PSSelectStamp: PSSelectStamp::<Impl, IMPL_OFFSET>,
            GetInitialRegisterContents: GetInitialRegisterContents::<Impl, IMPL_OFFSET>,
            GetStep: GetStep::<Impl, IMPL_OFFSET>,
            GetWrittenRegister: GetWrittenRegister::<Impl, IMPL_OFFSET>,
            GetReadRegister: GetReadRegister::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTrace as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ShaderTraceFactory_Impl: Sized {
    fn CreateShaderTrace(&mut self, pshader: &::core::option::Option<::windows::core::IUnknown>, ptracedesc: *const D3D11_SHADER_TRACE_DESC) -> ::windows::core::Result<ID3D11ShaderTrace>;
}
impl ID3D11ShaderTraceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderTraceFactory_Vtbl {
        unsafe extern "system" fn CreateShaderTrace<Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderTrace(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&ptracedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppshadertrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateShaderTrace: CreateShaderTrace::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTraceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11SwitchToRef_Impl: Sized {
    fn SetUseRef(&mut self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11SwitchToRef_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRef_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11SwitchToRef_Vtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseRef(::core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUseRef()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetUseRef: SetUseRef::<Impl, IMPL_OFFSET>,
            GetUseRef: GetUseRef::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture1D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture1D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture1D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture1D_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture1D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture1D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture2D_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl + ID3D11Texture2D_Impl {
    fn GetDesc1(&mut self, pdesc: *mut D3D11_TEXTURE2D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture2D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture2D1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Texture2D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture3D_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl + ID3D11Texture3D_Impl {
    fn GetDesc1(&mut self, pdesc: *mut D3D11_TEXTURE3D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture3D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture3D1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Texture3D_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11TracingDevice_Impl: Sized {
    fn SetShaderTrackingOptionsByType(&mut self, resourcetypeflags: u32, options: u32) -> ::windows::core::Result<()>;
    fn SetShaderTrackingOptions(&mut self, pshader: &::core::option::Option<::windows::core::IUnknown>, options: u32) -> ::windows::core::Result<()>;
}
impl ID3D11TracingDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11TracingDevice_Vtbl {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShaderTrackingOptionsByType(::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShaderTrackingOptions(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetShaderTrackingOptionsByType: SetShaderTrackingOptionsByType::<Impl, IMPL_OFFSET>,
            SetShaderTrackingOptions: SetShaderTrackingOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11TracingDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11UnorderedAccessView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11UnorderedAccessView_Impl {
    fn GetDesc1(&mut self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11UnorderedAccessView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11UnorderedAccessView_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VertexShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11VertexShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VertexShader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VertexShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VertexShader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseDecoderBuffer(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::Result<()>;
    fn DecoderBeginFrame(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pview: &::core::option::Option<ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DecoderEndFrame(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>) -> ::windows::core::Result<()>;
    fn SubmitDecoderBuffers(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::Result<()>;
    fn DecoderExtension(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32;
    fn VideoProcessorSetOutputTargetRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetOutputBackgroundColor(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR);
    fn VideoProcessorSetOutputColorSpace(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetOutputAlphaFillMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32);
    fn VideoProcessorSetOutputConstriction(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, size: &super::super::Foundation::SIZE);
    fn VideoProcessorSetOutputStereoMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetOutputExtension(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetOutputTargetRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetOutputBackgroundColor(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR);
    fn VideoProcessorGetOutputColorSpace(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetOutputAlphaFillMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32);
    fn VideoProcessorGetOutputConstriction(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE);
    fn VideoProcessorGetOutputStereoMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputExtension(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorSetStreamFrameFormat(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorSetStreamColorSpace(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetStreamOutputRate(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamSourceRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamDestRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamAlpha(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32);
    fn VideoProcessorSetStreamPalette(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *const u32);
    fn VideoProcessorSetStreamPixelAspectRatio(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamLumaKey(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32);
    fn VideoProcessorSetStreamStereoFormat(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32);
    fn VideoProcessorSetStreamAutoProcessingMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamFilter(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32);
    fn VideoProcessorSetStreamExtension(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetStreamFrameFormat(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorGetStreamColorSpace(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetStreamOutputRate(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamSourceRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamDestRect(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamAlpha(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32);
    fn VideoProcessorGetStreamPalette(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *mut u32);
    fn VideoProcessorGetStreamPixelAspectRatio(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamLumaKey(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32);
    fn VideoProcessorGetStreamStereoFormat(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32);
    fn VideoProcessorGetStreamAutoProcessingMode(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamFilter(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32);
    fn VideoProcessorGetStreamExtension(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorBlt(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pview: &::core::option::Option<ID3D11VideoProcessorOutputView>, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EncryptionBlt(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, psrcsurface: &::core::option::Option<ID3D11Texture2D>, pdstsurface: &::core::option::Option<ID3D11Texture2D>, ivsize: u32, piv: *const ::core::ffi::c_void);
    fn DecryptionBlt(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, psrcsurface: &::core::option::Option<ID3D11Texture2D>, pdstsurface: &::core::option::Option<ID3D11Texture2D>, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void);
    fn StartSessionKeyRefresh(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void);
    fn FinishSessionKeyRefresh(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>);
    fn GetEncryptionBltKey(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(&mut self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn QueryAuthenticatedChannel(&mut self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ConfigureAuthenticatedChannel(&mut self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void) -> ::windows::core::Result<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>;
    fn VideoProcessorSetStreamRotation(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION);
    fn VideoProcessorGetStreamRotation(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext_Vtbl {
        unsafe extern "system" fn GetDecoderBuffer<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDecoderBuffer(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pbuffersize), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDecoderBuffer(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn DecoderBeginFrame<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderBeginFrame(::core::mem::transmute(&pdecoder), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey)).into()
        }
        unsafe extern "system" fn DecoderEndFrame<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderEndFrame(::core::mem::transmute(&pdecoder)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubmitDecoderBuffers(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn DecoderExtension<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderExtension(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&pextensiondata))
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputTargetRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputBackgroundColor(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&ycbcr), ::core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputAlphaFillMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&alphafillmode), ::core::mem::transmute_copy(&streamindex))
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputConstriction(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputStereoMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputTargetRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputBackgroundColor(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pycbcr), ::core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputAlphaFillMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&palphafillmode), ::core::mem::transmute_copy(&pstreamindex))
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputConstriction(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psize))
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputStereoMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamFrameFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&frameformat))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamOutputRate(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&outputrate), ::core::mem::transmute_copy(&repeatframe), ::core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamSourceRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamDestRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamAlpha(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&alpha))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamPalette(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamPixelAspectRatio(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamLumaKey(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&lower), ::core::mem::transmute_copy(&upper))
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamStereoFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&leftviewframe0), ::core::mem::transmute_copy(&baseviewframe0), ::core::mem::transmute_copy(&flipmode), ::core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamAutoProcessingMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamFilter(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&level))
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamFrameFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pframeformat))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamOutputRate(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&poutputrate), ::core::mem::transmute_copy(&prepeatframe), ::core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamSourceRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamDestRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamAlpha(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&palpha))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamPalette(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamPixelAspectRatio(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamLumaKey(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plower), ::core::mem::transmute_copy(&pupper))
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamStereoFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pleftviewframe0), ::core::mem::transmute_copy(&pbaseviewframe0), ::core::mem::transmute_copy(&pflipmode), ::core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamAutoProcessingMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamFilter(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plevel))
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorBlt<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorBlt(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&outputframe), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NegotiateCryptoSessionKeyExchange(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn EncryptionBlt<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptionBlt(::core::mem::transmute(&pcryptosession), ::core::mem::transmute(&psrcsurface), ::core::mem::transmute(&pdstsurface), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn DecryptionBlt<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecryptionBlt(::core::mem::transmute(&pcryptosession), ::core::mem::transmute(&psrcsurface), ::core::mem::transmute(&pdstsurface), ::core::mem::transmute_copy(&pencryptedblockinfo), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSessionKeyRefresh(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&randomnumbersize), ::core::mem::transmute_copy(&prandomnumber))
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishSessionKeyRefresh(::core::mem::transmute(&pcryptosession))
        }
        unsafe extern "system" fn GetEncryptionBltKey<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEncryptionBltKey(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&preadbackkey)).into()
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NegotiateAuthenticatedChannelKeyExchange(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryAuthenticatedChannel(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput), ::core::mem::transmute_copy(&outputsize), ::core::mem::transmute_copy(&poutput)).into()
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureAuthenticatedChannel(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamRotation(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&rotation))
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamRotation(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&protation))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDecoderBuffer: GetDecoderBuffer::<Impl, IMPL_OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Impl, IMPL_OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Impl, IMPL_OFFSET>,
            DecoderExtension: DecoderExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt: EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt: DecryptionBlt::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<Impl, IMPL_OFFSET>,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl {
    fn SubmitDecoderBuffers1(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::Result<()>;
    fn GetDataForNewHardwareKey(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn CheckCryptoSessionStatus(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>) -> ::windows::core::Result<D3D11_CRYPTO_SESSION_STATUS>;
    fn DecoderEnableDownsampling(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::Result<()>;
    fn DecoderUpdateDownsampling(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::Result<()>;
    fn VideoProcessorSetOutputColorSpace1(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetOutputShaderUsage(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, shaderusage: super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputColorSpace1(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetOutputShaderUsage(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pshaderusage: *mut super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamColorSpace1(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetStreamMirror(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamColorSpace1(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetStreamMirror(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetBehaviorHints(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext1_Vtbl {
        unsafe extern "system" fn SubmitDecoderBuffers1<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubmitDecoderBuffers1(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void, pprivateoutputdata: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataForNewHardwareKey(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&privateinputsize), ::core::mem::transmute_copy(&pprivatinputdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprivateoutputdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCryptoSessionStatus(::core::mem::transmute(&pcryptosession)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderEnableDownsampling(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&referenceframecount)).into()
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderUpdateDownsampling(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&poutputdesc)).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, shaderusage: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputShaderUsage(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&shaderusage))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pshaderusage: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputShaderUsage(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pshaderusage))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamMirror(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&fliphorizontal), ::core::mem::transmute_copy(&flipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamMirror(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pfliphorizontal), ::core::mem::transmute_copy(&pflipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorGetBehaviorHints(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&outputwidth), ::core::mem::transmute_copy(&outputheight), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbehaviorhints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoContext_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SubmitDecoderBuffers1: SubmitDecoderBuffers1::<Impl, IMPL_OFFSET>,
            GetDataForNewHardwareKey: GetDataForNewHardwareKey::<Impl, IMPL_OFFSET>,
            CheckCryptoSessionStatus: CheckCryptoSessionStatus::<Impl, IMPL_OFFSET>,
            DecoderEnableDownsampling: DecoderEnableDownsampling::<Impl, IMPL_OFFSET>,
            DecoderUpdateDownsampling: DecoderUpdateDownsampling::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace1: VideoProcessorSetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputShaderUsage: VideoProcessorSetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace1: VideoProcessorGetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputShaderUsage: VideoProcessorGetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace1: VideoProcessorSetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamMirror: VideoProcessorSetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace1: VideoProcessorGetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamMirror: VideoProcessorGetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetBehaviorHints: VideoProcessorGetBehaviorHints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl + ID3D11VideoContext1_Impl {
    fn VideoProcessorSetOutputHDRMetaData(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetOutputHDRMetaData(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
    fn VideoProcessorSetStreamHDRMetaData(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetStreamHDRMetaData(&mut self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext2_Vtbl {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata))
        }
        Self {
            base: ID3D11VideoContext1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            VideoProcessorSetOutputHDRMetaData: VideoProcessorSetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputHDRMetaData: VideoProcessorGetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamHDRMetaData: VideoProcessorSetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamHDRMetaData: VideoProcessorGetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext3_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl + ID3D11VideoContext1_Impl + ID3D11VideoContext2_Impl {
    fn DecoderBeginFrame1(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pview: &::core::option::Option<ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::core::option::Option<ID3D11Buffer>) -> ::windows::core::Result<()>;
    fn SubmitDecoderBuffers2(&mut self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext3_Vtbl {
        unsafe extern "system" fn DecoderBeginFrame1<Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecoderBeginFrame1(::core::mem::transmute(&pdecoder), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&numcomponenthistograms), ::core::mem::transmute_copy(&phistogramoffsets), ::core::mem::transmute_copy(&pphistogrambuffers)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubmitDecoderBuffers2(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        Self {
            base: ID3D11VideoContext2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecoderBeginFrame1: DecoderBeginFrame1::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers2: SubmitDecoderBuffers2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDecoder_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCreationParameters(&mut self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<()>;
    fn GetDriverHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDecoder_Vtbl {
        unsafe extern "system" fn GetCreationParameters<Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationParameters(::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)).into()
        }
        unsafe extern "system" fn GetDriverHandle<Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriverhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCreationParameters: GetCreationParameters::<Impl, IMPL_OFFSET>,
            GetDriverHandle: GetDriverHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoder as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoDecoderOutputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoDecoderOutputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderOutputView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDecoderOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoderOutputView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice_Impl: Sized {
    fn CreateVideoDecoder(&mut self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(&mut self, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, rateconversionindex: u32) -> ::windows::core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(&mut self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> ::windows::core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(&mut self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID) -> ::windows::core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(&mut self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoDecoderOutputView>;
    fn CreateVideoProcessorInputView(&mut self, presource: &::core::option::Option<ID3D11Resource>, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoProcessorInputView>;
    fn CreateVideoProcessorOutputView(&mut self, presource: &::core::option::Option<ID3D11Resource>, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoProcessorOutputView>;
    fn CreateVideoProcessorEnumerator(&mut self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(&mut self) -> u32;
    fn GetVideoDecoderProfile(&mut self, index: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckVideoDecoderFormat(&mut self, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetVideoDecoderConfigCount(&mut self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> ::windows::core::Result<u32>;
    fn GetVideoDecoderConfig(&mut self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32) -> ::windows::core::Result<D3D11_VIDEO_DECODER_CONFIG>;
    fn GetContentProtectionCaps(&mut self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID) -> ::windows::core::Result<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>;
    fn CheckCryptoKeyExchange(&mut self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetPrivateData(&mut self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDevice_Vtbl {
        unsafe extern "system" fn CreateVideoDecoder<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoDecoder(::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: ::windows::core::RawPtr, rateconversionindex: u32, ppvideoprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessor(::core::mem::transmute(&penum), ::core::mem::transmute_copy(&rateconversionindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvideoprocessor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAuthenticatedChannel(::core::mem::transmute_copy(&channeltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppauthenticatedchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, ppcryptosession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCryptoSession(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcryptosession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoDecoderOutputView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvdovview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorInputView(::core::mem::transmute(&presource), ::core::mem::transmute(&penum), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvpiview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorOutputView(::core::mem::transmute(&presource), ::core::mem::transmute(&penum), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvpoview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorEnumerator(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVideoDecoderProfileCount()
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdecoderprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderProfile(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdecoderprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoDecoderFormat(::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *psupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderConfigCount(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderConfig(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentProtectionCaps(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32, pkeyexchangetype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCryptoKeyExchange(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pkeyexchangetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateVideoDecoder: CreateVideoDecoder::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Impl, IMPL_OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Impl, IMPL_OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Impl, IMPL_OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Impl, IMPL_OFFSET>,
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice1_Impl: Sized + ID3D11VideoDevice_Impl {
    fn GetCryptoSessionPrivateDataSize(&mut self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetVideoDecoderCaps(&mut self, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn CheckVideoDecoderDownsampling(&mut self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RecommendVideoDecoderDownsampleParameters(&mut self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> ::windows::core::Result<D3D11_VIDEO_SAMPLE_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDevice1_Vtbl {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCryptoSessionPrivateDataSize(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype), ::core::mem::transmute_copy(&pprivateinputsize), ::core::mem::transmute_copy(&pprivateoutputsize)).into()
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID, pdecodercaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderCaps(::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&samplewidth), ::core::mem::transmute_copy(&sampleheight), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&bitrate), ::core::mem::transmute_copy(&pcryptotype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdecodercaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckVideoDecoderDownsampling(::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&psupported), ::core::mem::transmute_copy(&prealtimehint)).into()
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendVideoDecoderDownsampleParameters(::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate)) {
                ::core::result::Result::Ok(ok__) => {
                    *precommendedoutputdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoDevice_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCryptoSessionPrivateDataSize: GetCryptoSessionPrivateDataSize::<Impl, IMPL_OFFSET>,
            GetVideoDecoderCaps: GetVideoDecoderCaps::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderDownsampling: CheckVideoDecoderDownsampling::<Impl, IMPL_OFFSET>,
            RecommendVideoDecoderDownsampleParameters: RecommendVideoDecoderDownsampleParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice2_Impl: Sized + ID3D11VideoDevice_Impl + ID3D11VideoDevice1_Impl {
    fn CheckFeatureSupport(&mut self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
    fn NegotiateCryptoSessionKeyExchangeMT(&mut self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDevice2_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NegotiateCryptoSessionKeyExchangeMT(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base: ID3D11VideoDevice1_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT: NegotiateCryptoSessionKeyExchangeMT::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessor_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContentDesc(&mut self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(&mut self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessor_Vtbl {
        unsafe extern "system" fn GetContentDesc<Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetRateConversionCaps<Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRateConversionCaps(::core::mem::transmute_copy(&pcaps))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetContentDesc: GetContentDesc::<Impl, IMPL_OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(&mut self) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>;
    fn CheckVideoProcessorFormat(&mut self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn GetVideoProcessorCaps(&mut self) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CAPS>;
    fn GetVideoProcessorRateConversionCaps(&mut self, typeindex: u32) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>;
    fn GetVideoProcessorCustomRate(&mut self, typeindex: u32, customrateindex: u32) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>;
    fn GetVideoProcessorFilterRange(&mut self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorEnumerator_Vtbl {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorContentDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoProcessorFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorRateConversionCaps(::core::mem::transmute_copy(&typeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorCustomRate(::core::mem::transmute_copy(&typeindex), ::core::mem::transmute_copy(&customrateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *prate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorFilterRange(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *prange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Impl, IMPL_OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Impl, IMPL_OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoProcessorEnumerator_Impl {
    fn CheckVideoProcessorFormatConversion(&mut self, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumerator1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorEnumerator1_Vtbl {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoProcessorFormatConversion(::core::mem::transmute_copy(&inputformat), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&outputcolorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *psupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoProcessorEnumerator_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CheckVideoProcessorFormatConversion: CheckVideoProcessorFormatConversion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorInputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorInputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorInputView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorInputView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorInputView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorOutputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&mut self, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorOutputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorOutputView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc: GetDesc::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorOutputView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11View_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetResource(&mut self, ppresource: *mut ::core::option::Option<ID3D11Resource>);
}
impl ID3D11View_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11View_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11View_Vtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D11View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&ppresource))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetResource: GetResource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
pub trait ID3DDeviceContextState_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3DDeviceContextState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDeviceContextState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DDeviceContextState_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDeviceContextState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3DUserDefinedAnnotation_Impl: Sized {
    fn BeginEvent(&mut self, name: super::super::Foundation::PWSTR) -> i32;
    fn EndEvent(&mut self) -> i32;
    fn SetMarker(&mut self, name: super::super::Foundation::PWSTR);
    fn GetStatus(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3DUserDefinedAnnotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DUserDefinedAnnotation_Vtbl {
        unsafe extern "system" fn BeginEvent<Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn EndEvent<Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(::core::mem::transmute_copy(&name))
        }
        unsafe extern "system" fn GetStatus<Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginEvent: BeginEvent::<Impl, IMPL_OFFSET>,
            EndEvent: EndEvent::<Impl, IMPL_OFFSET>,
            SetMarker: SetMarker::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DUserDefinedAnnotation as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11FFT_Impl: Sized {
    fn SetForwardScale(&mut self, forwardscale: f32) -> ::windows::core::Result<()>;
    fn GetForwardScale(&mut self) -> f32;
    fn SetInverseScale(&mut self, inversescale: f32) -> ::windows::core::Result<()>;
    fn GetInverseScale(&mut self) -> f32;
    fn AttachBuffersAndPrecompute(&mut self, numtempbuffers: u32, pptempbuffers: *const ::core::option::Option<ID3D11UnorderedAccessView>, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn ForwardTransform(&mut self, pinputbuffer: &::core::option::Option<ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn InverseTransform(&mut self, pinputbuffer: &::core::option::Option<ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11FFT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11FFT_Vtbl {
        unsafe extern "system" fn SetForwardScale<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardscale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForwardScale(::core::mem::transmute_copy(&forwardscale)).into()
        }
        unsafe extern "system" fn GetForwardScale<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForwardScale()
        }
        unsafe extern "system" fn SetInverseScale<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inversescale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInverseScale(::core::mem::transmute_copy(&inversescale)).into()
        }
        unsafe extern "system" fn GetInverseScale<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInverseScale()
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const ::windows::core::RawPtr, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachBuffersAndPrecompute(::core::mem::transmute_copy(&numtempbuffers), ::core::mem::transmute_copy(&pptempbuffers), ::core::mem::transmute_copy(&numprecomputebuffers), ::core::mem::transmute_copy(&ppprecomputebuffersizes)).into()
        }
        unsafe extern "system" fn ForwardTransform<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForwardTransform(::core::mem::transmute(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        unsafe extern "system" fn InverseTransform<Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InverseTransform(::core::mem::transmute(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetForwardScale: SetForwardScale::<Impl, IMPL_OFFSET>,
            GetForwardScale: GetForwardScale::<Impl, IMPL_OFFSET>,
            SetInverseScale: SetInverseScale::<Impl, IMPL_OFFSET>,
            GetInverseScale: GetInverseScale::<Impl, IMPL_OFFSET>,
            AttachBuffersAndPrecompute: AttachBuffersAndPrecompute::<Impl, IMPL_OFFSET>,
            ForwardTransform: ForwardTransform::<Impl, IMPL_OFFSET>,
            InverseTransform: InverseTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11FFT as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11Scan_Impl: Sized {
    fn SetScanDirection(&mut self, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::Result<()>;
    fn Scan(&mut self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn Multiscan(&mut self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11Scan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11Scan_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11Scan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn Scan<Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute(&psrc), ::core::mem::transmute(&pdst)).into()
        }
        unsafe extern "system" fn Multiscan<Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Multiscan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute_copy(&elementscanpitch), ::core::mem::transmute_copy(&scancount), ::core::mem::transmute(&psrc), ::core::mem::transmute(&pdst)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetScanDirection: SetScanDirection::<Impl, IMPL_OFFSET>,
            Scan: Scan::<Impl, IMPL_OFFSET>,
            Multiscan: Multiscan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11Scan as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11SegmentedScan_Impl: Sized {
    fn SetScanDirection(&mut self, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::Result<()>;
    fn SegScan(&mut self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, psrcelementflags: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11SegmentedScan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScan_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11SegmentedScan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn SegScan<Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, psrcelementflags: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SegScan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute(&psrc), ::core::mem::transmute(&psrcelementflags), ::core::mem::transmute(&pdst)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetScanDirection: SetScanDirection::<Impl, IMPL_OFFSET>,
            SegScan: SegScan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11SegmentedScan as ::windows::core::Interface>::IID
    }
}

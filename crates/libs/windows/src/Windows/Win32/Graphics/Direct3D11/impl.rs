pub trait ID3D11Asynchronous_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDataSize(&self) -> u32;
}
impl ID3D11Asynchronous_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Asynchronous_Impl, const OFFSET: isize>() -> ID3D11Asynchronous_Vtbl {
        unsafe extern "system" fn GetDataSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Asynchronous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataSize()
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDataSize: GetDataSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11AuthenticatedChannel_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCertificateSize(&self) -> ::windows::core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32) -> ::windows::core::Result<u8>;
    fn GetChannelHandle(&self, pchannelhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11AuthenticatedChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>() -> ID3D11AuthenticatedChannel_Vtbl {
        unsafe extern "system" fn GetCertificateSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCertificateSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificatesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCertificate(::core::mem::transmute_copy(&certificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelHandle(::core::mem::transmute_copy(&pchannelhandle))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetChannelHandle: GetChannelHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11AuthenticatedChannel as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState_Impl, const OFFSET: isize>() -> ID3D11BlendState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendState1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState1_Impl, const OFFSET: isize>() -> ID3D11BlendState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11BlendState_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11BlendState as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Buffer_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BUFFER_DESC);
}
impl ID3D11Buffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Buffer_Impl, const OFFSET: isize>() -> ID3D11Buffer_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Buffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Buffer as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassInstance_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassLinkage(&self, pplinkage: *mut ::core::option::Option<ID3D11ClassLinkage>);
    fn GetDesc(&self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC);
    fn GetInstanceName(&self, pinstancename: ::windows::core::PSTR, pbufferlength: *mut usize);
    fn GetTypeName(&self, ptypename: ::windows::core::PSTR, pbufferlength: *mut usize);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ClassInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>() -> ID3D11ClassInstance_Vtbl {
        unsafe extern "system" fn GetClassLinkage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClassLinkage(::core::mem::transmute_copy(&pplinkage))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetInstanceName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstancename: ::windows::core::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInstanceName(::core::mem::transmute_copy(&pinstancename), ::core::mem::transmute_copy(&pbufferlength))
        }
        unsafe extern "system" fn GetTypeName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypename: ::windows::core::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTypeName(::core::mem::transmute_copy(&ptypename), ::core::mem::transmute_copy(&pbufferlength))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetClassLinkage: GetClassLinkage::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetInstanceName: GetInstanceName::<Identity, Impl, OFFSET>,
            GetTypeName: GetTypeName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassInstance as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ClassLinkage_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassInstance(&self, pclassinstancename: &::windows::core::PCSTR, instanceindex: u32) -> ::windows::core::Result<ID3D11ClassInstance>;
    fn CreateClassInstance(&self, pclasstypename: &::windows::core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> ::windows::core::Result<ID3D11ClassInstance>;
}
impl ID3D11ClassLinkage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>() -> ID3D11ClassLinkage_Vtbl {
        unsafe extern "system" fn GetClassInstance<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassinstancename: ::windows::core::PCSTR, instanceindex: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClassInstance(::core::mem::transmute(&pclassinstancename), ::core::mem::transmute_copy(&instanceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassInstance<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclasstypename: ::windows::core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateClassInstance(::core::mem::transmute(&pclasstypename), ::core::mem::transmute_copy(&constantbufferoffset), ::core::mem::transmute_copy(&constantvectoroffset), ::core::mem::transmute_copy(&textureoffset), ::core::mem::transmute_copy(&sampleroffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetClassInstance: GetClassInstance::<Identity, Impl, OFFSET>,
            CreateClassInstance: CreateClassInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassLinkage as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11CommandList_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContextFlags(&self) -> u32;
}
impl ID3D11CommandList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CommandList_Impl, const OFFSET: isize>() -> ID3D11CommandList_Vtbl {
        unsafe extern "system" fn GetContextFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CommandList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContextFlags()
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CommandList as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ComputeShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11ComputeShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ComputeShader_Impl, const OFFSET: isize>() -> ID3D11ComputeShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Counter_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_COUNTER_DESC);
}
impl ID3D11Counter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Counter_Impl, const OFFSET: isize>() -> ID3D11Counter_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Counter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Counter as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11CryptoSession_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCryptoType(&self, pcryptotype: *mut ::windows::core::GUID);
    fn GetDecoderProfile(&self, pdecoderprofile: *mut ::windows::core::GUID);
    fn GetCertificateSize(&self) -> ::windows::core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32) -> ::windows::core::Result<u8>;
    fn GetCryptoSessionHandle(&self, pcryptosessionhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11CryptoSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>() -> ID3D11CryptoSession_Vtbl {
        unsafe extern "system" fn GetCryptoType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCryptoType(::core::mem::transmute_copy(&pcryptotype))
        }
        unsafe extern "system" fn GetDecoderProfile<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDecoderProfile(::core::mem::transmute_copy(&pdecoderprofile))
        }
        unsafe extern "system" fn GetCertificateSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCertificateSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificatesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCertificate(::core::mem::transmute_copy(&certificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCryptoSessionHandle(::core::mem::transmute_copy(&pcryptosessionhandle))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCryptoType: GetCryptoType::<Identity, Impl, OFFSET>,
            GetDecoderProfile: GetDecoderProfile::<Identity, Impl, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetCryptoSessionHandle: GetCryptoSessionHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CryptoSession as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D11Debug_Impl: Sized {
    fn SetFeatureMask(&self, mask: u32) -> ::windows::core::Result<()>;
    fn GetFeatureMask(&self) -> u32;
    fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> ::windows::core::Result<()>;
    fn GetPresentPerRenderOpDelay(&self) -> u32;
    fn SetSwapChain(&self, pswapchain: &::core::option::Option<super::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
    fn GetSwapChain(&self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain>;
    fn ValidateContext(&self, pcontext: &::core::option::Option<ID3D11DeviceContext>) -> ::windows::core::Result<()>;
    fn ReportLiveDeviceObjects(&self, flags: D3D11_RLDO_FLAGS) -> ::windows::core::Result<()>;
    fn ValidateContextForDispatch(&self, pcontext: &::core::option::Option<ID3D11DeviceContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D11Debug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>() -> ID3D11Debug_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFeatureMask(::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureMask()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPresentPerRenderOpDelay(::core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPresentPerRenderOpDelay()
        }
        unsafe extern "system" fn SetSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSwapChain() {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateContext<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateContext(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportLiveDeviceObjects(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ValidateContextForDispatch<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateContextForDispatch(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            ValidateContext: ValidateContext::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
            ValidateContextForDispatch: ValidateContextForDispatch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11DepthStencilState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11DepthStencilState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilState_Impl, const OFFSET: isize>() -> ID3D11DepthStencilState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilState as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11DepthStencilView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11DepthStencilView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilView_Impl, const OFFSET: isize>() -> ID3D11DepthStencilView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device_Impl: Sized {
    fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Buffer>;
    fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture1D>;
    fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture2D>;
    fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture3D>;
    fn CreateShaderResourceView(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D11ShaderResourceView>;
    fn CreateUnorderedAccessView(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC) -> ::windows::core::Result<ID3D11UnorderedAccessView>;
    fn CreateRenderTargetView(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D11RenderTargetView>;
    fn CreateDepthStencilView(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D11DepthStencilView>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D11InputLayout>;
    fn CreateVertexShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11VertexShader>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11GeometryShader>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11GeometryShader>;
    fn CreatePixelShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11PixelShader>;
    fn CreateHullShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11HullShader>;
    fn CreateDomainShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11DomainShader>;
    fn CreateComputeShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: &::core::option::Option<ID3D11ClassLinkage>) -> ::windows::core::Result<ID3D11ComputeShader>;
    fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC) -> ::windows::core::Result<ID3D11BlendState>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D11DepthStencilState>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC) -> ::windows::core::Result<ID3D11RasterizerState>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC) -> ::windows::core::Result<ID3D11SamplerState>;
    fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC) -> ::windows::core::Result<ID3D11Query>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC) -> ::windows::core::Result<ID3D11Predicate>;
    fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC) -> ::windows::core::Result<ID3D11Counter>;
    fn CreateDeferredContext(&self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext>;
    fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()>;
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
    fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&self) -> u32;
    fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()>;
    fn GetImmediateContext(&self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext>);
    fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>() -> ID3D11Device_Vtbl {
        unsafe extern "system" fn CreateBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBuffer(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTexture1D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture1d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTexture2D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTexture3D(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateShaderResourceView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUnorderedAccessView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuaview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRenderTargetView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprtview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDepthStencilView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInputLayout(::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputlayout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVertexShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvertexshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGeometryShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGeometryShaderWithStreamOutput(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&pbufferstrides), ::core::mem::transmute_copy(&numstrides), ::core::mem::transmute_copy(&rasterizedstream), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgeometryshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePixelShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppixelshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHullShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pphullshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateHullShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pphullshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppdomainshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDomainShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdomainshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComputeShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppcomputeshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateComputeShader(::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute(&pclasslinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomputeshader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassLinkage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateClassLinkage() {
                ::core::result::Result::Ok(ok__) => {
                    *pplinkage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlendState(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDepthStencilState(::core::mem::transmute_copy(&pdepthstencildesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdepthstencilstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizerState(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSamplerState(::core::mem::transmute_copy(&psamplerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsamplerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQuery(::core::mem::transmute_copy(&pquerydesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePredicate(::core::mem::transmute_copy(&ppredicatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppredicate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCounter(::core::mem::transmute_copy(&pcounterdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeferredContext<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDeferredContext(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenSharedResource(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckFormatSupport(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformatsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckMultisampleQualityLevels(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnumqualitylevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckCounter(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureLevel()
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationFlags()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceRemovedReason().into()
        }
        unsafe extern "system" fn GetImmediateContext<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImmediateContext(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExceptionMode(::core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExceptionMode()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateBuffer: CreateBuffer::<Identity, Impl, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, Impl, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, Impl, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, Impl, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, Impl, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, Impl, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, Impl, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, Impl, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, Impl, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, Impl, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, Impl, OFFSET>,
            CreateCounter: CreateCounter::<Identity, Impl, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, Impl, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, Impl, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, Impl, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, Impl, OFFSET>,
            CheckCounter: CheckCounter::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, Impl, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, Impl, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, Impl, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device1_Impl: Sized + ID3D11Device_Impl {
    fn GetImmediateContext1(&self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext1>);
    fn CreateDeferredContext1(&self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext1>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1) -> ::windows::core::Result<ID3D11BlendState1>;
    fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1) -> ::windows::core::Result<ID3D11RasterizerState1>;
    fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::core::option::Option<ID3DDeviceContextState>) -> ::windows::core::Result<()>;
    fn OpenSharedResource1(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenSharedResourceByName(&self, lpname: &::windows::core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>() -> ID3D11Device1_Vtbl {
        unsafe extern "system" fn GetImmediateContext1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImmediateContext1(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDeferredContext1(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlendState1(::core::mem::transmute_copy(&pblendstatedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppblendstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizerState1(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceContextState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDeviceContextState(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pfeaturelevels), ::core::mem::transmute_copy(&featurelevels), ::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute_copy(&emulatedinterface), ::core::mem::transmute_copy(&pchosenfeaturelevel), ::core::mem::transmute_copy(&ppcontextstate)).into()
        }
        unsafe extern "system" fn OpenSharedResource1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenSharedResource1(::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn OpenSharedResourceByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: ::windows::core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenSharedResourceByName(::core::mem::transmute(&lpname), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into()
        }
        Self {
            base: ID3D11Device_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImmediateContext1: GetImmediateContext1::<Identity, Impl, OFFSET>,
            CreateDeferredContext1: CreateDeferredContext1::<Identity, Impl, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, Impl, OFFSET>,
            CreateRasterizerState1: CreateRasterizerState1::<Identity, Impl, OFFSET>,
            CreateDeviceContextState: CreateDeviceContextState::<Identity, Impl, OFFSET>,
            OpenSharedResource1: OpenSharedResource1::<Identity, Impl, OFFSET>,
            OpenSharedResourceByName: OpenSharedResourceByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device1 as ::windows::core::Interface>::IID || iid == &<ID3D11Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device2_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl {
    fn GetImmediateContext2(&self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext2>);
    fn CreateDeferredContext2(&self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext2>;
    fn GetResourceTiling(&self, ptiledresource: &::core::option::Option<ID3D11Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING);
    fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const OFFSET: isize>() -> ID3D11Device2_Vtbl {
        unsafe extern "system" fn GetImmediateContext2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImmediateContext2(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDeferredContext2(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTiling<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResourceTiling(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckMultisampleQualityLevels1(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnumqualitylevels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11Device1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImmediateContext2: GetImmediateContext2::<Identity, Impl, OFFSET>,
            CreateDeferredContext2: CreateDeferredContext2::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels1: CheckMultisampleQualityLevels1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device2 as ::windows::core::Interface>::IID || iid == &<ID3D11Device as ::windows::core::Interface>::IID || iid == &<ID3D11Device1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device3_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl {
    fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture2D1>;
    fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D11Texture3D1>;
    fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2) -> ::windows::core::Result<ID3D11RasterizerState2>;
    fn CreateShaderResourceView1(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D11ShaderResourceView1>;
    fn CreateUnorderedAccessView1(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1) -> ::windows::core::Result<ID3D11UnorderedAccessView1>;
    fn CreateRenderTargetView1(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1) -> ::windows::core::Result<ID3D11RenderTargetView1>;
    fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1) -> ::windows::core::Result<ID3D11Query1>;
    fn GetImmediateContext3(&self, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext3>);
    fn CreateDeferredContext3(&self, contextflags: u32) -> ::windows::core::Result<ID3D11DeviceContext3>;
    fn WriteToSubresource(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ReadFromSubresource(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>() -> ID3D11Device3_Vtbl {
        unsafe extern "system" fn CreateTexture2D1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTexture2D1(::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture2d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTexture3D1(::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptexture3d = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizerState2(::core::mem::transmute_copy(&prasterizerdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprasterizerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateShaderResourceView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsrview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUnorderedAccessView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuaview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRenderTargetView1(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprtview1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateQuery1(::core::mem::transmute_copy(&pquerydesc1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmediateContext3<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImmediateContext3(::core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext3<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDeferredContext3(::core::mem::transmute_copy(&contextflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeferredcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteToSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        Self {
            base: ID3D11Device2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTexture2D1: CreateTexture2D1::<Identity, Impl, OFFSET>,
            CreateTexture3D1: CreateTexture3D1::<Identity, Impl, OFFSET>,
            CreateRasterizerState2: CreateRasterizerState2::<Identity, Impl, OFFSET>,
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView1: CreateUnorderedAccessView1::<Identity, Impl, OFFSET>,
            CreateRenderTargetView1: CreateRenderTargetView1::<Identity, Impl, OFFSET>,
            CreateQuery1: CreateQuery1::<Identity, Impl, OFFSET>,
            GetImmediateContext3: GetImmediateContext3::<Identity, Impl, OFFSET>,
            CreateDeferredContext3: CreateDeferredContext3::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device3 as ::windows::core::Interface>::IID || iid == &<ID3D11Device as ::windows::core::Interface>::IID || iid == &<ID3D11Device1 as ::windows::core::Interface>::IID || iid == &<ID3D11Device2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device4_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl + ID3D11Device3_Impl {
    fn RegisterDeviceRemovedEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterDeviceRemoved(&self, dwcookie: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4_Impl, const OFFSET: isize>() -> ID3D11Device4_Vtbl {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterDeviceRemovedEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterDeviceRemoved(::core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base: ID3D11Device3_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterDeviceRemovedEvent: RegisterDeviceRemovedEvent::<Identity, Impl, OFFSET>,
            UnregisterDeviceRemoved: UnregisterDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device4 as ::windows::core::Interface>::IID || iid == &<ID3D11Device as ::windows::core::Interface>::IID || iid == &<ID3D11Device1 as ::windows::core::Interface>::IID || iid == &<ID3D11Device2 as ::windows::core::Interface>::IID || iid == &<ID3D11Device3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device5_Impl: Sized + ID3D11Device_Impl + ID3D11Device1_Impl + ID3D11Device2_Impl + ID3D11Device3_Impl + ID3D11Device4_Impl {
    fn OpenSharedFence(&self, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateFence(&self, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5_Impl, const OFFSET: isize>() -> ID3D11Device5_Vtbl {
        unsafe extern "system" fn OpenSharedFence<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenSharedFence(::core::mem::transmute_copy(&hfence), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn CreateFence<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateFence(::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into()
        }
        Self {
            base: ID3D11Device4_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenSharedFence: OpenSharedFence::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device5 as ::windows::core::Interface>::IID || iid == &<ID3D11Device as ::windows::core::Interface>::IID || iid == &<ID3D11Device1 as ::windows::core::Interface>::IID || iid == &<ID3D11Device2 as ::windows::core::Interface>::IID || iid == &<ID3D11Device3 as ::windows::core::Interface>::IID || iid == &<ID3D11Device4 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DeviceChild_Impl: Sized {
    fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D11Device>);
    fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>() -> ID3D11DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: &::core::option::Option<ID3D11PixelShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn VSSetShader(&self, pvertexshader: &::core::option::Option<ID3D11VertexShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn Map(&self, presource: &::core::option::Option<ID3D11Resource>, subresource: u32, maptype: D3D11_MAP, mapflags: u32) -> ::windows::core::Result<D3D11_MAPPED_SUBRESOURCE>;
    fn Unmap(&self, presource: &::core::option::Option<ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn IASetInputLayout(&self, pinputlayout: &::core::option::Option<ID3D11InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D11Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: &::core::option::Option<ID3D11Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn GSSetShader(&self, pshader: &::core::option::Option<ID3D11GeometryShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn Begin(&self, pasync: &::core::option::Option<ID3D11Asynchronous>);
    fn End(&self, pasync: &::core::option::Option<ID3D11Asynchronous>);
    fn GetData(&self, pasync: &::core::option::Option<ID3D11Asynchronous>, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>;
    fn SetPredication(&self, ppredicate: &::core::option::Option<ID3D11Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>);
    fn OMSetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn OMSetBlendState(&self, pblendstate: &::core::option::Option<ID3D11BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: &::core::option::Option<ID3D11DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D11Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn DrawIndexedInstancedIndirect(&self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn DrawInstancedIndirect(&self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(&self, pbufferforargs: &::core::option::Option<ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn RSSetState(&self, prasterizerstate: &::core::option::Option<ID3D11RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
    fn CopyResource(&self, pdstresource: &::core::option::Option<ID3D11Resource>, psrcresource: &::core::option::Option<ID3D11Resource>);
    fn UpdateSubresource(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn CopyStructureCount(&self, pdstbuffer: &::core::option::Option<ID3D11Buffer>, dstalignedbyteoffset: u32, psrcview: &::core::option::Option<ID3D11UnorderedAccessView>);
    fn ClearRenderTargetView(&self, prendertargetview: &::core::option::Option<ID3D11RenderTargetView>, colorrgba: *const f32);
    fn ClearUnorderedAccessViewUint(&self, punorderedaccessview: &::core::option::Option<ID3D11UnorderedAccessView>, values: *const u32);
    fn ClearUnorderedAccessViewFloat(&self, punorderedaccessview: &::core::option::Option<ID3D11UnorderedAccessView>, values: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: &::core::option::Option<ID3D11DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: &::core::option::Option<ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(&self, presource: &::core::option::Option<ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(&self, presource: &::core::option::Option<ID3D11Resource>) -> f32;
    fn ResolveSubresource(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn ExecuteCommandList(&self, pcommandlist: &::core::option::Option<ID3D11CommandList>, restorecontextstate: super::super::Foundation::BOOL);
    fn HSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSSetShader(&self, phullshader: &::core::option::Option<ID3D11HullShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn HSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn HSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn DSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSSetShader(&self, pdomainshader: &::core::option::Option<ID3D11DomainShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn DSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn CSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn CSSetShader(&self, pcomputeshader: &::core::option::Option<ID3D11ComputeShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn CSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn CSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D11InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D11Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D11Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D11Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>);
    fn OMGetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D11BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D11DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D11Buffer>);
    fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState>);
    fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn HSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSGetShader(&self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn HSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn HSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn DSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSGetShader(&self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn DSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn DSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn CSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSGetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn CSGetShader(&self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn CSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn CSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn ClearState(&self);
    fn Flush(&self);
    fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(&self) -> u32;
    fn FinishCommandList(&self, restoredeferredcontextstate: super::super::Foundation::BOOL) -> ::windows::core::Result<ID3D11CommandList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>() -> ID3D11DeviceContext_Vtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSetShader(::core::mem::transmute(&ppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSSetShader(::core::mem::transmute(&pvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DrawIndexed<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexed(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Map(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmappedresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unmap(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetInputLayout(::core::mem::transmute(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetIndexBuffer(::core::mem::transmute(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexedInstanced(::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawInstanced(::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSSetShader(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IASetPrimitiveTopology(::core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin(::core::mem::transmute(&pasync))
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End(::core::mem::transmute(&pasync))
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute(&pasync), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPredication(::core::mem::transmute(&ppredicate), ::core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetRenderTargetsAndUnorderedAccessViews(::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetBlendState(::core::mem::transmute(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMSetDepthStencilState(::core::mem::transmute(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SOSetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawAuto()
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawIndexedInstancedIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn DrawInstancedIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawInstancedIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Dispatch(::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn DispatchIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DispatchIndirect(::core::mem::transmute(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn RSSetState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetState(::core::mem::transmute(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetViewports(::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSSetScissorRects(::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopySubresourceRegion(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyResource(::core::mem::transmute(&pdstresource), ::core::mem::transmute(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn CopyStructureCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstalignedbyteoffset: u32, psrcview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyStructureCount(::core::mem::transmute(&pdstbuffer), ::core::mem::transmute_copy(&dstalignedbyteoffset), ::core::mem::transmute(&psrcview))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRenderTargetView(::core::mem::transmute(&prendertargetview), ::core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearUnorderedAccessViewUint(::core::mem::transmute(&punorderedaccessview), ::core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearUnorderedAccessViewFloat(::core::mem::transmute(&punorderedaccessview), ::core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearDepthStencilView(::core::mem::transmute(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateMips(::core::mem::transmute(&pshaderresourceview))
        }
        unsafe extern "system" fn SetResourceMinLOD<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, minlod: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResourceMinLOD(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&minlod))
        }
        unsafe extern "system" fn GetResourceMinLOD<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResourceMinLOD(::core::mem::transmute(&presource))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveSubresource(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn ExecuteCommandList<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr, restorecontextstate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecuteCommandList(::core::mem::transmute(&pcommandlist), ::core::mem::transmute_copy(&restorecontextstate))
        }
        unsafe extern "system" fn HSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phullshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSSetShader(::core::mem::transmute(&phullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn HSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSSetShader(::core::mem::transmute(&pdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSSetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetUnorderedAccessViews(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn CSSetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomputeshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetShader(::core::mem::transmute(&pcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn CSSetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSSetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSGetShader(::core::mem::transmute_copy(&pppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSGetShader(::core::mem::transmute_copy(&ppvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IAGetVertexBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSGetShader(::core::mem::transmute_copy(&ppgeometryshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMGetRenderTargets(::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMGetRenderTargetsAndUnorderedAccessViews(::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SOGetTargets(::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets))
        }
        unsafe extern "system" fn RSGetState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSGetState(::core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSGetViewports(::core::mem::transmute_copy(&pnumviewports), ::core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RSGetScissorRects(::core::mem::transmute_copy(&pnumrects), ::core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn HSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphullshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSGetShader(::core::mem::transmute_copy(&pphullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn HSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomainshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSGetShader(::core::mem::transmute_copy(&ppdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn DSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSGetShaderResources<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetShaderResources(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetUnorderedAccessViews(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn CSGetShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomputeshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetShader(::core::mem::transmute_copy(&ppcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn CSGetSamplers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetSamplers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSGetConstantBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetConstantBuffers(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn ClearState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearState()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetContextFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContextFlags()
        }
        unsafe extern "system" fn FinishCommandList<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FinishCommandList(::core::mem::transmute_copy(&restoredeferredcontextstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, Impl, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, Impl, OFFSET>,
            PSSetShader: PSSetShader::<Identity, Impl, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, Impl, OFFSET>,
            VSSetShader: VSSetShader::<Identity, Impl, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, Impl, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, Impl, OFFSET>,
            GSSetShader: GSSetShader::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, Impl, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, Impl, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, Impl, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            DrawAuto: DrawAuto::<Identity, Impl, OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Identity, Impl, OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            DispatchIndirect: DispatchIndirect::<Identity, Impl, OFFSET>,
            RSSetState: RSSetState::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, Impl, OFFSET>,
            CopyStructureCount: CopyStructureCount::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            GenerateMips: GenerateMips::<Identity, Impl, OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Identity, Impl, OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Identity, Impl, OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Identity, Impl, OFFSET>,
            HSSetShader: HSSetShader::<Identity, Impl, OFFSET>,
            HSSetSamplers: HSSetSamplers::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Identity, Impl, OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Identity, Impl, OFFSET>,
            DSSetShader: DSSetShader::<Identity, Impl, OFFSET>,
            DSSetSamplers: DSSetSamplers::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Identity, Impl, OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Identity, Impl, OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSSetShader: CSSetShader::<Identity, Impl, OFFSET>,
            CSSetSamplers: CSSetSamplers::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, Impl, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, Impl, OFFSET>,
            PSGetShader: PSGetShader::<Identity, Impl, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, Impl, OFFSET>,
            VSGetShader: VSGetShader::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, Impl, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, Impl, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, Impl, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, Impl, OFFSET>,
            GSGetShader: GSGetShader::<Identity, Impl, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, Impl, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, Impl, OFFSET>,
            GetPredication: GetPredication::<Identity, Impl, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, Impl, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, Impl, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, Impl, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, Impl, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, Impl, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, Impl, OFFSET>,
            RSGetState: RSGetState::<Identity, Impl, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, Impl, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, Impl, OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Identity, Impl, OFFSET>,
            HSGetShader: HSGetShader::<Identity, Impl, OFFSET>,
            HSGetSamplers: HSGetSamplers::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Identity, Impl, OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Identity, Impl, OFFSET>,
            DSGetShader: DSGetShader::<Identity, Impl, OFFSET>,
            DSGetSamplers: DSGetSamplers::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Identity, Impl, OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Identity, Impl, OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSGetShader: CSGetShader::<Identity, Impl, OFFSET>,
            CSGetSamplers: CSGetSamplers::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET>,
            FinishCommandList: FinishCommandList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl {
    fn CopySubresourceRegion1(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: &::core::option::Option<ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32);
    fn UpdateSubresource1(&self, pdstresource: &::core::option::Option<ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32);
    fn DiscardResource(&self, presource: &::core::option::Option<ID3D11Resource>);
    fn DiscardView(&self, presourceview: &::core::option::Option<ID3D11View>);
    fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn SwapDeviceContextState(&self, pstate: &::core::option::Option<ID3DDeviceContextState>, pppreviousstate: *mut ::core::option::Option<ID3DDeviceContextState>);
    fn ClearView(&self, pview: &::core::option::Option<ID3D11View>, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32);
    fn DiscardView1(&self, presourceview: &::core::option::Option<ID3D11View>, prects: *const super::super::Foundation::RECT, numrects: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>() -> ID3D11DeviceContext1_Vtbl {
        unsafe extern "system" fn CopySubresourceRegion1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopySubresourceRegion1(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::core::mem::transmute(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox), ::core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn UpdateSubresource1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateSubresource1(::core::mem::transmute(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch), ::core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn DiscardResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardResource(::core::mem::transmute(&presource))
        }
        unsafe extern "system" fn DiscardView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardView(::core::mem::transmute(&presourceview))
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSSetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CSGetConstantBuffers1(::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn SwapDeviceContextState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: ::windows::core::RawPtr, pppreviousstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwapDeviceContextState(::core::mem::transmute(&pstate), ::core::mem::transmute_copy(&pppreviousstate))
        }
        unsafe extern "system" fn ClearView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: ::windows::core::RawPtr, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearView(::core::mem::transmute(&pview), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&numrects))
        }
        unsafe extern "system" fn DiscardView1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr, prects: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardView1(::core::mem::transmute(&presourceview), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&numrects))
        }
        Self {
            base: ID3D11DeviceContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            CopySubresourceRegion1: CopySubresourceRegion1::<Identity, Impl, OFFSET>,
            UpdateSubresource1: UpdateSubresource1::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            DiscardView: DiscardView::<Identity, Impl, OFFSET>,
            VSSetConstantBuffers1: VSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers1: HSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers1: DSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers1: GSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers1: PSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers1: CSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers1: VSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers1: HSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers1: DSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers1: GSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers1: PSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers1: CSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            SwapDeviceContextState: SwapDeviceContextState::<Identity, Impl, OFFSET>,
            ClearView: ClearView::<Identity, Impl, OFFSET>,
            DiscardView1: DiscardView1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl {
    fn UpdateTileMappings(&self, ptiledresource: &::core::option::Option<ID3D11Resource>, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: &::core::option::Option<ID3D11Buffer>, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::Result<()>;
    fn CopyTileMappings(&self, pdesttiledresource: &::core::option::Option<ID3D11Resource>, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: &::core::option::Option<ID3D11Resource>, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::Result<()>;
    fn CopyTiles(&self, ptiledresource: &::core::option::Option<ID3D11Resource>, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: &::core::option::Option<ID3D11Buffer>, bufferstartoffsetinbytes: u64, flags: u32);
    fn UpdateTiles(&self, pdesttiledresource: &::core::option::Option<ID3D11Resource>, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32);
    fn ResizeTilePool(&self, ptilepool: &::core::option::Option<ID3D11Buffer>, newsizeinbytes: u64) -> ::windows::core::Result<()>;
    fn TiledResourceBarrier(&self, ptiledresourceorviewaccessbeforebarrier: &::core::option::Option<ID3D11DeviceChild>, ptiledresourceorviewaccessafterbarrier: &::core::option::Option<ID3D11DeviceChild>);
    fn IsAnnotationEnabled(&self) -> super::super::Foundation::BOOL;
    fn SetMarkerInt(&self, plabel: &::windows::core::PCWSTR, data: i32);
    fn BeginEventInt(&self, plabel: &::windows::core::PCWSTR, data: i32);
    fn EndEvent(&self);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>() -> ID3D11DeviceContext2_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: ::windows::core::RawPtr, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .UpdateTileMappings(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&numtiledresourceregions), ::core::mem::transmute_copy(&ptiledresourceregionstartcoordinates), ::core::mem::transmute_copy(&ptiledresourceregionsizes), ::core::mem::transmute(&ptilepool), ::core::mem::transmute_copy(&numranges), ::core::mem::transmute_copy(&prangeflags), ::core::mem::transmute_copy(&ptilepoolstartoffsets), ::core::mem::transmute_copy(&prangetilecounts), ::core::mem::transmute_copy(&flags))
                .into()
        }
        unsafe extern "system" fn CopyTileMappings<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: ::windows::core::RawPtr, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTileMappings(::core::mem::transmute(&pdesttiledresource), ::core::mem::transmute_copy(&pdestregionstartcoordinate), ::core::mem::transmute(&psourcetiledresource), ::core::mem::transmute_copy(&psourceregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CopyTiles<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTiles(::core::mem::transmute(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn UpdateTiles<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateTiles(::core::mem::transmute(&pdesttiledresource), ::core::mem::transmute_copy(&pdesttileregionstartcoordinate), ::core::mem::transmute_copy(&pdesttileregionsize), ::core::mem::transmute_copy(&psourcetiledata), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResizeTilePool<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilepool: ::windows::core::RawPtr, newsizeinbytes: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResizeTilePool(::core::mem::transmute(&ptilepool), ::core::mem::transmute_copy(&newsizeinbytes)).into()
        }
        unsafe extern "system" fn TiledResourceBarrier<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: ::windows::core::RawPtr, ptiledresourceorviewaccessafterbarrier: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TiledResourceBarrier(::core::mem::transmute(&ptiledresourceorviewaccessbeforebarrier), ::core::mem::transmute(&ptiledresourceorviewaccessafterbarrier))
        }
        unsafe extern "system" fn IsAnnotationEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsAnnotationEnabled()
        }
        unsafe extern "system" fn SetMarkerInt<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: ::windows::core::PCWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarkerInt(::core::mem::transmute(&plabel), ::core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn BeginEventInt<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: ::windows::core::PCWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEventInt(::core::mem::transmute(&plabel), ::core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndEvent()
        }
        Self {
            base: ID3D11DeviceContext1_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            UpdateTiles: UpdateTiles::<Identity, Impl, OFFSET>,
            ResizeTilePool: ResizeTilePool::<Identity, Impl, OFFSET>,
            TiledResourceBarrier: TiledResourceBarrier::<Identity, Impl, OFFSET>,
            IsAnnotationEnabled: IsAnnotationEnabled::<Identity, Impl, OFFSET>,
            SetMarkerInt: SetMarkerInt::<Identity, Impl, OFFSET>,
            BeginEventInt: BeginEventInt::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext2 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext3_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl + ID3D11DeviceContext2_Impl {
    fn Flush1(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE);
    fn SetHardwareProtectionState(&self, hwprotectionenable: super::super::Foundation::BOOL);
    fn GetHardwareProtectionState(&self, phwprotectionenable: *mut super::super::Foundation::BOOL);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>() -> ID3D11DeviceContext3_Vtbl {
        unsafe extern "system" fn Flush1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush1(::core::mem::transmute_copy(&contexttype), ::core::mem::transmute_copy(&hevent))
        }
        unsafe extern "system" fn SetHardwareProtectionState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHardwareProtectionState(::core::mem::transmute_copy(&hwprotectionenable))
        }
        unsafe extern "system" fn GetHardwareProtectionState<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHardwareProtectionState(::core::mem::transmute_copy(&phwprotectionenable))
        }
        Self {
            base: ID3D11DeviceContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            Flush1: Flush1::<Identity, Impl, OFFSET>,
            SetHardwareProtectionState: SetHardwareProtectionState::<Identity, Impl, OFFSET>,
            GetHardwareProtectionState: GetHardwareProtectionState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext3 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext4_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11DeviceContext_Impl + ID3D11DeviceContext1_Impl + ID3D11DeviceContext2_Impl + ID3D11DeviceContext3_Impl {
    fn Signal(&self, pfence: &::core::option::Option<ID3D11Fence>, value: u64) -> ::windows::core::Result<()>;
    fn Wait(&self, pfence: &::core::option::Option<ID3D11Fence>, value: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>() -> ID3D11DeviceContext4_Vtbl {
        unsafe extern "system" fn Signal<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Signal(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Wait(::core::mem::transmute(&pfence), ::core::mem::transmute_copy(&value)).into()
        }
        Self { base: ID3D11DeviceContext3_Vtbl::new::<Identity, Impl, OFFSET>(), Signal: Signal::<Identity, Impl, OFFSET>, Wait: Wait::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext4 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext2 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceContext3 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DomainShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11DomainShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DomainShader_Impl, const OFFSET: isize>() -> ID3D11DomainShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DomainShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ID3D11Fence_Impl: Sized + ID3D11DeviceChild_Impl {
    fn CreateSharedHandle(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetCompletedValue(&self) -> u64;
    fn SetEventOnCompletion(&self, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ID3D11Fence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Fence_Impl, const OFFSET: isize>() -> ID3D11Fence_Vtbl {
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompletedValue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompletedValue()
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventOnCompletion(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Fence as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionLinkingGraph_Impl: Sized {
    fn CreateModuleInstance(&self, ppmoduleinstance: *mut ::core::option::Option<ID3D11ModuleInstance>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>;
    fn SetInputSignature(&self, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn SetOutputSignature(&self, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn CallFunction(&self, pmoduleinstancenamespace: &::windows::core::PCSTR, pmodulewithfunctionprototype: &::core::option::Option<ID3D11Module>, pfunctionname: &::windows::core::PCSTR) -> ::windows::core::Result<ID3D11LinkingNode>;
    fn PassValue(&self, psrcnode: &::core::option::Option<ID3D11LinkingNode>, srcparameterindex: i32, pdstnode: &::core::option::Option<ID3D11LinkingNode>, dstparameterindex: i32) -> ::windows::core::Result<()>;
    fn PassValueWithSwizzle(&self, psrcnode: &::core::option::Option<ID3D11LinkingNode>, srcparameterindex: i32, psrcswizzle: &::windows::core::PCSTR, pdstnode: &::core::option::Option<ID3D11LinkingNode>, dstparameterindex: i32, pdstswizzle: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn GetLastError(&self) -> ::windows::core::Result<super::Direct3D::ID3DBlob>;
    fn GenerateHlsl(&self, uflags: u32) -> ::windows::core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionLinkingGraph_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>() -> ID3D11FunctionLinkingGraph_Vtbl {
        unsafe extern "system" fn CreateModuleInstance<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmoduleinstance: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateModuleInstance(::core::mem::transmute_copy(&ppmoduleinstance), ::core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn SetInputSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetInputSignature(::core::mem::transmute_copy(&pinputparameters), ::core::mem::transmute_copy(&cinputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinputnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSignature<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetOutputSignature(::core::mem::transmute_copy(&poutputparameters), ::core::mem::transmute_copy(&coutputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallFunction<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmoduleinstancenamespace: ::windows::core::PCSTR, pmodulewithfunctionprototype: ::windows::core::RawPtr, pfunctionname: ::windows::core::PCSTR, ppcallnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallFunction(::core::mem::transmute(&pmoduleinstancenamespace), ::core::mem::transmute(&pmodulewithfunctionprototype), ::core::mem::transmute(&pfunctionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PassValue<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PassValue(::core::mem::transmute(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::core::mem::transmute(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex)).into()
        }
        unsafe extern "system" fn PassValueWithSwizzle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, psrcswizzle: ::windows::core::PCSTR, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32, pdstswizzle: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PassValueWithSwizzle(::core::mem::transmute(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::core::mem::transmute(&psrcswizzle), ::core::mem::transmute(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex), ::core::mem::transmute(&pdstswizzle)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    *pperrorbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateHlsl<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateHlsl(::core::mem::transmute_copy(&uflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateModuleInstance: CreateModuleInstance::<Identity, Impl, OFFSET>,
            SetInputSignature: SetInputSignature::<Identity, Impl, OFFSET>,
            SetOutputSignature: SetOutputSignature::<Identity, Impl, OFFSET>,
            CallFunction: CallFunction::<Identity, Impl, OFFSET>,
            PassValue: PassValue::<Identity, Impl, OFFSET>,
            PassValueWithSwizzle: PassValueWithSwizzle::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GenerateHlsl: GenerateHlsl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionLinkingGraph as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_PARAMETER_DESC>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionParameterReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionParameterReflection_Impl, const OFFSET: isize>() -> ID3D11FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionParameterReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionParameterReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionReflection_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_FUNCTION_DESC>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &::windows::core::PCSTR) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>() -> ID3D11FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunctionParameter(::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetFunctionParameter: GetFunctionParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionReflection as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11GeometryShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11GeometryShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11GeometryShader_Impl, const OFFSET: isize>() -> ID3D11GeometryShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11HullShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11HullShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11HullShader_Impl, const OFFSET: isize>() -> ID3D11HullShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11HullShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D11_MESSAGE_SEVERITY, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&self, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11InfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>() -> ID3D11InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStoredMessages()
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter()
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessages()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilter()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit()
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessageCountLimit()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStorageFilter()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyStorageFilter().into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfStorageFilter().into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopStorageFilter()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilterStackSize()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRetrievalFilter()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyRetrievalFilter().into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfRetrievalFilter().into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopRetrievalFilter()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilterStackSize()
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMuteDebugOutput()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11InputLayout_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11InputLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InputLayout_Impl, const OFFSET: isize>() -> ID3D11InputLayout_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InputLayout as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11LibraryReflection_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_LIBRARY_DESC>;
    fn GetFunctionByIndex(&self, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection>;
}
impl ID3D11LibraryReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>() -> ID3D11LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFunctionByIndex(::core::mem::transmute_copy(&functionindex))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LibraryReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11Linker_Impl: Sized {
    fn Link(&self, pentry: &::core::option::Option<ID3D11ModuleInstance>, pentryname: &::windows::core::PCSTR, ptargetname: &::windows::core::PCSTR, uflags: u32, ppshaderblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>;
    fn UseLibrary(&self, plibrarymi: &::core::option::Option<ID3D11ModuleInstance>) -> ::windows::core::Result<()>;
    fn AddClipPlaneFromCBuffer(&self, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11Linker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Linker_Impl, const OFFSET: isize>() -> ID3D11Linker_Vtbl {
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentry: ::windows::core::RawPtr, pentryname: ::windows::core::PCSTR, ptargetname: ::windows::core::PCSTR, uflags: u32, ppshaderblob: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Link(::core::mem::transmute(&pentry), ::core::mem::transmute(&pentryname), ::core::mem::transmute(&ptargetname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ppshaderblob), ::core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn UseLibrary<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrarymi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UseLibrary(::core::mem::transmute(&plibrarymi)).into()
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddClipPlaneFromCBuffer(::core::mem::transmute_copy(&ucbufferslot), ::core::mem::transmute_copy(&ucbufferentry)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Link: Link::<Identity, Impl, OFFSET>,
            UseLibrary: UseLibrary::<Identity, Impl, OFFSET>,
            AddClipPlaneFromCBuffer: AddClipPlaneFromCBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Linker as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11LinkingNode_Impl: Sized {}
impl ID3D11LinkingNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkingNode_Impl, const OFFSET: isize>() -> ID3D11LinkingNode_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LinkingNode as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Module_Impl: Sized {
    fn CreateInstance(&self, pnamespace: &::windows::core::PCSTR) -> ::windows::core::Result<ID3D11ModuleInstance>;
}
impl ID3D11Module_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Module_Impl, const OFFSET: isize>() -> ID3D11Module_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Module_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::PCSTR, ppmoduleinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute(&pnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmoduleinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Module as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ModuleInstance_Impl: Sized {
    fn BindConstantBuffer(&self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::Result<()>;
    fn BindConstantBufferByName(&self, pname: &::windows::core::PCSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::Result<()>;
    fn BindResource(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceByName(&self, pname: &::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindSampler(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindSamplerByName(&self, pname: &::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindUnorderedAccessView(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindUnorderedAccessViewByName(&self, pname: &::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceAsUnorderedAccessView(&self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::Result<()>;
    fn BindResourceAsUnorderedAccessViewByName(&self, psrvname: &::windows::core::PCSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::Result<()>;
}
impl ID3D11ModuleInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>() -> ID3D11ModuleInstance_Vtbl {
        unsafe extern "system" fn BindConstantBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindConstantBuffer(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindConstantBufferByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindConstantBufferByName(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindResource(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindResourceByName(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSampler<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindSampler(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSamplerByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindSamplerByName(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindUnorderedAccessView(::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindUnorderedAccessViewByName(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindResourceAsUnorderedAccessView(::core::mem::transmute_copy(&usrcsrvslot), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrvname: ::windows::core::PCSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindResourceAsUnorderedAccessViewByName(::core::mem::transmute(&psrvname), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BindConstantBuffer: BindConstantBuffer::<Identity, Impl, OFFSET>,
            BindConstantBufferByName: BindConstantBufferByName::<Identity, Impl, OFFSET>,
            BindResource: BindResource::<Identity, Impl, OFFSET>,
            BindResourceByName: BindResourceByName::<Identity, Impl, OFFSET>,
            BindSampler: BindSampler::<Identity, Impl, OFFSET>,
            BindSamplerByName: BindSamplerByName::<Identity, Impl, OFFSET>,
            BindUnorderedAccessView: BindUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindUnorderedAccessViewByName: BindUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessView: BindResourceAsUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessViewByName: BindResourceAsUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ModuleInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11Multithread_Impl: Sized {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11Multithread_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const OFFSET: isize>() -> ID3D11Multithread_Vtbl {
        unsafe extern "system" fn Enter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enter()
        }
        unsafe extern "system" fn Leave<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Leave()
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultithreadProtected(::core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMultithreadProtected()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, Impl, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11PixelShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11PixelShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PixelShader_Impl, const OFFSET: isize>() -> ID3D11PixelShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11PixelShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Predicate_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl + ID3D11Query_Impl {}
impl ID3D11Predicate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Predicate_Impl, const OFFSET: isize>() -> ID3D11Predicate_Vtbl {
        Self { base: ID3D11Query_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Predicate as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID || iid == &<ID3D11Query as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Query_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_QUERY_DESC);
}
impl ID3D11Query_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query_Impl, const OFFSET: isize>() -> ID3D11Query_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Query1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Asynchronous_Impl + ID3D11Query_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_QUERY_DESC1);
}
impl ID3D11Query1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query1_Impl, const OFFSET: isize>() -> ID3D11Query1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11Query_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID || iid == &<ID3D11Query as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState_Impl, const OFFSET: isize>() -> ID3D11RasterizerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11RasterizerState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState1_Impl, const OFFSET: isize>() -> ID3D11RasterizerState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11RasterizerState_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11RasterizerState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11RasterizerState_Impl + ID3D11RasterizerState1_Impl {
    fn GetDesc2(&self, pdesc: *mut D3D11_RASTERIZER_DESC2);
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState2_Impl, const OFFSET: isize>() -> ID3D11RasterizerState2_Vtbl {
        unsafe extern "system" fn GetDesc2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc2(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11RasterizerState1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState2 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11RasterizerState as ::windows::core::Interface>::IID || iid == &<ID3D11RasterizerState1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefDefaultTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&self, resourcetypeflags: u32, options: u32) -> ::windows::core::Result<()>;
}
impl ID3D11RefDefaultTrackingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: isize>() -> ID3D11RefDefaultTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrackingOptions(::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefDefaultTrackingOptions as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&self, uoptions: u32) -> ::windows::core::Result<()>;
}
impl ID3D11RefTrackingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: isize>() -> ID3D11RefTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrackingOptions(::core::mem::transmute_copy(&uoptions)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefTrackingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView_Impl, const OFFSET: isize>() -> ID3D11RenderTargetView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11RenderTargetView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: isize>() -> ID3D11RenderTargetView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11RenderTargetView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID || iid == &<ID3D11RenderTargetView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Resource_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetType(&self, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ID3D11Resource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Resource_Impl, const OFFSET: isize>() -> ID3D11Resource_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&presourcedimension))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEvictionPriority()
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Resource as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11SamplerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SAMPLER_DESC);
}
impl ID3D11SamplerState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SamplerState_Impl, const OFFSET: isize>() -> ID3D11SamplerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SamplerState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SamplerState as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflection_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_SHADER_DESC>;
    fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetPatchConstantParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D11_SIGNATURE_PARAMETER_DESC>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &::windows::core::PCSTR) -> ::windows::core::Result<D3D11_SHADER_INPUT_BIND_DESC>;
    fn GetMovInstructionCount(&self) -> u32;
    fn GetMovcInstructionCount(&self) -> u32;
    fn GetConversionInstructionCount(&self) -> u32;
    fn GetBitwiseInstructionCount(&self) -> u32;
    fn GetGSInputPrimitive(&self) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&self) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(&self) -> u32;
    fn GetMinFeatureLevel(&self) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&self) -> u64;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>() -> ID3D11ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstantBufferByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDesc(::core::mem::transmute_copy(&resourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(::core::mem::transmute_copy(&parameterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceBindingDescByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMovInstructionCount()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMovcInstructionCount()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionInstructionCount()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBitwiseInstructionCount()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGSInputPrimitive()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSampleFrequencyShader()
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumInterfaceSlots()
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMinFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRequiresFlags()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>() -> ID3D11ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVariableByName(::core::mem::transmute(&name))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_SHADER_TYPE_DESC>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> ::windows::core::PSTR;
    fn IsEqual(&self, ptype: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn GetSubType(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBaseClass(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn IsOfType(&self, ptype: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
    fn ImplementsInterface(&self, pbase: &::core::option::Option<ID3D11ShaderReflectionType>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>() -> ID3D11ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeByName(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMemberTypeName(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEqual(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubType()
        }
        unsafe extern "system" fn GetBaseClass<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBaseClass()
        }
        unsafe extern "system" fn GetNumInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumInterfaces()
        }
        unsafe extern "system" fn GetInterfaceByIndex<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterfaceByIndex(::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOfType(::core::mem::transmute(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImplementsInterface(::core::mem::transmute(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity, Impl, OFFSET>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity, Impl, OFFSET>,
            GetMemberTypeName: GetMemberTypeName::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            GetSubType: GetSubType::<Identity, Impl, OFFSET>,
            GetBaseClass: GetBaseClass::<Identity, Impl, OFFSET>,
            GetNumInterfaces: GetNumInterfaces::<Identity, Impl, OFFSET>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
            ImplementsInterface: ImplementsInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self) -> ::windows::core::Result<D3D11_SHADER_VARIABLE_DESC>;
    fn GetType(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBuffer(&self) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D11ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>() -> ID3D11ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetType()
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBuffer()
        }
        unsafe extern "system" fn GetInterfaceSlot<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterfaceSlot(::core::mem::transmute_copy(&uarrayindex))
        }
        Self {
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetInterfaceSlot: GetInterfaceSlot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: isize>() -> ID3D11ShaderResourceView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>() -> ID3D11ShaderResourceView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11ShaderResourceView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID || iid == &<ID3D11ShaderResourceView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderTrace_Impl: Sized {
    fn TraceReady(&self) -> ::windows::core::Result<u64>;
    fn ResetTrace(&self);
    fn GetTraceStats(&self) -> ::windows::core::Result<D3D11_TRACE_STATS>;
    fn PSSelectStamp(&self, stampindex: u32) -> ::windows::core::Result<()>;
    fn GetInitialRegisterContents(&self, pregister: *const D3D11_TRACE_REGISTER) -> ::windows::core::Result<D3D11_TRACE_VALUE>;
    fn GetStep(&self, stepindex: u32) -> ::windows::core::Result<D3D11_TRACE_STEP>;
    fn GetWrittenRegister(&self, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::Result<()>;
    fn GetReadRegister(&self, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ShaderTrace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>() -> ID3D11ShaderTrace_Vtbl {
        unsafe extern "system" fn TraceReady<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptestcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TraceReady() {
                ::core::result::Result::Ok(ok__) => {
                    *ptestcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetTrace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetTrace()
        }
        unsafe extern "system" fn GetTraceStats<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTraceStats() {
                ::core::result::Result::Ok(ok__) => {
                    *ptracestats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PSSelectStamp<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stampindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PSSelectStamp(::core::mem::transmute_copy(&stampindex)).into()
        }
        unsafe extern "system" fn GetInitialRegisterContents<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInitialRegisterContents(::core::mem::transmute_copy(&pregister)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStep<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStep(::core::mem::transmute_copy(&stepindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptracestep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWrittenRegister<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWrittenRegister(::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&writtenregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadRegister<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetReadRegister(::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&readregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TraceReady: TraceReady::<Identity, Impl, OFFSET>,
            ResetTrace: ResetTrace::<Identity, Impl, OFFSET>,
            GetTraceStats: GetTraceStats::<Identity, Impl, OFFSET>,
            PSSelectStamp: PSSelectStamp::<Identity, Impl, OFFSET>,
            GetInitialRegisterContents: GetInitialRegisterContents::<Identity, Impl, OFFSET>,
            GetStep: GetStep::<Identity, Impl, OFFSET>,
            GetWrittenRegister: GetWrittenRegister::<Identity, Impl, OFFSET>,
            GetReadRegister: GetReadRegister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTrace as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ShaderTraceFactory_Impl: Sized {
    fn CreateShaderTrace(&self, pshader: &::core::option::Option<::windows::core::IUnknown>, ptracedesc: *const D3D11_SHADER_TRACE_DESC) -> ::windows::core::Result<ID3D11ShaderTrace>;
}
impl ID3D11ShaderTraceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: isize>() -> ID3D11ShaderTraceFactory_Vtbl {
        unsafe extern "system" fn CreateShaderTrace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateShaderTrace(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&ptracedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppshadertrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateShaderTrace: CreateShaderTrace::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTraceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11SwitchToRef_Impl: Sized {
    fn SetUseRef(&self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11SwitchToRef_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>() -> ID3D11SwitchToRef_Vtbl {
        unsafe extern "system" fn SetUseRef<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseRef(::core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUseRef()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetUseRef: SetUseRef::<Identity, Impl, OFFSET>,
            GetUseRef: GetUseRef::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture1D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture1D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture1D_Impl, const OFFSET: isize>() -> ID3D11Texture1D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture1D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture1D as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D_Impl, const OFFSET: isize>() -> ID3D11Texture2D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl + ID3D11Texture2D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE2D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D1_Impl, const OFFSET: isize>() -> ID3D11Texture2D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Texture2D_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID || iid == &<ID3D11Texture2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D_Impl, const OFFSET: isize>() -> ID3D11Texture3D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11Resource_Impl + ID3D11Texture3D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE3D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D1_Impl, const OFFSET: isize>() -> ID3D11Texture3D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11Texture3D_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11Resource as ::windows::core::Interface>::IID || iid == &<ID3D11Texture3D as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11TracingDevice_Impl: Sized {
    fn SetShaderTrackingOptionsByType(&self, resourcetypeflags: u32, options: u32) -> ::windows::core::Result<()>;
    fn SetShaderTrackingOptions(&self, pshader: &::core::option::Option<::windows::core::IUnknown>, options: u32) -> ::windows::core::Result<()>;
}
impl ID3D11TracingDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>() -> ID3D11TracingDevice_Vtbl {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShaderTrackingOptionsByType(::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShaderTrackingOptions(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetShaderTrackingOptionsByType: SetShaderTrackingOptionsByType::<Identity, Impl, OFFSET>,
            SetShaderTrackingOptions: SetShaderTrackingOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11TracingDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>() -> ID3D11UnorderedAccessView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl + ID3D11UnorderedAccessView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>() -> ID3D11UnorderedAccessView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1))
        }
        Self { base: ID3D11UnorderedAccessView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID || iid == &<ID3D11UnorderedAccessView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VertexShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3D11VertexShader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VertexShader_Impl, const OFFSET: isize>() -> ID3D11VertexShader_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VertexShader as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseDecoderBuffer(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::Result<()>;
    fn DecoderBeginFrame(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pview: &::core::option::Option<ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DecoderEndFrame(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>) -> ::windows::core::Result<()>;
    fn SubmitDecoderBuffers(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::Result<()>;
    fn DecoderExtension(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32;
    fn VideoProcessorSetOutputTargetRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetOutputBackgroundColor(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR);
    fn VideoProcessorSetOutputColorSpace(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetOutputAlphaFillMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32);
    fn VideoProcessorSetOutputConstriction(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, size: &super::super::Foundation::SIZE);
    fn VideoProcessorSetOutputStereoMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetOutputExtension(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetOutputTargetRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetOutputBackgroundColor(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR);
    fn VideoProcessorGetOutputColorSpace(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetOutputAlphaFillMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32);
    fn VideoProcessorGetOutputConstriction(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE);
    fn VideoProcessorGetOutputStereoMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputExtension(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorSetStreamFrameFormat(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorSetStreamColorSpace(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetStreamOutputRate(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamSourceRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamDestRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamAlpha(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32);
    fn VideoProcessorSetStreamPalette(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *const u32);
    fn VideoProcessorSetStreamPixelAspectRatio(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamLumaKey(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32);
    fn VideoProcessorSetStreamStereoFormat(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32);
    fn VideoProcessorSetStreamAutoProcessingMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamFilter(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32);
    fn VideoProcessorSetStreamExtension(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetStreamFrameFormat(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorGetStreamColorSpace(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetStreamOutputRate(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamSourceRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamDestRect(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamAlpha(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32);
    fn VideoProcessorGetStreamPalette(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *mut u32);
    fn VideoProcessorGetStreamPixelAspectRatio(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamLumaKey(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32);
    fn VideoProcessorGetStreamStereoFormat(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32);
    fn VideoProcessorGetStreamAutoProcessingMode(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamFilter(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32);
    fn VideoProcessorGetStreamExtension(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorBlt(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pview: &::core::option::Option<ID3D11VideoProcessorOutputView>, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EncryptionBlt(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, psrcsurface: &::core::option::Option<ID3D11Texture2D>, pdstsurface: &::core::option::Option<ID3D11Texture2D>, ivsize: u32, piv: *const ::core::ffi::c_void);
    fn DecryptionBlt(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, psrcsurface: &::core::option::Option<ID3D11Texture2D>, pdstsurface: &::core::option::Option<ID3D11Texture2D>, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void);
    fn StartSessionKeyRefresh(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void);
    fn FinishSessionKeyRefresh(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>);
    fn GetEncryptionBltKey(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(&self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn QueryAuthenticatedChannel(&self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ConfigureAuthenticatedChannel(&self, pchannel: &::core::option::Option<ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void) -> ::windows::core::Result<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>;
    fn VideoProcessorSetStreamRotation(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION);
    fn VideoProcessorGetStreamRotation(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>() -> ID3D11VideoContext_Vtbl {
        unsafe extern "system" fn GetDecoderBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDecoderBuffer(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pbuffersize), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDecoderBuffer(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn DecoderBeginFrame<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderBeginFrame(::core::mem::transmute(&pdecoder), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey)).into()
        }
        unsafe extern "system" fn DecoderEndFrame<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderEndFrame(::core::mem::transmute(&pdecoder)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubmitDecoderBuffers(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn DecoderExtension<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderExtension(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&pextensiondata))
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputTargetRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputBackgroundColor(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&ycbcr), ::core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputAlphaFillMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&alphafillmode), ::core::mem::transmute_copy(&streamindex))
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputConstriction(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute(&size))
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputStereoMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputTargetRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&enabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputBackgroundColor(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pycbcr), ::core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputAlphaFillMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&palphafillmode), ::core::mem::transmute_copy(&pstreamindex))
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputConstriction(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psize))
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputStereoMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamFrameFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&frameformat))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamOutputRate(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&outputrate), ::core::mem::transmute_copy(&repeatframe), ::core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamSourceRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamDestRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamAlpha(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&alpha))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamPalette(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamPixelAspectRatio(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamLumaKey(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&lower), ::core::mem::transmute_copy(&upper))
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamStereoFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&leftviewframe0), ::core::mem::transmute_copy(&baseviewframe0), ::core::mem::transmute_copy(&flipmode), ::core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamAutoProcessingMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamFilter(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&level))
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamFrameFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pframeformat))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamOutputRate(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&poutputrate), ::core::mem::transmute_copy(&prepeatframe), ::core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamSourceRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamDestRect(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamAlpha(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&palpha))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamPalette(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamPixelAspectRatio(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamLumaKey(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plower), ::core::mem::transmute_copy(&pupper))
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamStereoFormat(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pleftviewframe0), ::core::mem::transmute_copy(&pbaseviewframe0), ::core::mem::transmute_copy(&pflipmode), ::core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamAutoProcessingMode(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamFilter(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plevel))
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamExtension(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorBlt<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorBlt(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&outputframe), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NegotiateCryptoSessionKeyExchange(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn EncryptionBlt<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EncryptionBlt(::core::mem::transmute(&pcryptosession), ::core::mem::transmute(&psrcsurface), ::core::mem::transmute(&pdstsurface), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn DecryptionBlt<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecryptionBlt(::core::mem::transmute(&pcryptosession), ::core::mem::transmute(&psrcsurface), ::core::mem::transmute(&pdstsurface), ::core::mem::transmute_copy(&pencryptedblockinfo), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartSessionKeyRefresh(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&randomnumbersize), ::core::mem::transmute_copy(&prandomnumber))
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishSessionKeyRefresh(::core::mem::transmute(&pcryptosession))
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEncryptionBltKey(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&preadbackkey)).into()
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NegotiateAuthenticatedChannelKeyExchange(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryAuthenticatedChannel(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput), ::core::mem::transmute_copy(&outputsize), ::core::mem::transmute_copy(&poutput)).into()
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConfigureAuthenticatedChannel(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamRotation(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&rotation))
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamRotation(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&protation))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDecoderBuffer: GetDecoderBuffer::<Identity, Impl, OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Identity, Impl, OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Identity, Impl, OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Identity, Impl, OFFSET>,
            DecoderExtension: DecoderExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Identity, Impl, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, Impl, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, Impl, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, Impl, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, Impl, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, Impl, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<Identity, Impl, OFFSET>,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Identity, Impl, OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl {
    fn SubmitDecoderBuffers1(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::Result<()>;
    fn GetDataForNewHardwareKey(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn CheckCryptoSessionStatus(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>) -> ::windows::core::Result<D3D11_CRYPTO_SESSION_STATUS>;
    fn DecoderEnableDownsampling(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::Result<()>;
    fn DecoderUpdateDownsampling(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::Result<()>;
    fn VideoProcessorSetOutputColorSpace1(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetOutputShaderUsage(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, shaderusage: super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputColorSpace1(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetOutputShaderUsage(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, pshaderusage: *mut super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamColorSpace1(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetStreamMirror(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamColorSpace1(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetStreamMirror(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetBehaviorHints(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>() -> ID3D11VideoContext1_Vtbl {
        unsafe extern "system" fn SubmitDecoderBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubmitDecoderBuffers1(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void, pprivateoutputdata: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataForNewHardwareKey(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&privateinputsize), ::core::mem::transmute_copy(&pprivatinputdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprivateoutputdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckCryptoSessionStatus(::core::mem::transmute(&pcryptosession)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderEnableDownsampling(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&referenceframecount)).into()
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderUpdateDownsampling(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&poutputdesc)).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, shaderusage: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputShaderUsage(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&shaderusage))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pshaderusage: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputShaderUsage(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&pshaderusage))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamMirror(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&fliphorizontal), ::core::mem::transmute_copy(&flipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace1(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamMirror(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pfliphorizontal), ::core::mem::transmute_copy(&pflipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VideoProcessorGetBehaviorHints(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&outputwidth), ::core::mem::transmute_copy(&outputheight), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbehaviorhints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmitDecoderBuffers1: SubmitDecoderBuffers1::<Identity, Impl, OFFSET>,
            GetDataForNewHardwareKey: GetDataForNewHardwareKey::<Identity, Impl, OFFSET>,
            CheckCryptoSessionStatus: CheckCryptoSessionStatus::<Identity, Impl, OFFSET>,
            DecoderEnableDownsampling: DecoderEnableDownsampling::<Identity, Impl, OFFSET>,
            DecoderUpdateDownsampling: DecoderUpdateDownsampling::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace1: VideoProcessorSetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputShaderUsage: VideoProcessorSetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace1: VideoProcessorGetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputShaderUsage: VideoProcessorGetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace1: VideoProcessorSetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamMirror: VideoProcessorSetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace1: VideoProcessorGetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamMirror: VideoProcessorGetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetBehaviorHints: VideoProcessorGetBehaviorHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext2_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl + ID3D11VideoContext1_Impl {
    fn VideoProcessorSetOutputHDRMetaData(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetOutputHDRMetaData(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
    fn VideoProcessorSetStreamHDRMetaData(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetStreamHDRMetaData(&self, pvideoprocessor: &::core::option::Option<ID3D11VideoProcessor>, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>() -> ID3D11VideoContext2_Vtbl {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetOutputHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetOutputHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorSetStreamHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VideoProcessorGetStreamHDRMetaData(::core::mem::transmute(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata))
        }
        Self {
            base: ID3D11VideoContext1_Vtbl::new::<Identity, Impl, OFFSET>(),
            VideoProcessorSetOutputHDRMetaData: VideoProcessorSetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputHDRMetaData: VideoProcessorGetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamHDRMetaData: VideoProcessorSetStreamHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamHDRMetaData: VideoProcessorGetStreamHDRMetaData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext2 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext3_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoContext_Impl + ID3D11VideoContext1_Impl + ID3D11VideoContext2_Impl {
    fn DecoderBeginFrame1(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, pview: &::core::option::Option<ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::core::option::Option<ID3D11Buffer>) -> ::windows::core::Result<()>;
    fn SubmitDecoderBuffers2(&self, pdecoder: &::core::option::Option<ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>() -> ID3D11VideoContext3_Vtbl {
        unsafe extern "system" fn DecoderBeginFrame1<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecoderBeginFrame1(::core::mem::transmute(&pdecoder), ::core::mem::transmute(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&numcomponenthistograms), ::core::mem::transmute_copy(&phistogramoffsets), ::core::mem::transmute_copy(&pphistogrambuffers)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubmitDecoderBuffers2(::core::mem::transmute(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into()
        }
        Self {
            base: ID3D11VideoContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            DecoderBeginFrame1: DecoderBeginFrame1::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers2: SubmitDecoderBuffers2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext3 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext1 as ::windows::core::Interface>::IID || iid == &<ID3D11VideoContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDecoder_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCreationParameters(&self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<()>;
    fn GetDriverHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDecoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>() -> ID3D11VideoDecoder_Vtbl {
        unsafe extern "system" fn GetCreationParameters<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationParameters(::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)).into()
        }
        unsafe extern "system" fn GetDriverHandle<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDriverHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pdriverhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCreationParameters: GetCreationParameters::<Identity, Impl, OFFSET>,
            GetDriverHandle: GetDriverHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoder as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoDecoderOutputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoDecoderOutputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>() -> ID3D11VideoDecoderOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoderOutputView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice_Impl: Sized {
    fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(&self, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, rateconversionindex: u32) -> ::windows::core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> ::windows::core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID) -> ::windows::core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(&self, presource: &::core::option::Option<ID3D11Resource>, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoDecoderOutputView>;
    fn CreateVideoProcessorInputView(&self, presource: &::core::option::Option<ID3D11Resource>, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoProcessorInputView>;
    fn CreateVideoProcessorOutputView(&self, presource: &::core::option::Option<ID3D11Resource>, penum: &::core::option::Option<ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) -> ::windows::core::Result<ID3D11VideoProcessorOutputView>;
    fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(&self) -> u32;
    fn GetVideoDecoderProfile(&self, index: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> ::windows::core::Result<u32>;
    fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32) -> ::windows::core::Result<D3D11_VIDEO_DECODER_CONFIG>;
    fn GetContentProtectionCaps(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID) -> ::windows::core::Result<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>;
    fn CheckCryptoKeyExchange(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const ::windows::core::GUID, pdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>() -> ID3D11VideoDevice_Vtbl {
        unsafe extern "system" fn CreateVideoDecoder<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoDecoder(::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdecoder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: ::windows::core::RawPtr, rateconversionindex: u32, ppvideoprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoProcessor(::core::mem::transmute(&penum), ::core::mem::transmute_copy(&rateconversionindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvideoprocessor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAuthenticatedChannel(::core::mem::transmute_copy(&channeltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppauthenticatedchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, ppcryptosession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCryptoSession(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcryptosession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoDecoderOutputView(::core::mem::transmute(&presource), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvdovview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoProcessorInputView(::core::mem::transmute(&presource), ::core::mem::transmute(&penum), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvpiview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoProcessorOutputView(::core::mem::transmute(&presource), ::core::mem::transmute(&penum), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvpoview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVideoProcessorEnumerator(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVideoDecoderProfileCount()
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdecoderprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoDecoderProfile(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdecoderprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckVideoDecoderFormat(::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *psupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoDecoderConfigCount(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoDecoderConfig(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentProtectionCaps(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32, pkeyexchangetype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckCryptoKeyExchange(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pkeyexchangetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&guid), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateVideoDecoder: CreateVideoDecoder::<Identity, Impl, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, Impl, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, Impl, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, Impl, OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Identity, Impl, OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Identity, Impl, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, Impl, OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice1_Impl: Sized + ID3D11VideoDevice_Impl {
    fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetVideoDecoderCaps(&self, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> ::windows::core::Result<D3D11_VIDEO_SAMPLE_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>() -> ID3D11VideoDevice1_Vtbl {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCryptoSessionPrivateDataSize(::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype), ::core::mem::transmute_copy(&pprivateinputsize), ::core::mem::transmute_copy(&pprivateoutputsize)).into()
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID, pdecodercaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoDecoderCaps(::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&samplewidth), ::core::mem::transmute_copy(&sampleheight), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&bitrate), ::core::mem::transmute_copy(&pcryptotype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdecodercaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckVideoDecoderDownsampling(::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&psupported), ::core::mem::transmute_copy(&prealtimehint)).into()
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecommendVideoDecoderDownsampleParameters(::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate)) {
                ::core::result::Result::Ok(ok__) => {
                    *precommendedoutputdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCryptoSessionPrivateDataSize: GetCryptoSessionPrivateDataSize::<Identity, Impl, OFFSET>,
            GetVideoDecoderCaps: GetVideoDecoderCaps::<Identity, Impl, OFFSET>,
            CheckVideoDecoderDownsampling: CheckVideoDecoderDownsampling::<Identity, Impl, OFFSET>,
            RecommendVideoDecoderDownsampleParameters: RecommendVideoDecoderDownsampleParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice1 as ::windows::core::Interface>::IID || iid == &<ID3D11VideoDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice2_Impl: Sized + ID3D11VideoDevice_Impl + ID3D11VideoDevice1_Impl {
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
    fn NegotiateCryptoSessionKeyExchangeMT(&self, pcryptosession: &::core::option::Option<ID3D11CryptoSession>, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>() -> ID3D11VideoDevice2_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NegotiateCryptoSessionKeyExchangeMT(::core::mem::transmute(&pcryptosession), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base: ID3D11VideoDevice1_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT: NegotiateCryptoSessionKeyExchangeMT::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice2 as ::windows::core::Interface>::IID || iid == &<ID3D11VideoDevice as ::windows::core::Interface>::IID || iid == &<ID3D11VideoDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessor_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>() -> ID3D11VideoProcessor_Vtbl {
        unsafe extern "system" fn GetContentDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContentDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetRateConversionCaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRateConversionCaps(::core::mem::transmute_copy(&pcaps))
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContentDesc: GetContentDesc::<Identity, Impl, OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(&self) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>;
    fn CheckVideoProcessorFormat(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32>;
    fn GetVideoProcessorCaps(&self) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CAPS>;
    fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>;
    fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>;
    fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumerator_Vtbl {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoProcessorContentDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckVideoProcessorFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoProcessorCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoProcessorRateConversionCaps(::core::mem::transmute_copy(&typeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoProcessorCustomRate(::core::mem::transmute_copy(&typeindex), ::core::mem::transmute_copy(&customrateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *prate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVideoProcessorFilterRange(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *prange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Identity, Impl, OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Identity, Impl, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Identity, Impl, OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator1_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11VideoProcessorEnumerator_Impl {
    fn CheckVideoProcessorFormatConversion(&self, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumerator1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumerator1_Vtbl {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckVideoProcessorFormatConversion(::core::mem::transmute_copy(&inputformat), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&outputcolorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *psupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ID3D11VideoProcessorEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckVideoProcessorFormatConversion: CheckVideoProcessorFormatConversion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator1 as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11VideoProcessorEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorInputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorInputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorInputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorInputView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorOutputView_Impl: Sized + ID3D11DeviceChild_Impl + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC);
}
impl ID3D11VideoProcessorOutputView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        Self { base: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorOutputView as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID || iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11View_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D11Resource>);
}
impl ID3D11View_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11View_Impl, const OFFSET: isize>() -> ID3D11View_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11View_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&ppresource))
        }
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11View as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
pub trait ID3DDeviceContextState_Impl: Sized + ID3D11DeviceChild_Impl {}
impl ID3DDeviceContextState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDeviceContextState_Impl, const OFFSET: isize>() -> ID3DDeviceContextState_Vtbl {
        Self { base: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDeviceContextState as ::windows::core::Interface>::IID || iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3DUserDefinedAnnotation_Impl: Sized {
    fn BeginEvent(&self, name: &::windows::core::PCWSTR) -> i32;
    fn EndEvent(&self) -> i32;
    fn SetMarker(&self, name: &::windows::core::PCWSTR);
    fn GetStatus(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3DUserDefinedAnnotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>() -> ID3DUserDefinedAnnotation_Vtbl {
        unsafe extern "system" fn BeginEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEvent(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndEvent()
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMarker(::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DUserDefinedAnnotation as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11FFT_Impl: Sized {
    fn SetForwardScale(&self, forwardscale: f32) -> ::windows::core::Result<()>;
    fn GetForwardScale(&self) -> f32;
    fn SetInverseScale(&self, inversescale: f32) -> ::windows::core::Result<()>;
    fn GetInverseScale(&self) -> f32;
    fn AttachBuffersAndPrecompute(&self, numtempbuffers: u32, pptempbuffers: *const ::core::option::Option<ID3D11UnorderedAccessView>, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn ForwardTransform(&self, pinputbuffer: &::core::option::Option<ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn InverseTransform(&self, pinputbuffer: &::core::option::Option<ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11FFT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>() -> ID3DX11FFT_Vtbl {
        unsafe extern "system" fn SetForwardScale<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardscale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForwardScale(::core::mem::transmute_copy(&forwardscale)).into()
        }
        unsafe extern "system" fn GetForwardScale<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForwardScale()
        }
        unsafe extern "system" fn SetInverseScale<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inversescale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInverseScale(::core::mem::transmute_copy(&inversescale)).into()
        }
        unsafe extern "system" fn GetInverseScale<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInverseScale()
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const ::windows::core::RawPtr, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachBuffersAndPrecompute(::core::mem::transmute_copy(&numtempbuffers), ::core::mem::transmute_copy(&pptempbuffers), ::core::mem::transmute_copy(&numprecomputebuffers), ::core::mem::transmute_copy(&ppprecomputebuffersizes)).into()
        }
        unsafe extern "system" fn ForwardTransform<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ForwardTransform(::core::mem::transmute(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        unsafe extern "system" fn InverseTransform<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InverseTransform(::core::mem::transmute(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetForwardScale: SetForwardScale::<Identity, Impl, OFFSET>,
            GetForwardScale: GetForwardScale::<Identity, Impl, OFFSET>,
            SetInverseScale: SetInverseScale::<Identity, Impl, OFFSET>,
            GetInverseScale: GetInverseScale::<Identity, Impl, OFFSET>,
            AttachBuffersAndPrecompute: AttachBuffersAndPrecompute::<Identity, Impl, OFFSET>,
            ForwardTransform: ForwardTransform::<Identity, Impl, OFFSET>,
            InverseTransform: InverseTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11FFT as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11Scan_Impl: Sized {
    fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::Result<()>;
    fn Scan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
    fn Multiscan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11Scan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11Scan_Impl, const OFFSET: isize>() -> ID3DX11Scan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScanDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn Scan<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute(&psrc), ::core::mem::transmute(&pdst)).into()
        }
        unsafe extern "system" fn Multiscan<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Multiscan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute_copy(&elementscanpitch), ::core::mem::transmute_copy(&scancount), ::core::mem::transmute(&psrc), ::core::mem::transmute(&pdst)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            Scan: Scan::<Identity, Impl, OFFSET>,
            Multiscan: Multiscan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11Scan as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11SegmentedScan_Impl: Sized {
    fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::Result<()>;
    fn SegScan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: &::core::option::Option<ID3D11UnorderedAccessView>, psrcelementflags: &::core::option::Option<ID3D11UnorderedAccessView>, pdst: &::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows::core::Result<()>;
}
impl ID3DX11SegmentedScan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>() -> ID3DX11SegmentedScan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScanDirection(::core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn SegScan<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, psrcelementflags: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SegScan(::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute(&psrc), ::core::mem::transmute(&psrcelementflags), ::core::mem::transmute(&pdst)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            SegScan: SegScan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11SegmentedScan as ::windows::core::Interface>::IID
    }
}

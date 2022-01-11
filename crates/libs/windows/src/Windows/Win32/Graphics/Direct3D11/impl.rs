pub trait ID3D11AsynchronousImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDataSize();
}
impl ID3D11AsynchronousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AsynchronousImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11AsynchronousVtbl {
        unsafe extern "system" fn GetDataSize<Impl: ID3D11AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Asynchronous as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11AuthenticatedChannelImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetChannelHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11AuthenticatedChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11AuthenticatedChannelVtbl {
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelHandle<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetCertificateSize::<Impl, IMPL_OFFSET>, GetCertificate::<Impl, IMPL_OFFSET>, GetChannelHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11AuthenticatedChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11BlendStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11BlendStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState1Impl: Sized + ID3D11BlendStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11BlendState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11BlendState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11BlendState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11BlendState1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11BufferImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11BufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11BufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Buffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassInstanceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassLinkage();
    fn GetDesc();
    fn GetInstanceName();
    fn GetTypeName();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ClassInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ClassInstanceVtbl {
        unsafe extern "system" fn GetClassLinkage<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceName<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstancename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeName<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetClassLinkage::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetInstanceName::<Impl, IMPL_OFFSET>, GetTypeName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassLinkageImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassInstance();
    fn CreateClassInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ClassLinkageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ClassLinkageVtbl {
        unsafe extern "system" fn GetClassInstance<Impl: ID3D11ClassLinkageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassinstancename: super::super::Foundation::PSTR, instanceindex: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassInstance<Impl: ID3D11ClassLinkageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclasstypename: super::super::Foundation::PSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetClassInstance::<Impl, IMPL_OFFSET>, CreateClassInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ClassLinkage as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11CommandListImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContextFlags();
}
impl ID3D11CommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CommandListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11CommandListVtbl {
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11CommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetContextFlags::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CommandList as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ComputeShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11ComputeShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ComputeShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ComputeShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11CounterImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11CounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CounterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11CounterVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11CounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Counter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11CryptoSessionImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCryptoType();
    fn GetDecoderProfile();
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetCryptoSessionHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11CryptoSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11CryptoSessionVtbl {
        unsafe extern "system" fn GetCryptoType<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDecoderProfile<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetCryptoType::<Impl, IMPL_OFFSET>,
            GetDecoderProfile::<Impl, IMPL_OFFSET>,
            GetCertificateSize::<Impl, IMPL_OFFSET>,
            GetCertificate::<Impl, IMPL_OFFSET>,
            GetCryptoSessionHandle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11CryptoSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D11DebugImpl: Sized {
    fn SetFeatureMask();
    fn GetFeatureMask();
    fn SetPresentPerRenderOpDelay();
    fn GetPresentPerRenderOpDelay();
    fn SetSwapChain();
    fn GetSwapChain();
    fn ValidateContext();
    fn ReportLiveDeviceObjects();
    fn ValidateContextForDispatch();
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D11DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DebugVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateContext<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateContextForDispatch<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetFeatureMask::<Impl, IMPL_OFFSET>,
            GetFeatureMask::<Impl, IMPL_OFFSET>,
            SetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            GetPresentPerRenderOpDelay::<Impl, IMPL_OFFSET>,
            SetSwapChain::<Impl, IMPL_OFFSET>,
            GetSwapChain::<Impl, IMPL_OFFSET>,
            ValidateContext::<Impl, IMPL_OFFSET>,
            ReportLiveDeviceObjects::<Impl, IMPL_OFFSET>,
            ValidateContextForDispatch::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Debug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11DepthStencilStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11DepthStencilStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DepthStencilStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11DepthStencilViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11DepthStencilViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DepthStencilViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DepthStencilView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceImpl: Sized {
    fn CreateBuffer();
    fn CreateTexture1D();
    fn CreateTexture2D();
    fn CreateTexture3D();
    fn CreateShaderResourceView();
    fn CreateUnorderedAccessView();
    fn CreateRenderTargetView();
    fn CreateDepthStencilView();
    fn CreateInputLayout();
    fn CreateVertexShader();
    fn CreateGeometryShader();
    fn CreateGeometryShaderWithStreamOutput();
    fn CreatePixelShader();
    fn CreateHullShader();
    fn CreateDomainShader();
    fn CreateComputeShader();
    fn CreateClassLinkage();
    fn CreateBlendState();
    fn CreateDepthStencilState();
    fn CreateRasterizerState();
    fn CreateSamplerState();
    fn CreateQuery();
    fn CreatePredicate();
    fn CreateCounter();
    fn CreateDeferredContext();
    fn OpenSharedResource();
    fn CheckFormatSupport();
    fn CheckMultisampleQualityLevels();
    fn CheckCounterInfo();
    fn CheckCounter();
    fn CheckFeatureSupport();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn GetFeatureLevel();
    fn GetCreationFlags();
    fn GetDeviceRemovedReason();
    fn GetImmediateContext();
    fn SetExceptionMode();
    fn GetExceptionMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceVtbl {
        unsafe extern "system" fn CreateBuffer<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateHullShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pphullshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDomainShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppdomainshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateComputeShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppcomputeshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassLinkage<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeferredContext<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImmediateContext<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device1Impl: Sized + ID3D11DeviceImpl {
    fn GetImmediateContext1();
    fn CreateDeferredContext1();
    fn CreateBlendState1();
    fn CreateRasterizerState1();
    fn CreateDeviceContextState();
    fn OpenSharedResource1();
    fn OpenSharedResourceByName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device1Vtbl {
        unsafe extern "system" fn GetImmediateContext1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeferredContext1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRasterizerState1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeviceContextState<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenSharedResource1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenSharedResourceByName<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device2Impl: Sized + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn GetImmediateContext2();
    fn CreateDeferredContext2();
    fn GetResourceTiling();
    fn CheckMultisampleQualityLevels1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device2Vtbl {
        unsafe extern "system" fn GetImmediateContext2<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeferredContext2<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceTiling<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
            GetImmediateContext2::<Impl, IMPL_OFFSET>,
            CreateDeferredContext2::<Impl, IMPL_OFFSET>,
            GetResourceTiling::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device3Impl: Sized + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn CreateTexture2D1();
    fn CreateTexture3D1();
    fn CreateRasterizerState2();
    fn CreateShaderResourceView1();
    fn CreateUnorderedAccessView1();
    fn CreateRenderTargetView1();
    fn CreateQuery1();
    fn GetImmediateContext3();
    fn CreateDeferredContext3();
    fn WriteToSubresource();
    fn ReadFromSubresource();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device3Vtbl {
        unsafe extern "system" fn CreateTexture2D1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTexture3D1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRasterizerState2<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRenderTargetView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuery1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImmediateContext3<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDeferredContext3<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteToSubresource<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadFromSubresource<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
            GetImmediateContext2::<Impl, IMPL_OFFSET>,
            CreateDeferredContext2::<Impl, IMPL_OFFSET>,
            GetResourceTiling::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels1::<Impl, IMPL_OFFSET>,
            CreateTexture2D1::<Impl, IMPL_OFFSET>,
            CreateTexture3D1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState2::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView1::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView1::<Impl, IMPL_OFFSET>,
            CreateQuery1::<Impl, IMPL_OFFSET>,
            GetImmediateContext3::<Impl, IMPL_OFFSET>,
            CreateDeferredContext3::<Impl, IMPL_OFFSET>,
            WriteToSubresource::<Impl, IMPL_OFFSET>,
            ReadFromSubresource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device4Impl: Sized + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn RegisterDeviceRemovedEvent();
    fn UnregisterDeviceRemoved();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device4Vtbl {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Impl: ID3D11Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Impl: ID3D11Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
            GetImmediateContext2::<Impl, IMPL_OFFSET>,
            CreateDeferredContext2::<Impl, IMPL_OFFSET>,
            GetResourceTiling::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels1::<Impl, IMPL_OFFSET>,
            CreateTexture2D1::<Impl, IMPL_OFFSET>,
            CreateTexture3D1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState2::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView1::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView1::<Impl, IMPL_OFFSET>,
            CreateQuery1::<Impl, IMPL_OFFSET>,
            GetImmediateContext3::<Impl, IMPL_OFFSET>,
            CreateDeferredContext3::<Impl, IMPL_OFFSET>,
            WriteToSubresource::<Impl, IMPL_OFFSET>,
            ReadFromSubresource::<Impl, IMPL_OFFSET>,
            RegisterDeviceRemovedEvent::<Impl, IMPL_OFFSET>,
            UnregisterDeviceRemoved::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device5Impl: Sized + ID3D11Device4Impl + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn OpenSharedFence();
    fn CreateFence();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Device5Vtbl {
        unsafe extern "system" fn OpenSharedFence<Impl: ID3D11Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFence<Impl: ID3D11Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateBuffer::<Impl, IMPL_OFFSET>,
            CreateTexture1D::<Impl, IMPL_OFFSET>,
            CreateTexture2D::<Impl, IMPL_OFFSET>,
            CreateTexture3D::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView::<Impl, IMPL_OFFSET>,
            CreateDepthStencilView::<Impl, IMPL_OFFSET>,
            CreateInputLayout::<Impl, IMPL_OFFSET>,
            CreateVertexShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShader::<Impl, IMPL_OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, IMPL_OFFSET>,
            CreatePixelShader::<Impl, IMPL_OFFSET>,
            CreateHullShader::<Impl, IMPL_OFFSET>,
            CreateDomainShader::<Impl, IMPL_OFFSET>,
            CreateComputeShader::<Impl, IMPL_OFFSET>,
            CreateClassLinkage::<Impl, IMPL_OFFSET>,
            CreateBlendState::<Impl, IMPL_OFFSET>,
            CreateDepthStencilState::<Impl, IMPL_OFFSET>,
            CreateRasterizerState::<Impl, IMPL_OFFSET>,
            CreateSamplerState::<Impl, IMPL_OFFSET>,
            CreateQuery::<Impl, IMPL_OFFSET>,
            CreatePredicate::<Impl, IMPL_OFFSET>,
            CreateCounter::<Impl, IMPL_OFFSET>,
            CreateDeferredContext::<Impl, IMPL_OFFSET>,
            OpenSharedResource::<Impl, IMPL_OFFSET>,
            CheckFormatSupport::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels::<Impl, IMPL_OFFSET>,
            CheckCounterInfo::<Impl, IMPL_OFFSET>,
            CheckCounter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetFeatureLevel::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            GetDeviceRemovedReason::<Impl, IMPL_OFFSET>,
            GetImmediateContext::<Impl, IMPL_OFFSET>,
            SetExceptionMode::<Impl, IMPL_OFFSET>,
            GetExceptionMode::<Impl, IMPL_OFFSET>,
            GetImmediateContext1::<Impl, IMPL_OFFSET>,
            CreateDeferredContext1::<Impl, IMPL_OFFSET>,
            CreateBlendState1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState1::<Impl, IMPL_OFFSET>,
            CreateDeviceContextState::<Impl, IMPL_OFFSET>,
            OpenSharedResource1::<Impl, IMPL_OFFSET>,
            OpenSharedResourceByName::<Impl, IMPL_OFFSET>,
            GetImmediateContext2::<Impl, IMPL_OFFSET>,
            CreateDeferredContext2::<Impl, IMPL_OFFSET>,
            GetResourceTiling::<Impl, IMPL_OFFSET>,
            CheckMultisampleQualityLevels1::<Impl, IMPL_OFFSET>,
            CreateTexture2D1::<Impl, IMPL_OFFSET>,
            CreateTexture3D1::<Impl, IMPL_OFFSET>,
            CreateRasterizerState2::<Impl, IMPL_OFFSET>,
            CreateShaderResourceView1::<Impl, IMPL_OFFSET>,
            CreateUnorderedAccessView1::<Impl, IMPL_OFFSET>,
            CreateRenderTargetView1::<Impl, IMPL_OFFSET>,
            CreateQuery1::<Impl, IMPL_OFFSET>,
            GetImmediateContext3::<Impl, IMPL_OFFSET>,
            CreateDeferredContext3::<Impl, IMPL_OFFSET>,
            WriteToSubresource::<Impl, IMPL_OFFSET>,
            ReadFromSubresource::<Impl, IMPL_OFFSET>,
            RegisterDeviceRemovedEvent::<Impl, IMPL_OFFSET>,
            UnregisterDeviceRemoved::<Impl, IMPL_OFFSET>,
            OpenSharedFence::<Impl, IMPL_OFFSET>,
            CreateFence::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Device5 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
impl ID3D11DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChildImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContextImpl: Sized + ID3D11DeviceChildImpl {
    fn VSSetConstantBuffers();
    fn PSSetShaderResources();
    fn PSSetShader();
    fn PSSetSamplers();
    fn VSSetShader();
    fn DrawIndexed();
    fn Draw();
    fn Map();
    fn Unmap();
    fn PSSetConstantBuffers();
    fn IASetInputLayout();
    fn IASetVertexBuffers();
    fn IASetIndexBuffer();
    fn DrawIndexedInstanced();
    fn DrawInstanced();
    fn GSSetConstantBuffers();
    fn GSSetShader();
    fn IASetPrimitiveTopology();
    fn VSSetShaderResources();
    fn VSSetSamplers();
    fn Begin();
    fn End();
    fn GetData();
    fn SetPredication();
    fn GSSetShaderResources();
    fn GSSetSamplers();
    fn OMSetRenderTargets();
    fn OMSetRenderTargetsAndUnorderedAccessViews();
    fn OMSetBlendState();
    fn OMSetDepthStencilState();
    fn SOSetTargets();
    fn DrawAuto();
    fn DrawIndexedInstancedIndirect();
    fn DrawInstancedIndirect();
    fn Dispatch();
    fn DispatchIndirect();
    fn RSSetState();
    fn RSSetViewports();
    fn RSSetScissorRects();
    fn CopySubresourceRegion();
    fn CopyResource();
    fn UpdateSubresource();
    fn CopyStructureCount();
    fn ClearRenderTargetView();
    fn ClearUnorderedAccessViewUint();
    fn ClearUnorderedAccessViewFloat();
    fn ClearDepthStencilView();
    fn GenerateMips();
    fn SetResourceMinLOD();
    fn GetResourceMinLOD();
    fn ResolveSubresource();
    fn ExecuteCommandList();
    fn HSSetShaderResources();
    fn HSSetShader();
    fn HSSetSamplers();
    fn HSSetConstantBuffers();
    fn DSSetShaderResources();
    fn DSSetShader();
    fn DSSetSamplers();
    fn DSSetConstantBuffers();
    fn CSSetShaderResources();
    fn CSSetUnorderedAccessViews();
    fn CSSetShader();
    fn CSSetSamplers();
    fn CSSetConstantBuffers();
    fn VSGetConstantBuffers();
    fn PSGetShaderResources();
    fn PSGetShader();
    fn PSGetSamplers();
    fn VSGetShader();
    fn PSGetConstantBuffers();
    fn IAGetInputLayout();
    fn IAGetVertexBuffers();
    fn IAGetIndexBuffer();
    fn GSGetConstantBuffers();
    fn GSGetShader();
    fn IAGetPrimitiveTopology();
    fn VSGetShaderResources();
    fn VSGetSamplers();
    fn GetPredication();
    fn GSGetShaderResources();
    fn GSGetSamplers();
    fn OMGetRenderTargets();
    fn OMGetRenderTargetsAndUnorderedAccessViews();
    fn OMGetBlendState();
    fn OMGetDepthStencilState();
    fn SOGetTargets();
    fn RSGetState();
    fn RSGetViewports();
    fn RSGetScissorRects();
    fn HSGetShaderResources();
    fn HSGetShader();
    fn HSGetSamplers();
    fn HSGetConstantBuffers();
    fn DSGetShaderResources();
    fn DSGetShader();
    fn DSGetSamplers();
    fn DSGetConstantBuffers();
    fn CSGetShaderResources();
    fn CSGetUnorderedAccessViews();
    fn CSGetShader();
    fn CSGetSamplers();
    fn CSGetConstantBuffers();
    fn ClearState();
    fn Flush();
    fn GetType();
    fn GetContextFlags();
    fn FinishCommandList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContextVtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Draw<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Map<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawInstancedIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dispatch<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DispatchIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyStructureCount<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstalignedbyteoffset: u32, psrcview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResourceMinLOD<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, minlod: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceMinLOD<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteCommandList<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr, restorecontextstate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phullshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomputeshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphullshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomainshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomputeshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishCommandList<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            End::<Impl, IMPL_OFFSET>,
            GetData::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext1Impl: Sized + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn CopySubresourceRegion1();
    fn UpdateSubresource1();
    fn DiscardResource();
    fn DiscardView();
    fn VSSetConstantBuffers1();
    fn HSSetConstantBuffers1();
    fn DSSetConstantBuffers1();
    fn GSSetConstantBuffers1();
    fn PSSetConstantBuffers1();
    fn CSSetConstantBuffers1();
    fn VSGetConstantBuffers1();
    fn HSGetConstantBuffers1();
    fn DSGetConstantBuffers1();
    fn GSGetConstantBuffers1();
    fn PSGetConstantBuffers1();
    fn CSGetConstantBuffers1();
    fn SwapDeviceContextState();
    fn ClearView();
    fn DiscardView1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext1Vtbl {
        unsafe extern "system" fn CopySubresourceRegion1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSubresource1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardView<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwapDeviceContextState<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: ::windows::core::RawPtr, pppreviousstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearView<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: ::windows::core::RawPtr, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardView1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr, prects: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            End::<Impl, IMPL_OFFSET>,
            GetData::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion1::<Impl, IMPL_OFFSET>,
            UpdateSubresource1::<Impl, IMPL_OFFSET>,
            DiscardResource::<Impl, IMPL_OFFSET>,
            DiscardView::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            SwapDeviceContextState::<Impl, IMPL_OFFSET>,
            ClearView::<Impl, IMPL_OFFSET>,
            DiscardView1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext2Impl: Sized + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn UpdateTileMappings();
    fn CopyTileMappings();
    fn CopyTiles();
    fn UpdateTiles();
    fn ResizeTilePool();
    fn TiledResourceBarrier();
    fn IsAnnotationEnabled();
    fn SetMarkerInt();
    fn BeginEventInt();
    fn EndEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext2Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: ::windows::core::RawPtr, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTileMappings<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: ::windows::core::RawPtr, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTiles<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTiles<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeTilePool<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilepool: ::windows::core::RawPtr, newsizeinbytes: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TiledResourceBarrier<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: ::windows::core::RawPtr, ptiledresourceorviewaccessafterbarrier: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAnnotationEnabled<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarkerInt<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEventInt<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            End::<Impl, IMPL_OFFSET>,
            GetData::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion1::<Impl, IMPL_OFFSET>,
            UpdateSubresource1::<Impl, IMPL_OFFSET>,
            DiscardResource::<Impl, IMPL_OFFSET>,
            DiscardView::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            SwapDeviceContextState::<Impl, IMPL_OFFSET>,
            ClearView::<Impl, IMPL_OFFSET>,
            DiscardView1::<Impl, IMPL_OFFSET>,
            UpdateTileMappings::<Impl, IMPL_OFFSET>,
            CopyTileMappings::<Impl, IMPL_OFFSET>,
            CopyTiles::<Impl, IMPL_OFFSET>,
            UpdateTiles::<Impl, IMPL_OFFSET>,
            ResizeTilePool::<Impl, IMPL_OFFSET>,
            TiledResourceBarrier::<Impl, IMPL_OFFSET>,
            IsAnnotationEnabled::<Impl, IMPL_OFFSET>,
            SetMarkerInt::<Impl, IMPL_OFFSET>,
            BeginEventInt::<Impl, IMPL_OFFSET>,
            EndEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext3Impl: Sized + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Flush1();
    fn SetHardwareProtectionState();
    fn GetHardwareProtectionState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext3Vtbl {
        unsafe extern "system" fn Flush1<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHardwareProtectionState<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHardwareProtectionState<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            End::<Impl, IMPL_OFFSET>,
            GetData::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion1::<Impl, IMPL_OFFSET>,
            UpdateSubresource1::<Impl, IMPL_OFFSET>,
            DiscardResource::<Impl, IMPL_OFFSET>,
            DiscardView::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            SwapDeviceContextState::<Impl, IMPL_OFFSET>,
            ClearView::<Impl, IMPL_OFFSET>,
            DiscardView1::<Impl, IMPL_OFFSET>,
            UpdateTileMappings::<Impl, IMPL_OFFSET>,
            CopyTileMappings::<Impl, IMPL_OFFSET>,
            CopyTiles::<Impl, IMPL_OFFSET>,
            UpdateTiles::<Impl, IMPL_OFFSET>,
            ResizeTilePool::<Impl, IMPL_OFFSET>,
            TiledResourceBarrier::<Impl, IMPL_OFFSET>,
            IsAnnotationEnabled::<Impl, IMPL_OFFSET>,
            SetMarkerInt::<Impl, IMPL_OFFSET>,
            BeginEventInt::<Impl, IMPL_OFFSET>,
            EndEvent::<Impl, IMPL_OFFSET>,
            Flush1::<Impl, IMPL_OFFSET>,
            SetHardwareProtectionState::<Impl, IMPL_OFFSET>,
            GetHardwareProtectionState::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext4Impl: Sized + ID3D11DeviceContext3Impl + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Signal();
    fn Wait();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DeviceContext4Vtbl {
        unsafe extern "system" fn Signal<Impl: ID3D11DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Wait<Impl: ID3D11DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSSetShaderResources::<Impl, IMPL_OFFSET>,
            PSSetShader::<Impl, IMPL_OFFSET>,
            PSSetSamplers::<Impl, IMPL_OFFSET>,
            VSSetShader::<Impl, IMPL_OFFSET>,
            DrawIndexed::<Impl, IMPL_OFFSET>,
            Draw::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            IASetInputLayout::<Impl, IMPL_OFFSET>,
            IASetVertexBuffers::<Impl, IMPL_OFFSET>,
            IASetIndexBuffer::<Impl, IMPL_OFFSET>,
            DrawIndexedInstanced::<Impl, IMPL_OFFSET>,
            DrawInstanced::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSSetShader::<Impl, IMPL_OFFSET>,
            IASetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSSetShaderResources::<Impl, IMPL_OFFSET>,
            VSSetSamplers::<Impl, IMPL_OFFSET>,
            Begin::<Impl, IMPL_OFFSET>,
            End::<Impl, IMPL_OFFSET>,
            GetData::<Impl, IMPL_OFFSET>,
            SetPredication::<Impl, IMPL_OFFSET>,
            GSSetShaderResources::<Impl, IMPL_OFFSET>,
            GSSetSamplers::<Impl, IMPL_OFFSET>,
            OMSetRenderTargets::<Impl, IMPL_OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMSetBlendState::<Impl, IMPL_OFFSET>,
            OMSetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOSetTargets::<Impl, IMPL_OFFSET>,
            DrawAuto::<Impl, IMPL_OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, IMPL_OFFSET>,
            DrawInstancedIndirect::<Impl, IMPL_OFFSET>,
            Dispatch::<Impl, IMPL_OFFSET>,
            DispatchIndirect::<Impl, IMPL_OFFSET>,
            RSSetState::<Impl, IMPL_OFFSET>,
            RSSetViewports::<Impl, IMPL_OFFSET>,
            RSSetScissorRects::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion::<Impl, IMPL_OFFSET>,
            CopyResource::<Impl, IMPL_OFFSET>,
            UpdateSubresource::<Impl, IMPL_OFFSET>,
            CopyStructureCount::<Impl, IMPL_OFFSET>,
            ClearRenderTargetView::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, IMPL_OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, IMPL_OFFSET>,
            ClearDepthStencilView::<Impl, IMPL_OFFSET>,
            GenerateMips::<Impl, IMPL_OFFSET>,
            SetResourceMinLOD::<Impl, IMPL_OFFSET>,
            GetResourceMinLOD::<Impl, IMPL_OFFSET>,
            ResolveSubresource::<Impl, IMPL_OFFSET>,
            ExecuteCommandList::<Impl, IMPL_OFFSET>,
            HSSetShaderResources::<Impl, IMPL_OFFSET>,
            HSSetShader::<Impl, IMPL_OFFSET>,
            HSSetSamplers::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSSetShaderResources::<Impl, IMPL_OFFSET>,
            DSSetShader::<Impl, IMPL_OFFSET>,
            DSSetSamplers::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSSetShaderResources::<Impl, IMPL_OFFSET>,
            CSSetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSSetShader::<Impl, IMPL_OFFSET>,
            CSSetSamplers::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            PSGetShaderResources::<Impl, IMPL_OFFSET>,
            PSGetShader::<Impl, IMPL_OFFSET>,
            PSGetSamplers::<Impl, IMPL_OFFSET>,
            VSGetShader::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            IAGetInputLayout::<Impl, IMPL_OFFSET>,
            IAGetVertexBuffers::<Impl, IMPL_OFFSET>,
            IAGetIndexBuffer::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            GSGetShader::<Impl, IMPL_OFFSET>,
            IAGetPrimitiveTopology::<Impl, IMPL_OFFSET>,
            VSGetShaderResources::<Impl, IMPL_OFFSET>,
            VSGetSamplers::<Impl, IMPL_OFFSET>,
            GetPredication::<Impl, IMPL_OFFSET>,
            GSGetShaderResources::<Impl, IMPL_OFFSET>,
            GSGetSamplers::<Impl, IMPL_OFFSET>,
            OMGetRenderTargets::<Impl, IMPL_OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            OMGetBlendState::<Impl, IMPL_OFFSET>,
            OMGetDepthStencilState::<Impl, IMPL_OFFSET>,
            SOGetTargets::<Impl, IMPL_OFFSET>,
            RSGetState::<Impl, IMPL_OFFSET>,
            RSGetViewports::<Impl, IMPL_OFFSET>,
            RSGetScissorRects::<Impl, IMPL_OFFSET>,
            HSGetShaderResources::<Impl, IMPL_OFFSET>,
            HSGetShader::<Impl, IMPL_OFFSET>,
            HSGetSamplers::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            DSGetShaderResources::<Impl, IMPL_OFFSET>,
            DSGetShader::<Impl, IMPL_OFFSET>,
            DSGetSamplers::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            CSGetShaderResources::<Impl, IMPL_OFFSET>,
            CSGetUnorderedAccessViews::<Impl, IMPL_OFFSET>,
            CSGetShader::<Impl, IMPL_OFFSET>,
            CSGetSamplers::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers::<Impl, IMPL_OFFSET>,
            ClearState::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetContextFlags::<Impl, IMPL_OFFSET>,
            FinishCommandList::<Impl, IMPL_OFFSET>,
            CopySubresourceRegion1::<Impl, IMPL_OFFSET>,
            UpdateSubresource1::<Impl, IMPL_OFFSET>,
            DiscardResource::<Impl, IMPL_OFFSET>,
            DiscardView::<Impl, IMPL_OFFSET>,
            VSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSSetConstantBuffers1::<Impl, IMPL_OFFSET>,
            VSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            HSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            DSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            GSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            PSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            CSGetConstantBuffers1::<Impl, IMPL_OFFSET>,
            SwapDeviceContextState::<Impl, IMPL_OFFSET>,
            ClearView::<Impl, IMPL_OFFSET>,
            DiscardView1::<Impl, IMPL_OFFSET>,
            UpdateTileMappings::<Impl, IMPL_OFFSET>,
            CopyTileMappings::<Impl, IMPL_OFFSET>,
            CopyTiles::<Impl, IMPL_OFFSET>,
            UpdateTiles::<Impl, IMPL_OFFSET>,
            ResizeTilePool::<Impl, IMPL_OFFSET>,
            TiledResourceBarrier::<Impl, IMPL_OFFSET>,
            IsAnnotationEnabled::<Impl, IMPL_OFFSET>,
            SetMarkerInt::<Impl, IMPL_OFFSET>,
            BeginEventInt::<Impl, IMPL_OFFSET>,
            EndEvent::<Impl, IMPL_OFFSET>,
            Flush1::<Impl, IMPL_OFFSET>,
            SetHardwareProtectionState::<Impl, IMPL_OFFSET>,
            GetHardwareProtectionState::<Impl, IMPL_OFFSET>,
            Signal::<Impl, IMPL_OFFSET>,
            Wait::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DeviceContext4 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11DomainShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11DomainShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DomainShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11DomainShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11DomainShader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ID3D11FenceImpl: Sized + ID3D11DeviceChildImpl {
    fn CreateSharedHandle();
    fn GetCompletedValue();
    fn SetEventOnCompletion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ID3D11FenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FenceVtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompletedValue<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventOnCompletion<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, CreateSharedHandle::<Impl, IMPL_OFFSET>, GetCompletedValue::<Impl, IMPL_OFFSET>, SetEventOnCompletion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Fence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionLinkingGraphImpl: Sized {
    fn CreateModuleInstance();
    fn SetInputSignature();
    fn SetOutputSignature();
    fn CallFunction();
    fn PassValue();
    fn PassValueWithSwizzle();
    fn GetLastError();
    fn GenerateHlsl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionLinkingGraphVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraphImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionLinkingGraphVtbl {
        unsafe extern "system" fn CreateModuleInstance<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmoduleinstance: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputSignature<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputSignature<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallFunction<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmoduleinstancenamespace: super::super::Foundation::PSTR, pmodulewithfunctionprototype: ::windows::core::RawPtr, pfunctionname: super::super::Foundation::PSTR, ppcallnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PassValue<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PassValueWithSwizzle<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, psrcswizzle: super::super::Foundation::PSTR, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32, pdstswizzle: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastError<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateHlsl<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateModuleInstance::<Impl, IMPL_OFFSET>, SetInputSignature::<Impl, IMPL_OFFSET>, SetOutputSignature::<Impl, IMPL_OFFSET>, CallFunction::<Impl, IMPL_OFFSET>, PassValue::<Impl, IMPL_OFFSET>, PassValueWithSwizzle::<Impl, IMPL_OFFSET>, GetLastError::<Impl, IMPL_OFFSET>, GenerateHlsl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionLinkingGraph as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionParameterReflectionImpl: Sized {
    fn GetDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionParameterReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionParameterReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionParameterReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionParameterReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionParameterReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetFunctionParameter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11FunctionReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetConstantBufferByIndex::<Impl, IMPL_OFFSET>, GetConstantBufferByName::<Impl, IMPL_OFFSET>, GetResourceBindingDesc::<Impl, IMPL_OFFSET>, GetVariableByName::<Impl, IMPL_OFFSET>, GetResourceBindingDescByName::<Impl, IMPL_OFFSET>, GetFunctionParameter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11FunctionReflection as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11GeometryShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11GeometryShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11GeometryShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11GeometryShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11HullShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11HullShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11HullShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11HullShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11HullShader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11InfoQueueImpl: Sized {
    fn SetMessageCountLimit();
    fn ClearStoredMessages();
    fn GetMessage();
    fn GetNumMessagesAllowedByStorageFilter();
    fn GetNumMessagesDeniedByStorageFilter();
    fn GetNumStoredMessages();
    fn GetNumStoredMessagesAllowedByRetrievalFilter();
    fn GetNumMessagesDiscardedByMessageCountLimit();
    fn GetMessageCountLimit();
    fn AddStorageFilterEntries();
    fn GetStorageFilter();
    fn ClearStorageFilter();
    fn PushEmptyStorageFilter();
    fn PushCopyOfStorageFilter();
    fn PushStorageFilter();
    fn PopStorageFilter();
    fn GetStorageFilterStackSize();
    fn AddRetrievalFilterEntries();
    fn GetRetrievalFilter();
    fn ClearRetrievalFilter();
    fn PushEmptyRetrievalFilter();
    fn PushCopyOfRetrievalFilter();
    fn PushRetrievalFilter();
    fn PopRetrievalFilter();
    fn GetRetrievalFilterStackSize();
    fn AddMessage();
    fn AddApplicationMessage();
    fn SetBreakOnCategory();
    fn SetBreakOnSeverity();
    fn SetBreakOnID();
    fn GetBreakOnCategory();
    fn GetBreakOnSeverity();
    fn GetBreakOnID();
    fn SetMuteDebugOutput();
    fn GetMuteDebugOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11InputLayoutImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11InputLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InputLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11InputLayoutVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11InputLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11LibraryReflectionImpl: Sized {
    fn GetDesc();
    fn GetFunctionByIndex();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11LibraryReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11LibraryReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunctionByIndex<Impl: ID3D11LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetFunctionByIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LibraryReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11LinkerImpl: Sized {
    fn Link();
    fn UseLibrary();
    fn AddClipPlaneFromCBuffer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11LinkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11LinkerVtbl {
        unsafe extern "system" fn Link<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentry: ::windows::core::RawPtr, pentryname: super::super::Foundation::PSTR, ptargetname: super::super::Foundation::PSTR, uflags: u32, ppshaderblob: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseLibrary<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrarymi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Link::<Impl, IMPL_OFFSET>, UseLibrary::<Impl, IMPL_OFFSET>, AddClipPlaneFromCBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Linker as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11LinkingNodeImpl: Sized {}
impl ID3D11LinkingNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkingNodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11LinkingNodeVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11LinkingNode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ModuleImpl: Sized {
    fn CreateInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ModuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ModuleVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ID3D11ModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: super::super::Foundation::PSTR, ppmoduleinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Module as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ModuleInstanceImpl: Sized {
    fn BindConstantBuffer();
    fn BindConstantBufferByName();
    fn BindResource();
    fn BindResourceByName();
    fn BindSampler();
    fn BindSamplerByName();
    fn BindUnorderedAccessView();
    fn BindUnorderedAccessViewByName();
    fn BindResourceAsUnorderedAccessView();
    fn BindResourceAsUnorderedAccessViewByName();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ModuleInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ModuleInstanceVtbl {
        unsafe extern "system" fn BindConstantBuffer<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindConstantBufferByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindResource<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindResourceByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindSampler<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindSamplerByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindUnorderedAccessView<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrvname: super::super::Foundation::PSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            BindConstantBuffer::<Impl, IMPL_OFFSET>,
            BindConstantBufferByName::<Impl, IMPL_OFFSET>,
            BindResource::<Impl, IMPL_OFFSET>,
            BindResourceByName::<Impl, IMPL_OFFSET>,
            BindSampler::<Impl, IMPL_OFFSET>,
            BindSamplerByName::<Impl, IMPL_OFFSET>,
            BindUnorderedAccessView::<Impl, IMPL_OFFSET>,
            BindUnorderedAccessViewByName::<Impl, IMPL_OFFSET>,
            BindResourceAsUnorderedAccessView::<Impl, IMPL_OFFSET>,
            BindResourceAsUnorderedAccessViewByName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ModuleInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11MultithreadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11MultithreadVtbl {
        unsafe extern "system" fn Enter<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Leave<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Enter::<Impl, IMPL_OFFSET>, Leave::<Impl, IMPL_OFFSET>, SetMultithreadProtected::<Impl, IMPL_OFFSET>, GetMultithreadProtected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Multithread as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11PixelShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11PixelShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PixelShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11PixelShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11PixelShader as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11PredicateImpl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {}
impl ID3D11PredicateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PredicateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11PredicateVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Predicate as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11QueryImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11QueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11QueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11QueryVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11QueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11Query1Impl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ID3D11Query1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Query1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Query1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDataSize::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Query1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RasterizerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState1Impl: Sized + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RasterizerState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState2Impl: Sized + ID3D11RasterizerState1Impl + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc2();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11RasterizerState2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RasterizerState2Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: ID3D11RasterizerState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>, GetDesc2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RasterizerState2 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefDefaultTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
impl ID3D11RefDefaultTrackingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefDefaultTrackingOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RefDefaultTrackingOptionsVtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefDefaultTrackingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTrackingOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefDefaultTrackingOptions as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11RefTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
impl ID3D11RefTrackingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefTrackingOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RefTrackingOptionsVtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefTrackingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTrackingOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RefTrackingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RenderTargetViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RenderTargetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView1Impl: Sized + ID3D11RenderTargetViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11RenderTargetView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RenderTargetView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ResourceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
impl ID3D11ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ResourceVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Resource as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11SamplerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11SamplerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SamplerStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11SamplerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11SamplerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SamplerState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetInputParameterDesc();
    fn GetOutputParameterDesc();
    fn GetPatchConstantParameterDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetMovInstructionCount();
    fn GetMovcInstructionCount();
    fn GetConversionInstructionCount();
    fn GetBitwiseInstructionCount();
    fn GetGSInputPrimitive();
    fn IsSampleFrequencyShader();
    fn GetNumInterfaceSlots();
    fn GetMinFeatureLevel();
    fn GetThreadGroupSize();
    fn GetRequiresFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinFeatureLevel<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThreadGroupSize<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequiresFlags<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetConstantBufferByIndex::<Impl, IMPL_OFFSET>,
            GetConstantBufferByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDesc::<Impl, IMPL_OFFSET>,
            GetInputParameterDesc::<Impl, IMPL_OFFSET>,
            GetOutputParameterDesc::<Impl, IMPL_OFFSET>,
            GetPatchConstantParameterDesc::<Impl, IMPL_OFFSET>,
            GetVariableByName::<Impl, IMPL_OFFSET>,
            GetResourceBindingDescByName::<Impl, IMPL_OFFSET>,
            GetMovInstructionCount::<Impl, IMPL_OFFSET>,
            GetMovcInstructionCount::<Impl, IMPL_OFFSET>,
            GetConversionInstructionCount::<Impl, IMPL_OFFSET>,
            GetBitwiseInstructionCount::<Impl, IMPL_OFFSET>,
            GetGSInputPrimitive::<Impl, IMPL_OFFSET>,
            IsSampleFrequencyShader::<Impl, IMPL_OFFSET>,
            GetNumInterfaceSlots::<Impl, IMPL_OFFSET>,
            GetMinFeatureLevel::<Impl, IMPL_OFFSET>,
            GetThreadGroupSize::<Impl, IMPL_OFFSET>,
            GetRequiresFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetVariableByIndex::<Impl, IMPL_OFFSET>, GetVariableByName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionConstantBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflectionTypeImpl: Sized {
    fn GetDesc();
    fn GetMemberTypeByIndex();
    fn GetMemberTypeByName();
    fn GetMemberTypeName();
    fn IsEqual();
    fn GetSubType();
    fn GetBaseClass();
    fn GetNumInterfaces();
    fn GetInterfaceByIndex();
    fn IsOfType();
    fn ImplementsInterface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetMemberTypeByIndex::<Impl, IMPL_OFFSET>, GetMemberTypeByName::<Impl, IMPL_OFFSET>, GetMemberTypeName::<Impl, IMPL_OFFSET>, IsEqual::<Impl, IMPL_OFFSET>, GetSubType::<Impl, IMPL_OFFSET>, GetBaseClass::<Impl, IMPL_OFFSET>, GetNumInterfaces::<Impl, IMPL_OFFSET>, GetInterfaceByIndex::<Impl, IMPL_OFFSET>, IsOfType::<Impl, IMPL_OFFSET>, ImplementsInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
    fn GetBuffer();
    fn GetInterfaceSlot();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(GetDesc::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetBuffer::<Impl, IMPL_OFFSET>, GetInterfaceSlot::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderReflectionVariable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderResourceViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderResourceViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView1Impl: Sized + ID3D11ShaderResourceViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderResourceView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11ShaderResourceView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderTraceImpl: Sized {
    fn TraceReady();
    fn ResetTrace();
    fn GetTraceStats();
    fn PSSelectStamp();
    fn GetInitialRegisterContents();
    fn GetStep();
    fn GetWrittenRegister();
    fn GetReadRegister();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11ShaderTraceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderTraceVtbl {
        unsafe extern "system" fn TraceReady<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptestcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetTrace<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTraceStats<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PSSelectStamp<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stampindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInitialRegisterContents<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStep<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWrittenRegister<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadRegister<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TraceReady::<Impl, IMPL_OFFSET>, ResetTrace::<Impl, IMPL_OFFSET>, GetTraceStats::<Impl, IMPL_OFFSET>, PSSelectStamp::<Impl, IMPL_OFFSET>, GetInitialRegisterContents::<Impl, IMPL_OFFSET>, GetStep::<Impl, IMPL_OFFSET>, GetWrittenRegister::<Impl, IMPL_OFFSET>, GetReadRegister::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTrace as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ShaderTraceFactoryImpl: Sized {
    fn CreateShaderTrace();
}
impl ID3D11ShaderTraceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ShaderTraceFactoryVtbl {
        unsafe extern "system" fn CreateShaderTrace<Impl: ID3D11ShaderTraceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateShaderTrace::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11ShaderTraceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D11SwitchToRefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRefImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11SwitchToRefVtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D11SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D11SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetUseRef::<Impl, IMPL_OFFSET>, GetUseRef::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11SwitchToRef as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture1DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture1DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture1DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture1DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture1D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture2DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D1Impl: Sized + ID3D11Texture2DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture2D1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture2D1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture2D1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture3DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, SetEvictionPriority::<Impl, IMPL_OFFSET>, GetEvictionPriority::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D1Impl: Sized + ID3D11Texture3DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11Texture3D1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture3D1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11Texture3D1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11TracingDeviceImpl: Sized {
    fn SetShaderTrackingOptionsByType();
    fn SetShaderTrackingOptions();
}
impl ID3D11TracingDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11TracingDeviceVtbl {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Impl: ID3D11TracingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Impl: ID3D11TracingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetShaderTrackingOptionsByType::<Impl, IMPL_OFFSET>, SetShaderTrackingOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11TracingDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11UnorderedAccessViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11UnorderedAccessViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView1Impl: Sized + ID3D11UnorderedAccessViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11UnorderedAccessView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11UnorderedAccessView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VertexShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3D11VertexShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VertexShaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VertexShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VertexShader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContextImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDecoderBuffer();
    fn ReleaseDecoderBuffer();
    fn DecoderBeginFrame();
    fn DecoderEndFrame();
    fn SubmitDecoderBuffers();
    fn DecoderExtension();
    fn VideoProcessorSetOutputTargetRect();
    fn VideoProcessorSetOutputBackgroundColor();
    fn VideoProcessorSetOutputColorSpace();
    fn VideoProcessorSetOutputAlphaFillMode();
    fn VideoProcessorSetOutputConstriction();
    fn VideoProcessorSetOutputStereoMode();
    fn VideoProcessorSetOutputExtension();
    fn VideoProcessorGetOutputTargetRect();
    fn VideoProcessorGetOutputBackgroundColor();
    fn VideoProcessorGetOutputColorSpace();
    fn VideoProcessorGetOutputAlphaFillMode();
    fn VideoProcessorGetOutputConstriction();
    fn VideoProcessorGetOutputStereoMode();
    fn VideoProcessorGetOutputExtension();
    fn VideoProcessorSetStreamFrameFormat();
    fn VideoProcessorSetStreamColorSpace();
    fn VideoProcessorSetStreamOutputRate();
    fn VideoProcessorSetStreamSourceRect();
    fn VideoProcessorSetStreamDestRect();
    fn VideoProcessorSetStreamAlpha();
    fn VideoProcessorSetStreamPalette();
    fn VideoProcessorSetStreamPixelAspectRatio();
    fn VideoProcessorSetStreamLumaKey();
    fn VideoProcessorSetStreamStereoFormat();
    fn VideoProcessorSetStreamAutoProcessingMode();
    fn VideoProcessorSetStreamFilter();
    fn VideoProcessorSetStreamExtension();
    fn VideoProcessorGetStreamFrameFormat();
    fn VideoProcessorGetStreamColorSpace();
    fn VideoProcessorGetStreamOutputRate();
    fn VideoProcessorGetStreamSourceRect();
    fn VideoProcessorGetStreamDestRect();
    fn VideoProcessorGetStreamAlpha();
    fn VideoProcessorGetStreamPalette();
    fn VideoProcessorGetStreamPixelAspectRatio();
    fn VideoProcessorGetStreamLumaKey();
    fn VideoProcessorGetStreamStereoFormat();
    fn VideoProcessorGetStreamAutoProcessingMode();
    fn VideoProcessorGetStreamFilter();
    fn VideoProcessorGetStreamExtension();
    fn VideoProcessorBlt();
    fn NegotiateCryptoSessionKeyExchange();
    fn EncryptionBlt();
    fn DecryptionBlt();
    fn StartSessionKeyRefresh();
    fn FinishSessionKeyRefresh();
    fn GetEncryptionBltKey();
    fn NegotiateAuthenticatedChannelKeyExchange();
    fn QueryAuthenticatedChannel();
    fn ConfigureAuthenticatedChannel();
    fn VideoProcessorSetStreamRotation();
    fn VideoProcessorGetStreamRotation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContextVtbl {
        unsafe extern "system" fn GetDecoderBuffer<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecoderBeginFrame<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecoderEndFrame<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecoderExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecryptionBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncryptionBltKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetDecoderBuffer::<Impl, IMPL_OFFSET>,
            ReleaseDecoderBuffer::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame::<Impl, IMPL_OFFSET>,
            DecoderEndFrame::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers::<Impl, IMPL_OFFSET>,
            DecoderExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorBlt::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange::<Impl, IMPL_OFFSET>,
            QueryAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            ConfigureAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamRotation::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamRotation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext1Impl: Sized + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn SubmitDecoderBuffers1();
    fn GetDataForNewHardwareKey();
    fn CheckCryptoSessionStatus();
    fn DecoderEnableDownsampling();
    fn DecoderUpdateDownsampling();
    fn VideoProcessorSetOutputColorSpace1();
    fn VideoProcessorSetOutputShaderUsage();
    fn VideoProcessorGetOutputColorSpace1();
    fn VideoProcessorGetOutputShaderUsage();
    fn VideoProcessorSetStreamColorSpace1();
    fn VideoProcessorSetStreamMirror();
    fn VideoProcessorGetStreamColorSpace1();
    fn VideoProcessorGetStreamMirror();
    fn VideoProcessorGetBehaviorHints();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext1Vtbl {
        unsafe extern "system" fn SubmitDecoderBuffers1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void, pprivateoutputdata: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, shaderusage: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pshaderusage: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetDecoderBuffer::<Impl, IMPL_OFFSET>,
            ReleaseDecoderBuffer::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame::<Impl, IMPL_OFFSET>,
            DecoderEndFrame::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers::<Impl, IMPL_OFFSET>,
            DecoderExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorBlt::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange::<Impl, IMPL_OFFSET>,
            QueryAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            ConfigureAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamRotation::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamRotation::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers1::<Impl, IMPL_OFFSET>,
            GetDataForNewHardwareKey::<Impl, IMPL_OFFSET>,
            CheckCryptoSessionStatus::<Impl, IMPL_OFFSET>,
            DecoderEnableDownsampling::<Impl, IMPL_OFFSET>,
            DecoderUpdateDownsampling::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetBehaviorHints::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext2Impl: Sized + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn VideoProcessorSetOutputHDRMetaData();
    fn VideoProcessorGetOutputHDRMetaData();
    fn VideoProcessorSetStreamHDRMetaData();
    fn VideoProcessorGetStreamHDRMetaData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext2Vtbl {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetDecoderBuffer::<Impl, IMPL_OFFSET>,
            ReleaseDecoderBuffer::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame::<Impl, IMPL_OFFSET>,
            DecoderEndFrame::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers::<Impl, IMPL_OFFSET>,
            DecoderExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorBlt::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange::<Impl, IMPL_OFFSET>,
            QueryAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            ConfigureAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamRotation::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamRotation::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers1::<Impl, IMPL_OFFSET>,
            GetDataForNewHardwareKey::<Impl, IMPL_OFFSET>,
            CheckCryptoSessionStatus::<Impl, IMPL_OFFSET>,
            DecoderEnableDownsampling::<Impl, IMPL_OFFSET>,
            DecoderUpdateDownsampling::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetBehaviorHints::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext3Impl: Sized + ID3D11VideoContext2Impl + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn DecoderBeginFrame1();
    fn SubmitDecoderBuffers2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoContext3Vtbl {
        unsafe extern "system" fn DecoderBeginFrame1<Impl: ID3D11VideoContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Impl: ID3D11VideoContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetDecoderBuffer::<Impl, IMPL_OFFSET>,
            ReleaseDecoderBuffer::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame::<Impl, IMPL_OFFSET>,
            DecoderEndFrame::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers::<Impl, IMPL_OFFSET>,
            DecoderExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputTargetRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputBackgroundColor::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputAlphaFillMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputConstriction::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputStereoMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFrameFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamOutputRate::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamSourceRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamDestRect::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAlpha::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPalette::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamLumaKey::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamStereoFormat::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamFilter::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamExtension::<Impl, IMPL_OFFSET>,
            VideoProcessorBlt::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchange::<Impl, IMPL_OFFSET>,
            EncryptionBlt::<Impl, IMPL_OFFSET>,
            DecryptionBlt::<Impl, IMPL_OFFSET>,
            StartSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            FinishSessionKeyRefresh::<Impl, IMPL_OFFSET>,
            GetEncryptionBltKey::<Impl, IMPL_OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange::<Impl, IMPL_OFFSET>,
            QueryAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            ConfigureAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamRotation::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamRotation::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers1::<Impl, IMPL_OFFSET>,
            GetDataForNewHardwareKey::<Impl, IMPL_OFFSET>,
            CheckCryptoSessionStatus::<Impl, IMPL_OFFSET>,
            DecoderEnableDownsampling::<Impl, IMPL_OFFSET>,
            DecoderUpdateDownsampling::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputShaderUsage::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamColorSpace1::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamMirror::<Impl, IMPL_OFFSET>,
            VideoProcessorGetBehaviorHints::<Impl, IMPL_OFFSET>,
            VideoProcessorSetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetOutputHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorSetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
            VideoProcessorGetStreamHDRMetaData::<Impl, IMPL_OFFSET>,
            DecoderBeginFrame1::<Impl, IMPL_OFFSET>,
            SubmitDecoderBuffers2::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDecoderImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCreationParameters();
    fn GetDriverHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDecoderVtbl {
        unsafe extern "system" fn GetCreationParameters<Impl: ID3D11VideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDriverHandle<Impl: ID3D11VideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetCreationParameters::<Impl, IMPL_OFFSET>, GetDriverHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoder as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoDecoderOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11VideoDecoderOutputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderOutputViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDecoderOutputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoDecoderOutputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDecoderOutputView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDeviceImpl: Sized {
    fn CreateVideoDecoder();
    fn CreateVideoProcessor();
    fn CreateAuthenticatedChannel();
    fn CreateCryptoSession();
    fn CreateVideoDecoderOutputView();
    fn CreateVideoProcessorInputView();
    fn CreateVideoProcessorOutputView();
    fn CreateVideoProcessorEnumerator();
    fn GetVideoDecoderProfileCount();
    fn GetVideoDecoderProfile();
    fn CheckVideoDecoderFormat();
    fn GetVideoDecoderConfigCount();
    fn GetVideoDecoderConfig();
    fn GetContentProtectionCaps();
    fn CheckCryptoKeyExchange();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDeviceVtbl {
        unsafe extern "system" fn CreateVideoDecoder<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: ::windows::core::RawPtr, rateconversionindex: u32, ppvideoprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCryptoSession<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, ppcryptosession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdecoderprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentProtectionCaps<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32, pkeyexchangetype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateVideoDecoder::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor::<Impl, IMPL_OFFSET>,
            CreateAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            CreateCryptoSession::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorInputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorEnumerator::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfileCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfile::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderFormat::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfigCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfig::<Impl, IMPL_OFFSET>,
            GetContentProtectionCaps::<Impl, IMPL_OFFSET>,
            CheckCryptoKeyExchange::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice1Impl: Sized + ID3D11VideoDeviceImpl {
    fn GetCryptoSessionPrivateDataSize();
    fn GetVideoDecoderCaps();
    fn CheckVideoDecoderDownsampling();
    fn RecommendVideoDecoderDownsampleParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDevice1Vtbl {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID, pdecodercaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateVideoDecoder::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor::<Impl, IMPL_OFFSET>,
            CreateAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            CreateCryptoSession::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorInputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorEnumerator::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfileCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfile::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderFormat::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfigCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfig::<Impl, IMPL_OFFSET>,
            GetContentProtectionCaps::<Impl, IMPL_OFFSET>,
            CheckCryptoKeyExchange::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetCryptoSessionPrivateDataSize::<Impl, IMPL_OFFSET>,
            GetVideoDecoderCaps::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderDownsampling::<Impl, IMPL_OFFSET>,
            RecommendVideoDecoderDownsampleParameters::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice2Impl: Sized + ID3D11VideoDevice1Impl + ID3D11VideoDeviceImpl {
    fn CheckFeatureSupport();
    fn NegotiateCryptoSessionKeyExchangeMT();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoDevice2Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Impl: ID3D11VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateVideoDecoder::<Impl, IMPL_OFFSET>,
            CreateVideoProcessor::<Impl, IMPL_OFFSET>,
            CreateAuthenticatedChannel::<Impl, IMPL_OFFSET>,
            CreateCryptoSession::<Impl, IMPL_OFFSET>,
            CreateVideoDecoderOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorInputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorOutputView::<Impl, IMPL_OFFSET>,
            CreateVideoProcessorEnumerator::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfileCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderProfile::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderFormat::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfigCount::<Impl, IMPL_OFFSET>,
            GetVideoDecoderConfig::<Impl, IMPL_OFFSET>,
            GetContentProtectionCaps::<Impl, IMPL_OFFSET>,
            CheckCryptoKeyExchange::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetCryptoSessionPrivateDataSize::<Impl, IMPL_OFFSET>,
            GetVideoDecoderCaps::<Impl, IMPL_OFFSET>,
            CheckVideoDecoderDownsampling::<Impl, IMPL_OFFSET>,
            RecommendVideoDecoderDownsampleParameters::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContentDesc();
    fn GetRateConversionCaps();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorVtbl {
        unsafe extern "system" fn GetContentDesc<Impl: ID3D11VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRateConversionCaps<Impl: ID3D11VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetContentDesc::<Impl, IMPL_OFFSET>, GetRateConversionCaps::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumeratorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetVideoProcessorContentDesc();
    fn CheckVideoProcessorFormat();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorRateConversionCaps();
    fn GetVideoProcessorCustomRate();
    fn GetVideoProcessorFilterRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorEnumeratorVtbl {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetVideoProcessorContentDesc::<Impl, IMPL_OFFSET>,
            CheckVideoProcessorFormat::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorRateConversionCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCustomRate::<Impl, IMPL_OFFSET>,
            GetVideoProcessorFilterRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator1Impl: Sized + ID3D11VideoProcessorEnumeratorImpl + ID3D11DeviceChildImpl {
    fn CheckVideoProcessorFormatConversion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11VideoProcessorEnumerator1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorEnumerator1Vtbl {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Impl: ID3D11VideoProcessorEnumerator1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetVideoProcessorContentDesc::<Impl, IMPL_OFFSET>,
            CheckVideoProcessorFormat::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorRateConversionCaps::<Impl, IMPL_OFFSET>,
            GetVideoProcessorCustomRate::<Impl, IMPL_OFFSET>,
            GetVideoProcessorFilterRange::<Impl, IMPL_OFFSET>,
            CheckVideoProcessorFormatConversion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator1 as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorInputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11VideoProcessorInputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorInputViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorInputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorInputView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ID3D11VideoProcessorOutputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorOutputViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11VideoProcessorOutputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorOutputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorOutputView as ::windows::core::Interface>::IID
    }
}
pub trait ID3D11ViewImpl: Sized + ID3D11DeviceChildImpl {
    fn GetResource();
}
impl ID3D11ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3D11ViewVtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D11ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11View as ::windows::core::Interface>::IID
    }
}
pub trait ID3DDeviceContextStateImpl: Sized + ID3D11DeviceChildImpl {}
impl ID3DDeviceContextStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDeviceContextStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DDeviceContextStateVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDeviceContextState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3DUserDefinedAnnotationImpl: Sized {
    fn BeginEvent();
    fn EndEvent();
    fn SetMarker();
    fn GetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3DUserDefinedAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DUserDefinedAnnotationVtbl {
        unsafe extern "system" fn BeginEvent<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMarker<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BeginEvent::<Impl, IMPL_OFFSET>, EndEvent::<Impl, IMPL_OFFSET>, SetMarker::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DUserDefinedAnnotation as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11FFTImpl: Sized {
    fn SetForwardScale();
    fn GetForwardScale();
    fn SetInverseScale();
    fn GetInverseScale();
    fn AttachBuffersAndPrecompute();
    fn ForwardTransform();
    fn InverseTransform();
}
impl ID3DX11FFTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11FFTVtbl {
        unsafe extern "system" fn SetForwardScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardscale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForwardScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInverseScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inversescale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInverseScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const ::windows::core::RawPtr, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForwardTransform<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InverseTransform<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetForwardScale::<Impl, IMPL_OFFSET>, GetForwardScale::<Impl, IMPL_OFFSET>, SetInverseScale::<Impl, IMPL_OFFSET>, GetInverseScale::<Impl, IMPL_OFFSET>, AttachBuffersAndPrecompute::<Impl, IMPL_OFFSET>, ForwardTransform::<Impl, IMPL_OFFSET>, InverseTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11FFT as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11ScanImpl: Sized {
    fn SetScanDirection();
    fn Scan();
    fn Multiscan();
}
impl ID3DX11ScanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11ScanImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11ScanVtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scan<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Multiscan<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetScanDirection::<Impl, IMPL_OFFSET>, Scan::<Impl, IMPL_OFFSET>, Multiscan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11Scan as ::windows::core::Interface>::IID
    }
}
pub trait ID3DX11SegmentedScanImpl: Sized {
    fn SetScanDirection();
    fn SegScan();
}
impl ID3DX11SegmentedScanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScanImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DX11SegmentedScanVtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11SegmentedScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SegScan<Impl: ID3DX11SegmentedScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, psrcelementflags: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetScanDirection::<Impl, IMPL_OFFSET>, SegScan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DX11SegmentedScan as ::windows::core::Interface>::IID
    }
}

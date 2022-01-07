pub trait ID3D11AsynchronousImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDataSize();
}
impl ::windows::core::RuntimeName for ID3D11Asynchronous {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Asynchronous";
}
impl ID3D11AsynchronousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AsynchronousImpl, const OFFSET: isize>() -> ID3D11AsynchronousVtbl {
        unsafe extern "system" fn GetDataSize<Impl: ID3D11AsynchronousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Asynchronous>, ::windows::core::GetTrustLevel, GetDataSize::<Impl, OFFSET>)
    }
}
pub trait ID3D11AuthenticatedChannelImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetChannelHandle();
}
impl ::windows::core::RuntimeName for ID3D11AuthenticatedChannel {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11AuthenticatedChannel";
}
impl ID3D11AuthenticatedChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>() -> ID3D11AuthenticatedChannelVtbl {
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSize(::core::mem::transmute_copy(&pcertificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificate(certificatesize, ::core::mem::transmute_copy(&pcertificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelHandle<Impl: ID3D11AuthenticatedChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelHandle(::core::mem::transmute_copy(&pchannelhandle)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11AuthenticatedChannel>, ::windows::core::GetTrustLevel, GetCertificateSize::<Impl, OFFSET>, GetCertificate::<Impl, OFFSET>, GetChannelHandle::<Impl, OFFSET>)
    }
}
pub trait ID3D11BlendStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11BlendState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11BlendState";
}
impl ID3D11BlendStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendStateImpl, const OFFSET: isize>() -> ID3D11BlendStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11BlendStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11BlendState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11BlendState1Impl: Sized + ID3D11BlendStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11BlendState1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11BlendState1";
}
impl ID3D11BlendState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BlendState1Impl, const OFFSET: isize>() -> ID3D11BlendState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11BlendState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11BlendState1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11BufferImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Buffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Buffer";
}
impl ID3D11BufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11BufferImpl, const OFFSET: isize>() -> ID3D11BufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11BufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Buffer>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11ClassInstanceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassLinkage();
    fn GetDesc();
    fn GetInstanceName();
    fn GetTypeName();
}
impl ::windows::core::RuntimeName for ID3D11ClassInstance {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ClassInstance";
}
impl ID3D11ClassInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>() -> ID3D11ClassInstanceVtbl {
        unsafe extern "system" fn GetClassLinkage<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClassLinkage(::core::mem::transmute_copy(&pplinkage)).into()
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetInstanceName<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstancename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInstanceName(::core::mem::transmute_copy(&pinstancename), pbufferlength).into()
        }
        unsafe extern "system" fn GetTypeName<Impl: ID3D11ClassInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypename: super::super::Foundation::PSTR, pbufferlength: *mut usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeName(::core::mem::transmute_copy(&ptypename), pbufferlength).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ClassInstance>, ::windows::core::GetTrustLevel, GetClassLinkage::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>, GetInstanceName::<Impl, OFFSET>, GetTypeName::<Impl, OFFSET>)
    }
}
pub trait ID3D11ClassLinkageImpl: Sized + ID3D11DeviceChildImpl {
    fn GetClassInstance();
    fn CreateClassInstance();
}
impl ::windows::core::RuntimeName for ID3D11ClassLinkage {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ClassLinkage";
}
impl ID3D11ClassLinkageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ClassLinkageImpl, const OFFSET: isize>() -> ID3D11ClassLinkageVtbl {
        unsafe extern "system" fn GetClassInstance<Impl: ID3D11ClassLinkageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassinstancename: super::super::Foundation::PSTR, instanceindex: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassInstance(&*(&pclassinstancename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), instanceindex, ::core::mem::transmute_copy(&ppinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassInstance<Impl: ID3D11ClassLinkageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclasstypename: super::super::Foundation::PSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClassInstance(&*(&pclasstypename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), constantbufferoffset, constantvectoroffset, textureoffset, sampleroffset, ::core::mem::transmute_copy(&ppinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ClassLinkage>, ::windows::core::GetTrustLevel, GetClassInstance::<Impl, OFFSET>, CreateClassInstance::<Impl, OFFSET>)
    }
}
pub trait ID3D11CommandListImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContextFlags();
}
impl ::windows::core::RuntimeName for ID3D11CommandList {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11CommandList";
}
impl ID3D11CommandListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CommandListImpl, const OFFSET: isize>() -> ID3D11CommandListVtbl {
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11CommandListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11CommandList>, ::windows::core::GetTrustLevel, GetContextFlags::<Impl, OFFSET>)
    }
}
pub trait ID3D11ComputeShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11ComputeShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ComputeShader";
}
impl ID3D11ComputeShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ComputeShaderImpl, const OFFSET: isize>() -> ID3D11ComputeShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ComputeShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11CounterImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Counter {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Counter";
}
impl ID3D11CounterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CounterImpl, const OFFSET: isize>() -> ID3D11CounterVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11CounterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Counter>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11CryptoSessionImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCryptoType();
    fn GetDecoderProfile();
    fn GetCertificateSize();
    fn GetCertificate();
    fn GetCryptoSessionHandle();
}
impl ::windows::core::RuntimeName for ID3D11CryptoSession {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11CryptoSession";
}
impl ID3D11CryptoSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>() -> ID3D11CryptoSessionVtbl {
        unsafe extern "system" fn GetCryptoType<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCryptoType(::core::mem::transmute_copy(&pcryptotype)).into()
        }
        unsafe extern "system" fn GetDecoderProfile<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDecoderProfile(::core::mem::transmute_copy(&pdecoderprofile)).into()
        }
        unsafe extern "system" fn GetCertificateSize<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSize(::core::mem::transmute_copy(&pcertificatesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificate(certificatesize, ::core::mem::transmute_copy(&pcertificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Impl: ID3D11CryptoSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCryptoSessionHandle(::core::mem::transmute_copy(&pcryptosessionhandle)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11CryptoSession>, ::windows::core::GetTrustLevel, GetCryptoType::<Impl, OFFSET>, GetDecoderProfile::<Impl, OFFSET>, GetCertificateSize::<Impl, OFFSET>, GetCertificate::<Impl, OFFSET>, GetCryptoSessionHandle::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11Debug {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Debug";
}
impl ID3D11DebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DebugImpl, const OFFSET: isize>() -> ID3D11DebugVtbl {
        unsafe extern "system" fn SetFeatureMask<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFeatureMask(mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureMask<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresentPerRenderOpDelay(milliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresentPerRenderOpDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSwapChain<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSwapChain(&*(&pswapchain as *const <super::Dxgi::IDXGISwapChain as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISwapChain as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSwapChain<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwapChain(::core::mem::transmute_copy(&ppswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateContext<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateContext(&*(&pcontext as *const <ID3D11DeviceContext as ::windows::core::Abi>::Abi as *const <ID3D11DeviceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLiveDeviceObjects(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateContextForDispatch<Impl: ID3D11DebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateContextForDispatch(&*(&pcontext as *const <ID3D11DeviceContext as ::windows::core::Abi>::Abi as *const <ID3D11DeviceContext as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11Debug>,
            ::windows::core::GetTrustLevel,
            SetFeatureMask::<Impl, OFFSET>,
            GetFeatureMask::<Impl, OFFSET>,
            SetPresentPerRenderOpDelay::<Impl, OFFSET>,
            GetPresentPerRenderOpDelay::<Impl, OFFSET>,
            SetSwapChain::<Impl, OFFSET>,
            GetSwapChain::<Impl, OFFSET>,
            ValidateContext::<Impl, OFFSET>,
            ReportLiveDeviceObjects::<Impl, OFFSET>,
            ValidateContextForDispatch::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11DepthStencilStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11DepthStencilState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DepthStencilState";
}
impl ID3D11DepthStencilStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilStateImpl, const OFFSET: isize>() -> ID3D11DepthStencilStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DepthStencilState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11DepthStencilViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11DepthStencilView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DepthStencilView";
}
impl ID3D11DepthStencilViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DepthStencilViewImpl, const OFFSET: isize>() -> ID3D11DepthStencilViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11DepthStencilViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DepthStencilView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11Device {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device";
}
impl ID3D11DeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceImpl, const OFFSET: isize>() -> ID3D11DeviceVtbl {
        unsafe extern "system" fn CreateBuffer<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBuffer(&*(&pdesc as *const <D3D11_BUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_BUFFER_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture1D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture1D(&*(&pdesc as *const <D3D11_TEXTURE1D_DESC as ::windows::core::Abi>::Abi as *const <D3D11_TEXTURE1D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture1d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D(&*(&pdesc as *const <D3D11_TEXTURE2D_DESC as ::windows::core::Abi>::Abi as *const <D3D11_TEXTURE2D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture2d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D(&*(&pdesc as *const <D3D11_TEXTURE3D_DESC as ::windows::core::Abi>::Abi as *const <D3D11_TEXTURE3D_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D11_SHADER_RESOURCE_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_SHADER_RESOURCE_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsrview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnorderedAccessView(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D11_UNORDERED_ACCESS_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_UNORDERED_ACCESS_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppuaview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D11_RENDER_TARGET_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_RENDER_TARGET_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprtview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilView(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D11_DEPTH_STENCIL_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_DEPTH_STENCIL_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdepthstencilview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInputLayout<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInputLayout(&*(&pinputelementdescs as *const <D3D11_INPUT_ELEMENT_DESC as ::windows::core::Abi>::Abi as *const <D3D11_INPUT_ELEMENT_DESC as ::windows::core::DefaultType>::DefaultType), numelements, &*(&pshaderbytecodewithinputsignature as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, ::core::mem::transmute_copy(&ppinputlayout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVertexShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvertexshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgeometryshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: ::windows::core::RawPtr, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryShaderWithStreamOutput(
                &*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                bytecodelength,
                &*(&psodeclaration as *const <D3D11_SO_DECLARATION_ENTRY as ::windows::core::Abi>::Abi as *const <D3D11_SO_DECLARATION_ENTRY as ::windows::core::DefaultType>::DefaultType),
                numentries,
                pbufferstrides,
                numstrides,
                rasterizedstream,
                &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppgeometryshader),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePixelShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePixelShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppixelshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHullShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, pphullshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHullShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pphullshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppdomainshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDomainShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdomainshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComputeShader<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::windows::core::RawPtr, ppcomputeshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComputeShader(&*(&pshaderbytecode as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), bytecodelength, &*(&pclasslinkage as *const <ID3D11ClassLinkage as ::windows::core::Abi>::Abi as *const <ID3D11ClassLinkage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomputeshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassLinkage<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplinkage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClassLinkage(::core::mem::transmute_copy(&pplinkage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState(&*(&pblendstatedesc as *const <D3D11_BLEND_DESC as ::windows::core::Abi>::Abi as *const <D3D11_BLEND_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppblendstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDepthStencilState(&*(&pdepthstencildesc as *const <D3D11_DEPTH_STENCIL_DESC as ::windows::core::Abi>::Abi as *const <D3D11_DEPTH_STENCIL_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdepthstencilstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState(&*(&prasterizerdesc as *const <D3D11_RASTERIZER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_RASTERIZER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprasterizerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSamplerState<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSamplerState(&*(&psamplerdesc as *const <D3D11_SAMPLER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_SAMPLER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsamplerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(&*(&pquerydesc as *const <D3D11_QUERY_DESC as ::windows::core::Abi>::Abi as *const <D3D11_QUERY_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePredicate<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePredicate(&*(&ppredicatedesc as *const <D3D11_QUERY_DESC as ::windows::core::Abi>::Abi as *const <D3D11_QUERY_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppredicate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCounter<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCounter(&*(&pcounterdesc as *const <D3D11_COUNTER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_COUNTER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcounter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeferredContext<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext(contextflags, ::core::mem::transmute_copy(&ppdeferredcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResource<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedResource(&*(&hresource as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFormatSupport(format, ::core::mem::transmute_copy(&pformatsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels(format, samplecount, ::core::mem::transmute_copy(&pnumqualitylevels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCounterInfo(::core::mem::transmute_copy(&pcounterinfo)).into()
        }
        unsafe extern "system" fn CheckCounter<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCounter(&*(&pdesc as *const <D3D11_COUNTER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_COUNTER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), pnamelength, ::core::mem::transmute_copy(&szunits), punitslength, ::core::mem::transmute_copy(&szdescription), pdescriptionlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, ::core::mem::transmute_copy(&pfeaturesupportdata), featuresupportdatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreationFlags<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceRemovedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmediateContext<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext(::core::mem::transmute_copy(&ppimmediatecontext)).into()
        }
        unsafe extern "system" fn SetExceptionMode<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExceptionMode(raiseflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExceptionMode<Impl: ID3D11DeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExceptionMode() {
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
            ::windows::core::GetRuntimeClassName::<ID3D11Device>,
            ::windows::core::GetTrustLevel,
            CreateBuffer::<Impl, OFFSET>,
            CreateTexture1D::<Impl, OFFSET>,
            CreateTexture2D::<Impl, OFFSET>,
            CreateTexture3D::<Impl, OFFSET>,
            CreateShaderResourceView::<Impl, OFFSET>,
            CreateUnorderedAccessView::<Impl, OFFSET>,
            CreateRenderTargetView::<Impl, OFFSET>,
            CreateDepthStencilView::<Impl, OFFSET>,
            CreateInputLayout::<Impl, OFFSET>,
            CreateVertexShader::<Impl, OFFSET>,
            CreateGeometryShader::<Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput::<Impl, OFFSET>,
            CreatePixelShader::<Impl, OFFSET>,
            CreateHullShader::<Impl, OFFSET>,
            CreateDomainShader::<Impl, OFFSET>,
            CreateComputeShader::<Impl, OFFSET>,
            CreateClassLinkage::<Impl, OFFSET>,
            CreateBlendState::<Impl, OFFSET>,
            CreateDepthStencilState::<Impl, OFFSET>,
            CreateRasterizerState::<Impl, OFFSET>,
            CreateSamplerState::<Impl, OFFSET>,
            CreateQuery::<Impl, OFFSET>,
            CreatePredicate::<Impl, OFFSET>,
            CreateCounter::<Impl, OFFSET>,
            CreateDeferredContext::<Impl, OFFSET>,
            OpenSharedResource::<Impl, OFFSET>,
            CheckFormatSupport::<Impl, OFFSET>,
            CheckMultisampleQualityLevels::<Impl, OFFSET>,
            CheckCounterInfo::<Impl, OFFSET>,
            CheckCounter::<Impl, OFFSET>,
            CheckFeatureSupport::<Impl, OFFSET>,
            GetPrivateData::<Impl, OFFSET>,
            SetPrivateData::<Impl, OFFSET>,
            SetPrivateDataInterface::<Impl, OFFSET>,
            GetFeatureLevel::<Impl, OFFSET>,
            GetCreationFlags::<Impl, OFFSET>,
            GetDeviceRemovedReason::<Impl, OFFSET>,
            GetImmediateContext::<Impl, OFFSET>,
            SetExceptionMode::<Impl, OFFSET>,
            GetExceptionMode::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11Device1Impl: Sized + ID3D11DeviceImpl {
    fn GetImmediateContext1();
    fn CreateDeferredContext1();
    fn CreateBlendState1();
    fn CreateRasterizerState1();
    fn CreateDeviceContextState();
    fn OpenSharedResource1();
    fn OpenSharedResourceByName();
}
impl ::windows::core::RuntimeName for ID3D11Device1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device1";
}
impl ID3D11Device1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device1Impl, const OFFSET: isize>() -> ID3D11Device1Vtbl {
        unsafe extern "system" fn GetImmediateContext1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext1(::core::mem::transmute_copy(&ppimmediatecontext)).into()
        }
        unsafe extern "system" fn CreateDeferredContext1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext1(contextflags, ::core::mem::transmute_copy(&ppdeferredcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlendState1(&*(&pblendstatedesc as *const <D3D11_BLEND_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_BLEND_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppblendstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState1(&*(&prasterizerdesc as *const <D3D11_RASTERIZER_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_RASTERIZER_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprasterizerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceContextState<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeviceContextState(flags, pfeaturelevels, featurelevels, sdkversion, &*(&emulatedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pchosenfeaturelevel), ::core::mem::transmute_copy(&ppcontextstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResource1<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedResource1(&*(&hresource as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedResourceByName<Impl: ID3D11Device1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedResourceByName(&*(&lpname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwdesiredaccess, &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresource)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11Device1>,
            ::windows::core::GetTrustLevel,
            GetImmediateContext1::<Impl, OFFSET>,
            CreateDeferredContext1::<Impl, OFFSET>,
            CreateBlendState1::<Impl, OFFSET>,
            CreateRasterizerState1::<Impl, OFFSET>,
            CreateDeviceContextState::<Impl, OFFSET>,
            OpenSharedResource1::<Impl, OFFSET>,
            OpenSharedResourceByName::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11Device2Impl: Sized + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn GetImmediateContext2();
    fn CreateDeferredContext2();
    fn GetResourceTiling();
    fn CheckMultisampleQualityLevels1();
}
impl ::windows::core::RuntimeName for ID3D11Device2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device2";
}
impl ID3D11Device2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device2Impl, const OFFSET: isize>() -> ID3D11Device2Vtbl {
        unsafe extern "system" fn GetImmediateContext2<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext2(::core::mem::transmute_copy(&ppimmediatecontext)).into()
        }
        unsafe extern "system" fn CreateDeferredContext2<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext2(contextflags, ::core::mem::transmute_copy(&ppdeferredcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTiling<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceTiling(&*(&ptiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), pnumsubresourcetilings, firstsubresourcetilingtoget, ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips)).into()
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Impl: ID3D11Device2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMultisampleQualityLevels1(format, samplecount, flags, ::core::mem::transmute_copy(&pnumqualitylevels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Device2>, ::windows::core::GetTrustLevel, GetImmediateContext2::<Impl, OFFSET>, CreateDeferredContext2::<Impl, OFFSET>, GetResourceTiling::<Impl, OFFSET>, CheckMultisampleQualityLevels1::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11Device3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device3";
}
impl ID3D11Device3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device3Impl, const OFFSET: isize>() -> ID3D11Device3Vtbl {
        unsafe extern "system" fn CreateTexture2D1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture2D1(&*(&pdesc1 as *const <D3D11_TEXTURE2D_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_TEXTURE2D_DESC1 as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture2d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTexture3D1(&*(&pdesc1 as *const <D3D11_TEXTURE3D_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_TEXTURE3D_DESC1 as ::windows::core::DefaultType>::DefaultType), &*(&pinitialdata as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::Abi>::Abi as *const <D3D11_SUBRESOURCE_DATA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptexture3d)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRasterizerState2<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizerState2(&*(&prasterizerdesc as *const <D3D11_RASTERIZER_DESC2 as ::windows::core::Abi>::Abi as *const <D3D11_RASTERIZER_DESC2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprasterizerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderResourceView1(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc1 as *const <D3D11_SHADER_RESOURCE_VIEW_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_SHADER_RESOURCE_VIEW_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsrview1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUnorderedAccessView1(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc1 as *const <D3D11_UNORDERED_ACCESS_VIEW_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_UNORDERED_ACCESS_VIEW_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppuaview1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderTargetView1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRenderTargetView1(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc1 as *const <D3D11_RENDER_TARGET_VIEW_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_RENDER_TARGET_VIEW_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprtview1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuery1<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery1(&*(&pquerydesc1 as *const <D3D11_QUERY_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_QUERY_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppquery1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmediateContext3<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImmediateContext3(::core::mem::transmute_copy(&ppimmediatecontext)).into()
        }
        unsafe extern "system" fn CreateDeferredContext3<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeferredContext3(contextflags, ::core::mem::transmute_copy(&ppdeferredcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToSubresource<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .WriteToSubresource(&*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&pdstbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType), &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), srcrowpitch, srcdepthpitch)
                .into()
        }
        unsafe extern "system" fn ReadFromSubresource<Impl: ID3D11Device3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadFromSubresource(::core::mem::transmute_copy(&pdstdata), dstrowpitch, dstdepthpitch, &*(&psrcresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, &*(&psrcbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D11Device3>,
            ::windows::core::GetTrustLevel,
            CreateTexture2D1::<Impl, OFFSET>,
            CreateTexture3D1::<Impl, OFFSET>,
            CreateRasterizerState2::<Impl, OFFSET>,
            CreateShaderResourceView1::<Impl, OFFSET>,
            CreateUnorderedAccessView1::<Impl, OFFSET>,
            CreateRenderTargetView1::<Impl, OFFSET>,
            CreateQuery1::<Impl, OFFSET>,
            GetImmediateContext3::<Impl, OFFSET>,
            CreateDeferredContext3::<Impl, OFFSET>,
            WriteToSubresource::<Impl, OFFSET>,
            ReadFromSubresource::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11Device4Impl: Sized + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn RegisterDeviceRemovedEvent();
    fn UnregisterDeviceRemoved();
}
impl ::windows::core::RuntimeName for ID3D11Device4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device4";
}
impl ID3D11Device4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device4Impl, const OFFSET: isize>() -> ID3D11Device4Vtbl {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Impl: ID3D11Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDeviceRemovedEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Impl: ID3D11Device4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDeviceRemoved(dwcookie).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Device4>, ::windows::core::GetTrustLevel, RegisterDeviceRemovedEvent::<Impl, OFFSET>, UnregisterDeviceRemoved::<Impl, OFFSET>)
    }
}
pub trait ID3D11Device5Impl: Sized + ID3D11Device4Impl + ID3D11Device3Impl + ID3D11Device2Impl + ID3D11Device1Impl + ID3D11DeviceImpl {
    fn OpenSharedFence();
    fn CreateFence();
}
impl ::windows::core::RuntimeName for ID3D11Device5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Device5";
}
impl ID3D11Device5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Device5Impl, const OFFSET: isize>() -> ID3D11Device5Vtbl {
        unsafe extern "system" fn OpenSharedFence<Impl: ID3D11Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenSharedFence(&*(&hfence as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFence<Impl: ID3D11Device5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows::core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFence(initialvalue, flags, &*(&returnedinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Device5>, ::windows::core::GetTrustLevel, OpenSharedFence::<Impl, OFFSET>, CreateFence::<Impl, OFFSET>)
    }
}
pub trait ID3D11DeviceChildImpl: Sized {
    fn GetDevice();
    fn GetPrivateData();
    fn SetPrivateData();
    fn SetPrivateDataInterface();
}
impl ::windows::core::RuntimeName for ID3D11DeviceChild {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceChild";
}
impl ID3D11DeviceChildVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceChildImpl, const OFFSET: isize>() -> ID3D11DeviceChildVtbl {
        unsafe extern "system" fn GetDevice<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11DeviceChildImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DeviceChild>, ::windows::core::GetTrustLevel, GetDevice::<Impl, OFFSET>, GetPrivateData::<Impl, OFFSET>, SetPrivateData::<Impl, OFFSET>, SetPrivateDataInterface::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11DeviceContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext";
}
impl ID3D11DeviceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContextImpl, const OFFSET: isize>() -> ID3D11DeviceContextVtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetShader(&*(&ppixelshader as *const <ID3D11PixelShader as ::windows::core::Abi>::Abi as *const <ID3D11PixelShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn PSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShader(&*(&pvertexshader as *const <ID3D11VertexShader as ::windows::core::Abi>::Abi as *const <ID3D11VertexShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn DrawIndexed<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexed(indexcount, startindexlocation, basevertexlocation).into()
        }
        unsafe extern "system" fn Draw<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(vertexcount, startvertexlocation).into()
        }
        unsafe extern "system" fn Map<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), subresource, maptype, mapflags, ::core::mem::transmute_copy(&pmappedresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, subresource: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), subresource).into()
        }
        unsafe extern "system" fn PSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetInputLayout<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetInputLayout(&*(&pinputlayout as *const <ID3D11InputLayout as ::windows::core::Abi>::Abi as *const <ID3D11InputLayout as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IASetVertexBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetVertexBuffers(startslot, numbuffers, &*(&ppvertexbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pstrides, poffsets).into()
        }
        unsafe extern "system" fn IASetIndexBuffer<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetIndexBuffer(&*(&pindexbuffer as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), format, offset).into()
        }
        unsafe extern "system" fn DrawIndexedInstanced<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstanced(indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn DrawInstanced<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstanced(vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation).into()
        }
        unsafe extern "system" fn GSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShader(&*(&pshader as *const <ID3D11GeometryShader as ::windows::core::Abi>::Abi as *const <ID3D11GeometryShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IASetPrimitiveTopology(topology).into()
        }
        unsafe extern "system" fn VSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Begin<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(&*(&pasync as *const <ID3D11Asynchronous as ::windows::core::Abi>::Abi as *const <ID3D11Asynchronous as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn End<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End(&*(&pasync as *const <ID3D11Asynchronous as ::windows::core::Abi>::Abi as *const <ID3D11Asynchronous as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetData<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasync: ::windows::core::RawPtr, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&pasync as *const <ID3D11Asynchronous as ::windows::core::Abi>::Abi as *const <ID3D11Asynchronous as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdata), datasize, getdataflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPredication<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPredication(&*(&ppredicate as *const <ID3D11Predicate as ::windows::core::Abi>::Abi as *const <ID3D11Predicate as ::windows::core::DefaultType>::DefaultType), &*(&predicatevalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetRenderTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetRenderTargets(numviews, &*(&pprendertargetviews as *const <ID3D11RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D11RenderTargetView as ::windows::core::DefaultType>::DefaultType), &*(&pdepthstencilview as *const <ID3D11DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D11DepthStencilView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OMSetRenderTargetsAndUnorderedAccessViews(
                    numrtvs,
                    &*(&pprendertargetviews as *const <ID3D11RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D11RenderTargetView as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdepthstencilview as *const <ID3D11DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D11DepthStencilView as ::windows::core::DefaultType>::DefaultType),
                    uavstartslot,
                    numuavs,
                    &*(&ppunorderedaccessviews as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType),
                    puavinitialcounts,
                )
                .into()
        }
        unsafe extern "system" fn OMSetBlendState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetBlendState(&*(&pblendstate as *const <ID3D11BlendState as ::windows::core::Abi>::Abi as *const <ID3D11BlendState as ::windows::core::DefaultType>::DefaultType), blendfactor, samplemask).into()
        }
        unsafe extern "system" fn OMSetDepthStencilState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMSetDepthStencilState(&*(&pdepthstencilstate as *const <ID3D11DepthStencilState as ::windows::core::Abi>::Abi as *const <ID3D11DepthStencilState as ::windows::core::DefaultType>::DefaultType), stencilref).into()
        }
        unsafe extern "system" fn SOSetTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOSetTargets(numbuffers, &*(&ppsotargets as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), poffsets).into()
        }
        unsafe extern "system" fn DrawAuto<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawAuto().into()
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawIndexedInstancedIndirect(&*(&pbufferforargs as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), alignedbyteoffsetforargs).into()
        }
        unsafe extern "system" fn DrawInstancedIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawInstancedIndirect(&*(&pbufferforargs as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), alignedbyteoffsetforargs).into()
        }
        unsafe extern "system" fn Dispatch<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dispatch(threadgroupcountx, threadgroupcounty, threadgroupcountz).into()
        }
        unsafe extern "system" fn DispatchIndirect<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferforargs: ::windows::core::RawPtr, alignedbyteoffsetforargs: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DispatchIndirect(&*(&pbufferforargs as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), alignedbyteoffsetforargs).into()
        }
        unsafe extern "system" fn RSSetState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetState(&*(&prasterizerstate as *const <ID3D11RasterizerState as ::windows::core::Abi>::Abi as *const <ID3D11RasterizerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RSSetViewports<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetViewports(numviewports, &*(&pviewports as *const <D3D11_VIEWPORT as ::windows::core::Abi>::Abi as *const <D3D11_VIEWPORT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RSSetScissorRects<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSSetScissorRects(numrects, &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopySubresourceRegion<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopySubresourceRegion(&*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, dstx, dsty, dstz, &*(&psrcresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, &*(&psrcbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CopyResource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyResource(&*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&psrcresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateSubresource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateSubresource(&*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&pdstbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType), &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), srcrowpitch, srcdepthpitch)
                .into()
        }
        unsafe extern "system" fn CopyStructureCount<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstbuffer: ::windows::core::RawPtr, dstalignedbyteoffset: u32, psrcview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyStructureCount(&*(&pdstbuffer as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), dstalignedbyteoffset, &*(&psrcview as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearRenderTargetView<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRenderTargetView(&*(&prendertargetview as *const <ID3D11RenderTargetView as ::windows::core::Abi>::Abi as *const <ID3D11RenderTargetView as ::windows::core::DefaultType>::DefaultType), colorrgba).into()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewUint(&*(&punorderedaccessview as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), values).into()
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punorderedaccessview: ::windows::core::RawPtr, values: *const f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUnorderedAccessViewFloat(&*(&punorderedaccessview as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), values).into()
        }
        unsafe extern "system" fn ClearDepthStencilView<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearDepthStencilView(&*(&pdepthstencilview as *const <ID3D11DepthStencilView as ::windows::core::Abi>::Abi as *const <ID3D11DepthStencilView as ::windows::core::DefaultType>::DefaultType), clearflags, depth, stencil).into()
        }
        unsafe extern "system" fn GenerateMips<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateMips(&*(&pshaderresourceview as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetResourceMinLOD<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, minlod: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceMinLOD(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), minlod).into()
        }
        unsafe extern "system" fn GetResourceMinLOD<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceMinLOD(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveSubresource<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveSubresource(&*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), dstsubresource, &*(&psrcresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), srcsubresource, format).into()
        }
        unsafe extern "system" fn ExecuteCommandList<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommandlist: ::windows::core::RawPtr, restorecontextstate: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteCommandList(&*(&pcommandlist as *const <ID3D11CommandList as ::windows::core::Abi>::Abi as *const <ID3D11CommandList as ::windows::core::DefaultType>::DefaultType), &*(&restorecontextstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phullshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetShader(&*(&phullshader as *const <ID3D11HullShader as ::windows::core::Abi>::Abi as *const <ID3D11HullShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn HSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetShader(&*(&pdomainshader as *const <ID3D11DomainShader as ::windows::core::Abi>::Abi as *const <ID3D11DomainShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn DSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CSSetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetShaderResources(startslot, numviews, &*(&ppshaderresourceviews as *const <ID3D11ShaderResourceView as ::windows::core::Abi>::Abi as *const <ID3D11ShaderResourceView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::windows::core::RawPtr, puavinitialcounts: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetUnorderedAccessViews(startslot, numuavs, &*(&ppunorderedaccessviews as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), puavinitialcounts).into()
        }
        unsafe extern "system" fn CSSetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomputeshader: ::windows::core::RawPtr, ppclassinstances: *const ::windows::core::RawPtr, numclassinstances: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetShader(&*(&pcomputeshader as *const <ID3D11ComputeShader as ::windows::core::Abi>::Abi as *const <ID3D11ComputeShader as ::windows::core::DefaultType>::DefaultType), &*(&ppclassinstances as *const <ID3D11ClassInstance as ::windows::core::Abi>::Abi as *const <ID3D11ClassInstance as ::windows::core::DefaultType>::DefaultType), numclassinstances).into()
        }
        unsafe extern "system" fn CSSetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetSamplers(startslot, numsamplers, &*(&ppsamplers as *const <ID3D11SamplerState as ::windows::core::Abi>::Abi as *const <ID3D11SamplerState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CSSetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetConstantBuffers(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn PSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn PSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetShader(::core::mem::transmute_copy(&pppixelshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn PSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn VSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShader(::core::mem::transmute_copy(&ppvertexshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn PSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn IAGetInputLayout<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetInputLayout(::core::mem::transmute_copy(&ppinputlayout)).into()
        }
        unsafe extern "system" fn IAGetVertexBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetVertexBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)).into()
        }
        unsafe extern "system" fn IAGetIndexBuffer<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetIndexBuffer(::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn GSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShader(::core::mem::transmute_copy(&ppgeometryshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IAGetPrimitiveTopology(::core::mem::transmute_copy(&ptopology)).into()
        }
        unsafe extern "system" fn VSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn VSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn GetPredication<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPredication(::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue)).into()
        }
        unsafe extern "system" fn GSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn GSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn OMGetRenderTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargets(numviews, ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview)).into()
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetRenderTargetsAndUnorderedAccessViews(numrtvs, ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview), uavstartslot, numuavs, ::core::mem::transmute_copy(&ppunorderedaccessviews)).into()
        }
        unsafe extern "system" fn OMGetBlendState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetBlendState(::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask)).into()
        }
        unsafe extern "system" fn OMGetDepthStencilState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OMGetDepthStencilState(::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref)).into()
        }
        unsafe extern "system" fn SOGetTargets<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SOGetTargets(numbuffers, ::core::mem::transmute_copy(&ppsotargets)).into()
        }
        unsafe extern "system" fn RSGetState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetState(::core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn RSGetViewports<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetViewports(pnumviewports, ::core::mem::transmute_copy(&pviewports)).into()
        }
        unsafe extern "system" fn RSGetScissorRects<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RSGetScissorRects(pnumrects, ::core::mem::transmute_copy(&prects)).into()
        }
        unsafe extern "system" fn HSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn HSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphullshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetShader(::core::mem::transmute_copy(&pphullshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn HSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn HSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn DSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn DSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomainshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetShader(::core::mem::transmute_copy(&ppdomainshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn DSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn DSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn CSGetShaderResources<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetShaderResources(startslot, numviews, ::core::mem::transmute_copy(&ppshaderresourceviews)).into()
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetUnorderedAccessViews(startslot, numuavs, ::core::mem::transmute_copy(&ppunorderedaccessviews)).into()
        }
        unsafe extern "system" fn CSGetShader<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomputeshader: *mut ::windows::core::RawPtr, ppclassinstances: *mut ::windows::core::RawPtr, pnumclassinstances: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetShader(::core::mem::transmute_copy(&ppcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), pnumclassinstances).into()
        }
        unsafe extern "system" fn CSGetSamplers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetSamplers(startslot, numsamplers, ::core::mem::transmute_copy(&ppsamplers)).into()
        }
        unsafe extern "system" fn CSGetConstantBuffers<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetConstantBuffers(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers)).into()
        }
        unsafe extern "system" fn ClearState<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearState().into()
        }
        unsafe extern "system" fn Flush<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextFlags<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishCommandList<Impl: ID3D11DeviceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishCommandList(&*(&restoredeferredcontextstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandlist)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11DeviceContext>,
            ::windows::core::GetTrustLevel,
            VSSetConstantBuffers::<Impl, OFFSET>,
            PSSetShaderResources::<Impl, OFFSET>,
            PSSetShader::<Impl, OFFSET>,
            PSSetSamplers::<Impl, OFFSET>,
            VSSetShader::<Impl, OFFSET>,
            DrawIndexed::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            Map::<Impl, OFFSET>,
            Unmap::<Impl, OFFSET>,
            PSSetConstantBuffers::<Impl, OFFSET>,
            IASetInputLayout::<Impl, OFFSET>,
            IASetVertexBuffers::<Impl, OFFSET>,
            IASetIndexBuffer::<Impl, OFFSET>,
            DrawIndexedInstanced::<Impl, OFFSET>,
            DrawInstanced::<Impl, OFFSET>,
            GSSetConstantBuffers::<Impl, OFFSET>,
            GSSetShader::<Impl, OFFSET>,
            IASetPrimitiveTopology::<Impl, OFFSET>,
            VSSetShaderResources::<Impl, OFFSET>,
            VSSetSamplers::<Impl, OFFSET>,
            Begin::<Impl, OFFSET>,
            End::<Impl, OFFSET>,
            GetData::<Impl, OFFSET>,
            SetPredication::<Impl, OFFSET>,
            GSSetShaderResources::<Impl, OFFSET>,
            GSSetSamplers::<Impl, OFFSET>,
            OMSetRenderTargets::<Impl, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews::<Impl, OFFSET>,
            OMSetBlendState::<Impl, OFFSET>,
            OMSetDepthStencilState::<Impl, OFFSET>,
            SOSetTargets::<Impl, OFFSET>,
            DrawAuto::<Impl, OFFSET>,
            DrawIndexedInstancedIndirect::<Impl, OFFSET>,
            DrawInstancedIndirect::<Impl, OFFSET>,
            Dispatch::<Impl, OFFSET>,
            DispatchIndirect::<Impl, OFFSET>,
            RSSetState::<Impl, OFFSET>,
            RSSetViewports::<Impl, OFFSET>,
            RSSetScissorRects::<Impl, OFFSET>,
            CopySubresourceRegion::<Impl, OFFSET>,
            CopyResource::<Impl, OFFSET>,
            UpdateSubresource::<Impl, OFFSET>,
            CopyStructureCount::<Impl, OFFSET>,
            ClearRenderTargetView::<Impl, OFFSET>,
            ClearUnorderedAccessViewUint::<Impl, OFFSET>,
            ClearUnorderedAccessViewFloat::<Impl, OFFSET>,
            ClearDepthStencilView::<Impl, OFFSET>,
            GenerateMips::<Impl, OFFSET>,
            SetResourceMinLOD::<Impl, OFFSET>,
            GetResourceMinLOD::<Impl, OFFSET>,
            ResolveSubresource::<Impl, OFFSET>,
            ExecuteCommandList::<Impl, OFFSET>,
            HSSetShaderResources::<Impl, OFFSET>,
            HSSetShader::<Impl, OFFSET>,
            HSSetSamplers::<Impl, OFFSET>,
            HSSetConstantBuffers::<Impl, OFFSET>,
            DSSetShaderResources::<Impl, OFFSET>,
            DSSetShader::<Impl, OFFSET>,
            DSSetSamplers::<Impl, OFFSET>,
            DSSetConstantBuffers::<Impl, OFFSET>,
            CSSetShaderResources::<Impl, OFFSET>,
            CSSetUnorderedAccessViews::<Impl, OFFSET>,
            CSSetShader::<Impl, OFFSET>,
            CSSetSamplers::<Impl, OFFSET>,
            CSSetConstantBuffers::<Impl, OFFSET>,
            VSGetConstantBuffers::<Impl, OFFSET>,
            PSGetShaderResources::<Impl, OFFSET>,
            PSGetShader::<Impl, OFFSET>,
            PSGetSamplers::<Impl, OFFSET>,
            VSGetShader::<Impl, OFFSET>,
            PSGetConstantBuffers::<Impl, OFFSET>,
            IAGetInputLayout::<Impl, OFFSET>,
            IAGetVertexBuffers::<Impl, OFFSET>,
            IAGetIndexBuffer::<Impl, OFFSET>,
            GSGetConstantBuffers::<Impl, OFFSET>,
            GSGetShader::<Impl, OFFSET>,
            IAGetPrimitiveTopology::<Impl, OFFSET>,
            VSGetShaderResources::<Impl, OFFSET>,
            VSGetSamplers::<Impl, OFFSET>,
            GetPredication::<Impl, OFFSET>,
            GSGetShaderResources::<Impl, OFFSET>,
            GSGetSamplers::<Impl, OFFSET>,
            OMGetRenderTargets::<Impl, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews::<Impl, OFFSET>,
            OMGetBlendState::<Impl, OFFSET>,
            OMGetDepthStencilState::<Impl, OFFSET>,
            SOGetTargets::<Impl, OFFSET>,
            RSGetState::<Impl, OFFSET>,
            RSGetViewports::<Impl, OFFSET>,
            RSGetScissorRects::<Impl, OFFSET>,
            HSGetShaderResources::<Impl, OFFSET>,
            HSGetShader::<Impl, OFFSET>,
            HSGetSamplers::<Impl, OFFSET>,
            HSGetConstantBuffers::<Impl, OFFSET>,
            DSGetShaderResources::<Impl, OFFSET>,
            DSGetShader::<Impl, OFFSET>,
            DSGetSamplers::<Impl, OFFSET>,
            DSGetConstantBuffers::<Impl, OFFSET>,
            CSGetShaderResources::<Impl, OFFSET>,
            CSGetUnorderedAccessViews::<Impl, OFFSET>,
            CSGetShader::<Impl, OFFSET>,
            CSGetSamplers::<Impl, OFFSET>,
            CSGetConstantBuffers::<Impl, OFFSET>,
            ClearState::<Impl, OFFSET>,
            Flush::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetContextFlags::<Impl, OFFSET>,
            FinishCommandList::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D11DeviceContext1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext1";
}
impl ID3D11DeviceContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>() -> ID3D11DeviceContext1Vtbl {
        unsafe extern "system" fn CopySubresourceRegion1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopySubresourceRegion1(
                    &*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                    dstsubresource,
                    dstx,
                    dsty,
                    dstz,
                    &*(&psrcresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                    srcsubresource,
                    &*(&psrcbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType),
                    copyflags,
                )
                .into()
        }
        unsafe extern "system" fn UpdateSubresource1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateSubresource1(
                    &*(&pdstresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                    dstsubresource,
                    &*(&pdstbox as *const <D3D11_BOX as ::windows::core::Abi>::Abi as *const <D3D11_BOX as ::windows::core::DefaultType>::DefaultType),
                    &*(&psrcdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    srcrowpitch,
                    srcdepthpitch,
                    copyflags,
                )
                .into()
        }
        unsafe extern "system" fn DiscardResource<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResource(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DiscardView<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardView(&*(&presourceview as *const <ID3D11View as ::windows::core::Abi>::Abi as *const <ID3D11View as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSSetConstantBuffers1(startslot, numbuffers, &*(&ppconstantbuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), pfirstconstant, pnumconstants).into()
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CSGetConstantBuffers1(startslot, numbuffers, ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)).into()
        }
        unsafe extern "system" fn SwapDeviceContextState<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: ::windows::core::RawPtr, pppreviousstate: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwapDeviceContextState(&*(&pstate as *const <ID3DDeviceContextState as ::windows::core::Abi>::Abi as *const <ID3DDeviceContextState as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppreviousstate)).into()
        }
        unsafe extern "system" fn ClearView<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pview: ::windows::core::RawPtr, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearView(&*(&pview as *const <ID3D11View as ::windows::core::Abi>::Abi as *const <ID3D11View as ::windows::core::DefaultType>::DefaultType), color, &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), numrects).into()
        }
        unsafe extern "system" fn DiscardView1<Impl: ID3D11DeviceContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceview: ::windows::core::RawPtr, prects: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardView1(&*(&presourceview as *const <ID3D11View as ::windows::core::Abi>::Abi as *const <ID3D11View as ::windows::core::DefaultType>::DefaultType), &*(&prects as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), numrects).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D11DeviceContext1>,
            ::windows::core::GetTrustLevel,
            CopySubresourceRegion1::<Impl, OFFSET>,
            UpdateSubresource1::<Impl, OFFSET>,
            DiscardResource::<Impl, OFFSET>,
            DiscardView::<Impl, OFFSET>,
            VSSetConstantBuffers1::<Impl, OFFSET>,
            HSSetConstantBuffers1::<Impl, OFFSET>,
            DSSetConstantBuffers1::<Impl, OFFSET>,
            GSSetConstantBuffers1::<Impl, OFFSET>,
            PSSetConstantBuffers1::<Impl, OFFSET>,
            CSSetConstantBuffers1::<Impl, OFFSET>,
            VSGetConstantBuffers1::<Impl, OFFSET>,
            HSGetConstantBuffers1::<Impl, OFFSET>,
            DSGetConstantBuffers1::<Impl, OFFSET>,
            GSGetConstantBuffers1::<Impl, OFFSET>,
            PSGetConstantBuffers1::<Impl, OFFSET>,
            CSGetConstantBuffers1::<Impl, OFFSET>,
            SwapDeviceContextState::<Impl, OFFSET>,
            ClearView::<Impl, OFFSET>,
            DiscardView1::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D11DeviceContext2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext2";
}
impl ID3D11DeviceContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>() -> ID3D11DeviceContext2Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: ::windows::core::RawPtr, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTileMappings(
                &*(&ptiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                numtiledresourceregions,
                &*(&ptiledresourceregionstartcoordinates as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                &*(&ptiledresourceregionsizes as *const <D3D11_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D11_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                &*(&ptilepool as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType),
                numranges,
                prangeflags,
                ptilepoolstartoffsets,
                prangetilecounts,
                flags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTileMappings<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: ::windows::core::RawPtr, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTileMappings(
                &*(&pdesttiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestregionstartcoordinate as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcetiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&psourceregionstartcoordinate as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                &*(&ptileregionsize as *const <D3D11_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D11_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                flags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiles<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresource: ::windows::core::RawPtr, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: ::windows::core::RawPtr, bufferstartoffsetinbytes: u64, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyTiles(
                    &*(&ptiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&ptileregionstartcoordinate as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&ptileregionsize as *const <D3D11_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D11_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                    &*(&pbuffer as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType),
                    bufferstartoffsetinbytes,
                    flags,
                )
                .into()
        }
        unsafe extern "system" fn UpdateTiles<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesttiledresource: ::windows::core::RawPtr, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateTiles(
                    &*(&pdesttiledresource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesttileregionstartcoordinate as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::Abi>::Abi as *const <D3D11_TILED_RESOURCE_COORDINATE as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdesttileregionsize as *const <D3D11_TILE_REGION_SIZE as ::windows::core::Abi>::Abi as *const <D3D11_TILE_REGION_SIZE as ::windows::core::DefaultType>::DefaultType),
                    &*(&psourcetiledata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    flags,
                )
                .into()
        }
        unsafe extern "system" fn ResizeTilePool<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptilepool: ::windows::core::RawPtr, newsizeinbytes: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeTilePool(&*(&ptilepool as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType), newsizeinbytes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TiledResourceBarrier<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: ::windows::core::RawPtr, ptiledresourceorviewaccessafterbarrier: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TiledResourceBarrier(&*(&ptiledresourceorviewaccessbeforebarrier as *const <ID3D11DeviceChild as ::windows::core::Abi>::Abi as *const <ID3D11DeviceChild as ::windows::core::DefaultType>::DefaultType), &*(&ptiledresourceorviewaccessafterbarrier as *const <ID3D11DeviceChild as ::windows::core::Abi>::Abi as *const <ID3D11DeviceChild as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsAnnotationEnabled<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnnotationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarkerInt<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarkerInt(&*(&plabel as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), data).into()
        }
        unsafe extern "system" fn BeginEventInt<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: super::super::Foundation::PWSTR, data: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEventInt(&*(&plabel as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), data).into()
        }
        unsafe extern "system" fn EndEvent<Impl: ID3D11DeviceContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEvent().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D11DeviceContext2>,
            ::windows::core::GetTrustLevel,
            UpdateTileMappings::<Impl, OFFSET>,
            CopyTileMappings::<Impl, OFFSET>,
            CopyTiles::<Impl, OFFSET>,
            UpdateTiles::<Impl, OFFSET>,
            ResizeTilePool::<Impl, OFFSET>,
            TiledResourceBarrier::<Impl, OFFSET>,
            IsAnnotationEnabled::<Impl, OFFSET>,
            SetMarkerInt::<Impl, OFFSET>,
            BeginEventInt::<Impl, OFFSET>,
            EndEvent::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11DeviceContext3Impl: Sized + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Flush1();
    fn SetHardwareProtectionState();
    fn GetHardwareProtectionState();
}
impl ::windows::core::RuntimeName for ID3D11DeviceContext3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext3";
}
impl ID3D11DeviceContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>() -> ID3D11DeviceContext3Vtbl {
        unsafe extern "system" fn Flush1<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush1(contexttype, &*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetHardwareProtectionState<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardwareProtectionState(&*(&hwprotectionenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetHardwareProtectionState<Impl: ID3D11DeviceContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHardwareProtectionState(::core::mem::transmute_copy(&phwprotectionenable)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DeviceContext3>, ::windows::core::GetTrustLevel, Flush1::<Impl, OFFSET>, SetHardwareProtectionState::<Impl, OFFSET>, GetHardwareProtectionState::<Impl, OFFSET>)
    }
}
pub trait ID3D11DeviceContext4Impl: Sized + ID3D11DeviceContext3Impl + ID3D11DeviceContext2Impl + ID3D11DeviceContext1Impl + ID3D11DeviceContextImpl + ID3D11DeviceChildImpl {
    fn Signal();
    fn Wait();
}
impl ::windows::core::RuntimeName for ID3D11DeviceContext4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext4";
}
impl ID3D11DeviceContext4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DeviceContext4Impl, const OFFSET: isize>() -> ID3D11DeviceContext4Vtbl {
        unsafe extern "system" fn Signal<Impl: ID3D11DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signal(&*(&pfence as *const <ID3D11Fence as ::windows::core::Abi>::Abi as *const <ID3D11Fence as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: ID3D11DeviceContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfence: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(&*(&pfence as *const <ID3D11Fence as ::windows::core::Abi>::Abi as *const <ID3D11Fence as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DeviceContext4>, ::windows::core::GetTrustLevel, Signal::<Impl, OFFSET>, Wait::<Impl, OFFSET>)
    }
}
pub trait ID3D11DomainShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11DomainShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11DomainShader";
}
impl ID3D11DomainShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11DomainShaderImpl, const OFFSET: isize>() -> ID3D11DomainShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11DomainShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11FenceImpl: Sized + ID3D11DeviceChildImpl {
    fn CreateSharedHandle();
    fn GetCompletedValue();
    fn SetEventOnCompletion();
}
impl ::windows::core::RuntimeName for ID3D11Fence {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Fence";
}
impl ID3D11FenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FenceImpl, const OFFSET: isize>() -> ID3D11FenceVtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(&*(&pattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType), dwaccess, &*(&lpname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompletedValue<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompletedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventOnCompletion<Impl: ID3D11FenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventOnCompletion(value, &*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Fence>, ::windows::core::GetTrustLevel, CreateSharedHandle::<Impl, OFFSET>, GetCompletedValue::<Impl, OFFSET>, SetEventOnCompletion::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11FunctionLinkingGraph {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11FunctionLinkingGraph";
}
impl ID3D11FunctionLinkingGraphVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>() -> ID3D11FunctionLinkingGraphVtbl {
        unsafe extern "system" fn CreateModuleInstance<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmoduleinstance: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateModuleInstance(::core::mem::transmute_copy(&ppmoduleinstance), ::core::mem::transmute_copy(&pperrorbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSignature<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInputSignature(&*(&pinputparameters as *const <D3D11_PARAMETER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_PARAMETER_DESC as ::windows::core::DefaultType>::DefaultType), cinputparameters, ::core::mem::transmute_copy(&ppinputnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSignature<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputSignature(&*(&poutputparameters as *const <D3D11_PARAMETER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_PARAMETER_DESC as ::windows::core::DefaultType>::DefaultType), coutputparameters, ::core::mem::transmute_copy(&ppoutputnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallFunction<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmoduleinstancenamespace: super::super::Foundation::PSTR, pmodulewithfunctionprototype: ::windows::core::RawPtr, pfunctionname: super::super::Foundation::PSTR, ppcallnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallFunction(
                &*(&pmoduleinstancenamespace as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pmodulewithfunctionprototype as *const <ID3D11Module as ::windows::core::Abi>::Abi as *const <ID3D11Module as ::windows::core::DefaultType>::DefaultType),
                &*(&pfunctionname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcallnode),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PassValue<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PassValue(&*(&psrcnode as *const <ID3D11LinkingNode as ::windows::core::Abi>::Abi as *const <ID3D11LinkingNode as ::windows::core::DefaultType>::DefaultType), srcparameterindex, &*(&pdstnode as *const <ID3D11LinkingNode as ::windows::core::Abi>::Abi as *const <ID3D11LinkingNode as ::windows::core::DefaultType>::DefaultType), dstparameterindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PassValueWithSwizzle<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcnode: ::windows::core::RawPtr, srcparameterindex: i32, psrcswizzle: super::super::Foundation::PSTR, pdstnode: ::windows::core::RawPtr, dstparameterindex: i32, pdstswizzle: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PassValueWithSwizzle(
                &*(&psrcnode as *const <ID3D11LinkingNode as ::windows::core::Abi>::Abi as *const <ID3D11LinkingNode as ::windows::core::DefaultType>::DefaultType),
                srcparameterindex,
                &*(&psrcswizzle as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdstnode as *const <ID3D11LinkingNode as ::windows::core::Abi>::Abi as *const <ID3D11LinkingNode as ::windows::core::DefaultType>::DefaultType),
                dstparameterindex,
                &*(&pdstswizzle as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastError<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(::core::mem::transmute_copy(&pperrorbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateHlsl<Impl: ID3D11FunctionLinkingGraphImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateHlsl(uflags, ::core::mem::transmute_copy(&ppbuffer)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11FunctionLinkingGraph>,
            ::windows::core::GetTrustLevel,
            CreateModuleInstance::<Impl, OFFSET>,
            SetInputSignature::<Impl, OFFSET>,
            SetOutputSignature::<Impl, OFFSET>,
            CallFunction::<Impl, OFFSET>,
            PassValue::<Impl, OFFSET>,
            PassValueWithSwizzle::<Impl, OFFSET>,
            GetLastError::<Impl, OFFSET>,
            GenerateHlsl::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11FunctionParameterReflectionImpl: Sized {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11FunctionParameterReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11FunctionParameterReflection";
}
impl ID3D11FunctionParameterReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionParameterReflectionImpl, const OFFSET: isize>() -> ID3D11FunctionParameterReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionParameterReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11FunctionParameterReflection>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11FunctionReflectionImpl: Sized {
    fn GetDesc();
    fn GetConstantBufferByIndex();
    fn GetConstantBufferByName();
    fn GetResourceBindingDesc();
    fn GetVariableByName();
    fn GetResourceBindingDescByName();
    fn GetFunctionParameter();
}
impl ::windows::core::RuntimeName for ID3D11FunctionReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11FunctionReflection";
}
impl ID3D11FunctionReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>() -> ID3D11FunctionReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(bufferindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D11FunctionReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionParameter(parameterindex) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11FunctionReflection>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetResourceBindingDescByName::<Impl, OFFSET>,
            GetFunctionParameter::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11GeometryShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11GeometryShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11GeometryShader";
}
impl ID3D11GeometryShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11GeometryShaderImpl, const OFFSET: isize>() -> ID3D11GeometryShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11GeometryShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11HullShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11HullShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11HullShader";
}
impl ID3D11HullShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11HullShaderImpl, const OFFSET: isize>() -> ID3D11HullShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11HullShader>, ::windows::core::GetTrustLevel)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11InfoQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11InfoQueue";
}
impl ID3D11InfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InfoQueueImpl, const OFFSET: isize>() -> ID3D11InfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageCountLimit(messagecountlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages().into()
        }
        unsafe extern "system" fn GetMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(messageindex, ::core::mem::transmute_copy(&pmessage), pmessagebytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesAllowedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDeniedByStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessagesAllowedByRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDiscardedByMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStorageFilterEntries(&*(&pfilter as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter().into()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfStorageFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushStorageFilter(&*(&pfilter as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopStorageFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter().into()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRetrievalFilterEntries(&*(&pfilter as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilter(::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter().into()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfRetrievalFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushRetrievalFilter(&*(&pfilter as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <D3D11_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter().into()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilterStackSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMessage(category, severity, id, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddApplicationMessage(severity, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnCategory(category, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnSeverity(severity, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnID<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnID(id, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnCategory(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnSeverity(severity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnID<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnID(id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(&*(&bmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: ID3D11InfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMuteDebugOutput() {
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
            ::windows::core::GetRuntimeClassName::<ID3D11InfoQueue>,
            ::windows::core::GetTrustLevel,
            SetMessageCountLimit::<Impl, OFFSET>,
            ClearStoredMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, OFFSET>,
            GetNumStoredMessages::<Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter::<Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, OFFSET>,
            GetMessageCountLimit::<Impl, OFFSET>,
            AddStorageFilterEntries::<Impl, OFFSET>,
            GetStorageFilter::<Impl, OFFSET>,
            ClearStorageFilter::<Impl, OFFSET>,
            PushEmptyStorageFilter::<Impl, OFFSET>,
            PushCopyOfStorageFilter::<Impl, OFFSET>,
            PushStorageFilter::<Impl, OFFSET>,
            PopStorageFilter::<Impl, OFFSET>,
            GetStorageFilterStackSize::<Impl, OFFSET>,
            AddRetrievalFilterEntries::<Impl, OFFSET>,
            GetRetrievalFilter::<Impl, OFFSET>,
            ClearRetrievalFilter::<Impl, OFFSET>,
            PushEmptyRetrievalFilter::<Impl, OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, OFFSET>,
            PushRetrievalFilter::<Impl, OFFSET>,
            PopRetrievalFilter::<Impl, OFFSET>,
            GetRetrievalFilterStackSize::<Impl, OFFSET>,
            AddMessage::<Impl, OFFSET>,
            AddApplicationMessage::<Impl, OFFSET>,
            SetBreakOnCategory::<Impl, OFFSET>,
            SetBreakOnSeverity::<Impl, OFFSET>,
            SetBreakOnID::<Impl, OFFSET>,
            GetBreakOnCategory::<Impl, OFFSET>,
            GetBreakOnSeverity::<Impl, OFFSET>,
            GetBreakOnID::<Impl, OFFSET>,
            SetMuteDebugOutput::<Impl, OFFSET>,
            GetMuteDebugOutput::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11InputLayoutImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11InputLayout {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11InputLayout";
}
impl ID3D11InputLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11InputLayoutImpl, const OFFSET: isize>() -> ID3D11InputLayoutVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11InputLayout>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11LibraryReflectionImpl: Sized {
    fn GetDesc();
    fn GetFunctionByIndex();
}
impl ::windows::core::RuntimeName for ID3D11LibraryReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11LibraryReflection";
}
impl ID3D11LibraryReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LibraryReflectionImpl, const OFFSET: isize>() -> ID3D11LibraryReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Impl: ID3D11LibraryReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionByIndex(functionindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11LibraryReflection>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetFunctionByIndex::<Impl, OFFSET>)
    }
}
pub trait ID3D11LinkerImpl: Sized {
    fn Link();
    fn UseLibrary();
    fn AddClipPlaneFromCBuffer();
}
impl ::windows::core::RuntimeName for ID3D11Linker {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Linker";
}
impl ID3D11LinkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkerImpl, const OFFSET: isize>() -> ID3D11LinkerVtbl {
        unsafe extern "system" fn Link<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentry: ::windows::core::RawPtr, pentryname: super::super::Foundation::PSTR, ptargetname: super::super::Foundation::PSTR, uflags: u32, ppshaderblob: *mut ::windows::core::RawPtr, pperrorbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(
                &*(&pentry as *const <ID3D11ModuleInstance as ::windows::core::Abi>::Abi as *const <ID3D11ModuleInstance as ::windows::core::DefaultType>::DefaultType),
                &*(&pentryname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptargetname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                uflags,
                ::core::mem::transmute_copy(&ppshaderblob),
                ::core::mem::transmute_copy(&pperrorbuffer),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseLibrary<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrarymi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseLibrary(&*(&plibrarymi as *const <ID3D11ModuleInstance as ::windows::core::Abi>::Abi as *const <ID3D11ModuleInstance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Impl: ID3D11LinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddClipPlaneFromCBuffer(ucbufferslot, ucbufferentry) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Linker>, ::windows::core::GetTrustLevel, Link::<Impl, OFFSET>, UseLibrary::<Impl, OFFSET>, AddClipPlaneFromCBuffer::<Impl, OFFSET>)
    }
}
pub trait ID3D11LinkingNodeImpl: Sized {}
impl ::windows::core::RuntimeName for ID3D11LinkingNode {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11LinkingNode";
}
impl ID3D11LinkingNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11LinkingNodeImpl, const OFFSET: isize>() -> ID3D11LinkingNodeVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11LinkingNode>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11ModuleImpl: Sized {
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for ID3D11Module {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Module";
}
impl ID3D11ModuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleImpl, const OFFSET: isize>() -> ID3D11ModuleVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ID3D11ModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: super::super::Foundation::PSTR, ppmoduleinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&pnamespace as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmoduleinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Module>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11ModuleInstance {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ModuleInstance";
}
impl ID3D11ModuleInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>() -> ID3D11ModuleInstanceVtbl {
        unsafe extern "system" fn BindConstantBuffer<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindConstantBuffer(usrcslot, udstslot, cbdstoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindConstantBufferByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, cbdstoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindConstantBufferByName(&*(&pname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), udstslot, cbdstoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindResource<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindResource(usrcslot, udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindResourceByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindResourceByName(&*(&pname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindSampler<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindSampler(usrcslot, udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindSamplerByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindSamplerByName(&*(&pname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindUnorderedAccessView<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindUnorderedAccessView(usrcslot, udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PSTR, udstslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindUnorderedAccessViewByName(&*(&pname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), udstslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindResourceAsUnorderedAccessView(usrcsrvslot, udstuavslot, ucount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Impl: ID3D11ModuleInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrvname: super::super::Foundation::PSTR, udstuavslot: u32, ucount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindResourceAsUnorderedAccessViewByName(&*(&psrvname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), udstuavslot, ucount) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11ModuleInstance>,
            ::windows::core::GetTrustLevel,
            BindConstantBuffer::<Impl, OFFSET>,
            BindConstantBufferByName::<Impl, OFFSET>,
            BindResource::<Impl, OFFSET>,
            BindResourceByName::<Impl, OFFSET>,
            BindSampler::<Impl, OFFSET>,
            BindSamplerByName::<Impl, OFFSET>,
            BindUnorderedAccessView::<Impl, OFFSET>,
            BindUnorderedAccessViewByName::<Impl, OFFSET>,
            BindResourceAsUnorderedAccessView::<Impl, OFFSET>,
            BindResourceAsUnorderedAccessViewByName::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11MultithreadImpl: Sized {
    fn Enter();
    fn Leave();
    fn SetMultithreadProtected();
    fn GetMultithreadProtected();
}
impl ::windows::core::RuntimeName for ID3D11Multithread {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Multithread";
}
impl ID3D11MultithreadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11MultithreadImpl, const OFFSET: isize>() -> ID3D11MultithreadVtbl {
        unsafe extern "system" fn Enter<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enter().into()
        }
        unsafe extern "system" fn Leave<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Leave().into()
        }
        unsafe extern "system" fn SetMultithreadProtected<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultithreadProtected(&*(&bmtprotect as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMultithreadProtected<Impl: ID3D11MultithreadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMultithreadProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Multithread>, ::windows::core::GetTrustLevel, Enter::<Impl, OFFSET>, Leave::<Impl, OFFSET>, SetMultithreadProtected::<Impl, OFFSET>, GetMultithreadProtected::<Impl, OFFSET>)
    }
}
pub trait ID3D11PixelShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11PixelShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11PixelShader";
}
impl ID3D11PixelShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PixelShaderImpl, const OFFSET: isize>() -> ID3D11PixelShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11PixelShader>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11PredicateImpl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11Predicate {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Predicate";
}
impl ID3D11PredicateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11PredicateImpl, const OFFSET: isize>() -> ID3D11PredicateVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Predicate>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3D11QueryImpl: Sized + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Query {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Query";
}
impl ID3D11QueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11QueryImpl, const OFFSET: isize>() -> ID3D11QueryVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11QueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Query>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11Query1Impl: Sized + ID3D11QueryImpl + ID3D11AsynchronousImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11Query1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Query1";
}
impl ID3D11Query1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Query1Impl, const OFFSET: isize>() -> ID3D11Query1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Query1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Query1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11RasterizerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11RasterizerState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RasterizerState";
}
impl ID3D11RasterizerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerStateImpl, const OFFSET: isize>() -> ID3D11RasterizerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RasterizerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RasterizerState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11RasterizerState1Impl: Sized + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11RasterizerState1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RasterizerState1";
}
impl ID3D11RasterizerState1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState1Impl, const OFFSET: isize>() -> ID3D11RasterizerState1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RasterizerState1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RasterizerState1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11RasterizerState2Impl: Sized + ID3D11RasterizerState1Impl + ID3D11RasterizerStateImpl + ID3D11DeviceChildImpl {
    fn GetDesc2();
}
impl ::windows::core::RuntimeName for ID3D11RasterizerState2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RasterizerState2";
}
impl ID3D11RasterizerState2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RasterizerState2Impl, const OFFSET: isize>() -> ID3D11RasterizerState2Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: ID3D11RasterizerState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc2(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RasterizerState2>, ::windows::core::GetTrustLevel, GetDesc2::<Impl, OFFSET>)
    }
}
pub trait ID3D11RefDefaultTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
impl ::windows::core::RuntimeName for ID3D11RefDefaultTrackingOptions {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RefDefaultTrackingOptions";
}
impl ID3D11RefDefaultTrackingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefDefaultTrackingOptionsImpl, const OFFSET: isize>() -> ID3D11RefDefaultTrackingOptionsVtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefDefaultTrackingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrackingOptions(resourcetypeflags, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RefDefaultTrackingOptions>, ::windows::core::GetTrustLevel, SetTrackingOptions::<Impl, OFFSET>)
    }
}
pub trait ID3D11RefTrackingOptionsImpl: Sized {
    fn SetTrackingOptions();
}
impl ::windows::core::RuntimeName for ID3D11RefTrackingOptions {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RefTrackingOptions";
}
impl ID3D11RefTrackingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RefTrackingOptionsImpl, const OFFSET: isize>() -> ID3D11RefTrackingOptionsVtbl {
        unsafe extern "system" fn SetTrackingOptions<Impl: ID3D11RefTrackingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrackingOptions(uoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RefTrackingOptions>, ::windows::core::GetTrustLevel, SetTrackingOptions::<Impl, OFFSET>)
    }
}
pub trait ID3D11RenderTargetViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11RenderTargetView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RenderTargetView";
}
impl ID3D11RenderTargetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetViewImpl, const OFFSET: isize>() -> ID3D11RenderTargetViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11RenderTargetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RenderTargetView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11RenderTargetView1Impl: Sized + ID3D11RenderTargetViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11RenderTargetView1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11RenderTargetView1";
}
impl ID3D11RenderTargetView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11RenderTargetView1Impl, const OFFSET: isize>() -> ID3D11RenderTargetView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11RenderTargetView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11RenderTargetView1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11ResourceImpl: Sized + ID3D11DeviceChildImpl {
    fn GetType();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
impl ::windows::core::RuntimeName for ID3D11Resource {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Resource";
}
impl ID3D11ResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ResourceImpl, const OFFSET: isize>() -> ID3D11ResourceVtbl {
        unsafe extern "system" fn GetType<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&presourcedimension)).into()
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvictionPriority(evictionpriority).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: ID3D11ResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvictionPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Resource>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>, SetEvictionPriority::<Impl, OFFSET>, GetEvictionPriority::<Impl, OFFSET>)
    }
}
pub trait ID3D11SamplerStateImpl: Sized + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11SamplerState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11SamplerState";
}
impl ID3D11SamplerStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SamplerStateImpl, const OFFSET: isize>() -> ID3D11SamplerStateVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11SamplerStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11SamplerState>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11ShaderReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderReflection";
}
impl ID3D11ShaderReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>() -> ID3D11ShaderReflectionVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConstantBufferByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDesc(resourceindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatchConstantParameterDesc(parameterindex, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceBindingDescByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMovcInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitwiseInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGSInputPrimitive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSampleFrequencyShader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumInterfaceSlots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinFeatureLevel<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinFeatureLevel(::core::mem::transmute_copy(&plevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadGroupSize(::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequiresFlags<Impl: ID3D11ShaderReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiresFlags() {
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
            ::windows::core::GetRuntimeClassName::<ID3D11ShaderReflection>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetConstantBufferByIndex::<Impl, OFFSET>,
            GetConstantBufferByName::<Impl, OFFSET>,
            GetResourceBindingDesc::<Impl, OFFSET>,
            GetInputParameterDesc::<Impl, OFFSET>,
            GetOutputParameterDesc::<Impl, OFFSET>,
            GetPatchConstantParameterDesc::<Impl, OFFSET>,
            GetVariableByName::<Impl, OFFSET>,
            GetResourceBindingDescByName::<Impl, OFFSET>,
            GetMovInstructionCount::<Impl, OFFSET>,
            GetMovcInstructionCount::<Impl, OFFSET>,
            GetConversionInstructionCount::<Impl, OFFSET>,
            GetBitwiseInstructionCount::<Impl, OFFSET>,
            GetGSInputPrimitive::<Impl, OFFSET>,
            IsSampleFrequencyShader::<Impl, OFFSET>,
            GetNumInterfaceSlots::<Impl, OFFSET>,
            GetMinFeatureLevel::<Impl, OFFSET>,
            GetThreadGroupSize::<Impl, OFFSET>,
            GetRequiresFlags::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11ShaderReflectionConstantBufferImpl: Sized {
    fn GetDesc();
    fn GetVariableByIndex();
    fn GetVariableByName();
}
impl ::windows::core::RuntimeName for ID3D11ShaderReflectionConstantBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderReflectionConstantBuffer";
}
impl ID3D11ShaderReflectionConstantBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>() -> ID3D11ShaderReflectionConstantBufferVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(&*(&pdesc as *const <D3D11_SHADER_BUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_SHADER_BUFFER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionConstantBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVariableByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ShaderReflectionConstantBuffer>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetVariableByIndex::<Impl, OFFSET>, GetVariableByName::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11ShaderReflectionType {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderReflectionType";
}
impl ID3D11ShaderReflectionTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>() -> ID3D11ShaderReflectionTypeVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeByName(&*(&name as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemberTypeName(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&ptype as *const <ID3D11ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D11ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceByIndex(uindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOfType(&*(&ptype as *const <ID3D11ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D11ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D11ShaderReflectionTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImplementsInterface(&*(&pbase as *const <ID3D11ShaderReflectionType as ::windows::core::Abi>::Abi as *const <ID3D11ShaderReflectionType as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11ShaderReflectionType>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetMemberTypeByIndex::<Impl, OFFSET>,
            GetMemberTypeByName::<Impl, OFFSET>,
            GetMemberTypeName::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            GetSubType::<Impl, OFFSET>,
            GetBaseClass::<Impl, OFFSET>,
            GetNumInterfaces::<Impl, OFFSET>,
            GetInterfaceByIndex::<Impl, OFFSET>,
            IsOfType::<Impl, OFFSET>,
            ImplementsInterface::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11ShaderReflectionVariableImpl: Sized {
    fn GetDesc();
    fn GetType();
    fn GetBuffer();
    fn GetInterfaceSlot();
}
impl ::windows::core::RuntimeName for ID3D11ShaderReflectionVariable {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderReflectionVariable";
}
impl ID3D11ShaderReflectionVariableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>() -> ID3D11ShaderReflectionVariableVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D11ShaderReflectionVariableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceSlot(uarrayindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ShaderReflectionVariable>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetBuffer::<Impl, OFFSET>, GetInterfaceSlot::<Impl, OFFSET>)
    }
}
pub trait ID3D11ShaderResourceViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11ShaderResourceView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderResourceView";
}
impl ID3D11ShaderResourceViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceViewImpl, const OFFSET: isize>() -> ID3D11ShaderResourceViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderResourceViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ShaderResourceView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11ShaderResourceView1Impl: Sized + ID3D11ShaderResourceViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11ShaderResourceView1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderResourceView1";
}
impl ID3D11ShaderResourceView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderResourceView1Impl, const OFFSET: isize>() -> ID3D11ShaderResourceView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11ShaderResourceView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ShaderResourceView1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11ShaderTrace {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderTrace";
}
impl ID3D11ShaderTraceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>() -> ID3D11ShaderTraceVtbl {
        unsafe extern "system" fn TraceReady<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptestcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TraceReady(::core::mem::transmute_copy(&ptestcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetTrace<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTrace().into()
        }
        unsafe extern "system" fn GetTraceStats<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTraceStats(::core::mem::transmute_copy(&ptracestats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PSSelectStamp<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stampindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PSSelectStamp(stampindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitialRegisterContents<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInitialRegisterContents(&*(&pregister as *const <D3D11_TRACE_REGISTER as ::windows::core::Abi>::Abi as *const <D3D11_TRACE_REGISTER as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStep<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStep(stepindex, ::core::mem::transmute_copy(&ptracestep)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWrittenRegister<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWrittenRegister(stepindex, writtenregisterindex, ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadRegister<Impl: ID3D11ShaderTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadRegister(stepindex, readregisterindex, ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11ShaderTrace>,
            ::windows::core::GetTrustLevel,
            TraceReady::<Impl, OFFSET>,
            ResetTrace::<Impl, OFFSET>,
            GetTraceStats::<Impl, OFFSET>,
            PSSelectStamp::<Impl, OFFSET>,
            GetInitialRegisterContents::<Impl, OFFSET>,
            GetStep::<Impl, OFFSET>,
            GetWrittenRegister::<Impl, OFFSET>,
            GetReadRegister::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11ShaderTraceFactoryImpl: Sized {
    fn CreateShaderTrace();
}
impl ::windows::core::RuntimeName for ID3D11ShaderTraceFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11ShaderTraceFactory";
}
impl ID3D11ShaderTraceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ShaderTraceFactoryImpl, const OFFSET: isize>() -> ID3D11ShaderTraceFactoryVtbl {
        unsafe extern "system" fn CreateShaderTrace<Impl: ID3D11ShaderTraceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateShaderTrace(&*(&pshader as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&ptracedesc as *const <D3D11_SHADER_TRACE_DESC as ::windows::core::Abi>::Abi as *const <D3D11_SHADER_TRACE_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppshadertrace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11ShaderTraceFactory>, ::windows::core::GetTrustLevel, CreateShaderTrace::<Impl, OFFSET>)
    }
}
pub trait ID3D11SwitchToRefImpl: Sized {
    fn SetUseRef();
    fn GetUseRef();
}
impl ::windows::core::RuntimeName for ID3D11SwitchToRef {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11SwitchToRef";
}
impl ID3D11SwitchToRefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11SwitchToRefImpl, const OFFSET: isize>() -> ID3D11SwitchToRefVtbl {
        unsafe extern "system" fn SetUseRef<Impl: ID3D11SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseRef(&*(&useref as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseRef<Impl: ID3D11SwitchToRefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseRef() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11SwitchToRef>, ::windows::core::GetTrustLevel, SetUseRef::<Impl, OFFSET>, GetUseRef::<Impl, OFFSET>)
    }
}
pub trait ID3D11Texture1DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Texture1D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Texture1D";
}
impl ID3D11Texture1DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture1DImpl, const OFFSET: isize>() -> ID3D11Texture1DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture1DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Texture1D>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11Texture2DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Texture2D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Texture2D";
}
impl ID3D11Texture2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2DImpl, const OFFSET: isize>() -> ID3D11Texture2DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Texture2D>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11Texture2D1Impl: Sized + ID3D11Texture2DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11Texture2D1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Texture2D1";
}
impl ID3D11Texture2D1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture2D1Impl, const OFFSET: isize>() -> ID3D11Texture2D1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture2D1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Texture2D1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11Texture3DImpl: Sized + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11Texture3D {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Texture3D";
}
impl ID3D11Texture3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3DImpl, const OFFSET: isize>() -> ID3D11Texture3DVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11Texture3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Texture3D>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11Texture3D1Impl: Sized + ID3D11Texture3DImpl + ID3D11ResourceImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11Texture3D1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11Texture3D1";
}
impl ID3D11Texture3D1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11Texture3D1Impl, const OFFSET: isize>() -> ID3D11Texture3D1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11Texture3D1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11Texture3D1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11TracingDeviceImpl: Sized {
    fn SetShaderTrackingOptionsByType();
    fn SetShaderTrackingOptions();
}
impl ::windows::core::RuntimeName for ID3D11TracingDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11TracingDevice";
}
impl ID3D11TracingDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11TracingDeviceImpl, const OFFSET: isize>() -> ID3D11TracingDeviceVtbl {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Impl: ID3D11TracingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetShaderTrackingOptionsByType(resourcetypeflags, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Impl: ID3D11TracingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, options: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetShaderTrackingOptions(&*(&pshader as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11TracingDevice>, ::windows::core::GetTrustLevel, SetShaderTrackingOptionsByType::<Impl, OFFSET>, SetShaderTrackingOptions::<Impl, OFFSET>)
    }
}
pub trait ID3D11UnorderedAccessViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11UnorderedAccessView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11UnorderedAccessView";
}
impl ID3D11UnorderedAccessViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessViewImpl, const OFFSET: isize>() -> ID3D11UnorderedAccessViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11UnorderedAccessViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11UnorderedAccessView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11UnorderedAccessView1Impl: Sized + ID3D11UnorderedAccessViewImpl + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for ID3D11UnorderedAccessView1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11UnorderedAccessView1";
}
impl ID3D11UnorderedAccessView1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11UnorderedAccessView1Impl, const OFFSET: isize>() -> ID3D11UnorderedAccessView1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: ID3D11UnorderedAccessView1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc1(::core::mem::transmute_copy(&pdesc1)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11UnorderedAccessView1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait ID3D11VertexShaderImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3D11VertexShader {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VertexShader";
}
impl ID3D11VertexShaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VertexShaderImpl, const OFFSET: isize>() -> ID3D11VertexShaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VertexShader>, ::windows::core::GetTrustLevel)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11VideoContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoContext";
}
impl ID3D11VideoContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContextImpl, const OFFSET: isize>() -> ID3D11VideoContextVtbl {
        unsafe extern "system" fn GetDecoderBuffer<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDecoderBuffer(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&pbuffersize), ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDecoderBuffer(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderBeginFrame<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderBeginFrame(
                &*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType),
                &*(&pview as *const <ID3D11VideoDecoderOutputView as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoderOutputView as ::windows::core::DefaultType>::DefaultType),
                contentkeysize,
                &*(&pcontentkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderEndFrame<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderEndFrame(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitDecoderBuffers(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), numbuffers, &*(&pbufferdesc as *const <D3D11_VIDEO_DECODER_BUFFER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_BUFFER_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderExtension(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), &*(&pextensiondata as *const <D3D11_VIDEO_DECODER_EXTENSION as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_EXTENSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetOutputTargetRect(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetOutputBackgroundColor(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), &*(&ycbcr as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pcolor as *const <D3D11_VIDEO_COLOR as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_COLOR as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), &*(&pcolorspace as *const <D3D11_VIDEO_PROCESSOR_COLOR_SPACE as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_COLOR_SPACE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputAlphaFillMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), alphafillmode, streamindex).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetOutputConstriction(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&size as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputStereoMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorSetOutputExtension(
                &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                &*(&pextensionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                datasize,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputTargetRect(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&enabled), ::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputBackgroundColor(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pycbcr), ::core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcolorspace)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputAlphaFillMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&palphafillmode), ::core::mem::transmute_copy(&pstreamindex)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputConstriction(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputStereoMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorGetOutputExtension(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), &*(&pextensionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamFrameFormat(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, frameformat).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&pcolorspace as *const <D3D11_VIDEO_PROCESSOR_COLOR_SPACE as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_COLOR_SPACE as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamOutputRate(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    outputrate,
                    &*(&repeatframe as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&pcustomrate as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamSourceRect(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamDestRect(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamAlpha(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), alpha).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *const u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamPalette(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, count, pentries).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamPixelAspectRatio(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&psourceaspectratio as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdestinationaspectratio as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamLumaKey(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), lower, upper).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamStereoFormat(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    format,
                    &*(&leftviewframe0 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&baseviewframe0 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    flipmode,
                    monooffset,
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamAutoProcessingMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamFilter(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, filter, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorSetStreamExtension(
                &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                streamindex,
                &*(&pextensionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                datasize,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamFrameFormat(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&pframeformat)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&pcolorspace)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamOutputRate(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&poutputrate), ::core::mem::transmute_copy(&prepeatframe), ::core::mem::transmute_copy(&pcustomrate)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamSourceRect(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamDestRect(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamAlpha(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&palpha)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, count: u32, pentries: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamPalette(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, count, ::core::mem::transmute_copy(&pentries)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamPixelAspectRatio(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamLumaKey(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plower), ::core::mem::transmute_copy(&pupper)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamStereoFormat(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pleftviewframe0), ::core::mem::transmute_copy(&pbaseviewframe0), ::core::mem::transmute_copy(&pflipmode), ::core::mem::transmute_copy(&monooffset)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamAutoProcessingMode(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamFilter(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, filter, ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorGetStreamExtension(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&pextensionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorBlt(
                &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                &*(&pview as *const <ID3D11VideoProcessorOutputView as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessorOutputView as ::windows::core::DefaultType>::DefaultType),
                outputframe,
                streamcount,
                &*(&pstreams as *const <D3D11_VIDEO_PROCESSOR_STREAM as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_STREAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateCryptoSessionKeyExchange(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .EncryptionBlt(
                    &*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType),
                    &*(&psrcsurface as *const <ID3D11Texture2D as ::windows::core::Abi>::Abi as *const <ID3D11Texture2D as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdstsurface as *const <ID3D11Texture2D as ::windows::core::Abi>::Abi as *const <ID3D11Texture2D as ::windows::core::DefaultType>::DefaultType),
                    ivsize,
                    &*(&piv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn DecryptionBlt<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, psrcsurface: ::windows::core::RawPtr, pdstsurface: ::windows::core::RawPtr, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .DecryptionBlt(
                    &*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType),
                    &*(&psrcsurface as *const <ID3D11Texture2D as ::windows::core::Abi>::Abi as *const <ID3D11Texture2D as ::windows::core::DefaultType>::DefaultType),
                    &*(&pdstsurface as *const <ID3D11Texture2D as ::windows::core::Abi>::Abi as *const <ID3D11Texture2D as ::windows::core::DefaultType>::DefaultType),
                    &*(&pencryptedblockinfo as *const <D3D11_ENCRYPTED_BLOCK_INFO as ::windows::core::Abi>::Abi as *const <D3D11_ENCRYPTED_BLOCK_INFO as ::windows::core::DefaultType>::DefaultType),
                    contentkeysize,
                    &*(&pcontentkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    ivsize,
                    &*(&piv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSessionKeyRefresh(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), randomnumbersize, ::core::mem::transmute_copy(&prandomnumber)).into()
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishSessionKeyRefresh(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncryptionBltKey<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncryptionBltKey(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), keysize, ::core::mem::transmute_copy(&preadbackkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateAuthenticatedChannelKeyExchange(&*(&pchannel as *const <ID3D11AuthenticatedChannel as ::windows::core::Abi>::Abi as *const <ID3D11AuthenticatedChannel as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAuthenticatedChannel(&*(&pchannel as *const <ID3D11AuthenticatedChannel as ::windows::core::Abi>::Abi as *const <ID3D11AuthenticatedChannel as ::windows::core::DefaultType>::DefaultType), inputsize, &*(&pinput as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), outputsize, ::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureAuthenticatedChannel(&*(&pchannel as *const <ID3D11AuthenticatedChannel as ::windows::core::Abi>::Abi as *const <ID3D11AuthenticatedChannel as ::windows::core::DefaultType>::DefaultType), inputsize, &*(&pinput as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamRotation(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), rotation).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Impl: ID3D11VideoContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamRotation(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&protation)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ID3D11VideoContext>,
            ::windows::core::GetTrustLevel,
            GetDecoderBuffer::<Impl, OFFSET>,
            ReleaseDecoderBuffer::<Impl, OFFSET>,
            DecoderBeginFrame::<Impl, OFFSET>,
            DecoderEndFrame::<Impl, OFFSET>,
            SubmitDecoderBuffers::<Impl, OFFSET>,
            DecoderExtension::<Impl, OFFSET>,
            VideoProcessorSetOutputTargetRect::<Impl, OFFSET>,
            VideoProcessorSetOutputBackgroundColor::<Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace::<Impl, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode::<Impl, OFFSET>,
            VideoProcessorSetOutputConstriction::<Impl, OFFSET>,
            VideoProcessorSetOutputStereoMode::<Impl, OFFSET>,
            VideoProcessorSetOutputExtension::<Impl, OFFSET>,
            VideoProcessorGetOutputTargetRect::<Impl, OFFSET>,
            VideoProcessorGetOutputBackgroundColor::<Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace::<Impl, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode::<Impl, OFFSET>,
            VideoProcessorGetOutputConstriction::<Impl, OFFSET>,
            VideoProcessorGetOutputStereoMode::<Impl, OFFSET>,
            VideoProcessorGetOutputExtension::<Impl, OFFSET>,
            VideoProcessorSetStreamFrameFormat::<Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace::<Impl, OFFSET>,
            VideoProcessorSetStreamOutputRate::<Impl, OFFSET>,
            VideoProcessorSetStreamSourceRect::<Impl, OFFSET>,
            VideoProcessorSetStreamDestRect::<Impl, OFFSET>,
            VideoProcessorSetStreamAlpha::<Impl, OFFSET>,
            VideoProcessorSetStreamPalette::<Impl, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio::<Impl, OFFSET>,
            VideoProcessorSetStreamLumaKey::<Impl, OFFSET>,
            VideoProcessorSetStreamStereoFormat::<Impl, OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode::<Impl, OFFSET>,
            VideoProcessorSetStreamFilter::<Impl, OFFSET>,
            VideoProcessorSetStreamExtension::<Impl, OFFSET>,
            VideoProcessorGetStreamFrameFormat::<Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace::<Impl, OFFSET>,
            VideoProcessorGetStreamOutputRate::<Impl, OFFSET>,
            VideoProcessorGetStreamSourceRect::<Impl, OFFSET>,
            VideoProcessorGetStreamDestRect::<Impl, OFFSET>,
            VideoProcessorGetStreamAlpha::<Impl, OFFSET>,
            VideoProcessorGetStreamPalette::<Impl, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio::<Impl, OFFSET>,
            VideoProcessorGetStreamLumaKey::<Impl, OFFSET>,
            VideoProcessorGetStreamStereoFormat::<Impl, OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode::<Impl, OFFSET>,
            VideoProcessorGetStreamFilter::<Impl, OFFSET>,
            VideoProcessorGetStreamExtension::<Impl, OFFSET>,
            VideoProcessorBlt::<Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchange::<Impl, OFFSET>,
            EncryptionBlt::<Impl, OFFSET>,
            DecryptionBlt::<Impl, OFFSET>,
            StartSessionKeyRefresh::<Impl, OFFSET>,
            FinishSessionKeyRefresh::<Impl, OFFSET>,
            GetEncryptionBltKey::<Impl, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange::<Impl, OFFSET>,
            QueryAuthenticatedChannel::<Impl, OFFSET>,
            ConfigureAuthenticatedChannel::<Impl, OFFSET>,
            VideoProcessorSetStreamRotation::<Impl, OFFSET>,
            VideoProcessorGetStreamRotation::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ID3D11VideoContext1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoContext1";
}
impl ID3D11VideoContext1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext1Impl, const OFFSET: isize>() -> ID3D11VideoContext1Vtbl {
        unsafe extern "system" fn SubmitDecoderBuffers1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitDecoderBuffers1(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), numbuffers, &*(&pbufferdesc as *const <D3D11_VIDEO_DECODER_BUFFER_DESC1 as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_BUFFER_DESC1 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void, pprivateoutputdata: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataForNewHardwareKey(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), privateinputsize, &*(&pprivatinputdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprivateoutputdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCryptoSessionStatus(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderEnableDownsampling(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), inputcolorspace, &*(&poutputdesc as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::DefaultType>::DefaultType), referenceframecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderUpdateDownsampling(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), &*(&poutputdesc as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputColorSpace1(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), colorspace).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, shaderusage: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputShaderUsage(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), &*(&shaderusage as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputColorSpace1(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcolorspace)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, pshaderusage: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputShaderUsage(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pshaderusage)).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamColorSpace1(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, colorspace).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .VideoProcessorSetStreamMirror(
                    &*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType),
                    streamindex,
                    &*(&enable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&fliphorizontal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                    &*(&flipvertical as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamColorSpace1(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&pcolorspace)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamMirror(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pfliphorizontal), ::core::mem::transmute_copy(&pflipvertical)).into()
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Impl: ID3D11VideoContext1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessorGetBehaviorHints(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), outputwidth, outputheight, outputformat, streamcount, &*(&pstreams as *const <D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbehaviorhints)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11VideoContext1>,
            ::windows::core::GetTrustLevel,
            SubmitDecoderBuffers1::<Impl, OFFSET>,
            GetDataForNewHardwareKey::<Impl, OFFSET>,
            CheckCryptoSessionStatus::<Impl, OFFSET>,
            DecoderEnableDownsampling::<Impl, OFFSET>,
            DecoderUpdateDownsampling::<Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace1::<Impl, OFFSET>,
            VideoProcessorSetOutputShaderUsage::<Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace1::<Impl, OFFSET>,
            VideoProcessorGetOutputShaderUsage::<Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace1::<Impl, OFFSET>,
            VideoProcessorSetStreamMirror::<Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace1::<Impl, OFFSET>,
            VideoProcessorGetStreamMirror::<Impl, OFFSET>,
            VideoProcessorGetBehaviorHints::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11VideoContext2Impl: Sized + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn VideoProcessorSetOutputHDRMetaData();
    fn VideoProcessorGetOutputHDRMetaData();
    fn VideoProcessorSetStreamHDRMetaData();
    fn VideoProcessorGetStreamHDRMetaData();
}
impl ::windows::core::RuntimeName for ID3D11VideoContext2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoContext2";
}
impl ID3D11VideoContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext2Impl, const OFFSET: isize>() -> ID3D11VideoContext2Vtbl {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetOutputHDRMetaData(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), r#type, size, &*(&phdrmetadata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetOutputHDRMetaData(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), size, ::core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorSetStreamHDRMetaData(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, r#type, size, &*(&phdrmetadata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Impl: ID3D11VideoContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideoprocessor: ::windows::core::RawPtr, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VideoProcessorGetStreamHDRMetaData(&*(&pvideoprocessor as *const <ID3D11VideoProcessor as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessor as ::windows::core::DefaultType>::DefaultType), streamindex, ::core::mem::transmute_copy(&ptype), size, ::core::mem::transmute_copy(&pmetadata)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoContext2>, ::windows::core::GetTrustLevel, VideoProcessorSetOutputHDRMetaData::<Impl, OFFSET>, VideoProcessorGetOutputHDRMetaData::<Impl, OFFSET>, VideoProcessorSetStreamHDRMetaData::<Impl, OFFSET>, VideoProcessorGetStreamHDRMetaData::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoContext3Impl: Sized + ID3D11VideoContext2Impl + ID3D11VideoContext1Impl + ID3D11VideoContextImpl + ID3D11DeviceChildImpl {
    fn DecoderBeginFrame1();
    fn SubmitDecoderBuffers2();
}
impl ::windows::core::RuntimeName for ID3D11VideoContext3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoContext3";
}
impl ID3D11VideoContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoContext3Impl, const OFFSET: isize>() -> ID3D11VideoContext3Vtbl {
        unsafe extern "system" fn DecoderBeginFrame1<Impl: ID3D11VideoContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, pview: ::windows::core::RawPtr, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderBeginFrame1(
                &*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType),
                &*(&pview as *const <ID3D11VideoDecoderOutputView as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoderOutputView as ::windows::core::DefaultType>::DefaultType),
                contentkeysize,
                &*(&pcontentkey as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                numcomponenthistograms,
                phistogramoffsets,
                &*(&pphistogrambuffers as *const <ID3D11Buffer as ::windows::core::Abi>::Abi as *const <ID3D11Buffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Impl: ID3D11VideoContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoder: ::windows::core::RawPtr, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitDecoderBuffers2(&*(&pdecoder as *const <ID3D11VideoDecoder as ::windows::core::Abi>::Abi as *const <ID3D11VideoDecoder as ::windows::core::DefaultType>::DefaultType), numbuffers, &*(&pbufferdesc as *const <D3D11_VIDEO_DECODER_BUFFER_DESC2 as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_BUFFER_DESC2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoContext3>, ::windows::core::GetTrustLevel, DecoderBeginFrame1::<Impl, OFFSET>, SubmitDecoderBuffers2::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoDecoderImpl: Sized + ID3D11DeviceChildImpl {
    fn GetCreationParameters();
    fn GetDriverHandle();
}
impl ::windows::core::RuntimeName for ID3D11VideoDecoder {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDecoder";
}
impl ID3D11VideoDecoderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderImpl, const OFFSET: isize>() -> ID3D11VideoDecoderVtbl {
        unsafe extern "system" fn GetCreationParameters<Impl: ID3D11VideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreationParameters(::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDriverHandle<Impl: ID3D11VideoDecoderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDriverHandle(::core::mem::transmute_copy(&pdriverhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoDecoder>, ::windows::core::GetTrustLevel, GetCreationParameters::<Impl, OFFSET>, GetDriverHandle::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoDecoderOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11VideoDecoderOutputView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDecoderOutputView";
}
impl ID3D11VideoDecoderOutputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDecoderOutputViewImpl, const OFFSET: isize>() -> ID3D11VideoDecoderOutputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoDecoderOutputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoDecoderOutputView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ID3D11VideoDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDevice";
}
impl ID3D11VideoDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>() -> ID3D11VideoDeviceVtbl {
        unsafe extern "system" fn CreateVideoDecoder<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoDecoder(&*(&pvideodesc as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::DefaultType>::DefaultType), &*(&pconfig as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penum: ::windows::core::RawPtr, rateconversionindex: u32, ppvideoprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessor(&*(&penum as *const <ID3D11VideoProcessorEnumerator as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessorEnumerator as ::windows::core::DefaultType>::DefaultType), rateconversionindex, ::core::mem::transmute_copy(&ppvideoprocessor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAuthenticatedChannel(channeltype, ::core::mem::transmute_copy(&ppauthenticatedchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, ppcryptosession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCryptoSession(
                &*(&pcryptotype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pkeyexchangetype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppcryptosession),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoDecoderOutputView(&*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvdovview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorInputView(
                &*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&penum as *const <ID3D11VideoProcessorEnumerator as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessorEnumerator as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvpiview),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorOutputView(
                &*(&presource as *const <ID3D11Resource as ::windows::core::Abi>::Abi as *const <ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&penum as *const <ID3D11VideoProcessorEnumerator as ::windows::core::Abi>::Abi as *const <ID3D11VideoProcessorEnumerator as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvpoview),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVideoProcessorEnumerator(&*(&pdesc as *const <D3D11_VIDEO_PROCESSOR_CONTENT_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_PROCESSOR_CONTENT_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderProfileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pdecoderprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderProfile(index, ::core::mem::transmute_copy(&pdecoderprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoDecoderFormat(&*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), format, ::core::mem::transmute_copy(&psupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderConfigCount(&*(&pdesc as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderConfig(&*(&pdesc as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::DefaultType>::DefaultType), index, ::core::mem::transmute_copy(&pconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentProtectionCaps(&*(&pcryptotype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, index: u32, pkeyexchangetype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCryptoKeyExchange(&*(&pcryptotype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), index, ::core::mem::transmute_copy(&pkeyexchangetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: ID3D11VideoDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11VideoDevice>,
            ::windows::core::GetTrustLevel,
            CreateVideoDecoder::<Impl, OFFSET>,
            CreateVideoProcessor::<Impl, OFFSET>,
            CreateAuthenticatedChannel::<Impl, OFFSET>,
            CreateCryptoSession::<Impl, OFFSET>,
            CreateVideoDecoderOutputView::<Impl, OFFSET>,
            CreateVideoProcessorInputView::<Impl, OFFSET>,
            CreateVideoProcessorOutputView::<Impl, OFFSET>,
            CreateVideoProcessorEnumerator::<Impl, OFFSET>,
            GetVideoDecoderProfileCount::<Impl, OFFSET>,
            GetVideoDecoderProfile::<Impl, OFFSET>,
            CheckVideoDecoderFormat::<Impl, OFFSET>,
            GetVideoDecoderConfigCount::<Impl, OFFSET>,
            GetVideoDecoderConfig::<Impl, OFFSET>,
            GetContentProtectionCaps::<Impl, OFFSET>,
            CheckCryptoKeyExchange::<Impl, OFFSET>,
            SetPrivateData::<Impl, OFFSET>,
            SetPrivateDataInterface::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11VideoDevice1Impl: Sized + ID3D11VideoDeviceImpl {
    fn GetCryptoSessionPrivateDataSize();
    fn GetVideoDecoderCaps();
    fn CheckVideoDecoderDownsampling();
    fn RecommendVideoDecoderDownsampleParameters();
}
impl ::windows::core::RuntimeName for ID3D11VideoDevice1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDevice1";
}
impl ID3D11VideoDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>() -> ID3D11VideoDevice1Vtbl {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: *const ::windows::core::GUID, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCryptoSessionPrivateDataSize(
                &*(&pcryptotype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pkeyexchangetype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprivateinputsize),
                ::core::mem::transmute_copy(&pprivateoutputsize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows::core::GUID, pdecodercaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoDecoderCaps(
                &*(&pdecoderprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                samplewidth,
                sampleheight,
                &*(&pframerate as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                bitrate,
                &*(&pcryptotype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdecodercaps),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoDecoderDownsampling(
                &*(&pinputdesc as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::DefaultType>::DefaultType),
                inputcolorspace,
                &*(&pinputconfig as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::DefaultType>::DefaultType),
                &*(&pframerate as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                &*(&poutputdesc as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_SAMPLE_DESC as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&psupported),
                ::core::mem::transmute_copy(&prealtimehint),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Impl: ID3D11VideoDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendVideoDecoderDownsampleParameters(
                &*(&pinputdesc as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_DESC as ::windows::core::DefaultType>::DefaultType),
                inputcolorspace,
                &*(&pinputconfig as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::Abi>::Abi as *const <D3D11_VIDEO_DECODER_CONFIG as ::windows::core::DefaultType>::DefaultType),
                &*(&pframerate as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::Abi>::Abi as *const <super::Dxgi::Common::DXGI_RATIONAL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&precommendedoutputdesc),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoDevice1>, ::windows::core::GetTrustLevel, GetCryptoSessionPrivateDataSize::<Impl, OFFSET>, GetVideoDecoderCaps::<Impl, OFFSET>, CheckVideoDecoderDownsampling::<Impl, OFFSET>, RecommendVideoDecoderDownsampleParameters::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoDevice2Impl: Sized + ID3D11VideoDevice1Impl + ID3D11VideoDeviceImpl {
    fn CheckFeatureSupport();
    fn NegotiateCryptoSessionKeyExchangeMT();
}
impl ::windows::core::RuntimeName for ID3D11VideoDevice2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDevice2";
}
impl ID3D11VideoDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoDevice2Impl, const OFFSET: isize>() -> ID3D11VideoDevice2Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: ID3D11VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, ::core::mem::transmute_copy(&pfeaturesupportdata), featuresupportdatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Impl: ID3D11VideoDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcryptosession: ::windows::core::RawPtr, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateCryptoSessionKeyExchangeMT(&*(&pcryptosession as *const <ID3D11CryptoSession as ::windows::core::Abi>::Abi as *const <ID3D11CryptoSession as ::windows::core::DefaultType>::DefaultType), flags, datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoDevice2>, ::windows::core::GetTrustLevel, CheckFeatureSupport::<Impl, OFFSET>, NegotiateCryptoSessionKeyExchangeMT::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoProcessorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetContentDesc();
    fn GetRateConversionCaps();
}
impl ::windows::core::RuntimeName for ID3D11VideoProcessor {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessor";
}
impl ID3D11VideoProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorImpl, const OFFSET: isize>() -> ID3D11VideoProcessorVtbl {
        unsafe extern "system" fn GetContentDesc<Impl: ID3D11VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetRateConversionCaps<Impl: ID3D11VideoProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRateConversionCaps(::core::mem::transmute_copy(&pcaps)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoProcessor>, ::windows::core::GetTrustLevel, GetContentDesc::<Impl, OFFSET>, GetRateConversionCaps::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoProcessorEnumeratorImpl: Sized + ID3D11DeviceChildImpl {
    fn GetVideoProcessorContentDesc();
    fn CheckVideoProcessorFormat();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorRateConversionCaps();
    fn GetVideoProcessorCustomRate();
    fn GetVideoProcessorFilterRange();
}
impl ::windows::core::RuntimeName for ID3D11VideoProcessorEnumerator {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorEnumerator";
}
impl ID3D11VideoProcessorEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumeratorVtbl {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorContentDesc(::core::mem::transmute_copy(&pcontentdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoProcessorFormat(format, ::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorCaps(::core::mem::transmute_copy(&pcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorRateConversionCaps(typeindex, ::core::mem::transmute_copy(&pcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorCustomRate(typeindex, customrateindex, ::core::mem::transmute_copy(&prate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Impl: ID3D11VideoProcessorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoProcessorFilterRange(filter, ::core::mem::transmute_copy(&prange)) {
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
            ::windows::core::GetRuntimeClassName::<ID3D11VideoProcessorEnumerator>,
            ::windows::core::GetTrustLevel,
            GetVideoProcessorContentDesc::<Impl, OFFSET>,
            CheckVideoProcessorFormat::<Impl, OFFSET>,
            GetVideoProcessorCaps::<Impl, OFFSET>,
            GetVideoProcessorRateConversionCaps::<Impl, OFFSET>,
            GetVideoProcessorCustomRate::<Impl, OFFSET>,
            GetVideoProcessorFilterRange::<Impl, OFFSET>,
        )
    }
}
pub trait ID3D11VideoProcessorEnumerator1Impl: Sized + ID3D11VideoProcessorEnumeratorImpl + ID3D11DeviceChildImpl {
    fn CheckVideoProcessorFormatConversion();
}
impl ::windows::core::RuntimeName for ID3D11VideoProcessorEnumerator1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorEnumerator1";
}
impl ID3D11VideoProcessorEnumerator1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorEnumerator1Impl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumerator1Vtbl {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Impl: ID3D11VideoProcessorEnumerator1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckVideoProcessorFormatConversion(inputformat, inputcolorspace, outputformat, outputcolorspace, ::core::mem::transmute_copy(&psupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoProcessorEnumerator1>, ::windows::core::GetTrustLevel, CheckVideoProcessorFormatConversion::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoProcessorInputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11VideoProcessorInputView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorInputView";
}
impl ID3D11VideoProcessorInputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorInputViewImpl, const OFFSET: isize>() -> ID3D11VideoProcessorInputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorInputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoProcessorInputView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11VideoProcessorOutputViewImpl: Sized + ID3D11ViewImpl + ID3D11DeviceChildImpl {
    fn GetDesc();
}
impl ::windows::core::RuntimeName for ID3D11VideoProcessorOutputView {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorOutputView";
}
impl ID3D11VideoProcessorOutputViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11VideoProcessorOutputViewImpl, const OFFSET: isize>() -> ID3D11VideoProcessorOutputViewVtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11VideoProcessorOutputViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11VideoProcessorOutputView>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>)
    }
}
pub trait ID3D11ViewImpl: Sized + ID3D11DeviceChildImpl {
    fn GetResource();
}
impl ::windows::core::RuntimeName for ID3D11View {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3D11View";
}
impl ID3D11ViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3D11ViewImpl, const OFFSET: isize>() -> ID3D11ViewVtbl {
        unsafe extern "system" fn GetResource<Impl: ID3D11ViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&ppresource)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3D11View>, ::windows::core::GetTrustLevel, GetResource::<Impl, OFFSET>)
    }
}
pub trait ID3DDeviceContextStateImpl: Sized + ID3D11DeviceChildImpl {}
impl ::windows::core::RuntimeName for ID3DDeviceContextState {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3DDeviceContextState";
}
impl ID3DDeviceContextStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDeviceContextStateImpl, const OFFSET: isize>() -> ID3DDeviceContextStateVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3DDeviceContextState>, ::windows::core::GetTrustLevel)
    }
}
pub trait ID3DUserDefinedAnnotationImpl: Sized {
    fn BeginEvent();
    fn EndEvent();
    fn SetMarker();
    fn GetStatus();
}
impl ::windows::core::RuntimeName for ID3DUserDefinedAnnotation {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3DUserDefinedAnnotation";
}
impl ID3DUserDefinedAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>() -> ID3DUserDefinedAnnotationVtbl {
        unsafe extern "system" fn BeginEvent<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginEvent(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEvent<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarker<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMarker(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: ID3DUserDefinedAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3DUserDefinedAnnotation>, ::windows::core::GetTrustLevel, BeginEvent::<Impl, OFFSET>, EndEvent::<Impl, OFFSET>, SetMarker::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for ID3DX11FFT {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3DX11FFT";
}
impl ID3DX11FFTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11FFTImpl, const OFFSET: isize>() -> ID3DX11FFTVtbl {
        unsafe extern "system" fn SetForwardScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardscale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetForwardScale(forwardscale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForwardScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInverseScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inversescale: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInverseScale(inversescale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInverseScale<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> f32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInverseScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const ::windows::core::RawPtr, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachBuffersAndPrecompute(numtempbuffers, &*(&pptempbuffers as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), numprecomputebuffers, &*(&ppprecomputebuffersizes as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTransform<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTransform(&*(&pinputbuffer as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), &*(&ppoutputbuffer as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InverseTransform<Impl: ID3DX11FFTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputbuffer: ::windows::core::RawPtr, ppoutputbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InverseTransform(&*(&pinputbuffer as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), &*(&ppoutputbuffer as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ID3DX11FFT>,
            ::windows::core::GetTrustLevel,
            SetForwardScale::<Impl, OFFSET>,
            GetForwardScale::<Impl, OFFSET>,
            SetInverseScale::<Impl, OFFSET>,
            GetInverseScale::<Impl, OFFSET>,
            AttachBuffersAndPrecompute::<Impl, OFFSET>,
            ForwardTransform::<Impl, OFFSET>,
            InverseTransform::<Impl, OFFSET>,
        )
    }
}
pub trait ID3DX11ScanImpl: Sized {
    fn SetScanDirection();
    fn Scan();
    fn Multiscan();
}
impl ::windows::core::RuntimeName for ID3DX11Scan {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3DX11Scan";
}
impl ID3DX11ScanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11ScanImpl, const OFFSET: isize>() -> ID3DX11ScanVtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScanDirection(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scan<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scan(elementtype, opcode, elementscansize, &*(&psrc as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), &*(&pdst as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Multiscan<Impl: ID3DX11ScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Multiscan(elementtype, opcode, elementscansize, elementscanpitch, scancount, &*(&psrc as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType), &*(&pdst as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3DX11Scan>, ::windows::core::GetTrustLevel, SetScanDirection::<Impl, OFFSET>, Scan::<Impl, OFFSET>, Multiscan::<Impl, OFFSET>)
    }
}
pub trait ID3DX11SegmentedScanImpl: Sized {
    fn SetScanDirection();
    fn SegScan();
}
impl ::windows::core::RuntimeName for ID3DX11SegmentedScan {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11.ID3DX11SegmentedScan";
}
impl ID3DX11SegmentedScanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DX11SegmentedScanImpl, const OFFSET: isize>() -> ID3DX11SegmentedScanVtbl {
        unsafe extern "system" fn SetScanDirection<Impl: ID3DX11SegmentedScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScanDirection(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SegScan<Impl: ID3DX11SegmentedScanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::windows::core::RawPtr, psrcelementflags: ::windows::core::RawPtr, pdst: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegScan(
                elementtype,
                opcode,
                elementscansize,
                &*(&psrc as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType),
                &*(&psrcelementflags as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType),
                &*(&pdst as *const <ID3D11UnorderedAccessView as ::windows::core::Abi>::Abi as *const <ID3D11UnorderedAccessView as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ID3DX11SegmentedScan>, ::windows::core::GetTrustLevel, SetScanDirection::<Impl, OFFSET>, SegScan::<Impl, OFFSET>)
    }
}

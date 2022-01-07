#[cfg(feature = "implement_exclusive")]
pub trait IAudioRoutingManagerImpl: Sized {
    fn GetAudioEndpoint(&self) -> ::windows::core::Result<AudioRoutingEndpoint>;
    fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows::core::Result<()>;
    fn AudioEndpointChanged(&self, endpointchangehandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioEndpointChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailableAudioEndpoints(&self) -> ::windows::core::Result<AvailableAudioRoutingEndpoints>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.IAudioRoutingManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioRoutingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRoutingManagerImpl, const OFFSET: isize>() -> IAudioRoutingManagerVtbl {
        unsafe extern "system" fn GetAudioEndpoint<Impl: IAudioRoutingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEndpoint<Impl: IAudioRoutingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEndpoint(endpoint).into()
        }
        unsafe extern "system" fn AudioEndpointChanged<Impl: IAudioRoutingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointchangehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEndpointChanged(&*(&endpointchangehandler as *const <super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioEndpointChanged<Impl: IAudioRoutingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioEndpointChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AvailableAudioEndpoints<Impl: IAudioRoutingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableAudioEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioRoutingManager>, ::windows::core::GetTrustLevel, GetAudioEndpoint::<Impl, OFFSET>, SetAudioEndpoint::<Impl, OFFSET>, AudioEndpointChanged::<Impl, OFFSET>, RemoveAudioEndpointChanged::<Impl, OFFSET>, AvailableAudioEndpoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioRoutingManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AudioRoutingManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioRoutingManagerStatics {
    const NAME: &'static str = "Windows.Phone.Media.Devices.IAudioRoutingManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioRoutingManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRoutingManagerStaticsImpl, const OFFSET: isize>() -> IAudioRoutingManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAudioRoutingManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioRoutingManagerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}

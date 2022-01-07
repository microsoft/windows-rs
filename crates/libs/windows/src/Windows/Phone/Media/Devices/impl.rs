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
    pub const fn new<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioRoutingManagerVtbl {
        unsafe extern "system" fn GetAudioEndpoint<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEndpoint<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioEndpoint(endpoint).into()
        }
        unsafe extern "system" fn AudioEndpointChanged<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpointchangehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEndpointChanged(&*(&endpointchangehandler as *const <super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioEndpointChanged<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioEndpointChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AvailableAudioEndpoints<Impl: IAudioRoutingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AvailableAudioEndpoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioRoutingManager>, base.5, GetAudioEndpoint::<Impl, OFFSET>, SetAudioEndpoint::<Impl, OFFSET>, AudioEndpointChanged::<Impl, OFFSET>, RemoveAudioEndpointChanged::<Impl, OFFSET>, AvailableAudioEndpoints::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAudioRoutingManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioRoutingManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAudioRoutingManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioRoutingManagerStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}

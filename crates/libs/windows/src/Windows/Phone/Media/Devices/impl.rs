#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioRoutingManager_Impl: Sized {
    fn GetAudioEndpoint(&mut self) -> ::windows::core::Result<AudioRoutingEndpoint>;
    fn SetAudioEndpoint(&mut self, endpoint: AudioRoutingEndpoint) -> ::windows::core::Result<()>;
    fn AudioEndpointChanged(&mut self, endpointchangehandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioEndpointChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailableAudioEndpoints(&mut self) -> ::windows::core::Result<AvailableAudioRoutingEndpoints>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.IAudioRoutingManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioRoutingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRoutingManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioRoutingManager_Vtbl {
        unsafe extern "system" fn GetAudioEndpoint<Impl: IAudioRoutingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioEndpoint<Impl: IAudioRoutingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEndpoint(endpoint).into()
        }
        unsafe extern "system" fn AudioEndpointChanged<Impl: IAudioRoutingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointchangehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAudioEndpointChanged<Impl: IAudioRoutingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioEndpointChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AvailableAudioEndpoints<Impl: IAudioRoutingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioRoutingManager, BASE_OFFSET>(),
            GetAudioEndpoint: GetAudioEndpoint::<Impl, IMPL_OFFSET>,
            SetAudioEndpoint: SetAudioEndpoint::<Impl, IMPL_OFFSET>,
            AudioEndpointChanged: AudioEndpointChanged::<Impl, IMPL_OFFSET>,
            RemoveAudioEndpointChanged: RemoveAudioEndpointChanged::<Impl, IMPL_OFFSET>,
            AvailableAudioEndpoints: AvailableAudioEndpoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRoutingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioRoutingManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AudioRoutingManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioRoutingManagerStatics {
    const NAME: &'static str = "Windows.Phone.Media.Devices.IAudioRoutingManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioRoutingManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRoutingManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioRoutingManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAudioRoutingManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioRoutingManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRoutingManagerStatics as ::windows::core::Interface>::IID
    }
}

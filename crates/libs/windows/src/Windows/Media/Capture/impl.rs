#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedCapturedPhotoImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Mode(&self) -> ::windows::core::Result<super::Devices::AdvancedPhotoMode>;
    fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedCapturedPhoto";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedCapturedPhotoVtbl {
    pub const fn new<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedCapturedPhotoVtbl {
        unsafe extern "system" fn Frame<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedCapturedPhoto>, base.5, Frame::<Impl, OFFSET>, Mode::<Impl, OFFSET>, Context::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedCapturedPhoto2Impl: Sized {
    fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedCapturedPhoto2 {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedCapturedPhoto2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedCapturedPhoto2Vtbl {
    pub const fn new<Impl: IAdvancedCapturedPhoto2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedCapturedPhoto2Vtbl {
        unsafe extern "system" fn FrameBoundsRelativeToReferencePhoto<Impl: IAdvancedCapturedPhoto2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameBoundsRelativeToReferencePhoto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedCapturedPhoto2>, base.5, FrameBoundsRelativeToReferencePhoto::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoCaptureImpl: Sized {
    fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn CaptureWithContextAsync(&self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn OptionalReferencePhotoCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionalReferencePhotoCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllPhotosCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAllPhotosCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedPhotoCapture";
}
#[cfg(feature = "implement_exclusive")]
impl IAdvancedPhotoCaptureVtbl {
    pub const fn new<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdvancedPhotoCaptureVtbl {
        unsafe extern "system" fn CaptureAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureWithContextAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureWithContextAsync(&*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionalReferencePhotoCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OptionalReferencePhotoCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionalReferencePhotoCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOptionalReferencePhotoCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllPhotosCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllPhotosCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllPhotosCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAllPhotosCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FinishAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdvancedPhotoCapture>, base.5, CaptureAsync::<Impl, OFFSET>, CaptureWithContextAsync::<Impl, OFFSET>, OptionalReferencePhotoCaptured::<Impl, OFFSET>, RemoveOptionalReferencePhotoCaptured::<Impl, OFFSET>, AllPhotosCaptured::<Impl, OFFSET>, RemoveAllPhotosCaptured::<Impl, OFFSET>, FinishAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceImpl: Sized {
    fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows::core::Result<()>;
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn SetSignInInfo(&self, value: &::core::option::Option<AppBroadcastBackgroundServiceSignInInfo>) -> ::windows::core::Result<()>;
    fn SignInInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceSignInInfo>;
    fn SetStreamInfo(&self, value: &::core::option::Option<AppBroadcastBackgroundServiceStreamInfo>) -> ::windows::core::Result<()>;
    fn StreamInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceStreamInfo>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetViewerCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
    fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::Result<()>;
    fn HeartbeatRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeartbeatRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundService";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceVtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundServiceVtbl {
        unsafe extern "system" fn SetPlugInState<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlugInState(value).into()
        }
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignInInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSignInInfo(&*(&value as *const <AppBroadcastBackgroundServiceSignInInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastBackgroundServiceSignInInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignInInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignInInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStreamInfo(&*(&value as *const <AppBroadcastBackgroundServiceStreamInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastBackgroundServiceStreamInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BroadcastTitle<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewerCount<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewerCount(value).into()
        }
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateBroadcast<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).TerminateBroadcast(reason, providerspecificreason).into()
        }
        unsafe extern "system" fn HeartbeatRequested<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeartbeatRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeartbeatRequested<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHeartbeatRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TitleId<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundService>,
            base.5,
            SetPlugInState::<Impl, OFFSET>,
            PlugInState::<Impl, OFFSET>,
            SetSignInInfo::<Impl, OFFSET>,
            SignInInfo::<Impl, OFFSET>,
            SetStreamInfo::<Impl, OFFSET>,
            StreamInfo::<Impl, OFFSET>,
            AppId::<Impl, OFFSET>,
            BroadcastTitle::<Impl, OFFSET>,
            SetViewerCount::<Impl, OFFSET>,
            ViewerCount::<Impl, OFFSET>,
            TerminateBroadcast::<Impl, OFFSET>,
            HeartbeatRequested::<Impl, OFFSET>,
            RemoveHeartbeatRequested::<Impl, OFFSET>,
            TitleId::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundService2Impl: Sized {
    fn SetBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastChannel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastChannel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastTitleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastTitleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastLanguageChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastLanguageChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastChannelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastChannelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundService2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundService2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundService2Vtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundService2Vtbl {
        unsafe extern "system" fn SetBroadcastTitle<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguage<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastLanguage<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBroadcastLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastChannel<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastChannel<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBroadcastChannel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastTitleChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastTitleChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastTitleChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastTitleChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguageChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguageChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastLanguageChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastLanguageChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastChannelChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastChannelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastChannelChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastChannelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundService2>,
            base.5,
            SetBroadcastTitle::<Impl, OFFSET>,
            BroadcastLanguage::<Impl, OFFSET>,
            SetBroadcastLanguage::<Impl, OFFSET>,
            BroadcastChannel::<Impl, OFFSET>,
            SetBroadcastChannel::<Impl, OFFSET>,
            BroadcastTitleChanged::<Impl, OFFSET>,
            RemoveBroadcastTitleChanged::<Impl, OFFSET>,
            BroadcastLanguageChanged::<Impl, OFFSET>,
            RemoveBroadcastLanguageChanged::<Impl, OFFSET>,
            BroadcastChannelChanged::<Impl, OFFSET>,
            RemoveBroadcastChannelChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceSignInInfoImpl: Sized {
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn SetOAuthRequestUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetOAuthCallbackUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetUserName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignInStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignInStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceSignInInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceSignInInfoVtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundServiceSignInInfoVtbl {
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOAuthRequestUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOAuthRequestUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthRequestUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OAuthRequestUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOAuthCallbackUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOAuthCallbackUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthCallbackUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OAuthCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationResult<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignInStateChanged<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignInStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignInStateChanged<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSignInStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundServiceSignInInfo>, base.5, SignInState::<Impl, OFFSET>, SetOAuthRequestUri::<Impl, OFFSET>, OAuthRequestUri::<Impl, OFFSET>, SetOAuthCallbackUri::<Impl, OFFSET>, OAuthCallbackUri::<Impl, OFFSET>, AuthenticationResult::<Impl, OFFSET>, SetUserName::<Impl, OFFSET>, UserName::<Impl, OFFSET>, SignInStateChanged::<Impl, OFFSET>, RemoveSignInStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceSignInInfo2Impl: Sized {
    fn UserNameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceSignInInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceSignInInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceSignInInfo2Vtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundServiceSignInInfo2Vtbl {
        unsafe extern "system" fn UserNameChanged<Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserNameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserNameChanged<Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUserNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundServiceSignInInfo2>, base.5, UserNameChanged::<Impl, OFFSET>, RemoveUserNameChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceStreamInfoImpl: Sized {
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows::core::Result<()>;
    fn DesiredVideoEncodingBitrate(&self) -> ::windows::core::Result<u64>;
    fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows::core::Result<()>;
    fn BandwidthTestBitrate(&self) -> ::windows::core::Result<u64>;
    fn SetAudioCodec(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioCodec(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastStreamReader(&self) -> ::windows::core::Result<AppBroadcastStreamReader>;
    fn StreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingResolutionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingBitrateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceStreamInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceStreamInfoVtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundServiceStreamInfoVtbl {
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredVideoEncodingBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn DesiredVideoEncodingBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBandwidthTestBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBandwidthTestBitrate(value).into()
        }
        unsafe extern "system" fn BandwidthTestBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BandwidthTestBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioCodec<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioCodec(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioCodec<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioCodec() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BroadcastStreamReader<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastStreamReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamStateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamStateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoEncodingResolutionChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVideoEncodingResolutionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoEncodingBitrateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVideoEncodingBitrateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundServiceStreamInfo>,
            base.5,
            StreamState::<Impl, OFFSET>,
            SetDesiredVideoEncodingBitrate::<Impl, OFFSET>,
            DesiredVideoEncodingBitrate::<Impl, OFFSET>,
            SetBandwidthTestBitrate::<Impl, OFFSET>,
            BandwidthTestBitrate::<Impl, OFFSET>,
            SetAudioCodec::<Impl, OFFSET>,
            AudioCodec::<Impl, OFFSET>,
            BroadcastStreamReader::<Impl, OFFSET>,
            StreamStateChanged::<Impl, OFFSET>,
            RemoveStreamStateChanged::<Impl, OFFSET>,
            VideoEncodingResolutionChanged::<Impl, OFFSET>,
            RemoveVideoEncodingResolutionChanged::<Impl, OFFSET>,
            VideoEncodingBitrateChanged::<Impl, OFFSET>,
            RemoveVideoEncodingBitrateChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceStreamInfo2Impl: Sized {
    fn ReportProblemWithStream(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceStreamInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceStreamInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceStreamInfo2Vtbl {
    pub const fn new<Impl: IAppBroadcastBackgroundServiceStreamInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastBackgroundServiceStreamInfo2Vtbl {
        unsafe extern "system" fn ReportProblemWithStream<Impl: IAppBroadcastBackgroundServiceStreamInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportProblemWithStream().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastBackgroundServiceStreamInfo2>, base.5, ReportProblemWithStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastCameraCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastCameraCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastCameraCaptureStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastCameraCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastCameraCaptureStateChangedEventArgs>, base.5, State::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastGlobalSettingsImpl: Sized {
    fn IsBroadcastEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool>;
    fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&self) -> ::windows::core::Result<f64>;
    fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCameraCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetSelectedCameraId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectedCameraId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::Result<()>;
    fn CameraOverlayLocation(&self) -> ::windows::core::Result<AppBroadcastCameraOverlayLocation>;
    fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows::core::Result<()>;
    fn CameraOverlaySize(&self) -> ::windows::core::Result<AppBroadcastCameraOverlaySize>;
    fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastGlobalSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastGlobalSettingsVtbl {
    pub const fn new<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastGlobalSettingsVtbl {
        unsafe extern "system" fn IsBroadcastEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBroadcastEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledByPolicy<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisabledByPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareEncoder<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasHardwareEncoder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAudioCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAudioCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAudioCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAudioCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEchoCancellationEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEchoCancellationEnabled(value).into()
        }
        unsafe extern "system" fn IsEchoCancellationEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEchoCancellationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAudioGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSystemAudioGain(value).into()
        }
        unsafe extern "system" fn SystemAudioGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemAudioGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrophoneGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMicrophoneGain(value).into()
        }
        unsafe extern "system" fn MicrophoneGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCameraCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCameraCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsCameraCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCameraCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedCameraId<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedCameraId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedCameraId<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedCameraId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCameraOverlayLocation<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCameraOverlayLocation(value).into()
        }
        unsafe extern "system" fn CameraOverlayLocation<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraOverlayLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCameraOverlaySize<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCameraOverlaySize(value).into()
        }
        unsafe extern "system" fn CameraOverlaySize<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraOverlaySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorImageCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCursorImageCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsCursorImageCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCursorImageCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastGlobalSettings>,
            base.5,
            IsBroadcastEnabled::<Impl, OFFSET>,
            IsDisabledByPolicy::<Impl, OFFSET>,
            IsGpuConstrained::<Impl, OFFSET>,
            HasHardwareEncoder::<Impl, OFFSET>,
            SetIsAudioCaptureEnabled::<Impl, OFFSET>,
            IsAudioCaptureEnabled::<Impl, OFFSET>,
            SetIsMicrophoneCaptureEnabledByDefault::<Impl, OFFSET>,
            IsMicrophoneCaptureEnabledByDefault::<Impl, OFFSET>,
            SetIsEchoCancellationEnabled::<Impl, OFFSET>,
            IsEchoCancellationEnabled::<Impl, OFFSET>,
            SetSystemAudioGain::<Impl, OFFSET>,
            SystemAudioGain::<Impl, OFFSET>,
            SetMicrophoneGain::<Impl, OFFSET>,
            MicrophoneGain::<Impl, OFFSET>,
            SetIsCameraCaptureEnabledByDefault::<Impl, OFFSET>,
            IsCameraCaptureEnabledByDefault::<Impl, OFFSET>,
            SetSelectedCameraId::<Impl, OFFSET>,
            SelectedCameraId::<Impl, OFFSET>,
            SetCameraOverlayLocation::<Impl, OFFSET>,
            CameraOverlayLocation::<Impl, OFFSET>,
            SetCameraOverlaySize::<Impl, OFFSET>,
            CameraOverlaySize::<Impl, OFFSET>,
            SetIsCursorImageCaptureEnabled::<Impl, OFFSET>,
            IsCursorImageCaptureEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastHeartbeatRequestedEventArgsImpl: Sized {
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastHeartbeatRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastHeartbeatRequestedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastHeartbeatRequestedEventArgsVtbl {
        unsafe extern "system" fn SetHandled<Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Handled<Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastHeartbeatRequestedEventArgs>, base.5, SetHandled::<Impl, OFFSET>, Handled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastManagerStaticsImpl: Sized {
    fn GetGlobalSettings(&self) -> ::windows::core::Result<AppBroadcastGlobalSettings>;
    fn ApplyGlobalSettings(&self, value: &::core::option::Option<AppBroadcastGlobalSettings>) -> ::windows::core::Result<()>;
    fn GetProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn ApplyProviderSettings(&self, value: &::core::option::Option<AppBroadcastProviderSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastManagerStaticsVtbl {
    pub const fn new<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastManagerStaticsVtbl {
        unsafe extern "system" fn GetGlobalSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlobalSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyGlobalSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ApplyGlobalSettings(&*(&value as *const <AppBroadcastGlobalSettings as ::windows::core::Abi>::Abi as *const <AppBroadcastGlobalSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetProviderSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProviderSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyProviderSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ApplyProviderSettings(&*(&value as *const <AppBroadcastProviderSettings as ::windows::core::Abi>::Abi as *const <AppBroadcastProviderSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastManagerStatics>, base.5, GetGlobalSettings::<Impl, OFFSET>, ApplyGlobalSettings::<Impl, OFFSET>, GetProviderSettings::<Impl, OFFSET>, ApplyProviderSettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastMicrophoneCaptureStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastMicrophoneCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastMicrophoneCaptureStateChangedEventArgs>, base.5, State::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugIn";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPlugInVtbl {
    pub const fn new<Impl: IAppBroadcastPlugInImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPlugInVtbl {
        unsafe extern "system" fn AppId<Impl: IAppBroadcastPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderSettings<Impl: IAppBroadcastPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logo<Impl: IAppBroadcastPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IAppBroadcastPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPlugIn>, base.5, AppId::<Impl, OFFSET>, ProviderSettings::<Impl, OFFSET>, Logo::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInManagerImpl: Sized {
    fn IsBroadcastProviderAvailable(&self) -> ::windows::core::Result<bool>;
    fn PlugInList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>;
    fn DefaultPlugIn(&self) -> ::windows::core::Result<AppBroadcastPlugIn>;
    fn SetDefaultPlugIn(&self, value: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPlugInManagerVtbl {
    pub const fn new<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPlugInManagerVtbl {
        unsafe extern "system" fn IsBroadcastProviderAvailable<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBroadcastProviderAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugInList<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPlugIn<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultPlugIn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPlugIn<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultPlugIn(&*(&value as *const <AppBroadcastPlugIn as ::windows::core::Abi>::Abi as *const <AppBroadcastPlugIn as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPlugInManager>, base.5, IsBroadcastProviderAvailable::<Impl, OFFSET>, PlugInList::<Impl, OFFSET>, DefaultPlugIn::<Impl, OFFSET>, SetDefaultPlugIn::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppBroadcastPlugInManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastPlugInManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPlugInManagerStaticsVtbl {
    pub const fn new<Impl: IAppBroadcastPlugInManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPlugInManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppBroadcastPlugInManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IAppBroadcastPlugInManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPlugInManagerStatics>, base.5, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInStateChangedEventArgsImpl: Sized {
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPlugInStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastPlugInStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPlugInStateChangedEventArgsVtbl {
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastPlugInStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPlugInStateChangedEventArgs>, base.5, PlugInState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewImpl: Sized {
    fn StopPreview(&self) -> ::windows::core::Result<()>;
    fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreviewStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewStreamReader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamReader>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreview";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewVtbl {
    pub const fn new<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPreviewVtbl {
        unsafe extern "system" fn StopPreview<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopPreview().into()
        }
        unsafe extern "system" fn PreviewState<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewStateChanged<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewStateChanged<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePreviewStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewStreamReader<Impl: IAppBroadcastPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewStreamReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPreview>, base.5, StopPreview::<Impl, OFFSET>, PreviewState::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>, PreviewStateChanged::<Impl, OFFSET>, RemovePreviewStateChanged::<Impl, OFFSET>, PreviewStreamReader::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStateChangedEventArgsImpl: Sized {
    fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPreviewStateChangedEventArgsVtbl {
        unsafe extern "system" fn PreviewState<Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPreviewStateChangedEventArgs>, base.5, PreviewState::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamReaderImpl: Sized {
    fn VideoWidth(&self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&self) -> ::windows::core::Result<u32>;
    fn VideoStride(&self) -> ::windows::core::Result<u32>;
    fn VideoBitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn VideoBitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoFrame>;
    fn VideoFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamReader";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewStreamReaderVtbl {
    pub const fn new<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPreviewStreamReaderVtbl {
        unsafe extern "system" fn VideoWidth<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoHeight<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoStride<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoStride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitmapPixelFormat<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoBitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitmapAlphaMode<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoBitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextVideoFrame<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetNextVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFrameArrived<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoFrameArrived<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPreviewStreamReader>, base.5, VideoWidth::<Impl, OFFSET>, VideoHeight::<Impl, OFFSET>, VideoStride::<Impl, OFFSET>, VideoBitmapPixelFormat::<Impl, OFFSET>, VideoBitmapAlphaMode::<Impl, OFFSET>, TryGetNextVideoFrame::<Impl, OFFSET>, VideoFrameArrived::<Impl, OFFSET>, RemoveVideoFrameArrived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamVideoFrameImpl: Sized {
    fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoHeader>;
    fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamVideoFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewStreamVideoFrameVtbl {
    pub const fn new<Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPreviewStreamVideoFrameVtbl {
        unsafe extern "system" fn VideoHeader<Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBuffer<Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPreviewStreamVideoFrame>, base.5, VideoHeader::<Impl, OFFSET>, VideoBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamVideoHeader";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewStreamVideoHeaderVtbl {
    pub const fn new<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastPreviewStreamVideoHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastPreviewStreamVideoHeader>, base.5, AbsoluteTimestamp::<Impl, OFFSET>, RelativeTimestamp::<Impl, OFFSET>, Duration::<Impl, OFFSET>, FrameId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastProviderSettingsImpl: Sized {
    fn SetDefaultBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultBroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32>;
    fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingResolutionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastProviderSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastProviderSettingsVtbl {
    pub const fn new<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastProviderSettingsVtbl {
        unsafe extern "system" fn SetDefaultBroadcastTitle<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultBroadcastTitle<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultBroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioEncodingBitrate(value).into()
        }
        unsafe extern "system" fn AudioEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingHeight<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingHeight(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingHeight<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingWidth<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingWidth(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingWidth<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingBitrateMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingBitrateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingResolutionMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingResolutionMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastProviderSettings>,
            base.5,
            SetDefaultBroadcastTitle::<Impl, OFFSET>,
            DefaultBroadcastTitle::<Impl, OFFSET>,
            SetAudioEncodingBitrate::<Impl, OFFSET>,
            AudioEncodingBitrate::<Impl, OFFSET>,
            SetCustomVideoEncodingBitrate::<Impl, OFFSET>,
            CustomVideoEncodingBitrate::<Impl, OFFSET>,
            SetCustomVideoEncodingHeight::<Impl, OFFSET>,
            CustomVideoEncodingHeight::<Impl, OFFSET>,
            SetCustomVideoEncodingWidth::<Impl, OFFSET>,
            CustomVideoEncodingWidth::<Impl, OFFSET>,
            SetVideoEncodingBitrateMode::<Impl, OFFSET>,
            VideoEncodingBitrateMode::<Impl, OFFSET>,
            SetVideoEncodingResolutionMode::<Impl, OFFSET>,
            VideoEncodingResolutionMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastServicesImpl: Sized {
    fn CaptureTargetType(&self) -> ::windows::core::Result<AppBroadcastCaptureTargetType>;
    fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows::core::Result<()>;
    fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanCapture(&self) -> ::windows::core::Result<bool>;
    fn EnterBroadcastModeAsync(&self, plugin: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::Result<()>;
    fn StartBroadcast(&self) -> ::windows::core::Result<()>;
    fn PauseBroadcast(&self) -> ::windows::core::Result<()>;
    fn ResumeBroadcast(&self) -> ::windows::core::Result<()>;
    fn StartPreview(&self, desiredsize: &super::super::Foundation::Size) -> ::windows::core::Result<AppBroadcastPreview>;
    fn State(&self) -> ::windows::core::Result<AppBroadcastState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastServices";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastServicesVtbl {
    pub const fn new<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastServicesVtbl {
        unsafe extern "system" fn CaptureTargetType<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureTargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaptureTargetType<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCaptureTargetType(value).into()
        }
        unsafe extern "system" fn BroadcastTitle<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastTitle<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguage<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastLanguage<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBroadcastLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCapture<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterBroadcastModeAsync<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plugin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnterBroadcastModeAsync(&*(&plugin as *const <AppBroadcastPlugIn as ::windows::core::Abi>::Abi as *const <AppBroadcastPlugIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitBroadcastMode<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ExitBroadcastMode(reason).into()
        }
        unsafe extern "system" fn StartBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartBroadcast().into()
        }
        unsafe extern "system" fn PauseBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PauseBroadcast().into()
        }
        unsafe extern "system" fn ResumeBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResumeBroadcast().into()
        }
        unsafe extern "system" fn StartPreview<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPreview(&*(&desiredsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAppBroadcastServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastServices>,
            base.5,
            CaptureTargetType::<Impl, OFFSET>,
            SetCaptureTargetType::<Impl, OFFSET>,
            BroadcastTitle::<Impl, OFFSET>,
            SetBroadcastTitle::<Impl, OFFSET>,
            BroadcastLanguage::<Impl, OFFSET>,
            SetBroadcastLanguage::<Impl, OFFSET>,
            UserName::<Impl, OFFSET>,
            CanCapture::<Impl, OFFSET>,
            EnterBroadcastModeAsync::<Impl, OFFSET>,
            ExitBroadcastMode::<Impl, OFFSET>,
            StartBroadcast::<Impl, OFFSET>,
            PauseBroadcast::<Impl, OFFSET>,
            ResumeBroadcast::<Impl, OFFSET>,
            StartPreview::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastSignInStateChangedEventArgsImpl: Sized {
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn Result(&self) -> ::windows::core::Result<AppBroadcastSignInResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastSignInStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastSignInStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastSignInStateChangedEventArgsVtbl {
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastSignInStateChangedEventArgs>, base.5, SignInState::<Impl, OFFSET>, Result::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStateImpl: Sized {
    fn IsCaptureTargetRunning(&self) -> ::windows::core::Result<bool>;
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
    fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()>;
    fn ShouldCaptureCamera(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureCamera(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartCameraCapture(&self) -> ::windows::core::Result<()>;
    fn EncodedVideoSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32>;
    fn CameraCaptureState(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn CameraCaptureError(&self) -> ::windows::core::Result<u32>;
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetAuthenticationResult(&self, value: &::core::option::Option<super::super::Security::Authentication::Web::WebAuthenticationResult>) -> ::windows::core::Result<()>;
    fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows::core::Result<()>;
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn TerminationReason(&self) -> ::windows::core::Result<AppBroadcastTerminationReason>;
    fn TerminationReasonPlugInSpecific(&self) -> ::windows::core::Result<u32>;
    fn ViewerCountChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewerCountChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlugInStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlugInStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastState";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStateVtbl {
    pub const fn new<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStateVtbl {
        unsafe extern "system" fn IsCaptureTargetRunning<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCaptureTargetRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldCaptureMicrophone<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureMicrophone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureMicrophone<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureMicrophone(value).into()
        }
        unsafe extern "system" fn RestartMicrophoneCapture<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RestartMicrophoneCapture().into()
        }
        unsafe extern "system" fn ShouldCaptureCamera<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureCamera<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureCamera(value).into()
        }
        unsafe extern "system" fn RestartCameraCapture<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RestartCameraCapture().into()
        }
        unsafe extern "system" fn EncodedVideoSize<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodedVideoSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureError<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraCaptureState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraCaptureError<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OAuthRequestUri<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OAuthRequestUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OAuthCallbackUri<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OAuthCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationResult<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationResult<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuthenticationResult(&*(&value as *const <super::super::Security::Authentication::Web::WebAuthenticationResult as ::windows::core::Abi>::Abi as *const <super::super::Security::Authentication::Web::WebAuthenticationResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSignInState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSignInState(value).into()
        }
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminationReason<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastTerminationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminationReasonPlugInSpecific<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminationReasonPlugInSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewerCountChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewerCountChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveViewerCountChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveViewerCountChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MicrophoneCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMicrophoneCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMicrophoneCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlugInStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlugInStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePlugInStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureTargetClosed<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureTargetClosed(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureTargetClosed<Impl: IAppBroadcastStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCaptureTargetClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastState>,
            base.5,
            IsCaptureTargetRunning::<Impl, OFFSET>,
            ViewerCount::<Impl, OFFSET>,
            ShouldCaptureMicrophone::<Impl, OFFSET>,
            SetShouldCaptureMicrophone::<Impl, OFFSET>,
            RestartMicrophoneCapture::<Impl, OFFSET>,
            ShouldCaptureCamera::<Impl, OFFSET>,
            SetShouldCaptureCamera::<Impl, OFFSET>,
            RestartCameraCapture::<Impl, OFFSET>,
            EncodedVideoSize::<Impl, OFFSET>,
            MicrophoneCaptureState::<Impl, OFFSET>,
            MicrophoneCaptureError::<Impl, OFFSET>,
            CameraCaptureState::<Impl, OFFSET>,
            CameraCaptureError::<Impl, OFFSET>,
            StreamState::<Impl, OFFSET>,
            PlugInState::<Impl, OFFSET>,
            OAuthRequestUri::<Impl, OFFSET>,
            OAuthCallbackUri::<Impl, OFFSET>,
            AuthenticationResult::<Impl, OFFSET>,
            SetAuthenticationResult::<Impl, OFFSET>,
            SetSignInState::<Impl, OFFSET>,
            SignInState::<Impl, OFFSET>,
            TerminationReason::<Impl, OFFSET>,
            TerminationReasonPlugInSpecific::<Impl, OFFSET>,
            ViewerCountChanged::<Impl, OFFSET>,
            RemoveViewerCountChanged::<Impl, OFFSET>,
            MicrophoneCaptureStateChanged::<Impl, OFFSET>,
            RemoveMicrophoneCaptureStateChanged::<Impl, OFFSET>,
            CameraCaptureStateChanged::<Impl, OFFSET>,
            RemoveCameraCaptureStateChanged::<Impl, OFFSET>,
            PlugInStateChanged::<Impl, OFFSET>,
            RemovePlugInStateChanged::<Impl, OFFSET>,
            StreamStateChanged::<Impl, OFFSET>,
            RemoveStreamStateChanged::<Impl, OFFSET>,
            CaptureTargetClosed::<Impl, OFFSET>,
            RemoveCaptureTargetClosed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamAudioFrameImpl: Sized {
    fn AudioHeader(&self) -> ::windows::core::Result<AppBroadcastStreamAudioHeader>;
    fn AudioBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamAudioFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamAudioFrameVtbl {
    pub const fn new<Impl: IAppBroadcastStreamAudioFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamAudioFrameVtbl {
        unsafe extern "system" fn AudioHeader<Impl: IAppBroadcastStreamAudioFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioBuffer<Impl: IAppBroadcastStreamAudioFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamAudioFrame>, base.5, AudioHeader::<Impl, OFFSET>, AudioBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamAudioHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn HasDiscontinuity(&self) -> ::windows::core::Result<bool>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamAudioHeader";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamAudioHeaderVtbl {
    pub const fn new<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamAudioHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDiscontinuity<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasDiscontinuity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamAudioHeader>, base.5, AbsoluteTimestamp::<Impl, OFFSET>, RelativeTimestamp::<Impl, OFFSET>, Duration::<Impl, OFFSET>, HasDiscontinuity::<Impl, OFFSET>, FrameId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamReaderImpl: Sized {
    fn AudioChannels(&self) -> ::windows::core::Result<u32>;
    fn AudioSampleRate(&self) -> ::windows::core::Result<u32>;
    fn AudioAacSequence(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AudioBitrate(&self) -> ::windows::core::Result<u32>;
    fn TryGetNextAudioFrame(&self) -> ::windows::core::Result<AppBroadcastStreamAudioFrame>;
    fn VideoWidth(&self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&self) -> ::windows::core::Result<u32>;
    fn VideoBitrate(&self) -> ::windows::core::Result<u32>;
    fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastStreamVideoFrame>;
    fn AudioFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamReader";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamReaderVtbl {
    pub const fn new<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamReaderVtbl {
        unsafe extern "system" fn AudioChannels<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioChannels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSampleRate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioSampleRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioAacSequence<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioAacSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioBitrate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextAudioFrame<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetNextAudioFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoWidth<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoHeight<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitrate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextVideoFrame<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetNextVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamReader>,
            base.5,
            AudioChannels::<Impl, OFFSET>,
            AudioSampleRate::<Impl, OFFSET>,
            AudioAacSequence::<Impl, OFFSET>,
            AudioBitrate::<Impl, OFFSET>,
            TryGetNextAudioFrame::<Impl, OFFSET>,
            VideoWidth::<Impl, OFFSET>,
            VideoHeight::<Impl, OFFSET>,
            VideoBitrate::<Impl, OFFSET>,
            TryGetNextVideoFrame::<Impl, OFFSET>,
            AudioFrameArrived::<Impl, OFFSET>,
            RemoveAudioFrameArrived::<Impl, OFFSET>,
            VideoFrameArrived::<Impl, OFFSET>,
            RemoveVideoFrameArrived::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamStateChangedEventArgsImpl: Sized {
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastStreamStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamStateChangedEventArgsVtbl {
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastStreamStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamStateChangedEventArgs>, base.5, StreamState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamVideoFrameImpl: Sized {
    fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastStreamVideoHeader>;
    fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamVideoFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamVideoFrameVtbl {
    pub const fn new<Impl: IAppBroadcastStreamVideoFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamVideoFrameVtbl {
        unsafe extern "system" fn VideoHeader<Impl: IAppBroadcastStreamVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBuffer<Impl: IAppBroadcastStreamVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamVideoFrame>, base.5, VideoHeader::<Impl, OFFSET>, VideoBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsKeyFrame(&self) -> ::windows::core::Result<bool>;
    fn HasDiscontinuity(&self) -> ::windows::core::Result<bool>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamVideoHeader";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamVideoHeaderVtbl {
    pub const fn new<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastStreamVideoHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyFrame<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsKeyFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDiscontinuity<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasDiscontinuity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastStreamVideoHeader>, base.5, AbsoluteTimestamp::<Impl, OFFSET>, RelativeTimestamp::<Impl, OFFSET>, Duration::<Impl, OFFSET>, IsKeyFrame::<Impl, OFFSET>, HasDiscontinuity::<Impl, OFFSET>, FrameId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerDetailsImpl: Sized {
    fn BackgroundService(&self) -> ::windows::core::Result<AppBroadcastBackgroundService>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerDetailsVtbl {
    pub const fn new<Impl: IAppBroadcastTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastTriggerDetailsVtbl {
        unsafe extern "system" fn BackgroundService<Impl: IAppBroadcastTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastTriggerDetails>, base.5, BackgroundService::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastViewerCountChangedEventArgsImpl: Sized {
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastViewerCountChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastViewerCountChangedEventArgsVtbl {
    pub const fn new<Impl: IAppBroadcastViewerCountChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastViewerCountChangedEventArgsVtbl {
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastViewerCountChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastViewerCountChangedEventArgs>, base.5, ViewerCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureImpl: Sized {
    fn IsCapturingAudio(&self) -> ::windows::core::Result<bool>;
    fn IsCapturingVideo(&self) -> ::windows::core::Result<bool>;
    fn CapturingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCapturingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCapture {
    const NAME: &'static str = "Windows.Media.Capture.IAppCapture";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureVtbl {
    pub const fn new<Impl: IAppCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureVtbl {
        unsafe extern "system" fn IsCapturingAudio<Impl: IAppCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCapturingAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCapturingVideo<Impl: IAppCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCapturingVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturingChanged<Impl: IAppCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CapturingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCapturingChanged<Impl: IAppCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCapturingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCapture>, base.5, IsCapturingAudio::<Impl, OFFSET>, IsCapturingVideo::<Impl, OFFSET>, CapturingChanged::<Impl, OFFSET>, RemoveCapturingChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeysImpl: Sized {
    fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleGameBarKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleGameBarKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn TakeScreenshotKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn TakeScreenshotKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureAlternateShortcutKeysVtbl {
    pub const fn new<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureAlternateShortcutKeysVtbl {
        unsafe extern "system" fn SetToggleGameBarKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleGameBarKey(value).into()
        }
        unsafe extern "system" fn ToggleGameBarKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleGameBarKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleGameBarKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleGameBarKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleGameBarKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleGameBarKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaveHistoricalVideoKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSaveHistoricalVideoKey(value).into()
        }
        unsafe extern "system" fn SaveHistoricalVideoKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveHistoricalVideoKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaveHistoricalVideoKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSaveHistoricalVideoKeyModifiers(value).into()
        }
        unsafe extern "system" fn SaveHistoricalVideoKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveHistoricalVideoKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingKey(value).into()
        }
        unsafe extern "system" fn ToggleRecordingKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleRecordingKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTakeScreenshotKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTakeScreenshotKey(value).into()
        }
        unsafe extern "system" fn TakeScreenshotKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TakeScreenshotKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTakeScreenshotKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTakeScreenshotKeyModifiers(value).into()
        }
        unsafe extern "system" fn TakeScreenshotKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TakeScreenshotKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingIndicatorKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingIndicatorKey(value).into()
        }
        unsafe extern "system" fn ToggleRecordingIndicatorKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingIndicatorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingIndicatorKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingIndicatorKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleRecordingIndicatorKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingIndicatorKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppCaptureAlternateShortcutKeys>,
            base.5,
            SetToggleGameBarKey::<Impl, OFFSET>,
            ToggleGameBarKey::<Impl, OFFSET>,
            SetToggleGameBarKeyModifiers::<Impl, OFFSET>,
            ToggleGameBarKeyModifiers::<Impl, OFFSET>,
            SetSaveHistoricalVideoKey::<Impl, OFFSET>,
            SaveHistoricalVideoKey::<Impl, OFFSET>,
            SetSaveHistoricalVideoKeyModifiers::<Impl, OFFSET>,
            SaveHistoricalVideoKeyModifiers::<Impl, OFFSET>,
            SetToggleRecordingKey::<Impl, OFFSET>,
            ToggleRecordingKey::<Impl, OFFSET>,
            SetToggleRecordingKeyModifiers::<Impl, OFFSET>,
            ToggleRecordingKeyModifiers::<Impl, OFFSET>,
            SetTakeScreenshotKey::<Impl, OFFSET>,
            TakeScreenshotKey::<Impl, OFFSET>,
            SetTakeScreenshotKeyModifiers::<Impl, OFFSET>,
            TakeScreenshotKeyModifiers::<Impl, OFFSET>,
            SetToggleRecordingIndicatorKey::<Impl, OFFSET>,
            ToggleRecordingIndicatorKey::<Impl, OFFSET>,
            SetToggleRecordingIndicatorKeyModifiers::<Impl, OFFSET>,
            ToggleRecordingIndicatorKeyModifiers::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeys2Impl: Sized {
    fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureAlternateShortcutKeys2Vtbl {
    pub const fn new<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureAlternateShortcutKeys2Vtbl {
        unsafe extern "system" fn SetToggleMicrophoneCaptureKey<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleMicrophoneCaptureKey(value).into()
        }
        unsafe extern "system" fn ToggleMicrophoneCaptureKey<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleMicrophoneCaptureKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleMicrophoneCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleMicrophoneCaptureKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleMicrophoneCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleMicrophoneCaptureKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureAlternateShortcutKeys2>, base.5, SetToggleMicrophoneCaptureKey::<Impl, OFFSET>, ToggleMicrophoneCaptureKey::<Impl, OFFSET>, SetToggleMicrophoneCaptureKeyModifiers::<Impl, OFFSET>, ToggleMicrophoneCaptureKeyModifiers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeys3Impl: Sized {
    fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys3 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureAlternateShortcutKeys3Vtbl {
    pub const fn new<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureAlternateShortcutKeys3Vtbl {
        unsafe extern "system" fn SetToggleCameraCaptureKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleCameraCaptureKey(value).into()
        }
        unsafe extern "system" fn ToggleCameraCaptureKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleCameraCaptureKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleCameraCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleCameraCaptureKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleCameraCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleCameraCaptureKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleBroadcastKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleBroadcastKey(value).into()
        }
        unsafe extern "system" fn ToggleBroadcastKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleBroadcastKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleBroadcastKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToggleBroadcastKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleBroadcastKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToggleBroadcastKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureAlternateShortcutKeys3>, base.5, SetToggleCameraCaptureKey::<Impl, OFFSET>, ToggleCameraCaptureKey::<Impl, OFFSET>, SetToggleCameraCaptureKeyModifiers::<Impl, OFFSET>, ToggleCameraCaptureKeyModifiers::<Impl, OFFSET>, SetToggleBroadcastKey::<Impl, OFFSET>, ToggleBroadcastKey::<Impl, OFFSET>, SetToggleBroadcastKeyModifiers::<Impl, OFFSET>, ToggleBroadcastKeyModifiers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureDurationGeneratedEventArgsImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureDurationGeneratedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureDurationGeneratedEventArgsVtbl {
    pub const fn new<Impl: IAppCaptureDurationGeneratedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureDurationGeneratedEventArgsVtbl {
        unsafe extern "system" fn Duration<Impl: IAppCaptureDurationGeneratedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureDurationGeneratedEventArgs>, base.5, Duration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureFileGeneratedEventArgsImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureFileGeneratedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureFileGeneratedEventArgsVtbl {
    pub const fn new<Impl: IAppCaptureFileGeneratedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureFileGeneratedEventArgsVtbl {
        unsafe extern "system" fn File<Impl: IAppCaptureFileGeneratedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureFileGeneratedEventArgs>, base.5, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureManagerStaticsImpl: Sized {
    fn GetCurrentSettings(&self) -> ::windows::core::Result<AppCaptureSettings>;
    fn ApplySettings(&self, appcapturesettings: &::core::option::Option<AppCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureManagerStaticsVtbl {
    pub const fn new<Impl: IAppCaptureManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureManagerStaticsVtbl {
        unsafe extern "system" fn GetCurrentSettings<Impl: IAppCaptureManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplySettings<Impl: IAppCaptureManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appcapturesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ApplySettings(&*(&appcapturesettings as *const <AppCaptureSettings as ::windows::core::Abi>::Abi as *const <AppCaptureSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureManagerStatics>, base.5, GetCurrentSettings::<Impl, OFFSET>, ApplySettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureMetadataWriterImpl: Sized {
    fn AddStringEvent(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddInt32Event(&self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddDoubleEvent(&self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartStringState(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartInt32State(&self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartDoubleState(&self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StopState(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopAllStates(&self) -> ::windows::core::Result<()>;
    fn RemainingStorageBytesAvailable(&self) -> ::windows::core::Result<u64>;
    fn MetadataPurged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMetadataPurged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureMetadataWriter";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureMetadataWriterVtbl {
    pub const fn new<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureMetadataWriterVtbl {
        unsafe extern "system" fn AddStringEvent<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddStringEvent(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), priority).into()
        }
        unsafe extern "system" fn AddInt32Event<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddInt32Event(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn AddDoubleEvent<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddDoubleEvent(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StartStringState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartStringState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), priority).into()
        }
        unsafe extern "system" fn StartInt32State<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartInt32State(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StartDoubleState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartDoubleState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StopState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAllStates<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopAllStates().into()
        }
        unsafe extern "system" fn RemainingStorageBytesAvailable<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemainingStorageBytesAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetadataPurged<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MetadataPurged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMetadataPurged<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMetadataPurged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureMetadataWriter>, base.5, AddStringEvent::<Impl, OFFSET>, AddInt32Event::<Impl, OFFSET>, AddDoubleEvent::<Impl, OFFSET>, StartStringState::<Impl, OFFSET>, StartInt32State::<Impl, OFFSET>, StartDoubleState::<Impl, OFFSET>, StopState::<Impl, OFFSET>, StopAllStates::<Impl, OFFSET>, RemainingStorageBytesAvailable::<Impl, OFFSET>, MetadataPurged::<Impl, OFFSET>, RemoveMetadataPurged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureMicrophoneCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureMicrophoneCaptureStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureMicrophoneCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureMicrophoneCaptureStateChangedEventArgs>, base.5, State::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureRecordOperationImpl: Sized {
    fn StopRecording(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn IsFileTruncated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn StateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DurationGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDurationGenerated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FileGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileGenerated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureRecordOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureRecordOperationVtbl {
    pub const fn new<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureRecordOperationVtbl {
        unsafe extern "system" fn StopRecording<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopRecording().into()
        }
        unsafe extern "system" fn State<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileTruncated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFileTruncated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DurationGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DurationGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDurationGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDurationGenerated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFileGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFileGenerated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureRecordOperation>, base.5, StopRecording::<Impl, OFFSET>, State::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>, Duration::<Impl, OFFSET>, File::<Impl, OFFSET>, IsFileTruncated::<Impl, OFFSET>, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>, DurationGenerated::<Impl, OFFSET>, RemoveDurationGenerated::<Impl, OFFSET>, FileGenerated::<Impl, OFFSET>, RemoveFileGenerated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureRecordingStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureRecordingStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureRecordingStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureRecordingStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureRecordingStateChangedEventArgs>, base.5, State::<Impl, OFFSET>, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureServicesImpl: Sized {
    fn Record(&self) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn RecordTimeSpan(&self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn CanCapture(&self) -> ::windows::core::Result<bool>;
    fn State(&self) -> ::windows::core::Result<AppCaptureState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureServices";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureServicesVtbl {
    pub const fn new<Impl: IAppCaptureServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureServicesVtbl {
        unsafe extern "system" fn Record<Impl: IAppCaptureServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Record() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordTimeSpan<Impl: IAppCaptureServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordTimeSpan(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCapture<Impl: IAppCaptureServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAppCaptureServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureServices>, base.5, Record::<Impl, OFFSET>, RecordTimeSpan::<Impl, OFFSET>, CanCapture::<Impl, OFFSET>, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettingsImpl: Sized {
    fn SetAppCaptureDestinationFolder(&self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn AppCaptureDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn HistoricalBufferLength(&self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::Result<()>;
    fn HistoricalBufferLengthUnit(&self) -> ::windows::core::Result<AppCaptureHistoricalBufferLengthUnit>;
    fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetMaximumRecordLength(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaximumRecordLength(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScreenshotDestinationFolder(&self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn ScreenshotDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingResolutionMode>;
    fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsCpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool>;
    fn IsMemoryConstrained(&self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettingsVtbl {
    pub const fn new<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureSettingsVtbl {
        unsafe extern "system" fn SetAppCaptureDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppCaptureDestinationFolder(&*(&value as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppCaptureDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppCaptureDestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioEncodingBitrate(value).into()
        }
        unsafe extern "system" fn AudioEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAudioCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAudioCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAudioCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAudioCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingHeight<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingHeight(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingHeight<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingWidth<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingWidth(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingWidth<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalBufferLength<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHistoricalBufferLength(value).into()
        }
        unsafe extern "system" fn HistoricalBufferLength<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HistoricalBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalBufferLengthUnit<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHistoricalBufferLengthUnit(value).into()
        }
        unsafe extern "system" fn HistoricalBufferLengthUnit<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HistoricalBufferLengthUnit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureOnBatteryAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureOnBatteryAllowed(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureOnBatteryAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureOnBatteryAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureOnWirelessDisplayAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureOnWirelessDisplayAllowed(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureOnWirelessDisplayAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureOnWirelessDisplayAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumRecordLength<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaximumRecordLength(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaximumRecordLength<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaximumRecordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScreenshotDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScreenshotDestinationFolder(&*(&value as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenshotDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScreenshotDestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingBitrateMode<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingBitrateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateMode<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingResolutionMode<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingResolutionMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionMode<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAppCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAppCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAppCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAppCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCpuConstrained<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledByPolicy<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisabledByPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMemoryConstrained<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMemoryConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareEncoder<Impl: IAppCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasHardwareEncoder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppCaptureSettings>,
            base.5,
            SetAppCaptureDestinationFolder::<Impl, OFFSET>,
            AppCaptureDestinationFolder::<Impl, OFFSET>,
            SetAudioEncodingBitrate::<Impl, OFFSET>,
            AudioEncodingBitrate::<Impl, OFFSET>,
            SetIsAudioCaptureEnabled::<Impl, OFFSET>,
            IsAudioCaptureEnabled::<Impl, OFFSET>,
            SetCustomVideoEncodingBitrate::<Impl, OFFSET>,
            CustomVideoEncodingBitrate::<Impl, OFFSET>,
            SetCustomVideoEncodingHeight::<Impl, OFFSET>,
            CustomVideoEncodingHeight::<Impl, OFFSET>,
            SetCustomVideoEncodingWidth::<Impl, OFFSET>,
            CustomVideoEncodingWidth::<Impl, OFFSET>,
            SetHistoricalBufferLength::<Impl, OFFSET>,
            HistoricalBufferLength::<Impl, OFFSET>,
            SetHistoricalBufferLengthUnit::<Impl, OFFSET>,
            HistoricalBufferLengthUnit::<Impl, OFFSET>,
            SetIsHistoricalCaptureEnabled::<Impl, OFFSET>,
            IsHistoricalCaptureEnabled::<Impl, OFFSET>,
            SetIsHistoricalCaptureOnBatteryAllowed::<Impl, OFFSET>,
            IsHistoricalCaptureOnBatteryAllowed::<Impl, OFFSET>,
            SetIsHistoricalCaptureOnWirelessDisplayAllowed::<Impl, OFFSET>,
            IsHistoricalCaptureOnWirelessDisplayAllowed::<Impl, OFFSET>,
            SetMaximumRecordLength::<Impl, OFFSET>,
            MaximumRecordLength::<Impl, OFFSET>,
            SetScreenshotDestinationFolder::<Impl, OFFSET>,
            ScreenshotDestinationFolder::<Impl, OFFSET>,
            SetVideoEncodingBitrateMode::<Impl, OFFSET>,
            VideoEncodingBitrateMode::<Impl, OFFSET>,
            SetVideoEncodingResolutionMode::<Impl, OFFSET>,
            VideoEncodingResolutionMode::<Impl, OFFSET>,
            SetIsAppCaptureEnabled::<Impl, OFFSET>,
            IsAppCaptureEnabled::<Impl, OFFSET>,
            IsCpuConstrained::<Impl, OFFSET>,
            IsDisabledByPolicy::<Impl, OFFSET>,
            IsMemoryConstrained::<Impl, OFFSET>,
            HasHardwareEncoder::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings2Impl: Sized {
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn AlternateShortcutKeys(&self) -> ::windows::core::Result<AppCaptureAlternateShortcutKeys>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings2Vtbl {
    pub const fn new<Impl: IAppCaptureSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureSettings2Vtbl {
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateShortcutKeys<Impl: IAppCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlternateShortcutKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureSettings2>, base.5, IsGpuConstrained::<Impl, OFFSET>, AlternateShortcutKeys::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings3Impl: Sized {
    fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings3Vtbl {
    pub const fn new<Impl: IAppCaptureSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureSettings3Vtbl {
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabled<Impl: IAppCaptureSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabled<Impl: IAppCaptureSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureSettings3>, base.5, SetIsMicrophoneCaptureEnabled::<Impl, OFFSET>, IsMicrophoneCaptureEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings4Impl: Sized {
    fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&self) -> ::windows::core::Result<f64>;
    fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingFrameRateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingFrameRateMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings4 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings4Vtbl {
    pub const fn new<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureSettings4Vtbl {
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabledByDefault<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabledByDefault<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAudioGain<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSystemAudioGain(value).into()
        }
        unsafe extern "system" fn SystemAudioGain<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemAudioGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrophoneGain<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMicrophoneGain(value).into()
        }
        unsafe extern "system" fn MicrophoneGain<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingFrameRateMode<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingFrameRateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingFrameRateMode<Impl: IAppCaptureSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoEncodingFrameRateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureSettings4>, base.5, SetIsMicrophoneCaptureEnabledByDefault::<Impl, OFFSET>, IsMicrophoneCaptureEnabledByDefault::<Impl, OFFSET>, SetSystemAudioGain::<Impl, OFFSET>, SystemAudioGain::<Impl, OFFSET>, SetMicrophoneGain::<Impl, OFFSET>, MicrophoneGain::<Impl, OFFSET>, SetVideoEncodingFrameRateMode::<Impl, OFFSET>, VideoEncodingFrameRateMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings5Impl: Sized {
    fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings5 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings5";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings5Vtbl {
    pub const fn new<Impl: IAppCaptureSettings5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureSettings5Vtbl {
        unsafe extern "system" fn SetIsEchoCancellationEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEchoCancellationEnabled(value).into()
        }
        unsafe extern "system" fn IsEchoCancellationEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEchoCancellationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorImageCaptureEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCursorImageCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsCursorImageCaptureEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCursorImageCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureSettings5>, base.5, SetIsEchoCancellationEnabled::<Impl, OFFSET>, IsEchoCancellationEnabled::<Impl, OFFSET>, SetIsCursorImageCaptureEnabled::<Impl, OFFSET>, IsCursorImageCaptureEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStateImpl: Sized {
    fn IsTargetRunning(&self) -> ::windows::core::Result<bool>;
    fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32>;
    fn MicrophoneCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureState";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureStateVtbl {
    pub const fn new<Impl: IAppCaptureStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureStateVtbl {
        unsafe extern "system" fn IsTargetRunning<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTargetRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHistoricalCaptureEnabled<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldCaptureMicrophone<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureMicrophone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureMicrophone<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureMicrophone(value).into()
        }
        unsafe extern "system" fn RestartMicrophoneCapture<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RestartMicrophoneCapture().into()
        }
        unsafe extern "system" fn MicrophoneCaptureState<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureError<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureStateChanged<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMicrophoneCaptureStateChanged<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMicrophoneCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureTargetClosed<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureTargetClosed(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureTargetClosed<Impl: IAppCaptureStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCaptureTargetClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppCaptureState>,
            base.5,
            IsTargetRunning::<Impl, OFFSET>,
            IsHistoricalCaptureEnabled::<Impl, OFFSET>,
            ShouldCaptureMicrophone::<Impl, OFFSET>,
            SetShouldCaptureMicrophone::<Impl, OFFSET>,
            RestartMicrophoneCapture::<Impl, OFFSET>,
            MicrophoneCaptureState::<Impl, OFFSET>,
            MicrophoneCaptureError::<Impl, OFFSET>,
            MicrophoneCaptureStateChanged::<Impl, OFFSET>,
            RemoveMicrophoneCaptureStateChanged::<Impl, OFFSET>,
            CaptureTargetClosed::<Impl, OFFSET>,
            RemoveCaptureTargetClosed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<AppCapture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureStaticsVtbl {
    pub const fn new<Impl: IAppCaptureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IAppCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStatics2Impl: Sized {
    fn SetAllowedAsync(&self, allowed: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureStatics2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureStatics2Vtbl {
    pub const fn new<Impl: IAppCaptureStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppCaptureStatics2Vtbl {
        unsafe extern "system" fn SetAllowedAsync<Impl: IAppCaptureStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowed: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllowedAsync(allowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppCaptureStatics2>, base.5, SetAllowedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIImpl: Sized {
    fn PhotoSettings(&self) -> ::windows::core::Result<CameraCaptureUIPhotoCaptureSettings>;
    fn VideoSettings(&self) -> ::windows::core::Result<CameraCaptureUIVideoCaptureSettings>;
    fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUI";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraCaptureUIVtbl {
    pub const fn new<Impl: ICameraCaptureUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraCaptureUIVtbl {
        unsafe extern "system" fn PhotoSettings<Impl: ICameraCaptureUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoSettings<Impl: ICameraCaptureUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureFileAsync<Impl: ICameraCaptureUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: CameraCaptureUIMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureFileAsync(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraCaptureUI>, base.5, PhotoSettings::<Impl, OFFSET>, VideoSettings::<Impl, OFFSET>, CaptureFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIPhotoCaptureSettingsImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<CameraCaptureUIPhotoFormat>;
    fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxPhotoResolution>;
    fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::Result<()>;
    fn CroppedSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedSizeInPixels(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CroppedAspectRatio(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedAspectRatio(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn AllowCropping(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCropping(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUIPhotoCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraCaptureUIPhotoCaptureSettingsVtbl {
    pub const fn new<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraCaptureUIPhotoCaptureSettingsVtbl {
        unsafe extern "system" fn Format<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn MaxResolution<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxResolution<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxResolution(value).into()
        }
        unsafe extern "system" fn CroppedSizeInPixels<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CroppedSizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCroppedSizeInPixels<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCroppedSizeInPixels(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CroppedAspectRatio<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CroppedAspectRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCroppedAspectRatio<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCroppedAspectRatio(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowCropping<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowCropping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCropping<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowCropping(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraCaptureUIPhotoCaptureSettings>, base.5, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, MaxResolution::<Impl, OFFSET>, SetMaxResolution::<Impl, OFFSET>, CroppedSizeInPixels::<Impl, OFFSET>, SetCroppedSizeInPixels::<Impl, OFFSET>, CroppedAspectRatio::<Impl, OFFSET>, SetCroppedAspectRatio::<Impl, OFFSET>, AllowCropping::<Impl, OFFSET>, SetAllowCropping::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIVideoCaptureSettingsImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<CameraCaptureUIVideoFormat>;
    fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxVideoResolution>;
    fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::Result<()>;
    fn MaxDurationInSeconds(&self) -> ::windows::core::Result<f32>;
    fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows::core::Result<()>;
    fn AllowTrimming(&self) -> ::windows::core::Result<bool>;
    fn SetAllowTrimming(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUIVideoCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraCaptureUIVideoCaptureSettingsVtbl {
    pub const fn new<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraCaptureUIVideoCaptureSettingsVtbl {
        unsafe extern "system" fn Format<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn MaxResolution<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxResolution<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxResolution(value).into()
        }
        unsafe extern "system" fn MaxDurationInSeconds<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxDurationInSeconds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDurationInSeconds<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxDurationInSeconds(value).into()
        }
        unsafe extern "system" fn AllowTrimming<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowTrimming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowTrimming<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowTrimming(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraCaptureUIVideoCaptureSettings>, base.5, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, MaxResolution::<Impl, OFFSET>, SetMaxResolution::<Impl, OFFSET>, MaxDurationInSeconds::<Impl, OFFSET>, SetMaxDurationInSeconds::<Impl, OFFSET>, AllowTrimming::<Impl, OFFSET>, SetAllowTrimming::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOptionsUIStaticsImpl: Sized {
    fn Show(&self, mediacapture: &::core::option::Option<MediaCapture>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOptionsUIStatics {
    const NAME: &'static str = "Windows.Media.Capture.ICameraOptionsUIStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOptionsUIStaticsVtbl {
    pub const fn new<Impl: ICameraOptionsUIStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraOptionsUIStaticsVtbl {
        unsafe extern "system" fn Show<Impl: ICameraOptionsUIStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediacapture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Show(&*(&mediacapture as *const <MediaCapture as ::windows::core::Abi>::Abi as *const <MediaCapture as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraOptionsUIStatics>, base.5, Show::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICapturedFrameImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrame";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICapturedFrameVtbl {
    pub const fn new<Impl: ICapturedFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedFrameVtbl {
        unsafe extern "system" fn Width<Impl: ICapturedFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: ICapturedFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedFrame>, base.5, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrame2Impl: Sized {
    fn ControlValues(&self) -> ::windows::core::Result<CapturedFrameControlValues>;
    fn BitmapProperties(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedFrame2 {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrame2";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedFrame2Vtbl {
    pub const fn new<Impl: ICapturedFrame2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedFrame2Vtbl {
        unsafe extern "system" fn ControlValues<Impl: ICapturedFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: ICapturedFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedFrame2>, base.5, ControlValues::<Impl, OFFSET>, BitmapProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameControlValuesImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn IsoSpeed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Focus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SceneMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>;
    fn Flashed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn FlashPowerPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn WhiteBalance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn ZoomFactor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameControlValues";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedFrameControlValuesVtbl {
    pub const fn new<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedFrameControlValuesVtbl {
        unsafe extern "system" fn Exposure<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exposure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensation<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeed<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Focus<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Focus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneMode<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SceneMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flashed<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlashPowerPercent<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlashPowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalance<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhiteBalance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomFactor<Impl: ICapturedFrameControlValuesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedFrameControlValues>, base.5, Exposure::<Impl, OFFSET>, ExposureCompensation::<Impl, OFFSET>, IsoSpeed::<Impl, OFFSET>, Focus::<Impl, OFFSET>, SceneMode::<Impl, OFFSET>, Flashed::<Impl, OFFSET>, FlashPowerPercent::<Impl, OFFSET>, WhiteBalance::<Impl, OFFSET>, ZoomFactor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameControlValues2Impl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>;
    fn IsoDigitalGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn IsoAnalogGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SensorFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn WhiteBalanceGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<WhiteBalanceGain>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedFrameControlValues2 {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameControlValues2";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedFrameControlValues2Vtbl {
    pub const fn new<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedFrameControlValues2Vtbl {
        unsafe extern "system" fn FocusState<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoDigitalGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoDigitalGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoAnalogGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoAnalogGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SensorFrameRate<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SensorFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalanceGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhiteBalanceGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedFrameControlValues2>, base.5, FocusState::<Impl, OFFSET>, IsoDigitalGain::<Impl, OFFSET>, IsoAnalogGain::<Impl, OFFSET>, SensorFrameRate::<Impl, OFFSET>, WhiteBalanceGain::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameWithSoftwareBitmapImpl: Sized {
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameWithSoftwareBitmap";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedFrameWithSoftwareBitmapVtbl {
    pub const fn new<Impl: ICapturedFrameWithSoftwareBitmapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedFrameWithSoftwareBitmapVtbl {
        unsafe extern "system" fn SoftwareBitmap<Impl: ICapturedFrameWithSoftwareBitmapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedFrameWithSoftwareBitmap>, base.5, SoftwareBitmap::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedPhotoImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedPhoto";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedPhotoVtbl {
    pub const fn new<Impl: ICapturedPhotoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICapturedPhotoVtbl {
        unsafe extern "system" fn Frame<Impl: ICapturedPhotoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: ICapturedPhotoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICapturedPhoto>, base.5, Frame::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesImpl: Sized {
    fn TargetCapturePolicy(&self) -> ::windows::core::Result<GameBarTargetCapturePolicy>;
    fn EnableCapture(&self) -> ::windows::core::Result<()>;
    fn DisableCapture(&self) -> ::windows::core::Result<()>;
    fn TargetInfo(&self) -> ::windows::core::Result<GameBarServicesTargetInfo>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppBroadcastServices(&self) -> ::windows::core::Result<AppBroadcastServices>;
    fn AppCaptureServices(&self) -> ::windows::core::Result<AppCaptureServices>;
    fn CommandReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServices";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesVtbl {
    pub const fn new<Impl: IGameBarServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesVtbl {
        unsafe extern "system" fn TargetCapturePolicy<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarTargetCapturePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetCapturePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableCapture<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableCapture().into()
        }
        unsafe extern "system" fn DisableCapture<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DisableCapture().into()
        }
        unsafe extern "system" fn TargetInfo<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppBroadcastServices<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppBroadcastServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppCaptureServices<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppCaptureServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandReceived<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandReceived(&*(&value as *const <super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandReceived<Impl: IGameBarServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCommandReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServices>, base.5, TargetCapturePolicy::<Impl, OFFSET>, EnableCapture::<Impl, OFFSET>, DisableCapture::<Impl, OFFSET>, TargetInfo::<Impl, OFFSET>, SessionId::<Impl, OFFSET>, AppBroadcastServices::<Impl, OFFSET>, AppCaptureServices::<Impl, OFFSET>, CommandReceived::<Impl, OFFSET>, RemoveCommandReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesCommandEventArgsImpl: Sized {
    fn Command(&self) -> ::windows::core::Result<GameBarCommand>;
    fn Origin(&self) -> ::windows::core::Result<GameBarCommandOrigin>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesCommandEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesCommandEventArgsVtbl {
    pub const fn new<Impl: IGameBarServicesCommandEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesCommandEventArgsVtbl {
        unsafe extern "system" fn Command<Impl: IGameBarServicesCommandEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: IGameBarServicesCommandEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommandOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServicesCommandEventArgs>, base.5, Command::<Impl, OFFSET>, Origin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerImpl: Sized {
    fn GameBarServicesCreated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameBarServicesCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManager";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesManagerVtbl {
    pub const fn new<Impl: IGameBarServicesManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesManagerVtbl {
        unsafe extern "system" fn GameBarServicesCreated<Impl: IGameBarServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GameBarServicesCreated(&*(&value as *const <super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameBarServicesCreated<Impl: IGameBarServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGameBarServicesCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServicesManager>, base.5, GameBarServicesCreated::<Impl, OFFSET>, RemoveGameBarServicesCreated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl: Sized {
    fn GameBarServices(&self) -> ::windows::core::Result<GameBarServices>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManagerGameBarServicesCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesManagerGameBarServicesCreatedEventArgsVtbl {
    pub const fn new<Impl: IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesManagerGameBarServicesCreatedEventArgsVtbl {
        unsafe extern "system" fn GameBarServices<Impl: IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GameBarServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServicesManagerGameBarServicesCreatedEventArgs>, base.5, GameBarServices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameBarServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesManagerStaticsVtbl {
    pub const fn new<Impl: IGameBarServicesManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameBarServicesManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServicesManagerStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesTargetInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayMode(&self) -> ::windows::core::Result<GameBarServicesDisplayMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesTargetInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesTargetInfoVtbl {
    pub const fn new<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarServicesTargetInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleId<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayMode<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarServicesDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarServicesTargetInfo>, base.5, DisplayName::<Impl, OFFSET>, AppId::<Impl, OFFSET>, TitleId::<Impl, OFFSET>, DisplayMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecordingImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagMediaRecordingVtbl {
    pub const fn new<Impl: ILowLagMediaRecordingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagMediaRecordingVtbl {
        unsafe extern "system" fn StartAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagMediaRecording>, base.5, StartAsync::<Impl, OFFSET>, StopAsync::<Impl, OFFSET>, FinishAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecording2Impl: Sized {
    fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagMediaRecording2 {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording2";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagMediaRecording2Vtbl {
    pub const fn new<Impl: ILowLagMediaRecording2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagMediaRecording2Vtbl {
        unsafe extern "system" fn PauseAsync<Impl: ILowLagMediaRecording2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeAsync<Impl: ILowLagMediaRecording2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagMediaRecording2>, base.5, PauseAsync::<Impl, OFFSET>, ResumeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecording3Impl: Sized {
    fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagMediaRecording3 {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording3";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagMediaRecording3Vtbl {
    pub const fn new<Impl: ILowLagMediaRecording3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagMediaRecording3Vtbl {
        unsafe extern "system" fn PauseWithResultAsync<Impl: ILowLagMediaRecording3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseWithResultAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopWithResultAsync<Impl: ILowLagMediaRecording3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopWithResultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagMediaRecording3>, base.5, PauseWithResultAsync::<Impl, OFFSET>, StopWithResultAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoCaptureImpl: Sized {
    fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagPhotoCapture";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagPhotoCaptureVtbl {
    pub const fn new<Impl: ILowLagPhotoCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagPhotoCaptureVtbl {
        unsafe extern "system" fn CaptureAsync<Impl: ILowLagPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagPhotoCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagPhotoCapture>, base.5, CaptureAsync::<Impl, OFFSET>, FinishAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoSequenceCaptureImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PhotoCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagPhotoSequenceCapture";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLagPhotoSequenceCaptureVtbl {
    pub const fn new<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLagPhotoSequenceCaptureVtbl {
        unsafe extern "system" fn StartAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoCaptured<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePhotoCaptured<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePhotoCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLagPhotoSequenceCapture>, base.5, StartAsync::<Impl, OFFSET>, StopAsync::<Impl, OFFSET>, FinishAsync::<Impl, OFFSET>, PhotoCaptured::<Impl, OFFSET>, RemovePhotoCaptured::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureImpl: Sized {
    fn InitializeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InitializeWithSettingsAsync(&self, mediacaptureinitializationsettings: &::core::option::Option<MediaCaptureInitializationSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStorageFileAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStreamAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStorageFileAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStreamAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AddEffectAsync(&self, mediastreamtype: MediaStreamType, effectactivationid: &::windows::core::HSTRING, effectsettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Failed(&self, erroreventhandler: &::core::option::Option<MediaCaptureFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordLimitationExceeded(&self, recordlimitationexceededeventhandler: &::core::option::Option<RecordLimitationExceededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecordLimitationExceeded(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaCaptureSettings(&self) -> ::windows::core::Result<MediaCaptureSettings>;
    fn AudioDeviceController(&self) -> ::windows::core::Result<super::Devices::AudioDeviceController>;
    fn VideoDeviceController(&self) -> ::windows::core::Result<super::Devices::VideoDeviceController>;
    fn SetPreviewMirroring(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPreviewMirroring(&self) -> ::windows::core::Result<bool>;
    fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetPreviewRotation(&self) -> ::windows::core::Result<VideoRotation>;
    fn SetRecordRotation(&self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetRecordRotation(&self) -> ::windows::core::Result<VideoRotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVtbl {
    pub const fn new<Impl: IMediaCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVtbl {
        unsafe extern "system" fn InitializeAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeWithSettingsAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediacaptureinitializationsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeWithSettingsAsync(&*(&mediacaptureinitializationsettings as *const <MediaCaptureInitializationSettings as ::windows::core::Abi>::Abi as *const <MediaCaptureInitializationSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToStorageFileAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartRecordToStorageFileAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToStreamAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartRecordToStreamAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToCustomSinkAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartRecordToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToCustomSinkIdAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartRecordToCustomSinkIdAsync(
                &*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinkactivationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinksettings as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRecordAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopRecordAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePhotoToStorageFileAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CapturePhotoToStorageFileAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePhotoToStreamAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CapturePhotoToStreamAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEffectAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, effectactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddEffectAsync(mediastreamtype, &*(&effectactivationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&effectsettings as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearEffectsAsync<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearEffectsAsync(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncoderProperty<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEncoderProperty(mediastreamtype, &*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncoderProperty<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEncoderProperty(mediastreamtype, &*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Failed<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, erroreventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&erroreventhandler as *const <MediaCaptureFailedEventHandler as ::windows::core::Abi>::Abi as *const <MediaCaptureFailedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordLimitationExceeded<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordlimitationexceededeventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordLimitationExceeded(&*(&recordlimitationexceededeventhandler as *const <RecordLimitationExceededEventHandler as ::windows::core::Abi>::Abi as *const <RecordLimitationExceededEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecordLimitationExceeded<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRecordLimitationExceeded(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCaptureSettings<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaCaptureSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioDeviceController<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceController<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewMirroring<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreviewMirroring(value).into()
        }
        unsafe extern "system" fn GetPreviewMirroring<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewRotation<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreviewRotation(value).into()
        }
        unsafe extern "system" fn GetPreviewRotation<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecordRotation<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRecordRotation(value).into()
        }
        unsafe extern "system" fn GetRecordRotation<Impl: IMediaCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecordRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaCapture>,
            base.5,
            InitializeAsync::<Impl, OFFSET>,
            InitializeWithSettingsAsync::<Impl, OFFSET>,
            StartRecordToStorageFileAsync::<Impl, OFFSET>,
            StartRecordToStreamAsync::<Impl, OFFSET>,
            StartRecordToCustomSinkAsync::<Impl, OFFSET>,
            StartRecordToCustomSinkIdAsync::<Impl, OFFSET>,
            StopRecordAsync::<Impl, OFFSET>,
            CapturePhotoToStorageFileAsync::<Impl, OFFSET>,
            CapturePhotoToStreamAsync::<Impl, OFFSET>,
            AddEffectAsync::<Impl, OFFSET>,
            ClearEffectsAsync::<Impl, OFFSET>,
            SetEncoderProperty::<Impl, OFFSET>,
            GetEncoderProperty::<Impl, OFFSET>,
            Failed::<Impl, OFFSET>,
            RemoveFailed::<Impl, OFFSET>,
            RecordLimitationExceeded::<Impl, OFFSET>,
            RemoveRecordLimitationExceeded::<Impl, OFFSET>,
            MediaCaptureSettings::<Impl, OFFSET>,
            AudioDeviceController::<Impl, OFFSET>,
            VideoDeviceController::<Impl, OFFSET>,
            SetPreviewMirroring::<Impl, OFFSET>,
            GetPreviewMirroring::<Impl, OFFSET>,
            SetPreviewRotation::<Impl, OFFSET>,
            GetPreviewRotation::<Impl, OFFSET>,
            SetRecordRotation::<Impl, OFFSET>,
            GetRecordRotation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture2Impl: Sized {
    fn PrepareLowLagRecordToStorageFileAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToStreamAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagPhotoCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>;
    fn PrepareLowLagPhotoSequenceCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>;
    fn SetEncodingPropertiesAsync(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>, encoderproperties: &::core::option::Option<super::MediaProperties::MediaPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture2Vtbl {
    pub const fn new<Impl: IMediaCapture2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture2Vtbl {
        unsafe extern "system" fn PrepareLowLagRecordToStorageFileAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToStorageFileAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToStreamAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToStreamAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToCustomSinkAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToCustomSinkIdAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToCustomSinkIdAsync(
                &*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinkactivationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinksettings as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagPhotoCaptureAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagPhotoCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagPhotoSequenceCaptureAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagPhotoSequenceCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingPropertiesAsync<Impl: IMediaCapture2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, encoderproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEncodingPropertiesAsync(mediastreamtype, &*(&mediaencodingproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&encoderproperties as *const <super::MediaProperties::MediaPropertySet as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapture2>, base.5, PrepareLowLagRecordToStorageFileAsync::<Impl, OFFSET>, PrepareLowLagRecordToStreamAsync::<Impl, OFFSET>, PrepareLowLagRecordToCustomSinkAsync::<Impl, OFFSET>, PrepareLowLagRecordToCustomSinkIdAsync::<Impl, OFFSET>, PrepareLowLagPhotoCaptureAsync::<Impl, OFFSET>, PrepareLowLagPhotoSequenceCaptureAsync::<Impl, OFFSET>, SetEncodingPropertiesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture3Impl: Sized {
    fn PrepareVariablePhotoSequenceCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>;
    fn FocusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PhotoConfirmationCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoConfirmationCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture3Vtbl {
    pub const fn new<Impl: IMediaCapture3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture3Vtbl {
        unsafe extern "system" fn PrepareVariablePhotoSequenceCaptureAsync<Impl: IMediaCapture3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareVariablePhotoSequenceCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusChanged<Impl: IMediaCapture3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFocusChanged<Impl: IMediaCapture3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFocusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhotoConfirmationCaptured<Impl: IMediaCapture3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePhotoConfirmationCaptured<Impl: IMediaCapture3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePhotoConfirmationCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapture3>, base.5, PrepareVariablePhotoSequenceCaptureAsync::<Impl, OFFSET>, FocusChanged::<Impl, OFFSET>, RemoveFocusChanged::<Impl, OFFSET>, PhotoConfirmationCaptured::<Impl, OFFSET>, RemovePhotoConfirmationCaptured::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture4Impl: Sized {
    fn AddAudioEffectAsync(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn AddVideoEffectAsync(&self, definition: &::core::option::Option<super::Effects::IVideoEffectDefinition>, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CameraStreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraStreamState(&self) -> ::windows::core::Result<super::Devices::CameraStreamState>;
    fn GetPreviewFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn GetPreviewFrameCopyAsync(&self, destination: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn ThermalStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThermalStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ThermalStatus(&self) -> ::windows::core::Result<MediaCaptureThermalStatus>;
    fn PrepareAdvancedPhotoCaptureAsync(&self, encodingproperties: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture4 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture4Vtbl {
    pub const fn new<Impl: IMediaCapture4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture4Vtbl {
        unsafe extern "system" fn AddAudioEffectAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAudioEffectAsync(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddVideoEffectAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddVideoEffectAsync(&*(&definition as *const <super::Effects::IVideoEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IVideoEffectDefinition as ::windows::core::DefaultType>::DefaultType), mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseRecordAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseRecordAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeRecordAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeRecordAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraStreamStateChanged<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraStreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraStreamStateChanged<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraStreamState<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::CameraStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraStreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewFrameAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewFrameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewFrameCopyAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewFrameCopyAsync(&*(&destination as *const <super::VideoFrame as ::windows::core::Abi>::Abi as *const <super::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThermalStatusChanged<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThermalStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThermalStatusChanged<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveThermalStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ThermalStatus<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureThermalStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThermalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareAdvancedPhotoCaptureAsync<Impl: IMediaCapture4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareAdvancedPhotoCaptureAsync(&*(&encodingproperties as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaCapture4>,
            base.5,
            AddAudioEffectAsync::<Impl, OFFSET>,
            AddVideoEffectAsync::<Impl, OFFSET>,
            PauseRecordAsync::<Impl, OFFSET>,
            ResumeRecordAsync::<Impl, OFFSET>,
            CameraStreamStateChanged::<Impl, OFFSET>,
            RemoveCameraStreamStateChanged::<Impl, OFFSET>,
            CameraStreamState::<Impl, OFFSET>,
            GetPreviewFrameAsync::<Impl, OFFSET>,
            GetPreviewFrameCopyAsync::<Impl, OFFSET>,
            ThermalStatusChanged::<Impl, OFFSET>,
            RemoveThermalStatusChanged::<Impl, OFFSET>,
            ThermalStatus::<Impl, OFFSET>,
            PrepareAdvancedPhotoCaptureAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture5Impl: Sized {
    fn RemoveEffectAsync(&self, effect: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopRecordWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
    fn FrameSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>>;
    fn CreateFrameReaderAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAndSizeAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING, outputsize: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture5 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture5";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture5Vtbl {
    pub const fn new<Impl: IMediaCapture5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture5Vtbl {
        unsafe extern "system" fn RemoveEffectAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveEffectAsync(&*(&effect as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseRecordWithResultAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseRecordWithResultAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRecordWithResultAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopRecordWithResultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameSources<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderAsync(&*(&inputsource as *const <Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithSubtypeAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderWithSubtypeAsync(&*(&inputsource as *const <Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType), &*(&outputsubtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithSubtypeAndSizeAsync<Impl: IMediaCapture5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderWithSubtypeAndSizeAsync(
                &*(&inputsource as *const <Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType),
                &*(&outputsubtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&outputsize as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapture5>, base.5, RemoveEffectAsync::<Impl, OFFSET>, PauseRecordWithResultAsync::<Impl, OFFSET>, StopRecordWithResultAsync::<Impl, OFFSET>, FrameSources::<Impl, OFFSET>, CreateFrameReaderAsync::<Impl, OFFSET>, CreateFrameReaderWithSubtypeAsync::<Impl, OFFSET>, CreateFrameReaderWithSubtypeAndSizeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture6Impl: Sized {
    fn CaptureDeviceExclusiveControlStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureDeviceExclusiveControlStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateMultiSourceFrameReaderAsync(&self, inputsources: &::core::option::Option<super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture6 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture6";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture6Vtbl {
    pub const fn new<Impl: IMediaCapture6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture6Vtbl {
        unsafe extern "system" fn CaptureDeviceExclusiveControlStatusChanged<Impl: IMediaCapture6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureDeviceExclusiveControlStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureDeviceExclusiveControlStatusChanged<Impl: IMediaCapture6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCaptureDeviceExclusiveControlStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateMultiSourceFrameReaderAsync<Impl: IMediaCapture6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputsources: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMultiSourceFrameReaderAsync(&*(&inputsources as *const <super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapture6>, base.5, CaptureDeviceExclusiveControlStatusChanged::<Impl, OFFSET>, RemoveCaptureDeviceExclusiveControlStatusChanged::<Impl, OFFSET>, CreateMultiSourceFrameReaderAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture7Impl: Sized {
    fn CreateRelativePanelWatcher(&self, capturemode: StreamingCaptureMode, displayregion: &::core::option::Option<super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<MediaCaptureRelativePanelWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapture7 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture7";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapture7Vtbl {
    pub const fn new<Impl: IMediaCapture7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapture7Vtbl {
        unsafe extern "system" fn CreateRelativePanelWatcher<Impl: IMediaCapture7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capturemode: StreamingCaptureMode, displayregion: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRelativePanelWatcher(capturemode, &*(&displayregion as *const <super::super::UI::WindowManagement::DisplayRegion as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowManagement::DisplayRegion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapture7>, base.5, CreateRelativePanelWatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<MediaCaptureDeviceExclusiveControlStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsVtbl {
    pub const fn new<Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs>, base.5, DeviceId::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureFailedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Code(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureFailedEventArgsVtbl {
    pub const fn new<Impl: IMediaCaptureFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureFailedEventArgsVtbl {
        unsafe extern "system" fn Message<Impl: IMediaCaptureFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Impl: IMediaCaptureFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureFailedEventArgs>, base.5, Message::<Impl, OFFSET>, Code::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureFocusChangedEventArgsImpl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::Devices::MediaCaptureFocusState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureFocusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureFocusChangedEventArgsVtbl {
    pub const fn new<Impl: IMediaCaptureFocusChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureFocusChangedEventArgsVtbl {
        unsafe extern "system" fn FocusState<Impl: IMediaCaptureFocusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureFocusChangedEventArgs>, base.5, FocusState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettingsImpl: Sized {
    fn SetAudioDeviceId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoDeviceId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows::core::Result<()>;
    fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows::core::Result<()>;
    fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettingsVtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettingsVtbl {
        unsafe extern "system" fn SetAudioDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioDeviceId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoDeviceId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamingCaptureMode<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStreamingCaptureMode(value).into()
        }
        unsafe extern "system" fn StreamingCaptureMode<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamingCaptureMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotoCaptureSource<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPhotoCaptureSource(value).into()
        }
        unsafe extern "system" fn PhotoCaptureSource<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoCaptureSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings>, base.5, SetAudioDeviceId::<Impl, OFFSET>, AudioDeviceId::<Impl, OFFSET>, SetVideoDeviceId::<Impl, OFFSET>, VideoDeviceId::<Impl, OFFSET>, SetStreamingCaptureMode::<Impl, OFFSET>, StreamingCaptureMode::<Impl, OFFSET>, SetPhotoCaptureSource::<Impl, OFFSET>, PhotoCaptureSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings2Impl: Sized {
    fn SetMediaCategory(&self, value: MediaCategory) -> ::windows::core::Result<()>;
    fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory>;
    fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()>;
    fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings2Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings2Vtbl {
        unsafe extern "system" fn SetMediaCategory<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMediaCategory(value).into()
        }
        unsafe extern "system" fn MediaCategory<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioProcessing<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioProcessing(value).into()
        }
        unsafe extern "system" fn AudioProcessing<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings2>, base.5, SetMediaCategory::<Impl, OFFSET>, MediaCategory::<Impl, OFFSET>, SetAudioProcessing::<Impl, OFFSET>, AudioProcessing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings3Impl: Sized {
    fn SetAudioSource(&self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn SetVideoSource(&self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings3Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings3Vtbl {
        unsafe extern "system" fn SetAudioSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioSource(&*(&value as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoSource(&*(&value as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings3>, base.5, SetAudioSource::<Impl, OFFSET>, AudioSource::<Impl, OFFSET>, SetVideoSource::<Impl, OFFSET>, VideoSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings4Impl: Sized {
    fn VideoProfile(&self) -> ::windows::core::Result<MediaCaptureVideoProfile>;
    fn SetVideoProfile(&self, value: &::core::option::Option<MediaCaptureVideoProfile>) -> ::windows::core::Result<()>;
    fn PreviewMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPreviewMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn RecordMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetRecordMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn PhotoMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPhotoMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings4 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings4Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings4Vtbl {
        unsafe extern "system" fn VideoProfile<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoProfile<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoProfile(&*(&value as *const <MediaCaptureVideoProfile as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreviewMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecordMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRecordMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhotoMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotoMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPhotoMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings4>, base.5, VideoProfile::<Impl, OFFSET>, SetVideoProfile::<Impl, OFFSET>, PreviewMediaDescription::<Impl, OFFSET>, SetPreviewMediaDescription::<Impl, OFFSET>, RecordMediaDescription::<Impl, OFFSET>, SetRecordMediaDescription::<Impl, OFFSET>, PhotoMediaDescription::<Impl, OFFSET>, SetPhotoMediaDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings5Impl: Sized {
    fn SourceGroup(&self) -> ::windows::core::Result<Frames::MediaFrameSourceGroup>;
    fn SetSourceGroup(&self, value: &::core::option::Option<Frames::MediaFrameSourceGroup>) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<MediaCaptureSharingMode>;
    fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows::core::Result<()>;
    fn MemoryPreference(&self) -> ::windows::core::Result<MediaCaptureMemoryPreference>;
    fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings5 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings5";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings5Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings5Vtbl {
        unsafe extern "system" fn SourceGroup<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceGroup<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSourceGroup(&*(&value as *const <Frames::MediaFrameSourceGroup as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSourceGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SharingMode<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharingMode<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaCaptureSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        unsafe extern "system" fn MemoryPreference<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureMemoryPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MemoryPreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemoryPreference<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaCaptureMemoryPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMemoryPreference(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings5>, base.5, SourceGroup::<Impl, OFFSET>, SetSourceGroup::<Impl, OFFSET>, SharingMode::<Impl, OFFSET>, SetSharingMode::<Impl, OFFSET>, MemoryPreference::<Impl, OFFSET>, SetMemoryPreference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings6Impl: Sized {
    fn AlwaysPlaySystemShutterSound(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings6 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings6";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings6Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings6Vtbl {
        unsafe extern "system" fn AlwaysPlaySystemShutterSound<Impl: IMediaCaptureInitializationSettings6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysPlaySystemShutterSound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysPlaySystemShutterSound<Impl: IMediaCaptureInitializationSettings6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlwaysPlaySystemShutterSound(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings6>, base.5, AlwaysPlaySystemShutterSound::<Impl, OFFSET>, SetAlwaysPlaySystemShutterSound::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings7Impl: Sized {
    fn DeviceUriPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetDeviceUriPasswordCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn DeviceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetDeviceUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings7 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings7";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings7Vtbl {
    pub const fn new<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureInitializationSettings7Vtbl {
        unsafe extern "system" fn DeviceUriPasswordCredential<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceUriPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceUriPasswordCredential<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeviceUriPasswordCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceUri<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceUri<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDeviceUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureInitializationSettings7>, base.5, DeviceUriPasswordCredential::<Impl, OFFSET>, SetDeviceUriPasswordCredential::<Impl, OFFSET>, DeviceUri::<Impl, OFFSET>, SetDeviceUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapturePauseResultImpl: Sized {
    fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapturePauseResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCapturePauseResultVtbl {
    pub const fn new<Impl: IMediaCapturePauseResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCapturePauseResultVtbl {
        unsafe extern "system" fn LastFrame<Impl: IMediaCapturePauseResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDuration<Impl: IMediaCapturePauseResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCapturePauseResult>, base.5, LastFrame::<Impl, OFFSET>, RecordDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureRelativePanelWatcherImpl: Sized {
    fn RelativePanel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureRelativePanelWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureRelativePanelWatcherVtbl {
    pub const fn new<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureRelativePanelWatcherVtbl {
        unsafe extern "system" fn RelativePanel<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativePanel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureRelativePanelWatcher>, base.5, RelativePanel::<Impl, OFFSET>, Changed::<Impl, OFFSET>, RemoveChanged::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettingsImpl: Sized {
    fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource>;
    fn VideoDeviceCharacteristic(&self) -> ::windows::core::Result<VideoDeviceCharacteristic>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureSettingsVtbl {
    pub const fn new<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureSettingsVtbl {
        unsafe extern "system" fn AudioDeviceId<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamingCaptureMode<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamingCaptureMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoCaptureSource<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoCaptureSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceCharacteristic<Impl: IMediaCaptureSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceCharacteristic) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceCharacteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureSettings>, base.5, AudioDeviceId::<Impl, OFFSET>, VideoDeviceId::<Impl, OFFSET>, StreamingCaptureMode::<Impl, OFFSET>, PhotoCaptureSource::<Impl, OFFSET>, VideoDeviceCharacteristic::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettings2Impl: Sized {
    fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows::core::Result<bool>;
    fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows::core::Result<bool>;
    fn CameraSoundRequiredForRegion(&self) -> ::windows::core::Result<bool>;
    fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PitchOffsetDegrees(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Vertical35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory>;
    fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureSettings2Vtbl {
    pub const fn new<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureSettings2Vtbl {
        unsafe extern "system" fn ConcurrentRecordAndPhotoSupported<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConcurrentRecordAndPhotoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConcurrentRecordAndPhotoSequenceSupported<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConcurrentRecordAndPhotoSequenceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraSoundRequiredForRegion<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraSoundRequiredForRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Horizontal35mmEquivalentFocalLength<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Horizontal35mmEquivalentFocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchOffsetDegrees<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PitchOffsetDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vertical35mmEquivalentFocalLength<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Vertical35mmEquivalentFocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCategory<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioProcessing<Impl: IMediaCaptureSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureSettings2>, base.5, ConcurrentRecordAndPhotoSupported::<Impl, OFFSET>, ConcurrentRecordAndPhotoSequenceSupported::<Impl, OFFSET>, CameraSoundRequiredForRegion::<Impl, OFFSET>, Horizontal35mmEquivalentFocalLength::<Impl, OFFSET>, PitchOffsetDegrees::<Impl, OFFSET>, Vertical35mmEquivalentFocalLength::<Impl, OFFSET>, MediaCategory::<Impl, OFFSET>, AudioProcessing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettings3Impl: Sized {
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureSettings3Vtbl {
    pub const fn new<Impl: IMediaCaptureSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureSettings3Vtbl {
        unsafe extern "system" fn Direct3D11Device<Impl: IMediaCaptureSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3D11Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureSettings3>, base.5, Direct3D11Device::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureStaticsImpl: Sized {
    fn IsVideoProfileSupported(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn FindAllVideoProfiles(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindConcurrentProfiles(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindKnownVideoProfiles(&self, videodeviceid: &::windows::core::HSTRING, name: KnownVideoProfile) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureStaticsVtbl {
    pub const fn new<Impl: IMediaCaptureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureStaticsVtbl {
        unsafe extern "system" fn IsVideoProfileSupported<Impl: IMediaCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVideoProfileSupported(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllVideoProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllVideoProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConcurrentProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindConcurrentProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindKnownVideoProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindKnownVideoProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), name) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureStatics>, base.5, IsVideoProfileSupported::<Impl, OFFSET>, FindAllVideoProfiles::<Impl, OFFSET>, FindConcurrentProfiles::<Impl, OFFSET>, FindKnownVideoProfiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureStopResultImpl: Sized {
    fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureStopResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureStopResultVtbl {
    pub const fn new<Impl: IMediaCaptureStopResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureStopResultVtbl {
        unsafe extern "system" fn LastFrame<Impl: IMediaCaptureStopResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDuration<Impl: IMediaCaptureStopResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureStopResult>, base.5, LastFrame::<Impl, OFFSET>, RecordDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoPreviewImpl: Sized {
    fn StartPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoPreview {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoPreview";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoPreviewVtbl {
    pub const fn new<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVideoPreviewVtbl {
        unsafe extern "system" fn StartPreviewAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPreviewToCustomSinkAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPreviewToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPreviewToCustomSinkIdAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPreviewToCustomSinkIdAsync(
                &*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinkactivationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&customsinksettings as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopPreviewAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureVideoPreview>, base.5, StartPreviewAsync::<Impl, OFFSET>, StartPreviewToCustomSinkAsync::<Impl, OFFSET>, StartPreviewToCustomSinkIdAsync::<Impl, OFFSET>, StopPreviewAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedPreviewMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedRecordMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedPhotoMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn GetConcurrency(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoProfileVtbl {
    pub const fn new<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVideoProfileVtbl {
        unsafe extern "system" fn Id<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPreviewMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedPreviewMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedRecordMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedRecordMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPhotoMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedPhotoMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConcurrency<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConcurrency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureVideoProfile>, base.5, Id::<Impl, OFFSET>, VideoDeviceId::<Impl, OFFSET>, SupportedPreviewMediaDescription::<Impl, OFFSET>, SupportedRecordMediaDescription::<Impl, OFFSET>, SupportedPhotoMediaDescription::<Impl, OFFSET>, GetConcurrency::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfile2Impl: Sized {
    fn FrameSourceInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfile2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoProfile2Vtbl {
    pub const fn new<Impl: IMediaCaptureVideoProfile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVideoProfile2Vtbl {
        unsafe extern "system" fn FrameSourceInfos<Impl: IMediaCaptureVideoProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameSourceInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaCaptureVideoProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureVideoProfile2>, base.5, FrameSourceInfos::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileMediaDescriptionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn FrameRate(&self) -> ::windows::core::Result<f64>;
    fn IsVariablePhotoSequenceSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHdrVideoSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfileMediaDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoProfileMediaDescriptionVtbl {
    pub const fn new<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVideoProfileMediaDescriptionVtbl {
        unsafe extern "system" fn Width<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameRate<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVariablePhotoSequenceSupported<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVariablePhotoSequenceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHdrVideoSupported<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHdrVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureVideoProfileMediaDescription>, base.5, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>, FrameRate::<Impl, OFFSET>, IsVariablePhotoSequenceSupported::<Impl, OFFSET>, IsHdrVideoSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileMediaDescription2Impl: Sized {
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfileMediaDescription2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfileMediaDescription2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoProfileMediaDescription2Vtbl {
    pub const fn new<Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCaptureVideoProfileMediaDescription2Vtbl {
        unsafe extern "system" fn Subtype<Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCaptureVideoProfileMediaDescription2>, base.5, Subtype::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOptionalReferencePhotoCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IOptionalReferencePhotoCapturedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOptionalReferencePhotoCapturedEventArgsVtbl {
    pub const fn new<Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOptionalReferencePhotoCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOptionalReferencePhotoCapturedEventArgs>, base.5, Frame::<Impl, OFFSET>, Context::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IPhotoCapturedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoCapturedEventArgsVtbl {
    pub const fn new<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhotoCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureTimeOffset<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhotoCapturedEventArgs>, base.5, Frame::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>, CaptureTimeOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoConfirmationCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IPhotoConfirmationCapturedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPhotoConfirmationCapturedEventArgsVtbl {
    pub const fn new<Impl: IPhotoConfirmationCapturedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhotoConfirmationCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IPhotoConfirmationCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureTimeOffset<Impl: IPhotoConfirmationCapturedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptureTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhotoConfirmationCapturedEventArgs>, base.5, Frame::<Impl, OFFSET>, CaptureTimeOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenCaptureImpl: Sized {
    fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn IsAudioSuspended(&self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&self) -> ::windows::core::Result<bool>;
    fn SourceSuspensionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceSuspensionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.IScreenCapture";
}
#[cfg(feature = "implement_exclusive")]
impl IScreenCaptureVtbl {
    pub const fn new<Impl: IScreenCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScreenCaptureVtbl {
        unsafe extern "system" fn AudioSource<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoSource<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioSuspended<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAudioSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVideoSuspended<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVideoSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceSuspensionChanged<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceSuspensionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceSuspensionChanged<Impl: IScreenCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceSuspensionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScreenCapture>, base.5, AudioSource::<Impl, OFFSET>, VideoSource::<Impl, OFFSET>, IsAudioSuspended::<Impl, OFFSET>, IsVideoSuspended::<Impl, OFFSET>, SourceSuspensionChanged::<Impl, OFFSET>, RemoveSourceSuspensionChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ScreenCapture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScreenCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IScreenCaptureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScreenCaptureStaticsVtbl {
    pub const fn new<Impl: IScreenCaptureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScreenCaptureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IScreenCaptureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScreenCaptureStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISourceSuspensionChangedEventArgsImpl: Sized {
    fn IsAudioSuspended(&self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.ISourceSuspensionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISourceSuspensionChangedEventArgsVtbl {
    pub const fn new<Impl: ISourceSuspensionChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISourceSuspensionChangedEventArgsVtbl {
        unsafe extern "system" fn IsAudioSuspended<Impl: ISourceSuspensionChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAudioSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVideoSuspended<Impl: ISourceSuspensionChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVideoSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISourceSuspensionChangedEventArgs>, base.5, IsAudioSuspended::<Impl, OFFSET>, IsVideoSuspended::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamConfigurationImpl: Sized {
    fn InputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn OutputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.IVideoStreamConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStreamConfigurationVtbl {
    pub const fn new<Impl: IVideoStreamConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStreamConfigurationVtbl {
        unsafe extern "system" fn InputProperties<Impl: IVideoStreamConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputProperties<Impl: IVideoStreamConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStreamConfiguration>, base.5, InputProperties::<Impl, OFFSET>, OutputProperties::<Impl, OFFSET>)
    }
}

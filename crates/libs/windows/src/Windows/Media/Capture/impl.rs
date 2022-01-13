#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait IAdvancedCapturedPhotoImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn Mode(&mut self) -> ::windows::core::Result<super::Devices::AdvancedPhotoMode>;
    fn Context(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedCapturedPhoto";
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl IAdvancedCapturedPhotoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedCapturedPhotoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedCapturedPhotoVtbl {
        unsafe extern "system" fn Frame<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IAdvancedCapturedPhotoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedCapturedPhoto, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedCapturedPhoto as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdvancedCapturedPhoto2Impl: Sized {
    fn FrameBoundsRelativeToReferencePhoto(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedCapturedPhoto2 {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedCapturedPhoto2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdvancedCapturedPhoto2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedCapturedPhoto2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedCapturedPhoto2Vtbl {
        unsafe extern "system" fn FrameBoundsRelativeToReferencePhoto<Impl: IAdvancedCapturedPhoto2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameBoundsRelativeToReferencePhoto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedCapturedPhoto2, BASE_OFFSET>(),
            FrameBoundsRelativeToReferencePhoto: FrameBoundsRelativeToReferencePhoto::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedCapturedPhoto2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdvancedPhotoCaptureImpl: Sized {
    fn CaptureAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn CaptureWithContextAsync(&mut self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn OptionalReferencePhotoCaptured(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionalReferencePhotoCaptured(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllPhotosCaptured(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAllPhotosCaptured(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.IAdvancedPhotoCapture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdvancedPhotoCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdvancedPhotoCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdvancedPhotoCaptureVtbl {
        unsafe extern "system" fn CaptureAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureWithContextAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureWithContextAsync(&*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionalReferencePhotoCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalReferencePhotoCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionalReferencePhotoCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOptionalReferencePhotoCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllPhotosCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllPhotosCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllPhotosCaptured<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllPhotosCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FinishAsync<Impl: IAdvancedPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdvancedPhotoCapture, BASE_OFFSET>(),
            CaptureAsync: CaptureAsync::<Impl, IMPL_OFFSET>,
            CaptureWithContextAsync: CaptureWithContextAsync::<Impl, IMPL_OFFSET>,
            OptionalReferencePhotoCaptured: OptionalReferencePhotoCaptured::<Impl, IMPL_OFFSET>,
            RemoveOptionalReferencePhotoCaptured: RemoveOptionalReferencePhotoCaptured::<Impl, IMPL_OFFSET>,
            AllPhotosCaptured: AllPhotosCaptured::<Impl, IMPL_OFFSET>,
            RemoveAllPhotosCaptured: RemoveAllPhotosCaptured::<Impl, IMPL_OFFSET>,
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdvancedPhotoCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastBackgroundServiceImpl: Sized {
    fn SetPlugInState(&mut self, value: AppBroadcastPlugInState) -> ::windows::core::Result<()>;
    fn PlugInState(&mut self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn SetSignInInfo(&mut self, value: &::core::option::Option<AppBroadcastBackgroundServiceSignInInfo>) -> ::windows::core::Result<()>;
    fn SignInInfo(&mut self) -> ::windows::core::Result<AppBroadcastBackgroundServiceSignInInfo>;
    fn SetStreamInfo(&mut self, value: &::core::option::Option<AppBroadcastBackgroundServiceStreamInfo>) -> ::windows::core::Result<()>;
    fn StreamInfo(&mut self) -> ::windows::core::Result<AppBroadcastBackgroundServiceStreamInfo>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetViewerCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ViewerCount(&mut self) -> ::windows::core::Result<u32>;
    fn TerminateBroadcast(&mut self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::Result<()>;
    fn HeartbeatRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeartbeatRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TitleId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundService";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastBackgroundServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundServiceVtbl {
        unsafe extern "system" fn SetPlugInState<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlugInState(value).into()
        }
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignInInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignInInfo(&*(&value as *const <AppBroadcastBackgroundServiceSignInInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastBackgroundServiceSignInInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignInInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamInfo(&*(&value as *const <AppBroadcastBackgroundServiceStreamInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastBackgroundServiceStreamInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamInfo<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BroadcastTitle<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewerCount<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewerCount(value).into()
        }
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateBroadcast<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateBroadcast(reason, providerspecificreason).into()
        }
        unsafe extern "system" fn HeartbeatRequested<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartbeatRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeartbeatRequested<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeartbeatRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TitleId<Impl: IAppBroadcastBackgroundServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundService, BASE_OFFSET>(),
            SetPlugInState: SetPlugInState::<Impl, IMPL_OFFSET>,
            PlugInState: PlugInState::<Impl, IMPL_OFFSET>,
            SetSignInInfo: SetSignInInfo::<Impl, IMPL_OFFSET>,
            SignInInfo: SignInInfo::<Impl, IMPL_OFFSET>,
            SetStreamInfo: SetStreamInfo::<Impl, IMPL_OFFSET>,
            StreamInfo: StreamInfo::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            BroadcastTitle: BroadcastTitle::<Impl, IMPL_OFFSET>,
            SetViewerCount: SetViewerCount::<Impl, IMPL_OFFSET>,
            ViewerCount: ViewerCount::<Impl, IMPL_OFFSET>,
            TerminateBroadcast: TerminateBroadcast::<Impl, IMPL_OFFSET>,
            HeartbeatRequested: HeartbeatRequested::<Impl, IMPL_OFFSET>,
            RemoveHeartbeatRequested: RemoveHeartbeatRequested::<Impl, IMPL_OFFSET>,
            TitleId: TitleId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastBackgroundService2Impl: Sized {
    fn SetBroadcastTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastChannel(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastChannel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastTitleChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastTitleChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastLanguageChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastLanguageChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastChannelChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastChannelChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundService2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundService2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastBackgroundService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundService2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundService2Vtbl {
        unsafe extern "system" fn SetBroadcastTitle<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguage<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastLanguage<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBroadcastLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastChannel<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastChannel<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBroadcastChannel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastTitleChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastTitleChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastTitleChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastTitleChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguageChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguageChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastLanguageChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastLanguageChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastChannelChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastChannelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBroadcastChannelChanged<Impl: IAppBroadcastBackgroundService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBroadcastChannelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundService2, BASE_OFFSET>(),
            SetBroadcastTitle: SetBroadcastTitle::<Impl, IMPL_OFFSET>,
            BroadcastLanguage: BroadcastLanguage::<Impl, IMPL_OFFSET>,
            SetBroadcastLanguage: SetBroadcastLanguage::<Impl, IMPL_OFFSET>,
            BroadcastChannel: BroadcastChannel::<Impl, IMPL_OFFSET>,
            SetBroadcastChannel: SetBroadcastChannel::<Impl, IMPL_OFFSET>,
            BroadcastTitleChanged: BroadcastTitleChanged::<Impl, IMPL_OFFSET>,
            RemoveBroadcastTitleChanged: RemoveBroadcastTitleChanged::<Impl, IMPL_OFFSET>,
            BroadcastLanguageChanged: BroadcastLanguageChanged::<Impl, IMPL_OFFSET>,
            RemoveBroadcastLanguageChanged: RemoveBroadcastLanguageChanged::<Impl, IMPL_OFFSET>,
            BroadcastChannelChanged: BroadcastChannelChanged::<Impl, IMPL_OFFSET>,
            RemoveBroadcastChannelChanged: RemoveBroadcastChannelChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
pub trait IAppBroadcastBackgroundServiceSignInInfoImpl: Sized {
    fn SignInState(&mut self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn SetOAuthRequestUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthRequestUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetOAuthCallbackUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthCallbackUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&mut self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetUserName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignInStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignInStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceSignInInfo";
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
impl IAppBroadcastBackgroundServiceSignInInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundServiceSignInInfoVtbl {
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOAuthRequestUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOAuthRequestUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthRequestUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OAuthRequestUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOAuthCallbackUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOAuthCallbackUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthCallbackUri<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OAuthCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationResult<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignInStateChanged<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignInStateChanged<Impl: IAppBroadcastBackgroundServiceSignInInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSignInStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundServiceSignInInfo, BASE_OFFSET>(),
            SignInState: SignInState::<Impl, IMPL_OFFSET>,
            SetOAuthRequestUri: SetOAuthRequestUri::<Impl, IMPL_OFFSET>,
            OAuthRequestUri: OAuthRequestUri::<Impl, IMPL_OFFSET>,
            SetOAuthCallbackUri: SetOAuthCallbackUri::<Impl, IMPL_OFFSET>,
            OAuthCallbackUri: OAuthCallbackUri::<Impl, IMPL_OFFSET>,
            AuthenticationResult: AuthenticationResult::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SignInStateChanged: SignInStateChanged::<Impl, IMPL_OFFSET>,
            RemoveSignInStateChanged: RemoveSignInStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundServiceSignInInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastBackgroundServiceSignInInfo2Impl: Sized {
    fn UserNameChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserNameChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceSignInInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceSignInInfo2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastBackgroundServiceSignInInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundServiceSignInInfo2Vtbl {
        unsafe extern "system" fn UserNameChanged<Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserNameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserNameChanged<Impl: IAppBroadcastBackgroundServiceSignInInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundServiceSignInInfo2, BASE_OFFSET>(),
            UserNameChanged: UserNameChanged::<Impl, IMPL_OFFSET>,
            RemoveUserNameChanged: RemoveUserNameChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundServiceSignInInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastBackgroundServiceStreamInfoImpl: Sized {
    fn StreamState(&mut self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn SetDesiredVideoEncodingBitrate(&mut self, value: u64) -> ::windows::core::Result<()>;
    fn DesiredVideoEncodingBitrate(&mut self) -> ::windows::core::Result<u64>;
    fn SetBandwidthTestBitrate(&mut self, value: u64) -> ::windows::core::Result<()>;
    fn BandwidthTestBitrate(&mut self) -> ::windows::core::Result<u64>;
    fn SetAudioCodec(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioCodec(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastStreamReader(&mut self) -> ::windows::core::Result<AppBroadcastStreamReader>;
    fn StreamStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingResolutionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingBitrateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceStreamInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastBackgroundServiceStreamInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundServiceStreamInfoVtbl {
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredVideoEncodingBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn DesiredVideoEncodingBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBandwidthTestBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBandwidthTestBitrate(value).into()
        }
        unsafe extern "system" fn BandwidthTestBitrate<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BandwidthTestBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioCodec<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioCodec(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioCodec<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioCodec() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BroadcastStreamReader<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastStreamReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamStateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamStateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoEncodingResolutionChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoEncodingResolutionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoEncodingBitrateChanged<Impl: IAppBroadcastBackgroundServiceStreamInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoEncodingBitrateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundServiceStreamInfo, BASE_OFFSET>(),
            StreamState: StreamState::<Impl, IMPL_OFFSET>,
            SetDesiredVideoEncodingBitrate: SetDesiredVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            DesiredVideoEncodingBitrate: DesiredVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            SetBandwidthTestBitrate: SetBandwidthTestBitrate::<Impl, IMPL_OFFSET>,
            BandwidthTestBitrate: BandwidthTestBitrate::<Impl, IMPL_OFFSET>,
            SetAudioCodec: SetAudioCodec::<Impl, IMPL_OFFSET>,
            AudioCodec: AudioCodec::<Impl, IMPL_OFFSET>,
            BroadcastStreamReader: BroadcastStreamReader::<Impl, IMPL_OFFSET>,
            StreamStateChanged: StreamStateChanged::<Impl, IMPL_OFFSET>,
            RemoveStreamStateChanged: RemoveStreamStateChanged::<Impl, IMPL_OFFSET>,
            VideoEncodingResolutionChanged: VideoEncodingResolutionChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoEncodingResolutionChanged: RemoveVideoEncodingResolutionChanged::<Impl, IMPL_OFFSET>,
            VideoEncodingBitrateChanged: VideoEncodingBitrateChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoEncodingBitrateChanged: RemoveVideoEncodingBitrateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundServiceStreamInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceStreamInfo2Impl: Sized {
    fn ReportProblemWithStream(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastBackgroundServiceStreamInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastBackgroundServiceStreamInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastBackgroundServiceStreamInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastBackgroundServiceStreamInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastBackgroundServiceStreamInfo2Vtbl {
        unsafe extern "system" fn ReportProblemWithStream<Impl: IAppBroadcastBackgroundServiceStreamInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProblemWithStream().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastBackgroundServiceStreamInfo2, BASE_OFFSET>(),
            ReportProblemWithStream: ReportProblemWithStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastBackgroundServiceStreamInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastCameraCaptureStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastCameraCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastCameraCaptureStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastCameraCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastCameraCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastCameraCaptureStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastCameraCaptureStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastGlobalSettingsImpl: Sized {
    fn IsBroadcastEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&mut self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAudioCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsMicrophoneCaptureEnabledByDefault(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEchoCancellationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&mut self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&mut self) -> ::windows::core::Result<f64>;
    fn SetIsCameraCaptureEnabledByDefault(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCameraCaptureEnabledByDefault(&mut self) -> ::windows::core::Result<bool>;
    fn SetSelectedCameraId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectedCameraId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCameraOverlayLocation(&mut self, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::Result<()>;
    fn CameraOverlayLocation(&mut self) -> ::windows::core::Result<AppBroadcastCameraOverlayLocation>;
    fn SetCameraOverlaySize(&mut self, value: AppBroadcastCameraOverlaySize) -> ::windows::core::Result<()>;
    fn CameraOverlaySize(&mut self) -> ::windows::core::Result<AppBroadcastCameraOverlaySize>;
    fn SetIsCursorImageCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastGlobalSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastGlobalSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastGlobalSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastGlobalSettingsVtbl {
        unsafe extern "system" fn IsBroadcastEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBroadcastEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledByPolicy<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledByPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareEncoder<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasHardwareEncoder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAudioCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAudioCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAudioCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAudioCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEchoCancellationEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEchoCancellationEnabled(value).into()
        }
        unsafe extern "system" fn IsEchoCancellationEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEchoCancellationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAudioGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemAudioGain(value).into()
        }
        unsafe extern "system" fn SystemAudioGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemAudioGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrophoneGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMicrophoneGain(value).into()
        }
        unsafe extern "system" fn MicrophoneGain<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCameraCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCameraCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsCameraCaptureEnabledByDefault<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCameraCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedCameraId<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedCameraId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedCameraId<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedCameraId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCameraOverlayLocation<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCameraOverlayLocation(value).into()
        }
        unsafe extern "system" fn CameraOverlayLocation<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraOverlayLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCameraOverlaySize<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCameraOverlaySize(value).into()
        }
        unsafe extern "system" fn CameraOverlaySize<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraOverlaySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorImageCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCursorImageCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsCursorImageCaptureEnabled<Impl: IAppBroadcastGlobalSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCursorImageCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastGlobalSettings, BASE_OFFSET>(),
            IsBroadcastEnabled: IsBroadcastEnabled::<Impl, IMPL_OFFSET>,
            IsDisabledByPolicy: IsDisabledByPolicy::<Impl, IMPL_OFFSET>,
            IsGpuConstrained: IsGpuConstrained::<Impl, IMPL_OFFSET>,
            HasHardwareEncoder: HasHardwareEncoder::<Impl, IMPL_OFFSET>,
            SetIsAudioCaptureEnabled: SetIsAudioCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsAudioCaptureEnabled: IsAudioCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetIsMicrophoneCaptureEnabledByDefault: SetIsMicrophoneCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            IsMicrophoneCaptureEnabledByDefault: IsMicrophoneCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            SetIsEchoCancellationEnabled: SetIsEchoCancellationEnabled::<Impl, IMPL_OFFSET>,
            IsEchoCancellationEnabled: IsEchoCancellationEnabled::<Impl, IMPL_OFFSET>,
            SetSystemAudioGain: SetSystemAudioGain::<Impl, IMPL_OFFSET>,
            SystemAudioGain: SystemAudioGain::<Impl, IMPL_OFFSET>,
            SetMicrophoneGain: SetMicrophoneGain::<Impl, IMPL_OFFSET>,
            MicrophoneGain: MicrophoneGain::<Impl, IMPL_OFFSET>,
            SetIsCameraCaptureEnabledByDefault: SetIsCameraCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            IsCameraCaptureEnabledByDefault: IsCameraCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            SetSelectedCameraId: SetSelectedCameraId::<Impl, IMPL_OFFSET>,
            SelectedCameraId: SelectedCameraId::<Impl, IMPL_OFFSET>,
            SetCameraOverlayLocation: SetCameraOverlayLocation::<Impl, IMPL_OFFSET>,
            CameraOverlayLocation: CameraOverlayLocation::<Impl, IMPL_OFFSET>,
            SetCameraOverlaySize: SetCameraOverlaySize::<Impl, IMPL_OFFSET>,
            CameraOverlaySize: CameraOverlaySize::<Impl, IMPL_OFFSET>,
            SetIsCursorImageCaptureEnabled: SetIsCursorImageCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsCursorImageCaptureEnabled: IsCursorImageCaptureEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastGlobalSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastHeartbeatRequestedEventArgsImpl: Sized {
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastHeartbeatRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastHeartbeatRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastHeartbeatRequestedEventArgsVtbl {
        unsafe extern "system" fn SetHandled<Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Handled<Impl: IAppBroadcastHeartbeatRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastHeartbeatRequestedEventArgs, BASE_OFFSET>(),
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastHeartbeatRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastManagerStaticsImpl: Sized {
    fn GetGlobalSettings(&mut self) -> ::windows::core::Result<AppBroadcastGlobalSettings>;
    fn ApplyGlobalSettings(&mut self, value: &::core::option::Option<AppBroadcastGlobalSettings>) -> ::windows::core::Result<()>;
    fn GetProviderSettings(&mut self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn ApplyProviderSettings(&mut self, value: &::core::option::Option<AppBroadcastProviderSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastManagerStaticsVtbl {
        unsafe extern "system" fn GetGlobalSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyGlobalSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyGlobalSettings(&*(&value as *const <AppBroadcastGlobalSettings as ::windows::core::Abi>::Abi as *const <AppBroadcastGlobalSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetProviderSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyProviderSettings<Impl: IAppBroadcastManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyProviderSettings(&*(&value as *const <AppBroadcastProviderSettings as ::windows::core::Abi>::Abi as *const <AppBroadcastProviderSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastManagerStatics, BASE_OFFSET>(),
            GetGlobalSettings: GetGlobalSettings::<Impl, IMPL_OFFSET>,
            ApplyGlobalSettings: ApplyGlobalSettings::<Impl, IMPL_OFFSET>,
            GetProviderSettings: GetProviderSettings::<Impl, IMPL_OFFSET>,
            ApplyProviderSettings: ApplyProviderSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastMicrophoneCaptureStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastMicrophoneCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastMicrophoneCaptureStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastMicrophoneCaptureStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppBroadcastPlugInImpl: Sized {
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderSettings(&mut self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn Logo(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugIn";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppBroadcastPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPlugInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPlugInVtbl {
        unsafe extern "system" fn AppId<Impl: IAppBroadcastPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderSettings<Impl: IAppBroadcastPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logo<Impl: IAppBroadcastPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IAppBroadcastPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPlugIn, BASE_OFFSET>(),
            AppId: AppId::<Impl, IMPL_OFFSET>,
            ProviderSettings: ProviderSettings::<Impl, IMPL_OFFSET>,
            Logo: Logo::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppBroadcastPlugInManagerImpl: Sized {
    fn IsBroadcastProviderAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn PlugInList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>;
    fn DefaultPlugIn(&mut self) -> ::windows::core::Result<AppBroadcastPlugIn>;
    fn SetDefaultPlugIn(&mut self, value: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppBroadcastPlugInManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPlugInManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPlugInManagerVtbl {
        unsafe extern "system" fn IsBroadcastProviderAvailable<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBroadcastProviderAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugInList<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPlugIn<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultPlugIn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPlugIn<Impl: IAppBroadcastPlugInManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultPlugIn(&*(&value as *const <AppBroadcastPlugIn as ::windows::core::Abi>::Abi as *const <AppBroadcastPlugIn as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPlugInManager, BASE_OFFSET>(),
            IsBroadcastProviderAvailable: IsBroadcastProviderAvailable::<Impl, IMPL_OFFSET>,
            PlugInList: PlugInList::<Impl, IMPL_OFFSET>,
            DefaultPlugIn: DefaultPlugIn::<Impl, IMPL_OFFSET>,
            SetDefaultPlugIn: SetDefaultPlugIn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPlugInManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppBroadcastPlugInManagerStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AppBroadcastPlugInManager>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastPlugInManager>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInManagerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppBroadcastPlugInManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPlugInManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPlugInManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppBroadcastPlugInManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IAppBroadcastPlugInManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPlugInManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPlugInManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInStateChangedEventArgsImpl: Sized {
    fn PlugInState(&mut self) -> ::windows::core::Result<AppBroadcastPlugInState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPlugInStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPlugInStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPlugInStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPlugInStateChangedEventArgsVtbl {
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastPlugInStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPlugInStateChangedEventArgs, BASE_OFFSET>(),
            PlugInState: PlugInState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPlugInStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastPreviewImpl: Sized {
    fn StopPreview(&mut self) -> ::windows::core::Result<()>;
    fn PreviewState(&mut self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreviewStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewStreamReader(&mut self) -> ::windows::core::Result<AppBroadcastPreviewStreamReader>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreview";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPreviewVtbl {
        unsafe extern "system" fn StopPreview<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopPreview().into()
        }
        unsafe extern "system" fn PreviewState<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewStateChanged<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewStateChanged<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviewStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewStreamReader<Impl: IAppBroadcastPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewStreamReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPreview, BASE_OFFSET>(),
            StopPreview: StopPreview::<Impl, IMPL_OFFSET>,
            PreviewState: PreviewState::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            PreviewStateChanged: PreviewStateChanged::<Impl, IMPL_OFFSET>,
            RemovePreviewStateChanged: RemovePreviewStateChanged::<Impl, IMPL_OFFSET>,
            PreviewStreamReader: PreviewStreamReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStateChangedEventArgsImpl: Sized {
    fn PreviewState(&mut self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastPreviewStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPreviewStateChangedEventArgsVtbl {
        unsafe extern "system" fn PreviewState<Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppBroadcastPreviewStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPreviewStateChangedEventArgs, BASE_OFFSET>(),
            PreviewState: PreviewState::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPreviewStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IAppBroadcastPreviewStreamReaderImpl: Sized {
    fn VideoWidth(&mut self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&mut self) -> ::windows::core::Result<u32>;
    fn VideoStride(&mut self) -> ::windows::core::Result<u32>;
    fn VideoBitmapPixelFormat(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn VideoBitmapAlphaMode(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn TryGetNextVideoFrame(&mut self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoFrame>;
    fn VideoFrameArrived(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamReader";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IAppBroadcastPreviewStreamReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPreviewStreamReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPreviewStreamReaderVtbl {
        unsafe extern "system" fn VideoWidth<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoHeight<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoStride<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoStride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitmapPixelFormat<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoBitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitmapAlphaMode<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoBitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextVideoFrame<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetNextVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFrameArrived<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoFrameArrived<Impl: IAppBroadcastPreviewStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPreviewStreamReader, BASE_OFFSET>(),
            VideoWidth: VideoWidth::<Impl, IMPL_OFFSET>,
            VideoHeight: VideoHeight::<Impl, IMPL_OFFSET>,
            VideoStride: VideoStride::<Impl, IMPL_OFFSET>,
            VideoBitmapPixelFormat: VideoBitmapPixelFormat::<Impl, IMPL_OFFSET>,
            VideoBitmapAlphaMode: VideoBitmapAlphaMode::<Impl, IMPL_OFFSET>,
            TryGetNextVideoFrame: TryGetNextVideoFrame::<Impl, IMPL_OFFSET>,
            VideoFrameArrived: VideoFrameArrived::<Impl, IMPL_OFFSET>,
            RemoveVideoFrameArrived: RemoveVideoFrameArrived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPreviewStreamReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppBroadcastPreviewStreamVideoFrameImpl: Sized {
    fn VideoHeader(&mut self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoHeader>;
    fn VideoBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamVideoFrame";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppBroadcastPreviewStreamVideoFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPreviewStreamVideoFrameVtbl {
        unsafe extern "system" fn VideoHeader<Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBuffer<Impl: IAppBroadcastPreviewStreamVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPreviewStreamVideoFrame, BASE_OFFSET>(),
            VideoHeader: VideoHeader::<Impl, IMPL_OFFSET>,
            VideoBuffer: VideoBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPreviewStreamVideoFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastPreviewStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FrameId(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastPreviewStreamVideoHeader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastPreviewStreamVideoHeaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastPreviewStreamVideoHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastPreviewStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastPreviewStreamVideoHeader, BASE_OFFSET>(),
            AbsoluteTimestamp: AbsoluteTimestamp::<Impl, IMPL_OFFSET>,
            RelativeTimestamp: RelativeTimestamp::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            FrameId: FrameId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastPreviewStreamVideoHeader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastProviderSettingsImpl: Sized {
    fn SetDefaultBroadcastTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultBroadcastTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAudioEncodingBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetVideoEncodingBitrateMode(&mut self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&mut self) -> ::windows::core::Result<AppBroadcastVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&mut self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&mut self) -> ::windows::core::Result<AppBroadcastVideoEncodingResolutionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastProviderSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastProviderSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastProviderSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastProviderSettingsVtbl {
        unsafe extern "system" fn SetDefaultBroadcastTitle<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultBroadcastTitle<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultBroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEncodingBitrate(value).into()
        }
        unsafe extern "system" fn AudioEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingBitrate<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingHeight<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingHeight(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingHeight<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingWidth<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingWidth(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingWidth<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingBitrateMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingBitrateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingResolutionMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingResolutionMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionMode<Impl: IAppBroadcastProviderSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastProviderSettings, BASE_OFFSET>(),
            SetDefaultBroadcastTitle: SetDefaultBroadcastTitle::<Impl, IMPL_OFFSET>,
            DefaultBroadcastTitle: DefaultBroadcastTitle::<Impl, IMPL_OFFSET>,
            SetAudioEncodingBitrate: SetAudioEncodingBitrate::<Impl, IMPL_OFFSET>,
            AudioEncodingBitrate: AudioEncodingBitrate::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingBitrate: SetCustomVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingBitrate: CustomVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingHeight: SetCustomVideoEncodingHeight::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingHeight: CustomVideoEncodingHeight::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingWidth: SetCustomVideoEncodingWidth::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingWidth: CustomVideoEncodingWidth::<Impl, IMPL_OFFSET>,
            SetVideoEncodingBitrateMode: SetVideoEncodingBitrateMode::<Impl, IMPL_OFFSET>,
            VideoEncodingBitrateMode: VideoEncodingBitrateMode::<Impl, IMPL_OFFSET>,
            SetVideoEncodingResolutionMode: SetVideoEncodingResolutionMode::<Impl, IMPL_OFFSET>,
            VideoEncodingResolutionMode: VideoEncodingResolutionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastProviderSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastServicesImpl: Sized {
    fn CaptureTargetType(&mut self) -> ::windows::core::Result<AppBroadcastCaptureTargetType>;
    fn SetCaptureTargetType(&mut self, value: AppBroadcastCaptureTargetType) -> ::windows::core::Result<()>;
    fn BroadcastTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanCapture(&mut self) -> ::windows::core::Result<bool>;
    fn EnterBroadcastModeAsync(&mut self, plugin: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn ExitBroadcastMode(&mut self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::Result<()>;
    fn StartBroadcast(&mut self) -> ::windows::core::Result<()>;
    fn PauseBroadcast(&mut self) -> ::windows::core::Result<()>;
    fn ResumeBroadcast(&mut self) -> ::windows::core::Result<()>;
    fn StartPreview(&mut self, desiredsize: &super::super::Foundation::Size) -> ::windows::core::Result<AppBroadcastPreview>;
    fn State(&mut self) -> ::windows::core::Result<AppBroadcastState>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastServices";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastServicesVtbl {
        unsafe extern "system" fn CaptureTargetType<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureTargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaptureTargetType<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaptureTargetType(value).into()
        }
        unsafe extern "system" fn BroadcastTitle<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastTitle<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBroadcastTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BroadcastLanguage<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBroadcastLanguage<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBroadcastLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCapture<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterBroadcastModeAsync<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterBroadcastModeAsync(&*(&plugin as *const <AppBroadcastPlugIn as ::windows::core::Abi>::Abi as *const <AppBroadcastPlugIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitBroadcastMode<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExitBroadcastMode(reason).into()
        }
        unsafe extern "system" fn StartBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartBroadcast().into()
        }
        unsafe extern "system" fn PauseBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseBroadcast().into()
        }
        unsafe extern "system" fn ResumeBroadcast<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeBroadcast().into()
        }
        unsafe extern "system" fn StartPreview<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPreview(&*(&desiredsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAppBroadcastServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastServices, BASE_OFFSET>(),
            CaptureTargetType: CaptureTargetType::<Impl, IMPL_OFFSET>,
            SetCaptureTargetType: SetCaptureTargetType::<Impl, IMPL_OFFSET>,
            BroadcastTitle: BroadcastTitle::<Impl, IMPL_OFFSET>,
            SetBroadcastTitle: SetBroadcastTitle::<Impl, IMPL_OFFSET>,
            BroadcastLanguage: BroadcastLanguage::<Impl, IMPL_OFFSET>,
            SetBroadcastLanguage: SetBroadcastLanguage::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            CanCapture: CanCapture::<Impl, IMPL_OFFSET>,
            EnterBroadcastModeAsync: EnterBroadcastModeAsync::<Impl, IMPL_OFFSET>,
            ExitBroadcastMode: ExitBroadcastMode::<Impl, IMPL_OFFSET>,
            StartBroadcast: StartBroadcast::<Impl, IMPL_OFFSET>,
            PauseBroadcast: PauseBroadcast::<Impl, IMPL_OFFSET>,
            ResumeBroadcast: ResumeBroadcast::<Impl, IMPL_OFFSET>,
            StartPreview: StartPreview::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastSignInStateChangedEventArgsImpl: Sized {
    fn SignInState(&mut self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn Result(&mut self) -> ::windows::core::Result<AppBroadcastSignInResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastSignInStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastSignInStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastSignInStateChangedEventArgsVtbl {
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IAppBroadcastSignInStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastSignInStateChangedEventArgs, BASE_OFFSET>(),
            SignInState: SignInState::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastSignInStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
pub trait IAppBroadcastStateImpl: Sized {
    fn IsCaptureTargetRunning(&mut self) -> ::windows::core::Result<bool>;
    fn ViewerCount(&mut self) -> ::windows::core::Result<u32>;
    fn ShouldCaptureMicrophone(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&mut self) -> ::windows::core::Result<()>;
    fn ShouldCaptureCamera(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureCamera(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RestartCameraCapture(&mut self) -> ::windows::core::Result<()>;
    fn EncodedVideoSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MicrophoneCaptureState(&mut self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&mut self) -> ::windows::core::Result<u32>;
    fn CameraCaptureState(&mut self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn CameraCaptureError(&mut self) -> ::windows::core::Result<u32>;
    fn StreamState(&mut self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn PlugInState(&mut self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn OAuthRequestUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OAuthCallbackUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&mut self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetAuthenticationResult(&mut self, value: &::core::option::Option<super::super::Security::Authentication::Web::WebAuthenticationResult>) -> ::windows::core::Result<()>;
    fn SetSignInState(&mut self, value: AppBroadcastSignInState) -> ::windows::core::Result<()>;
    fn SignInState(&mut self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn TerminationReason(&mut self) -> ::windows::core::Result<AppBroadcastTerminationReason>;
    fn TerminationReasonPlugInSpecific(&mut self) -> ::windows::core::Result<u32>;
    fn ViewerCountChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewerCountChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraCaptureStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraCaptureStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlugInStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlugInStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StreamStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastState";
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web", feature = "implement_exclusive"))]
impl IAppBroadcastStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStateVtbl {
        unsafe extern "system" fn IsCaptureTargetRunning<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCaptureTargetRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldCaptureMicrophone<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureMicrophone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureMicrophone<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureMicrophone(value).into()
        }
        unsafe extern "system" fn RestartMicrophoneCapture<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartMicrophoneCapture().into()
        }
        unsafe extern "system" fn ShouldCaptureCamera<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureCamera<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureCamera(value).into()
        }
        unsafe extern "system" fn RestartCameraCapture<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartCameraCapture().into()
        }
        unsafe extern "system" fn EncodedVideoSize<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodedVideoSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureError<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraCaptureState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraCaptureError<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugInState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OAuthRequestUri<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OAuthRequestUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OAuthCallbackUri<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OAuthCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationResult<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationResult<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationResult(&*(&value as *const <super::super::Security::Authentication::Web::WebAuthenticationResult as ::windows::core::Abi>::Abi as *const <super::super::Security::Authentication::Web::WebAuthenticationResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSignInState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignInState(value).into()
        }
        unsafe extern "system" fn SignInState<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminationReason<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastTerminationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminationReasonPlugInSpecific<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminationReasonPlugInSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewerCountChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewerCountChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveViewerCountChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveViewerCountChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MicrophoneCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMicrophoneCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMicrophoneCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraCaptureStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlugInStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlugInStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlugInStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamStateChanged<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureTargetClosed<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureTargetClosed(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureTargetClosed<Impl: IAppBroadcastStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCaptureTargetClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastState, BASE_OFFSET>(),
            IsCaptureTargetRunning: IsCaptureTargetRunning::<Impl, IMPL_OFFSET>,
            ViewerCount: ViewerCount::<Impl, IMPL_OFFSET>,
            ShouldCaptureMicrophone: ShouldCaptureMicrophone::<Impl, IMPL_OFFSET>,
            SetShouldCaptureMicrophone: SetShouldCaptureMicrophone::<Impl, IMPL_OFFSET>,
            RestartMicrophoneCapture: RestartMicrophoneCapture::<Impl, IMPL_OFFSET>,
            ShouldCaptureCamera: ShouldCaptureCamera::<Impl, IMPL_OFFSET>,
            SetShouldCaptureCamera: SetShouldCaptureCamera::<Impl, IMPL_OFFSET>,
            RestartCameraCapture: RestartCameraCapture::<Impl, IMPL_OFFSET>,
            EncodedVideoSize: EncodedVideoSize::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureState: MicrophoneCaptureState::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureError: MicrophoneCaptureError::<Impl, IMPL_OFFSET>,
            CameraCaptureState: CameraCaptureState::<Impl, IMPL_OFFSET>,
            CameraCaptureError: CameraCaptureError::<Impl, IMPL_OFFSET>,
            StreamState: StreamState::<Impl, IMPL_OFFSET>,
            PlugInState: PlugInState::<Impl, IMPL_OFFSET>,
            OAuthRequestUri: OAuthRequestUri::<Impl, IMPL_OFFSET>,
            OAuthCallbackUri: OAuthCallbackUri::<Impl, IMPL_OFFSET>,
            AuthenticationResult: AuthenticationResult::<Impl, IMPL_OFFSET>,
            SetAuthenticationResult: SetAuthenticationResult::<Impl, IMPL_OFFSET>,
            SetSignInState: SetSignInState::<Impl, IMPL_OFFSET>,
            SignInState: SignInState::<Impl, IMPL_OFFSET>,
            TerminationReason: TerminationReason::<Impl, IMPL_OFFSET>,
            TerminationReasonPlugInSpecific: TerminationReasonPlugInSpecific::<Impl, IMPL_OFFSET>,
            ViewerCountChanged: ViewerCountChanged::<Impl, IMPL_OFFSET>,
            RemoveViewerCountChanged: RemoveViewerCountChanged::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureStateChanged: MicrophoneCaptureStateChanged::<Impl, IMPL_OFFSET>,
            RemoveMicrophoneCaptureStateChanged: RemoveMicrophoneCaptureStateChanged::<Impl, IMPL_OFFSET>,
            CameraCaptureStateChanged: CameraCaptureStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCameraCaptureStateChanged: RemoveCameraCaptureStateChanged::<Impl, IMPL_OFFSET>,
            PlugInStateChanged: PlugInStateChanged::<Impl, IMPL_OFFSET>,
            RemovePlugInStateChanged: RemovePlugInStateChanged::<Impl, IMPL_OFFSET>,
            StreamStateChanged: StreamStateChanged::<Impl, IMPL_OFFSET>,
            RemoveStreamStateChanged: RemoveStreamStateChanged::<Impl, IMPL_OFFSET>,
            CaptureTargetClosed: CaptureTargetClosed::<Impl, IMPL_OFFSET>,
            RemoveCaptureTargetClosed: RemoveCaptureTargetClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppBroadcastStreamAudioFrameImpl: Sized {
    fn AudioHeader(&mut self) -> ::windows::core::Result<AppBroadcastStreamAudioHeader>;
    fn AudioBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamAudioFrame";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppBroadcastStreamAudioFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamAudioFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamAudioFrameVtbl {
        unsafe extern "system" fn AudioHeader<Impl: IAppBroadcastStreamAudioFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioBuffer<Impl: IAppBroadcastStreamAudioFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamAudioFrame, BASE_OFFSET>(),
            AudioHeader: AudioHeader::<Impl, IMPL_OFFSET>,
            AudioBuffer: AudioBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamAudioFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastStreamAudioHeaderImpl: Sized {
    fn AbsoluteTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn HasDiscontinuity(&mut self) -> ::windows::core::Result<bool>;
    fn FrameId(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamAudioHeader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastStreamAudioHeaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamAudioHeaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamAudioHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDiscontinuity<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasDiscontinuity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastStreamAudioHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamAudioHeader, BASE_OFFSET>(),
            AbsoluteTimestamp: AbsoluteTimestamp::<Impl, IMPL_OFFSET>,
            RelativeTimestamp: RelativeTimestamp::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            HasDiscontinuity: HasDiscontinuity::<Impl, IMPL_OFFSET>,
            FrameId: FrameId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamAudioHeader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppBroadcastStreamReaderImpl: Sized {
    fn AudioChannels(&mut self) -> ::windows::core::Result<u32>;
    fn AudioSampleRate(&mut self) -> ::windows::core::Result<u32>;
    fn AudioAacSequence(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AudioBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn TryGetNextAudioFrame(&mut self) -> ::windows::core::Result<AppBroadcastStreamAudioFrame>;
    fn VideoWidth(&mut self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&mut self) -> ::windows::core::Result<u32>;
    fn VideoBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn TryGetNextVideoFrame(&mut self) -> ::windows::core::Result<AppBroadcastStreamVideoFrame>;
    fn AudioFrameArrived(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoFrameArrived(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamReader";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppBroadcastStreamReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamReaderVtbl {
        unsafe extern "system" fn AudioChannels<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioChannels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSampleRate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSampleRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioAacSequence<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioAacSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioBitrate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextAudioFrame<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetNextAudioFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoWidth<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoHeight<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBitrate<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetNextVideoFrame<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetNextVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrameArrived(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoFrameArrived<Impl: IAppBroadcastStreamReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamReader, BASE_OFFSET>(),
            AudioChannels: AudioChannels::<Impl, IMPL_OFFSET>,
            AudioSampleRate: AudioSampleRate::<Impl, IMPL_OFFSET>,
            AudioAacSequence: AudioAacSequence::<Impl, IMPL_OFFSET>,
            AudioBitrate: AudioBitrate::<Impl, IMPL_OFFSET>,
            TryGetNextAudioFrame: TryGetNextAudioFrame::<Impl, IMPL_OFFSET>,
            VideoWidth: VideoWidth::<Impl, IMPL_OFFSET>,
            VideoHeight: VideoHeight::<Impl, IMPL_OFFSET>,
            VideoBitrate: VideoBitrate::<Impl, IMPL_OFFSET>,
            TryGetNextVideoFrame: TryGetNextVideoFrame::<Impl, IMPL_OFFSET>,
            AudioFrameArrived: AudioFrameArrived::<Impl, IMPL_OFFSET>,
            RemoveAudioFrameArrived: RemoveAudioFrameArrived::<Impl, IMPL_OFFSET>,
            VideoFrameArrived: VideoFrameArrived::<Impl, IMPL_OFFSET>,
            RemoveVideoFrameArrived: RemoveVideoFrameArrived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamStateChangedEventArgsImpl: Sized {
    fn StreamState(&mut self) -> ::windows::core::Result<AppBroadcastStreamState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastStreamStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamStateChangedEventArgsVtbl {
        unsafe extern "system" fn StreamState<Impl: IAppBroadcastStreamStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamStateChangedEventArgs, BASE_OFFSET>(),
            StreamState: StreamState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppBroadcastStreamVideoFrameImpl: Sized {
    fn VideoHeader(&mut self) -> ::windows::core::Result<AppBroadcastStreamVideoHeader>;
    fn VideoBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamVideoFrame";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppBroadcastStreamVideoFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamVideoFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamVideoFrameVtbl {
        unsafe extern "system" fn VideoHeader<Impl: IAppBroadcastStreamVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoBuffer<Impl: IAppBroadcastStreamVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamVideoFrame, BASE_OFFSET>(),
            VideoHeader: VideoHeader::<Impl, IMPL_OFFSET>,
            VideoBuffer: VideoBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamVideoFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsKeyFrame(&mut self) -> ::windows::core::Result<bool>;
    fn HasDiscontinuity(&mut self) -> ::windows::core::Result<bool>;
    fn FrameId(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastStreamVideoHeader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastStreamVideoHeaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastStreamVideoHeaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastStreamVideoHeaderVtbl {
        unsafe extern "system" fn AbsoluteTimestamp<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeTimestamp<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyFrame<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKeyFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDiscontinuity<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasDiscontinuity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameId<Impl: IAppBroadcastStreamVideoHeaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastStreamVideoHeader, BASE_OFFSET>(),
            AbsoluteTimestamp: AbsoluteTimestamp::<Impl, IMPL_OFFSET>,
            RelativeTimestamp: RelativeTimestamp::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            IsKeyFrame: IsKeyFrame::<Impl, IMPL_OFFSET>,
            HasDiscontinuity: HasDiscontinuity::<Impl, IMPL_OFFSET>,
            FrameId: FrameId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastStreamVideoHeader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerDetailsImpl: Sized {
    fn BackgroundService(&mut self) -> ::windows::core::Result<AppBroadcastBackgroundService>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastTriggerDetailsVtbl {
        unsafe extern "system" fn BackgroundService<Impl: IAppBroadcastTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastTriggerDetails, BASE_OFFSET>(),
            BackgroundService: BackgroundService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastViewerCountChangedEventArgsImpl: Sized {
    fn ViewerCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppBroadcastViewerCountChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastViewerCountChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastViewerCountChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastViewerCountChangedEventArgsVtbl {
        unsafe extern "system" fn ViewerCount<Impl: IAppBroadcastViewerCountChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastViewerCountChangedEventArgs, BASE_OFFSET>(),
            ViewerCount: ViewerCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastViewerCountChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureImpl: Sized {
    fn IsCapturingAudio(&mut self) -> ::windows::core::Result<bool>;
    fn IsCapturingVideo(&mut self) -> ::windows::core::Result<bool>;
    fn CapturingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCapturingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCapture {
    const NAME: &'static str = "Windows.Media.Capture.IAppCapture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureVtbl {
        unsafe extern "system" fn IsCapturingAudio<Impl: IAppCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCapturingAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCapturingVideo<Impl: IAppCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCapturingVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturingChanged<Impl: IAppCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCapturingChanged<Impl: IAppCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCapturingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCapture, BASE_OFFSET>(),
            IsCapturingAudio: IsCapturingAudio::<Impl, IMPL_OFFSET>,
            IsCapturingVideo: IsCapturingVideo::<Impl, IMPL_OFFSET>,
            CapturingChanged: CapturingChanged::<Impl, IMPL_OFFSET>,
            RemoveCapturingChanged: RemoveCapturingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppCaptureAlternateShortcutKeysImpl: Sized {
    fn SetToggleGameBarKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleGameBarKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleGameBarKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleGameBarKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetSaveHistoricalVideoKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetSaveHistoricalVideoKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetTakeScreenshotKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn TakeScreenshotKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetTakeScreenshotKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn TakeScreenshotKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingIndicatorKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingIndicatorKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppCaptureAlternateShortcutKeysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureAlternateShortcutKeysImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureAlternateShortcutKeysVtbl {
        unsafe extern "system" fn SetToggleGameBarKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleGameBarKey(value).into()
        }
        unsafe extern "system" fn ToggleGameBarKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleGameBarKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleGameBarKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleGameBarKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleGameBarKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleGameBarKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaveHistoricalVideoKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaveHistoricalVideoKey(value).into()
        }
        unsafe extern "system" fn SaveHistoricalVideoKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveHistoricalVideoKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaveHistoricalVideoKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaveHistoricalVideoKeyModifiers(value).into()
        }
        unsafe extern "system" fn SaveHistoricalVideoKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveHistoricalVideoKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingKey(value).into()
        }
        unsafe extern "system" fn ToggleRecordingKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleRecordingKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTakeScreenshotKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTakeScreenshotKey(value).into()
        }
        unsafe extern "system" fn TakeScreenshotKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeScreenshotKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTakeScreenshotKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTakeScreenshotKeyModifiers(value).into()
        }
        unsafe extern "system" fn TakeScreenshotKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeScreenshotKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingIndicatorKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingIndicatorKey(value).into()
        }
        unsafe extern "system" fn ToggleRecordingIndicatorKey<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingIndicatorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleRecordingIndicatorKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleRecordingIndicatorKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleRecordingIndicatorKeyModifiers<Impl: IAppCaptureAlternateShortcutKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleRecordingIndicatorKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureAlternateShortcutKeys, BASE_OFFSET>(),
            SetToggleGameBarKey: SetToggleGameBarKey::<Impl, IMPL_OFFSET>,
            ToggleGameBarKey: ToggleGameBarKey::<Impl, IMPL_OFFSET>,
            SetToggleGameBarKeyModifiers: SetToggleGameBarKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleGameBarKeyModifiers: ToggleGameBarKeyModifiers::<Impl, IMPL_OFFSET>,
            SetSaveHistoricalVideoKey: SetSaveHistoricalVideoKey::<Impl, IMPL_OFFSET>,
            SaveHistoricalVideoKey: SaveHistoricalVideoKey::<Impl, IMPL_OFFSET>,
            SetSaveHistoricalVideoKeyModifiers: SetSaveHistoricalVideoKeyModifiers::<Impl, IMPL_OFFSET>,
            SaveHistoricalVideoKeyModifiers: SaveHistoricalVideoKeyModifiers::<Impl, IMPL_OFFSET>,
            SetToggleRecordingKey: SetToggleRecordingKey::<Impl, IMPL_OFFSET>,
            ToggleRecordingKey: ToggleRecordingKey::<Impl, IMPL_OFFSET>,
            SetToggleRecordingKeyModifiers: SetToggleRecordingKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleRecordingKeyModifiers: ToggleRecordingKeyModifiers::<Impl, IMPL_OFFSET>,
            SetTakeScreenshotKey: SetTakeScreenshotKey::<Impl, IMPL_OFFSET>,
            TakeScreenshotKey: TakeScreenshotKey::<Impl, IMPL_OFFSET>,
            SetTakeScreenshotKeyModifiers: SetTakeScreenshotKeyModifiers::<Impl, IMPL_OFFSET>,
            TakeScreenshotKeyModifiers: TakeScreenshotKeyModifiers::<Impl, IMPL_OFFSET>,
            SetToggleRecordingIndicatorKey: SetToggleRecordingIndicatorKey::<Impl, IMPL_OFFSET>,
            ToggleRecordingIndicatorKey: ToggleRecordingIndicatorKey::<Impl, IMPL_OFFSET>,
            SetToggleRecordingIndicatorKeyModifiers: SetToggleRecordingIndicatorKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleRecordingIndicatorKeyModifiers: ToggleRecordingIndicatorKeyModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureAlternateShortcutKeys as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppCaptureAlternateShortcutKeys2Impl: Sized {
    fn SetToggleMicrophoneCaptureKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleMicrophoneCaptureKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppCaptureAlternateShortcutKeys2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureAlternateShortcutKeys2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureAlternateShortcutKeys2Vtbl {
        unsafe extern "system" fn SetToggleMicrophoneCaptureKey<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleMicrophoneCaptureKey(value).into()
        }
        unsafe extern "system" fn ToggleMicrophoneCaptureKey<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleMicrophoneCaptureKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleMicrophoneCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleMicrophoneCaptureKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleMicrophoneCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleMicrophoneCaptureKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureAlternateShortcutKeys2, BASE_OFFSET>(),
            SetToggleMicrophoneCaptureKey: SetToggleMicrophoneCaptureKey::<Impl, IMPL_OFFSET>,
            ToggleMicrophoneCaptureKey: ToggleMicrophoneCaptureKey::<Impl, IMPL_OFFSET>,
            SetToggleMicrophoneCaptureKeyModifiers: SetToggleMicrophoneCaptureKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleMicrophoneCaptureKeyModifiers: ToggleMicrophoneCaptureKeyModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureAlternateShortcutKeys2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppCaptureAlternateShortcutKeys3Impl: Sized {
    fn SetToggleCameraCaptureKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleCameraCaptureKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleBroadcastKey(&mut self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKey(&mut self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleBroadcastKeyModifiers(&mut self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKeyModifiers(&mut self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureAlternateShortcutKeys3 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureAlternateShortcutKeys3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppCaptureAlternateShortcutKeys3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureAlternateShortcutKeys3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureAlternateShortcutKeys3Vtbl {
        unsafe extern "system" fn SetToggleCameraCaptureKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleCameraCaptureKey(value).into()
        }
        unsafe extern "system" fn ToggleCameraCaptureKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleCameraCaptureKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleCameraCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleCameraCaptureKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleCameraCaptureKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleCameraCaptureKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleBroadcastKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleBroadcastKey(value).into()
        }
        unsafe extern "system" fn ToggleBroadcastKey<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleBroadcastKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToggleBroadcastKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToggleBroadcastKeyModifiers(value).into()
        }
        unsafe extern "system" fn ToggleBroadcastKeyModifiers<Impl: IAppCaptureAlternateShortcutKeys3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleBroadcastKeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureAlternateShortcutKeys3, BASE_OFFSET>(),
            SetToggleCameraCaptureKey: SetToggleCameraCaptureKey::<Impl, IMPL_OFFSET>,
            ToggleCameraCaptureKey: ToggleCameraCaptureKey::<Impl, IMPL_OFFSET>,
            SetToggleCameraCaptureKeyModifiers: SetToggleCameraCaptureKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleCameraCaptureKeyModifiers: ToggleCameraCaptureKeyModifiers::<Impl, IMPL_OFFSET>,
            SetToggleBroadcastKey: SetToggleBroadcastKey::<Impl, IMPL_OFFSET>,
            ToggleBroadcastKey: ToggleBroadcastKey::<Impl, IMPL_OFFSET>,
            SetToggleBroadcastKeyModifiers: SetToggleBroadcastKeyModifiers::<Impl, IMPL_OFFSET>,
            ToggleBroadcastKeyModifiers: ToggleBroadcastKeyModifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureAlternateShortcutKeys3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureDurationGeneratedEventArgsImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureDurationGeneratedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureDurationGeneratedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureDurationGeneratedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureDurationGeneratedEventArgsVtbl {
        unsafe extern "system" fn Duration<Impl: IAppCaptureDurationGeneratedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureDurationGeneratedEventArgs, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureDurationGeneratedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IAppCaptureFileGeneratedEventArgsImpl: Sized {
    fn File(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureFileGeneratedEventArgs";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IAppCaptureFileGeneratedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureFileGeneratedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureFileGeneratedEventArgsVtbl {
        unsafe extern "system" fn File<Impl: IAppCaptureFileGeneratedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureFileGeneratedEventArgs, BASE_OFFSET>(), File: File::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureFileGeneratedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureManagerStaticsImpl: Sized {
    fn GetCurrentSettings(&mut self) -> ::windows::core::Result<AppCaptureSettings>;
    fn ApplySettings(&mut self, appcapturesettings: &::core::option::Option<AppCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureManagerStaticsVtbl {
        unsafe extern "system" fn GetCurrentSettings<Impl: IAppCaptureManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplySettings<Impl: IAppCaptureManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appcapturesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySettings(&*(&appcapturesettings as *const <AppCaptureSettings as ::windows::core::Abi>::Abi as *const <AppCaptureSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureManagerStatics, BASE_OFFSET>(),
            GetCurrentSettings: GetCurrentSettings::<Impl, IMPL_OFFSET>,
            ApplySettings: ApplySettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureMetadataWriterImpl: Sized {
    fn AddStringEvent(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddInt32Event(&mut self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddDoubleEvent(&mut self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartStringState(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartInt32State(&mut self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartDoubleState(&mut self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StopState(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopAllStates(&mut self) -> ::windows::core::Result<()>;
    fn RemainingStorageBytesAvailable(&mut self) -> ::windows::core::Result<u64>;
    fn MetadataPurged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMetadataPurged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureMetadataWriter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureMetadataWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureMetadataWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureMetadataWriterVtbl {
        unsafe extern "system" fn AddStringEvent<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringEvent(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), priority).into()
        }
        unsafe extern "system" fn AddInt32Event<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInt32Event(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn AddDoubleEvent<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDoubleEvent(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StartStringState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartStringState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), priority).into()
        }
        unsafe extern "system" fn StartInt32State<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartInt32State(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StartDoubleState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartDoubleState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value, priority).into()
        }
        unsafe extern "system" fn StopState<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopState(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAllStates<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAllStates().into()
        }
        unsafe extern "system" fn RemainingStorageBytesAvailable<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingStorageBytesAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetadataPurged<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MetadataPurged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMetadataPurged<Impl: IAppCaptureMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMetadataPurged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureMetadataWriter, BASE_OFFSET>(),
            AddStringEvent: AddStringEvent::<Impl, IMPL_OFFSET>,
            AddInt32Event: AddInt32Event::<Impl, IMPL_OFFSET>,
            AddDoubleEvent: AddDoubleEvent::<Impl, IMPL_OFFSET>,
            StartStringState: StartStringState::<Impl, IMPL_OFFSET>,
            StartInt32State: StartInt32State::<Impl, IMPL_OFFSET>,
            StartDoubleState: StartDoubleState::<Impl, IMPL_OFFSET>,
            StopState: StopState::<Impl, IMPL_OFFSET>,
            StopAllStates: StopAllStates::<Impl, IMPL_OFFSET>,
            RemainingStorageBytesAvailable: RemainingStorageBytesAvailable::<Impl, IMPL_OFFSET>,
            MetadataPurged: MetadataPurged::<Impl, IMPL_OFFSET>,
            RemoveMetadataPurged: RemoveMetadataPurged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureMetadataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureMicrophoneCaptureStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureMicrophoneCaptureStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureMicrophoneCaptureStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureMicrophoneCaptureStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureMicrophoneCaptureStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IAppCaptureRecordOperationImpl: Sized {
    fn StopRecording(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn File(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn IsFileTruncated(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn StateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DurationGenerated(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDurationGenerated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FileGenerated(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileGenerated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureRecordOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IAppCaptureRecordOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureRecordOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureRecordOperationVtbl {
        unsafe extern "system" fn StopRecording<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopRecording().into()
        }
        unsafe extern "system" fn State<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileTruncated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFileTruncated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DurationGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DurationGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDurationGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDurationGenerated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFileGenerated<Impl: IAppCaptureRecordOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFileGenerated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureRecordOperation, BASE_OFFSET>(),
            StopRecording: StopRecording::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            File: File::<Impl, IMPL_OFFSET>,
            IsFileTruncated: IsFileTruncated::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            DurationGenerated: DurationGenerated::<Impl, IMPL_OFFSET>,
            RemoveDurationGenerated: RemoveDurationGenerated::<Impl, IMPL_OFFSET>,
            FileGenerated: FileGenerated::<Impl, IMPL_OFFSET>,
            RemoveFileGenerated: RemoveFileGenerated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureRecordOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureRecordingStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureRecordingStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureRecordingStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureRecordingStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppCaptureRecordingStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureRecordingStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureRecordingStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureServicesImpl: Sized {
    fn Record(&mut self) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn RecordTimeSpan(&mut self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn CanCapture(&mut self) -> ::windows::core::Result<bool>;
    fn State(&mut self) -> ::windows::core::Result<AppCaptureState>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureServices";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureServicesVtbl {
        unsafe extern "system" fn Record<Impl: IAppCaptureServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Record() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordTimeSpan<Impl: IAppCaptureServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordTimeSpan(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCapture<Impl: IAppCaptureServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAppCaptureServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureServices, BASE_OFFSET>(),
            Record: Record::<Impl, IMPL_OFFSET>,
            RecordTimeSpan: RecordTimeSpan::<Impl, IMPL_OFFSET>,
            CanCapture: CanCapture::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IAppCaptureSettingsImpl: Sized {
    fn SetAppCaptureDestinationFolder(&mut self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn AppCaptureDestinationFolder(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetAudioEncodingBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetIsAudioCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetCustomVideoEncodingBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLength(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn HistoricalBufferLength(&mut self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLengthUnit(&mut self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::Result<()>;
    fn HistoricalBufferLengthUnit(&mut self) -> ::windows::core::Result<AppCaptureHistoricalBufferLengthUnit>;
    fn SetIsHistoricalCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnBatteryAllowed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnBatteryAllowed(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnWirelessDisplayAllowed(&mut self) -> ::windows::core::Result<bool>;
    fn SetMaximumRecordLength(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaximumRecordLength(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScreenshotDestinationFolder(&mut self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn ScreenshotDestinationFolder(&mut self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetVideoEncodingBitrateMode(&mut self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&mut self) -> ::windows::core::Result<AppCaptureVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&mut self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&mut self) -> ::windows::core::Result<AppCaptureVideoEncodingResolutionMode>;
    fn SetIsAppCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsCpuConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&mut self) -> ::windows::core::Result<bool>;
    fn IsMemoryConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IAppCaptureSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureSettingsVtbl {
        unsafe extern "system" fn SetAppCaptureDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppCaptureDestinationFolder(&*(&value as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppCaptureDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppCaptureDestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEncodingBitrate(value).into()
        }
        unsafe extern "system" fn AudioEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAudioCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAudioCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAudioCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAudioCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingBitrate(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingBitrate<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingHeight<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingHeight(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingHeight<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVideoEncodingWidth<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVideoEncodingWidth(value).into()
        }
        unsafe extern "system" fn CustomVideoEncodingWidth<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVideoEncodingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalBufferLength<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHistoricalBufferLength(value).into()
        }
        unsafe extern "system" fn HistoricalBufferLength<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoricalBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoricalBufferLengthUnit<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHistoricalBufferLengthUnit(value).into()
        }
        unsafe extern "system" fn HistoricalBufferLengthUnit<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoricalBufferLengthUnit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureOnBatteryAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureOnBatteryAllowed(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureOnBatteryAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureOnBatteryAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHistoricalCaptureOnWirelessDisplayAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHistoricalCaptureOnWirelessDisplayAllowed(value).into()
        }
        unsafe extern "system" fn IsHistoricalCaptureOnWirelessDisplayAllowed<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureOnWirelessDisplayAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumRecordLength<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumRecordLength(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaximumRecordLength<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumRecordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScreenshotDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScreenshotDestinationFolder(&*(&value as *const <super::super::Storage::StorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFolder as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScreenshotDestinationFolder<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScreenshotDestinationFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingBitrateMode<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingBitrateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingBitrateMode<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingBitrateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingResolutionMode<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingResolutionMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingResolutionMode<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingResolutionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAppCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAppCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsAppCaptureEnabled<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCpuConstrained<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledByPolicy<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledByPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMemoryConstrained<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMemoryConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareEncoder<Impl: IAppCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasHardwareEncoder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureSettings, BASE_OFFSET>(),
            SetAppCaptureDestinationFolder: SetAppCaptureDestinationFolder::<Impl, IMPL_OFFSET>,
            AppCaptureDestinationFolder: AppCaptureDestinationFolder::<Impl, IMPL_OFFSET>,
            SetAudioEncodingBitrate: SetAudioEncodingBitrate::<Impl, IMPL_OFFSET>,
            AudioEncodingBitrate: AudioEncodingBitrate::<Impl, IMPL_OFFSET>,
            SetIsAudioCaptureEnabled: SetIsAudioCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsAudioCaptureEnabled: IsAudioCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingBitrate: SetCustomVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingBitrate: CustomVideoEncodingBitrate::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingHeight: SetCustomVideoEncodingHeight::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingHeight: CustomVideoEncodingHeight::<Impl, IMPL_OFFSET>,
            SetCustomVideoEncodingWidth: SetCustomVideoEncodingWidth::<Impl, IMPL_OFFSET>,
            CustomVideoEncodingWidth: CustomVideoEncodingWidth::<Impl, IMPL_OFFSET>,
            SetHistoricalBufferLength: SetHistoricalBufferLength::<Impl, IMPL_OFFSET>,
            HistoricalBufferLength: HistoricalBufferLength::<Impl, IMPL_OFFSET>,
            SetHistoricalBufferLengthUnit: SetHistoricalBufferLengthUnit::<Impl, IMPL_OFFSET>,
            HistoricalBufferLengthUnit: HistoricalBufferLengthUnit::<Impl, IMPL_OFFSET>,
            SetIsHistoricalCaptureEnabled: SetIsHistoricalCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsHistoricalCaptureEnabled: IsHistoricalCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetIsHistoricalCaptureOnBatteryAllowed: SetIsHistoricalCaptureOnBatteryAllowed::<Impl, IMPL_OFFSET>,
            IsHistoricalCaptureOnBatteryAllowed: IsHistoricalCaptureOnBatteryAllowed::<Impl, IMPL_OFFSET>,
            SetIsHistoricalCaptureOnWirelessDisplayAllowed: SetIsHistoricalCaptureOnWirelessDisplayAllowed::<Impl, IMPL_OFFSET>,
            IsHistoricalCaptureOnWirelessDisplayAllowed: IsHistoricalCaptureOnWirelessDisplayAllowed::<Impl, IMPL_OFFSET>,
            SetMaximumRecordLength: SetMaximumRecordLength::<Impl, IMPL_OFFSET>,
            MaximumRecordLength: MaximumRecordLength::<Impl, IMPL_OFFSET>,
            SetScreenshotDestinationFolder: SetScreenshotDestinationFolder::<Impl, IMPL_OFFSET>,
            ScreenshotDestinationFolder: ScreenshotDestinationFolder::<Impl, IMPL_OFFSET>,
            SetVideoEncodingBitrateMode: SetVideoEncodingBitrateMode::<Impl, IMPL_OFFSET>,
            VideoEncodingBitrateMode: VideoEncodingBitrateMode::<Impl, IMPL_OFFSET>,
            SetVideoEncodingResolutionMode: SetVideoEncodingResolutionMode::<Impl, IMPL_OFFSET>,
            VideoEncodingResolutionMode: VideoEncodingResolutionMode::<Impl, IMPL_OFFSET>,
            SetIsAppCaptureEnabled: SetIsAppCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsAppCaptureEnabled: IsAppCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsCpuConstrained: IsCpuConstrained::<Impl, IMPL_OFFSET>,
            IsDisabledByPolicy: IsDisabledByPolicy::<Impl, IMPL_OFFSET>,
            IsMemoryConstrained: IsMemoryConstrained::<Impl, IMPL_OFFSET>,
            HasHardwareEncoder: HasHardwareEncoder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings2Impl: Sized {
    fn IsGpuConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn AlternateShortcutKeys(&mut self) -> ::windows::core::Result<AppCaptureAlternateShortcutKeys>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureSettings2Vtbl {
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateShortcutKeys<Impl: IAppCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateShortcutKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureSettings2, BASE_OFFSET>(),
            IsGpuConstrained: IsGpuConstrained::<Impl, IMPL_OFFSET>,
            AlternateShortcutKeys: AlternateShortcutKeys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings3Impl: Sized {
    fn SetIsMicrophoneCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureSettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureSettings3Vtbl {
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabled<Impl: IAppCaptureSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabled<Impl: IAppCaptureSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureSettings3, BASE_OFFSET>(),
            SetIsMicrophoneCaptureEnabled: SetIsMicrophoneCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsMicrophoneCaptureEnabled: IsMicrophoneCaptureEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings4Impl: Sized {
    fn SetIsMicrophoneCaptureEnabledByDefault(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&mut self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&mut self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&mut self) -> ::windows::core::Result<f64>;
    fn SetVideoEncodingFrameRateMode(&mut self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingFrameRateMode(&mut self) -> ::windows::core::Result<AppCaptureVideoEncodingFrameRateMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings4 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureSettings4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureSettings4Vtbl {
        unsafe extern "system" fn SetIsMicrophoneCaptureEnabledByDefault<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMicrophoneCaptureEnabledByDefault(value).into()
        }
        unsafe extern "system" fn IsMicrophoneCaptureEnabledByDefault<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMicrophoneCaptureEnabledByDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAudioGain<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemAudioGain(value).into()
        }
        unsafe extern "system" fn SystemAudioGain<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemAudioGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrophoneGain<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMicrophoneGain(value).into()
        }
        unsafe extern "system" fn MicrophoneGain<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoEncodingFrameRateMode<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoEncodingFrameRateMode(value).into()
        }
        unsafe extern "system" fn VideoEncodingFrameRateMode<Impl: IAppCaptureSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEncodingFrameRateMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureSettings4, BASE_OFFSET>(),
            SetIsMicrophoneCaptureEnabledByDefault: SetIsMicrophoneCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            IsMicrophoneCaptureEnabledByDefault: IsMicrophoneCaptureEnabledByDefault::<Impl, IMPL_OFFSET>,
            SetSystemAudioGain: SetSystemAudioGain::<Impl, IMPL_OFFSET>,
            SystemAudioGain: SystemAudioGain::<Impl, IMPL_OFFSET>,
            SetMicrophoneGain: SetMicrophoneGain::<Impl, IMPL_OFFSET>,
            MicrophoneGain: MicrophoneGain::<Impl, IMPL_OFFSET>,
            SetVideoEncodingFrameRateMode: SetVideoEncodingFrameRateMode::<Impl, IMPL_OFFSET>,
            VideoEncodingFrameRateMode: VideoEncodingFrameRateMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureSettings4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings5Impl: Sized {
    fn SetIsEchoCancellationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCursorImageCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureSettings5 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureSettings5";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureSettings5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureSettings5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureSettings5Vtbl {
        unsafe extern "system" fn SetIsEchoCancellationEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEchoCancellationEnabled(value).into()
        }
        unsafe extern "system" fn IsEchoCancellationEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEchoCancellationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorImageCaptureEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCursorImageCaptureEnabled(value).into()
        }
        unsafe extern "system" fn IsCursorImageCaptureEnabled<Impl: IAppCaptureSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCursorImageCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureSettings5, BASE_OFFSET>(),
            SetIsEchoCancellationEnabled: SetIsEchoCancellationEnabled::<Impl, IMPL_OFFSET>,
            IsEchoCancellationEnabled: IsEchoCancellationEnabled::<Impl, IMPL_OFFSET>,
            SetIsCursorImageCaptureEnabled: SetIsCursorImageCaptureEnabled::<Impl, IMPL_OFFSET>,
            IsCursorImageCaptureEnabled: IsCursorImageCaptureEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureSettings5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureStateImpl: Sized {
    fn IsTargetRunning(&mut self) -> ::windows::core::Result<bool>;
    fn IsHistoricalCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn ShouldCaptureMicrophone(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&mut self) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureState(&mut self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&mut self) -> ::windows::core::Result<u32>;
    fn MicrophoneCaptureStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureState";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureStateVtbl {
        unsafe extern "system" fn IsTargetRunning<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTargetRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHistoricalCaptureEnabled<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHistoricalCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldCaptureMicrophone<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldCaptureMicrophone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldCaptureMicrophone<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldCaptureMicrophone(value).into()
        }
        unsafe extern "system" fn RestartMicrophoneCapture<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartMicrophoneCapture().into()
        }
        unsafe extern "system" fn MicrophoneCaptureState<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureError<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrophoneCaptureStateChanged<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrophoneCaptureStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMicrophoneCaptureStateChanged<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMicrophoneCaptureStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CaptureTargetClosed<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureTargetClosed(&*(&value as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureTargetClosed<Impl: IAppCaptureStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCaptureTargetClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureState, BASE_OFFSET>(),
            IsTargetRunning: IsTargetRunning::<Impl, IMPL_OFFSET>,
            IsHistoricalCaptureEnabled: IsHistoricalCaptureEnabled::<Impl, IMPL_OFFSET>,
            ShouldCaptureMicrophone: ShouldCaptureMicrophone::<Impl, IMPL_OFFSET>,
            SetShouldCaptureMicrophone: SetShouldCaptureMicrophone::<Impl, IMPL_OFFSET>,
            RestartMicrophoneCapture: RestartMicrophoneCapture::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureState: MicrophoneCaptureState::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureError: MicrophoneCaptureError::<Impl, IMPL_OFFSET>,
            MicrophoneCaptureStateChanged: MicrophoneCaptureStateChanged::<Impl, IMPL_OFFSET>,
            RemoveMicrophoneCaptureStateChanged: RemoveMicrophoneCaptureStateChanged::<Impl, IMPL_OFFSET>,
            CaptureTargetClosed: CaptureTargetClosed::<Impl, IMPL_OFFSET>,
            RemoveCaptureTargetClosed: RemoveCaptureTargetClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<AppCapture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppCaptureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IAppCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppCaptureStatics2Impl: Sized {
    fn SetAllowedAsync(&mut self, allowed: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppCaptureStatics2 {
    const NAME: &'static str = "Windows.Media.Capture.IAppCaptureStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppCaptureStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppCaptureStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppCaptureStatics2Vtbl {
        unsafe extern "system" fn SetAllowedAsync<Impl: IAppCaptureStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowed: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowedAsync(allowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppCaptureStatics2, BASE_OFFSET>(),
            SetAllowedAsync: SetAllowedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppCaptureStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ICameraCaptureUIImpl: Sized {
    fn PhotoSettings(&mut self) -> ::windows::core::Result<CameraCaptureUIPhotoCaptureSettings>;
    fn VideoSettings(&mut self) -> ::windows::core::Result<CameraCaptureUIVideoCaptureSettings>;
    fn CaptureFileAsync(&mut self, mode: CameraCaptureUIMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUI";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ICameraCaptureUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraCaptureUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraCaptureUIVtbl {
        unsafe extern "system" fn PhotoSettings<Impl: ICameraCaptureUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoSettings<Impl: ICameraCaptureUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureFileAsync<Impl: ICameraCaptureUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: CameraCaptureUIMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureFileAsync(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraCaptureUI, BASE_OFFSET>(),
            PhotoSettings: PhotoSettings::<Impl, IMPL_OFFSET>,
            VideoSettings: VideoSettings::<Impl, IMPL_OFFSET>,
            CaptureFileAsync: CaptureFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraCaptureUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICameraCaptureUIPhotoCaptureSettingsImpl: Sized {
    fn Format(&mut self) -> ::windows::core::Result<CameraCaptureUIPhotoFormat>;
    fn SetFormat(&mut self, value: CameraCaptureUIPhotoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&mut self) -> ::windows::core::Result<CameraCaptureUIMaxPhotoResolution>;
    fn SetMaxResolution(&mut self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::Result<()>;
    fn CroppedSizeInPixels(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedSizeInPixels(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CroppedAspectRatio(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedAspectRatio(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn AllowCropping(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowCropping(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUIPhotoCaptureSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICameraCaptureUIPhotoCaptureSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraCaptureUIPhotoCaptureSettingsVtbl {
        unsafe extern "system" fn Format<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn MaxResolution<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxResolution<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxResolution(value).into()
        }
        unsafe extern "system" fn CroppedSizeInPixels<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CroppedSizeInPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCroppedSizeInPixels<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCroppedSizeInPixels(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CroppedAspectRatio<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CroppedAspectRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCroppedAspectRatio<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCroppedAspectRatio(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowCropping<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowCropping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCropping<Impl: ICameraCaptureUIPhotoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCropping(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraCaptureUIPhotoCaptureSettings, BASE_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            MaxResolution: MaxResolution::<Impl, IMPL_OFFSET>,
            SetMaxResolution: SetMaxResolution::<Impl, IMPL_OFFSET>,
            CroppedSizeInPixels: CroppedSizeInPixels::<Impl, IMPL_OFFSET>,
            SetCroppedSizeInPixels: SetCroppedSizeInPixels::<Impl, IMPL_OFFSET>,
            CroppedAspectRatio: CroppedAspectRatio::<Impl, IMPL_OFFSET>,
            SetCroppedAspectRatio: SetCroppedAspectRatio::<Impl, IMPL_OFFSET>,
            AllowCropping: AllowCropping::<Impl, IMPL_OFFSET>,
            SetAllowCropping: SetAllowCropping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraCaptureUIPhotoCaptureSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIVideoCaptureSettingsImpl: Sized {
    fn Format(&mut self) -> ::windows::core::Result<CameraCaptureUIVideoFormat>;
    fn SetFormat(&mut self, value: CameraCaptureUIVideoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&mut self) -> ::windows::core::Result<CameraCaptureUIMaxVideoResolution>;
    fn SetMaxResolution(&mut self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::Result<()>;
    fn MaxDurationInSeconds(&mut self) -> ::windows::core::Result<f32>;
    fn SetMaxDurationInSeconds(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn AllowTrimming(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowTrimming(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.ICameraCaptureUIVideoCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraCaptureUIVideoCaptureSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraCaptureUIVideoCaptureSettingsVtbl {
        unsafe extern "system" fn Format<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn MaxResolution<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxResolution<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxResolution(value).into()
        }
        unsafe extern "system" fn MaxDurationInSeconds<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDurationInSeconds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDurationInSeconds<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxDurationInSeconds(value).into()
        }
        unsafe extern "system" fn AllowTrimming<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowTrimming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowTrimming<Impl: ICameraCaptureUIVideoCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowTrimming(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraCaptureUIVideoCaptureSettings, BASE_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            MaxResolution: MaxResolution::<Impl, IMPL_OFFSET>,
            SetMaxResolution: SetMaxResolution::<Impl, IMPL_OFFSET>,
            MaxDurationInSeconds: MaxDurationInSeconds::<Impl, IMPL_OFFSET>,
            SetMaxDurationInSeconds: SetMaxDurationInSeconds::<Impl, IMPL_OFFSET>,
            AllowTrimming: AllowTrimming::<Impl, IMPL_OFFSET>,
            SetAllowTrimming: SetAllowTrimming::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraCaptureUIVideoCaptureSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOptionsUIStaticsImpl: Sized {
    fn Show(&mut self, mediacapture: &::core::option::Option<MediaCapture>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraOptionsUIStatics {
    const NAME: &'static str = "Windows.Media.Capture.ICameraOptionsUIStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraOptionsUIStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraOptionsUIStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraOptionsUIStaticsVtbl {
        unsafe extern "system" fn Show<Impl: ICameraOptionsUIStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediacapture: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(&*(&mediacapture as *const <MediaCapture as ::windows::core::Abi>::Abi as *const <MediaCapture as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraOptionsUIStatics, BASE_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraOptionsUIStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICapturedFrameImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrame";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICapturedFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedFrameVtbl {
        unsafe extern "system" fn Width<Impl: ICapturedFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: ICapturedFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedFrame, BASE_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ICapturedFrame2Impl: Sized {
    fn ControlValues(&mut self) -> ::windows::core::Result<CapturedFrameControlValues>;
    fn BitmapProperties(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPropertySet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrame2 {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrame2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ICapturedFrame2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedFrame2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedFrame2Vtbl {
        unsafe extern "system" fn ControlValues<Impl: ICapturedFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Impl: ICapturedFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedFrame2, BASE_OFFSET>(),
            ControlValues: ControlValues::<Impl, IMPL_OFFSET>,
            BitmapProperties: BitmapProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICapturedFrameControlValuesImpl: Sized {
    fn Exposure(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn ExposureCompensation(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn IsoSpeed(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Focus(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SceneMode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>;
    fn Flashed(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn FlashPowerPercent(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn WhiteBalance(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn ZoomFactor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameControlValues";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICapturedFrameControlValuesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedFrameControlValuesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedFrameControlValuesVtbl {
        unsafe extern "system" fn Exposure<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exposure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensation<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeed<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsoSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Focus<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Focus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneMode<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SceneMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flashed<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flashed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlashPowerPercent<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlashPowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalance<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhiteBalance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomFactor<Impl: ICapturedFrameControlValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedFrameControlValues, BASE_OFFSET>(),
            Exposure: Exposure::<Impl, IMPL_OFFSET>,
            ExposureCompensation: ExposureCompensation::<Impl, IMPL_OFFSET>,
            IsoSpeed: IsoSpeed::<Impl, IMPL_OFFSET>,
            Focus: Focus::<Impl, IMPL_OFFSET>,
            SceneMode: SceneMode::<Impl, IMPL_OFFSET>,
            Flashed: Flashed::<Impl, IMPL_OFFSET>,
            FlashPowerPercent: FlashPowerPercent::<Impl, IMPL_OFFSET>,
            WhiteBalance: WhiteBalance::<Impl, IMPL_OFFSET>,
            ZoomFactor: ZoomFactor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedFrameControlValues as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait ICapturedFrameControlValues2Impl: Sized {
    fn FocusState(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>;
    fn IsoDigitalGain(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn IsoAnalogGain(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SensorFrameRate(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn WhiteBalanceGain(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<WhiteBalanceGain>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrameControlValues2 {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameControlValues2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ICapturedFrameControlValues2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedFrameControlValues2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedFrameControlValues2Vtbl {
        unsafe extern "system" fn FocusState<Impl: ICapturedFrameControlValues2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoDigitalGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsoDigitalGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoAnalogGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsoAnalogGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SensorFrameRate<Impl: ICapturedFrameControlValues2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SensorFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteBalanceGain<Impl: ICapturedFrameControlValues2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhiteBalanceGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedFrameControlValues2, BASE_OFFSET>(),
            FocusState: FocusState::<Impl, IMPL_OFFSET>,
            IsoDigitalGain: IsoDigitalGain::<Impl, IMPL_OFFSET>,
            IsoAnalogGain: IsoAnalogGain::<Impl, IMPL_OFFSET>,
            SensorFrameRate: SensorFrameRate::<Impl, IMPL_OFFSET>,
            WhiteBalanceGain: WhiteBalanceGain::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedFrameControlValues2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ICapturedFrameWithSoftwareBitmapImpl: Sized {
    fn SoftwareBitmap(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICapturedFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedFrameWithSoftwareBitmap";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ICapturedFrameWithSoftwareBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedFrameWithSoftwareBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedFrameWithSoftwareBitmapVtbl {
        unsafe extern "system" fn SoftwareBitmap<Impl: ICapturedFrameWithSoftwareBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedFrameWithSoftwareBitmap, BASE_OFFSET>(),
            SoftwareBitmap: SoftwareBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedFrameWithSoftwareBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedPhotoImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<CapturedFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.ICapturedPhoto";
}
#[cfg(feature = "implement_exclusive")]
impl ICapturedPhotoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICapturedPhotoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICapturedPhotoVtbl {
        unsafe extern "system" fn Frame<Impl: ICapturedPhotoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: ICapturedPhotoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICapturedPhoto, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICapturedPhoto as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameBarServicesImpl: Sized {
    fn TargetCapturePolicy(&mut self) -> ::windows::core::Result<GameBarTargetCapturePolicy>;
    fn EnableCapture(&mut self) -> ::windows::core::Result<()>;
    fn DisableCapture(&mut self) -> ::windows::core::Result<()>;
    fn TargetInfo(&mut self) -> ::windows::core::Result<GameBarServicesTargetInfo>;
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppBroadcastServices(&mut self) -> ::windows::core::Result<AppBroadcastServices>;
    fn AppCaptureServices(&mut self) -> ::windows::core::Result<AppCaptureServices>;
    fn CommandReceived(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServices";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameBarServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesVtbl {
        unsafe extern "system" fn TargetCapturePolicy<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarTargetCapturePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetCapturePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableCapture<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableCapture().into()
        }
        unsafe extern "system" fn DisableCapture<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableCapture().into()
        }
        unsafe extern "system" fn TargetInfo<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppBroadcastServices<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppBroadcastServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppCaptureServices<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppCaptureServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandReceived<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandReceived(&*(&value as *const <super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandReceived<Impl: IGameBarServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCommandReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServices, BASE_OFFSET>(),
            TargetCapturePolicy: TargetCapturePolicy::<Impl, IMPL_OFFSET>,
            EnableCapture: EnableCapture::<Impl, IMPL_OFFSET>,
            DisableCapture: DisableCapture::<Impl, IMPL_OFFSET>,
            TargetInfo: TargetInfo::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            AppBroadcastServices: AppBroadcastServices::<Impl, IMPL_OFFSET>,
            AppCaptureServices: AppCaptureServices::<Impl, IMPL_OFFSET>,
            CommandReceived: CommandReceived::<Impl, IMPL_OFFSET>,
            RemoveCommandReceived: RemoveCommandReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesCommandEventArgsImpl: Sized {
    fn Command(&mut self) -> ::windows::core::Result<GameBarCommand>;
    fn Origin(&mut self) -> ::windows::core::Result<GameBarCommandOrigin>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesCommandEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesCommandEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesCommandEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesCommandEventArgsVtbl {
        unsafe extern "system" fn Command<Impl: IGameBarServicesCommandEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: IGameBarServicesCommandEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommandOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServicesCommandEventArgs, BASE_OFFSET>(),
            Command: Command::<Impl, IMPL_OFFSET>,
            Origin: Origin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServicesCommandEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameBarServicesManagerImpl: Sized {
    fn GameBarServicesCreated(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameBarServicesCreated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameBarServicesManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesManagerVtbl {
        unsafe extern "system" fn GameBarServicesCreated<Impl: IGameBarServicesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameBarServicesCreated(&*(&value as *const <super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameBarServicesCreated<Impl: IGameBarServicesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameBarServicesCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServicesManager, BASE_OFFSET>(),
            GameBarServicesCreated: GameBarServicesCreated::<Impl, IMPL_OFFSET>,
            RemoveGameBarServicesCreated: RemoveGameBarServicesCreated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServicesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl: Sized {
    fn GameBarServices(&mut self) -> ::windows::core::Result<GameBarServices>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManagerGameBarServicesCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesManagerGameBarServicesCreatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesManagerGameBarServicesCreatedEventArgsVtbl {
        unsafe extern "system" fn GameBarServices<Impl: IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameBarServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServicesManagerGameBarServicesCreatedEventArgs, BASE_OFFSET>(),
            GameBarServices: GameBarServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServicesManagerGameBarServicesCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<GameBarServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesManagerStatics {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameBarServicesManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServicesManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServicesManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesTargetInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayMode(&mut self) -> ::windows::core::Result<GameBarServicesDisplayMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.IGameBarServicesTargetInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarServicesTargetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarServicesTargetInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarServicesTargetInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleId<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayMode<Impl: IGameBarServicesTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameBarServicesDisplayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarServicesTargetInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            TitleId: TitleId::<Impl, IMPL_OFFSET>,
            DisplayMode: DisplayMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarServicesTargetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILowLagMediaRecordingImpl: Sized {
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILowLagMediaRecordingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagMediaRecordingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagMediaRecordingVtbl {
        unsafe extern "system" fn StartAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagMediaRecordingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagMediaRecording, BASE_OFFSET>(),
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagMediaRecording as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait ILowLagMediaRecording2Impl: Sized {
    fn PauseAsync(&mut self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagMediaRecording2 {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ILowLagMediaRecording2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagMediaRecording2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagMediaRecording2Vtbl {
        unsafe extern "system" fn PauseAsync<Impl: ILowLagMediaRecording2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeAsync<Impl: ILowLagMediaRecording2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagMediaRecording2, BASE_OFFSET>(),
            PauseAsync: PauseAsync::<Impl, IMPL_OFFSET>,
            ResumeAsync: ResumeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagMediaRecording2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait ILowLagMediaRecording3Impl: Sized {
    fn PauseWithResultAsync(&mut self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopWithResultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagMediaRecording3 {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagMediaRecording3";
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ILowLagMediaRecording3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagMediaRecording3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagMediaRecording3Vtbl {
        unsafe extern "system" fn PauseWithResultAsync<Impl: ILowLagMediaRecording3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseWithResultAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopWithResultAsync<Impl: ILowLagMediaRecording3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopWithResultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagMediaRecording3, BASE_OFFSET>(),
            PauseWithResultAsync: PauseWithResultAsync::<Impl, IMPL_OFFSET>,
            StopWithResultAsync: StopWithResultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagMediaRecording3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILowLagPhotoCaptureImpl: Sized {
    fn CaptureAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>>;
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagPhotoCapture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILowLagPhotoCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagPhotoCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagPhotoCaptureVtbl {
        unsafe extern "system" fn CaptureAsync<Impl: ILowLagPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagPhotoCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagPhotoCapture, BASE_OFFSET>(),
            CaptureAsync: CaptureAsync::<Impl, IMPL_OFFSET>,
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagPhotoCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILowLagPhotoSequenceCaptureImpl: Sized {
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PhotoCaptured(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoCaptured(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.ILowLagPhotoSequenceCapture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILowLagPhotoSequenceCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLagPhotoSequenceCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLagPhotoSequenceCaptureVtbl {
        unsafe extern "system" fn StartAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsync<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoCaptured<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePhotoCaptured<Impl: ILowLagPhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePhotoCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLagPhotoSequenceCapture, BASE_OFFSET>(),
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
            PhotoCaptured: PhotoCaptured::<Impl, IMPL_OFFSET>,
            RemovePhotoCaptured: RemovePhotoCaptured::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLagPhotoSequenceCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaCaptureImpl: Sized {
    fn InitializeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InitializeWithSettingsAsync(&mut self, mediacaptureinitializationsettings: &::core::option::Option<MediaCaptureInitializationSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStorageFileAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStreamAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkIdAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopRecordAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStorageFileAsync(&mut self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStreamAsync(&mut self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AddEffectAsync(&mut self, mediastreamtype: MediaStreamType, effectactivationid: &::windows::core::HSTRING, effectsettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearEffectsAsync(&mut self, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetEncoderProperty(&mut self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetEncoderProperty(&mut self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Failed(&mut self, erroreventhandler: &::core::option::Option<MediaCaptureFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordLimitationExceeded(&mut self, recordlimitationexceededeventhandler: &::core::option::Option<RecordLimitationExceededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecordLimitationExceeded(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaCaptureSettings(&mut self) -> ::windows::core::Result<MediaCaptureSettings>;
    fn AudioDeviceController(&mut self) -> ::windows::core::Result<super::Devices::AudioDeviceController>;
    fn VideoDeviceController(&mut self) -> ::windows::core::Result<super::Devices::VideoDeviceController>;
    fn SetPreviewMirroring(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetPreviewMirroring(&mut self) -> ::windows::core::Result<bool>;
    fn SetPreviewRotation(&mut self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetPreviewRotation(&mut self) -> ::windows::core::Result<VideoRotation>;
    fn SetRecordRotation(&mut self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetRecordRotation(&mut self) -> ::windows::core::Result<VideoRotation>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVtbl {
        unsafe extern "system" fn InitializeAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeWithSettingsAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediacaptureinitializationsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeWithSettingsAsync(&*(&mediacaptureinitializationsettings as *const <MediaCaptureInitializationSettings as ::windows::core::Abi>::Abi as *const <MediaCaptureInitializationSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToStorageFileAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRecordToStorageFileAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToStreamAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRecordToStreamAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToCustomSinkAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRecordToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRecordToCustomSinkIdAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn StopRecordAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopRecordAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePhotoToStorageFileAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturePhotoToStorageFileAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePhotoToStreamAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturePhotoToStreamAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEffectAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, effectactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEffectAsync(mediastreamtype, &*(&effectactivationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&effectsettings as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearEffectsAsync<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearEffectsAsync(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncoderProperty<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncoderProperty(mediastreamtype, &*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncoderProperty<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncoderProperty(mediastreamtype, &*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Failed<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, erroreventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&erroreventhandler as *const <MediaCaptureFailedEventHandler as ::windows::core::Abi>::Abi as *const <MediaCaptureFailedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordLimitationExceeded<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordlimitationexceededeventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordLimitationExceeded(&*(&recordlimitationexceededeventhandler as *const <RecordLimitationExceededEventHandler as ::windows::core::Abi>::Abi as *const <RecordLimitationExceededEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecordLimitationExceeded<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecordLimitationExceeded(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCaptureSettings<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCaptureSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioDeviceController<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceController<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewMirroring<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviewMirroring(value).into()
        }
        unsafe extern "system" fn GetPreviewMirroring<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewRotation<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviewRotation(value).into()
        }
        unsafe extern "system" fn GetPreviewRotation<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecordRotation<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecordRotation(value).into()
        }
        unsafe extern "system" fn GetRecordRotation<Impl: IMediaCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture, BASE_OFFSET>(),
            InitializeAsync: InitializeAsync::<Impl, IMPL_OFFSET>,
            InitializeWithSettingsAsync: InitializeWithSettingsAsync::<Impl, IMPL_OFFSET>,
            StartRecordToStorageFileAsync: StartRecordToStorageFileAsync::<Impl, IMPL_OFFSET>,
            StartRecordToStreamAsync: StartRecordToStreamAsync::<Impl, IMPL_OFFSET>,
            StartRecordToCustomSinkAsync: StartRecordToCustomSinkAsync::<Impl, IMPL_OFFSET>,
            StartRecordToCustomSinkIdAsync: StartRecordToCustomSinkIdAsync::<Impl, IMPL_OFFSET>,
            StopRecordAsync: StopRecordAsync::<Impl, IMPL_OFFSET>,
            CapturePhotoToStorageFileAsync: CapturePhotoToStorageFileAsync::<Impl, IMPL_OFFSET>,
            CapturePhotoToStreamAsync: CapturePhotoToStreamAsync::<Impl, IMPL_OFFSET>,
            AddEffectAsync: AddEffectAsync::<Impl, IMPL_OFFSET>,
            ClearEffectsAsync: ClearEffectsAsync::<Impl, IMPL_OFFSET>,
            SetEncoderProperty: SetEncoderProperty::<Impl, IMPL_OFFSET>,
            GetEncoderProperty: GetEncoderProperty::<Impl, IMPL_OFFSET>,
            Failed: Failed::<Impl, IMPL_OFFSET>,
            RemoveFailed: RemoveFailed::<Impl, IMPL_OFFSET>,
            RecordLimitationExceeded: RecordLimitationExceeded::<Impl, IMPL_OFFSET>,
            RemoveRecordLimitationExceeded: RemoveRecordLimitationExceeded::<Impl, IMPL_OFFSET>,
            MediaCaptureSettings: MediaCaptureSettings::<Impl, IMPL_OFFSET>,
            AudioDeviceController: AudioDeviceController::<Impl, IMPL_OFFSET>,
            VideoDeviceController: VideoDeviceController::<Impl, IMPL_OFFSET>,
            SetPreviewMirroring: SetPreviewMirroring::<Impl, IMPL_OFFSET>,
            GetPreviewMirroring: GetPreviewMirroring::<Impl, IMPL_OFFSET>,
            SetPreviewRotation: SetPreviewRotation::<Impl, IMPL_OFFSET>,
            GetPreviewRotation: GetPreviewRotation::<Impl, IMPL_OFFSET>,
            SetRecordRotation: SetRecordRotation::<Impl, IMPL_OFFSET>,
            GetRecordRotation: GetRecordRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaCapture2Impl: Sized {
    fn PrepareLowLagRecordToStorageFileAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToStreamAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkIdAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagPhotoCaptureAsync(&mut self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>;
    fn PrepareLowLagPhotoSequenceCaptureAsync(&mut self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>;
    fn SetEncodingPropertiesAsync(&mut self, mediastreamtype: MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>, encoderproperties: &::core::option::Option<super::MediaProperties::MediaPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaCapture2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture2Vtbl {
        unsafe extern "system" fn PrepareLowLagRecordToStorageFileAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToStorageFileAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToStreamAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToStreamAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToCustomSinkAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagRecordToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagRecordToCustomSinkIdAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn PrepareLowLagPhotoCaptureAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagPhotoCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareLowLagPhotoSequenceCaptureAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareLowLagPhotoSequenceCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingPropertiesAsync<Impl: IMediaCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, encoderproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncodingPropertiesAsync(mediastreamtype, &*(&mediaencodingproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&encoderproperties as *const <super::MediaProperties::MediaPropertySet as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture2, BASE_OFFSET>(),
            PrepareLowLagRecordToStorageFileAsync: PrepareLowLagRecordToStorageFileAsync::<Impl, IMPL_OFFSET>,
            PrepareLowLagRecordToStreamAsync: PrepareLowLagRecordToStreamAsync::<Impl, IMPL_OFFSET>,
            PrepareLowLagRecordToCustomSinkAsync: PrepareLowLagRecordToCustomSinkAsync::<Impl, IMPL_OFFSET>,
            PrepareLowLagRecordToCustomSinkIdAsync: PrepareLowLagRecordToCustomSinkIdAsync::<Impl, IMPL_OFFSET>,
            PrepareLowLagPhotoCaptureAsync: PrepareLowLagPhotoCaptureAsync::<Impl, IMPL_OFFSET>,
            PrepareLowLagPhotoSequenceCaptureAsync: PrepareLowLagPhotoSequenceCaptureAsync::<Impl, IMPL_OFFSET>,
            SetEncodingPropertiesAsync: SetEncodingPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaCapture3Impl: Sized {
    fn PrepareVariablePhotoSequenceCaptureAsync(&mut self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>;
    fn FocusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PhotoConfirmationCaptured(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoConfirmationCaptured(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture3";
}
#[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaCapture3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture3Vtbl {
        unsafe extern "system" fn PrepareVariablePhotoSequenceCaptureAsync<Impl: IMediaCapture3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareVariablePhotoSequenceCaptureAsync(&*(&r#type as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusChanged<Impl: IMediaCapture3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFocusChanged<Impl: IMediaCapture3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFocusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhotoConfirmationCaptured<Impl: IMediaCapture3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationCaptured(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePhotoConfirmationCaptured<Impl: IMediaCapture3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePhotoConfirmationCaptured(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture3, BASE_OFFSET>(),
            PrepareVariablePhotoSequenceCaptureAsync: PrepareVariablePhotoSequenceCaptureAsync::<Impl, IMPL_OFFSET>,
            FocusChanged: FocusChanged::<Impl, IMPL_OFFSET>,
            RemoveFocusChanged: RemoveFocusChanged::<Impl, IMPL_OFFSET>,
            PhotoConfirmationCaptured: PhotoConfirmationCaptured::<Impl, IMPL_OFFSET>,
            RemovePhotoConfirmationCaptured: RemovePhotoConfirmationCaptured::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaCapture4Impl: Sized {
    fn AddAudioEffectAsync(&mut self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn AddVideoEffectAsync(&mut self, definition: &::core::option::Option<super::Effects::IVideoEffectDefinition>, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn PauseRecordAsync(&mut self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeRecordAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CameraStreamStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraStreamStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraStreamState(&mut self) -> ::windows::core::Result<super::Devices::CameraStreamState>;
    fn GetPreviewFrameAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn GetPreviewFrameCopyAsync(&mut self, destination: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn ThermalStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThermalStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ThermalStatus(&mut self) -> ::windows::core::Result<MediaCaptureThermalStatus>;
    fn PrepareAdvancedPhotoCaptureAsync(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture4 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture4";
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaCapture4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture4Vtbl {
        unsafe extern "system" fn AddAudioEffectAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAudioEffectAsync(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddVideoEffectAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddVideoEffectAsync(&*(&definition as *const <super::Effects::IVideoEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IVideoEffectDefinition as ::windows::core::DefaultType>::DefaultType), mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseRecordAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseRecordAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeRecordAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeRecordAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraStreamStateChanged<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraStreamStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraStreamStateChanged<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraStreamStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraStreamState<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::CameraStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraStreamState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewFrameAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewFrameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewFrameCopyAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewFrameCopyAsync(&*(&destination as *const <super::VideoFrame as ::windows::core::Abi>::Abi as *const <super::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThermalStatusChanged<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThermalStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThermalStatusChanged<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveThermalStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ThermalStatus<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureThermalStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThermalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareAdvancedPhotoCaptureAsync<Impl: IMediaCapture4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareAdvancedPhotoCaptureAsync(&*(&encodingproperties as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::ImageEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture4, BASE_OFFSET>(),
            AddAudioEffectAsync: AddAudioEffectAsync::<Impl, IMPL_OFFSET>,
            AddVideoEffectAsync: AddVideoEffectAsync::<Impl, IMPL_OFFSET>,
            PauseRecordAsync: PauseRecordAsync::<Impl, IMPL_OFFSET>,
            ResumeRecordAsync: ResumeRecordAsync::<Impl, IMPL_OFFSET>,
            CameraStreamStateChanged: CameraStreamStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCameraStreamStateChanged: RemoveCameraStreamStateChanged::<Impl, IMPL_OFFSET>,
            CameraStreamState: CameraStreamState::<Impl, IMPL_OFFSET>,
            GetPreviewFrameAsync: GetPreviewFrameAsync::<Impl, IMPL_OFFSET>,
            GetPreviewFrameCopyAsync: GetPreviewFrameCopyAsync::<Impl, IMPL_OFFSET>,
            ThermalStatusChanged: ThermalStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveThermalStatusChanged: RemoveThermalStatusChanged::<Impl, IMPL_OFFSET>,
            ThermalStatus: ThermalStatus::<Impl, IMPL_OFFSET>,
            PrepareAdvancedPhotoCaptureAsync: PrepareAdvancedPhotoCaptureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Capture_Frames", feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait IMediaCapture5Impl: Sized {
    fn RemoveEffectAsync(&mut self, effect: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseRecordWithResultAsync(&mut self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopRecordWithResultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
    fn FrameSources(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>>;
    fn CreateFrameReaderAsync(&mut self, inputsource: &::core::option::Option<Frames::MediaFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAsync(&mut self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAndSizeAsync(&mut self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING, outputsize: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Capture_Frames", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture5 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture5";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Capture_Frames", feature = "Media_Devices", feature = "implement_exclusive"))]
impl IMediaCapture5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture5Vtbl {
        unsafe extern "system" fn RemoveEffectAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEffectAsync(&*(&effect as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseRecordWithResultAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseRecordWithResultAsync(behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRecordWithResultAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopRecordWithResultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameSources<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderAsync(&*(&inputsource as *const <Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithSubtypeAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderWithSubtypeAsync(&*(&inputsource as *const <Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType), &*(&outputsubtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithSubtypeAndSizeAsync<Impl: IMediaCapture5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture5, BASE_OFFSET>(),
            RemoveEffectAsync: RemoveEffectAsync::<Impl, IMPL_OFFSET>,
            PauseRecordWithResultAsync: PauseRecordWithResultAsync::<Impl, IMPL_OFFSET>,
            StopRecordWithResultAsync: StopRecordWithResultAsync::<Impl, IMPL_OFFSET>,
            FrameSources: FrameSources::<Impl, IMPL_OFFSET>,
            CreateFrameReaderAsync: CreateFrameReaderAsync::<Impl, IMPL_OFFSET>,
            CreateFrameReaderWithSubtypeAsync: CreateFrameReaderWithSubtypeAsync::<Impl, IMPL_OFFSET>,
            CreateFrameReaderWithSubtypeAndSizeAsync: CreateFrameReaderWithSubtypeAndSizeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
pub trait IMediaCapture6Impl: Sized {
    fn CaptureDeviceExclusiveControlStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureDeviceExclusiveControlStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateMultiSourceFrameReaderAsync(&mut self, inputsources: &::core::option::Option<super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture6 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture6";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl IMediaCapture6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture6Vtbl {
        unsafe extern "system" fn CaptureDeviceExclusiveControlStatusChanged<Impl: IMediaCapture6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureDeviceExclusiveControlStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCaptureDeviceExclusiveControlStatusChanged<Impl: IMediaCapture6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCaptureDeviceExclusiveControlStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateMultiSourceFrameReaderAsync<Impl: IMediaCapture6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputsources: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMultiSourceFrameReaderAsync(&*(&inputsources as *const <super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture6, BASE_OFFSET>(),
            CaptureDeviceExclusiveControlStatusChanged: CaptureDeviceExclusiveControlStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveCaptureDeviceExclusiveControlStatusChanged: RemoveCaptureDeviceExclusiveControlStatusChanged::<Impl, IMPL_OFFSET>,
            CreateMultiSourceFrameReaderAsync: CreateMultiSourceFrameReaderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait IMediaCapture7Impl: Sized {
    fn CreateRelativePanelWatcher(&mut self, capturemode: StreamingCaptureMode, displayregion: &::core::option::Option<super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<MediaCaptureRelativePanelWatcher>;
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapture7 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapture7";
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IMediaCapture7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapture7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapture7Vtbl {
        unsafe extern "system" fn CreateRelativePanelWatcher<Impl: IMediaCapture7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capturemode: StreamingCaptureMode, displayregion: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRelativePanelWatcher(capturemode, &*(&displayregion as *const <super::super::UI::WindowManagement::DisplayRegion as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowManagement::DisplayRegion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapture7, BASE_OFFSET>(),
            CreateRelativePanelWatcher: CreateRelativePanelWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapture7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&mut self) -> ::windows::core::Result<MediaCaptureDeviceExclusiveControlStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureFailedEventArgsImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Code(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureFailedEventArgsVtbl {
        unsafe extern "system" fn Message<Impl: IMediaCaptureFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Impl: IMediaCaptureFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureFailedEventArgs, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            Code: Code::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait IMediaCaptureFocusChangedEventArgsImpl: Sized {
    fn FocusState(&mut self) -> ::windows::core::Result<super::Devices::MediaCaptureFocusState>;
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureFocusChangedEventArgs";
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl IMediaCaptureFocusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureFocusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureFocusChangedEventArgsVtbl {
        unsafe extern "system" fn FocusState<Impl: IMediaCaptureFocusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureFocusChangedEventArgs, BASE_OFFSET>(),
            FocusState: FocusState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureFocusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettingsImpl: Sized {
    fn SetAudioDeviceId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoDeviceId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VideoDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamingCaptureMode(&mut self, value: StreamingCaptureMode) -> ::windows::core::Result<()>;
    fn StreamingCaptureMode(&mut self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn SetPhotoCaptureSource(&mut self, value: PhotoCaptureSource) -> ::windows::core::Result<()>;
    fn PhotoCaptureSource(&mut self) -> ::windows::core::Result<PhotoCaptureSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettingsVtbl {
        unsafe extern "system" fn SetAudioDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDeviceId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoDeviceId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamingCaptureMode<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamingCaptureMode(value).into()
        }
        unsafe extern "system" fn StreamingCaptureMode<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamingCaptureMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotoCaptureSource<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhotoCaptureSource(value).into()
        }
        unsafe extern "system" fn PhotoCaptureSource<Impl: IMediaCaptureInitializationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoCaptureSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings, BASE_OFFSET>(),
            SetAudioDeviceId: SetAudioDeviceId::<Impl, IMPL_OFFSET>,
            AudioDeviceId: AudioDeviceId::<Impl, IMPL_OFFSET>,
            SetVideoDeviceId: SetVideoDeviceId::<Impl, IMPL_OFFSET>,
            VideoDeviceId: VideoDeviceId::<Impl, IMPL_OFFSET>,
            SetStreamingCaptureMode: SetStreamingCaptureMode::<Impl, IMPL_OFFSET>,
            StreamingCaptureMode: StreamingCaptureMode::<Impl, IMPL_OFFSET>,
            SetPhotoCaptureSource: SetPhotoCaptureSource::<Impl, IMPL_OFFSET>,
            PhotoCaptureSource: PhotoCaptureSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings2Impl: Sized {
    fn SetMediaCategory(&mut self, value: MediaCategory) -> ::windows::core::Result<()>;
    fn MediaCategory(&mut self) -> ::windows::core::Result<MediaCategory>;
    fn SetAudioProcessing(&mut self, value: super::AudioProcessing) -> ::windows::core::Result<()>;
    fn AudioProcessing(&mut self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings2Vtbl {
        unsafe extern "system" fn SetMediaCategory<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaCategory(value).into()
        }
        unsafe extern "system" fn MediaCategory<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioProcessing<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioProcessing(value).into()
        }
        unsafe extern "system" fn AudioProcessing<Impl: IMediaCaptureInitializationSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings2, BASE_OFFSET>(),
            SetMediaCategory: SetMediaCategory::<Impl, IMPL_OFFSET>,
            MediaCategory: MediaCategory::<Impl, IMPL_OFFSET>,
            SetAudioProcessing: SetAudioProcessing::<Impl, IMPL_OFFSET>,
            AudioProcessing: AudioProcessing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaCaptureInitializationSettings3Impl: Sized {
    fn SetAudioSource(&mut self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn AudioSource(&mut self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn SetVideoSource(&mut self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn VideoSource(&mut self) -> ::windows::core::Result<super::Core::IMediaSource>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings3";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaCaptureInitializationSettings3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings3Vtbl {
        unsafe extern "system" fn SetAudioSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioSource(&*(&value as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoSource(&*(&value as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoSource<Impl: IMediaCaptureInitializationSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings3, BASE_OFFSET>(),
            SetAudioSource: SetAudioSource::<Impl, IMPL_OFFSET>,
            AudioSource: AudioSource::<Impl, IMPL_OFFSET>,
            SetVideoSource: SetVideoSource::<Impl, IMPL_OFFSET>,
            VideoSource: VideoSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings4Impl: Sized {
    fn VideoProfile(&mut self) -> ::windows::core::Result<MediaCaptureVideoProfile>;
    fn SetVideoProfile(&mut self, value: &::core::option::Option<MediaCaptureVideoProfile>) -> ::windows::core::Result<()>;
    fn PreviewMediaDescription(&mut self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPreviewMediaDescription(&mut self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn RecordMediaDescription(&mut self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetRecordMediaDescription(&mut self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn PhotoMediaDescription(&mut self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPhotoMediaDescription(&mut self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings4 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings4Vtbl {
        unsafe extern "system" fn VideoProfile<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoProfile<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoProfile(&*(&value as *const <MediaCaptureVideoProfile as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviewMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecordMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecordMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhotoMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotoMediaDescription<Impl: IMediaCaptureInitializationSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhotoMediaDescription(&*(&value as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::Abi>::Abi as *const <MediaCaptureVideoProfileMediaDescription as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings4, BASE_OFFSET>(),
            VideoProfile: VideoProfile::<Impl, IMPL_OFFSET>,
            SetVideoProfile: SetVideoProfile::<Impl, IMPL_OFFSET>,
            PreviewMediaDescription: PreviewMediaDescription::<Impl, IMPL_OFFSET>,
            SetPreviewMediaDescription: SetPreviewMediaDescription::<Impl, IMPL_OFFSET>,
            RecordMediaDescription: RecordMediaDescription::<Impl, IMPL_OFFSET>,
            SetRecordMediaDescription: SetRecordMediaDescription::<Impl, IMPL_OFFSET>,
            PhotoMediaDescription: PhotoMediaDescription::<Impl, IMPL_OFFSET>,
            SetPhotoMediaDescription: SetPhotoMediaDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
pub trait IMediaCaptureInitializationSettings5Impl: Sized {
    fn SourceGroup(&mut self) -> ::windows::core::Result<Frames::MediaFrameSourceGroup>;
    fn SetSourceGroup(&mut self, value: &::core::option::Option<Frames::MediaFrameSourceGroup>) -> ::windows::core::Result<()>;
    fn SharingMode(&mut self) -> ::windows::core::Result<MediaCaptureSharingMode>;
    fn SetSharingMode(&mut self, value: MediaCaptureSharingMode) -> ::windows::core::Result<()>;
    fn MemoryPreference(&mut self) -> ::windows::core::Result<MediaCaptureMemoryPreference>;
    fn SetMemoryPreference(&mut self, value: MediaCaptureMemoryPreference) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings5 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings5";
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl IMediaCaptureInitializationSettings5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings5Vtbl {
        unsafe extern "system" fn SourceGroup<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceGroup<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceGroup(&*(&value as *const <Frames::MediaFrameSourceGroup as ::windows::core::Abi>::Abi as *const <Frames::MediaFrameSourceGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SharingMode<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharingMode<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCaptureSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharingMode(value).into()
        }
        unsafe extern "system" fn MemoryPreference<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureMemoryPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemoryPreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemoryPreference<Impl: IMediaCaptureInitializationSettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCaptureMemoryPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMemoryPreference(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings5, BASE_OFFSET>(),
            SourceGroup: SourceGroup::<Impl, IMPL_OFFSET>,
            SetSourceGroup: SetSourceGroup::<Impl, IMPL_OFFSET>,
            SharingMode: SharingMode::<Impl, IMPL_OFFSET>,
            SetSharingMode: SetSharingMode::<Impl, IMPL_OFFSET>,
            MemoryPreference: MemoryPreference::<Impl, IMPL_OFFSET>,
            SetMemoryPreference: SetMemoryPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings6Impl: Sized {
    fn AlwaysPlaySystemShutterSound(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysPlaySystemShutterSound(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings6 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings6";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureInitializationSettings6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings6Vtbl {
        unsafe extern "system" fn AlwaysPlaySystemShutterSound<Impl: IMediaCaptureInitializationSettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysPlaySystemShutterSound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysPlaySystemShutterSound<Impl: IMediaCaptureInitializationSettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysPlaySystemShutterSound(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings6, BASE_OFFSET>(),
            AlwaysPlaySystemShutterSound: AlwaysPlaySystemShutterSound::<Impl, IMPL_OFFSET>,
            SetAlwaysPlaySystemShutterSound: SetAlwaysPlaySystemShutterSound::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IMediaCaptureInitializationSettings7Impl: Sized {
    fn DeviceUriPasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetDeviceUriPasswordCredential(&mut self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn DeviceUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetDeviceUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureInitializationSettings7 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureInitializationSettings7";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IMediaCaptureInitializationSettings7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureInitializationSettings7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureInitializationSettings7Vtbl {
        unsafe extern "system" fn DeviceUriPasswordCredential<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceUriPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceUriPasswordCredential<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceUriPasswordCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceUri<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceUri<Impl: IMediaCaptureInitializationSettings7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureInitializationSettings7, BASE_OFFSET>(),
            DeviceUriPasswordCredential: DeviceUriPasswordCredential::<Impl, IMPL_OFFSET>,
            SetDeviceUriPasswordCredential: SetDeviceUriPasswordCredential::<Impl, IMPL_OFFSET>,
            DeviceUri: DeviceUri::<Impl, IMPL_OFFSET>,
            SetDeviceUri: SetDeviceUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureInitializationSettings7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaCapturePauseResultImpl: Sized {
    fn LastFrame(&mut self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCapturePauseResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaCapturePauseResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCapturePauseResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCapturePauseResultVtbl {
        unsafe extern "system" fn LastFrame<Impl: IMediaCapturePauseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDuration<Impl: IMediaCapturePauseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCapturePauseResult, BASE_OFFSET>(),
            LastFrame: LastFrame::<Impl, IMPL_OFFSET>,
            RecordDuration: RecordDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCapturePauseResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaCaptureRelativePanelWatcherImpl: Sized {
    fn RelativePanel(&mut self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureRelativePanelWatcher";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaCaptureRelativePanelWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureRelativePanelWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureRelativePanelWatcherVtbl {
        unsafe extern "system" fn RelativePanel<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativePanel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IMediaCaptureRelativePanelWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureRelativePanelWatcher, BASE_OFFSET>(),
            RelativePanel: RelativePanel::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureRelativePanelWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettingsImpl: Sized {
    fn AudioDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StreamingCaptureMode(&mut self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn PhotoCaptureSource(&mut self) -> ::windows::core::Result<PhotoCaptureSource>;
    fn VideoDeviceCharacteristic(&mut self) -> ::windows::core::Result<VideoDeviceCharacteristic>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureSettingsVtbl {
        unsafe extern "system" fn AudioDeviceId<Impl: IMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamingCaptureMode<Impl: IMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamingCaptureMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoCaptureSource<Impl: IMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoCaptureSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceCharacteristic<Impl: IMediaCaptureSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceCharacteristic) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceCharacteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureSettings, BASE_OFFSET>(),
            AudioDeviceId: AudioDeviceId::<Impl, IMPL_OFFSET>,
            VideoDeviceId: VideoDeviceId::<Impl, IMPL_OFFSET>,
            StreamingCaptureMode: StreamingCaptureMode::<Impl, IMPL_OFFSET>,
            PhotoCaptureSource: PhotoCaptureSource::<Impl, IMPL_OFFSET>,
            VideoDeviceCharacteristic: VideoDeviceCharacteristic::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaCaptureSettings2Impl: Sized {
    fn ConcurrentRecordAndPhotoSupported(&mut self) -> ::windows::core::Result<bool>;
    fn ConcurrentRecordAndPhotoSequenceSupported(&mut self) -> ::windows::core::Result<bool>;
    fn CameraSoundRequiredForRegion(&mut self) -> ::windows::core::Result<bool>;
    fn Horizontal35mmEquivalentFocalLength(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PitchOffsetDegrees(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Vertical35mmEquivalentFocalLength(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MediaCategory(&mut self) -> ::windows::core::Result<MediaCategory>;
    fn AudioProcessing(&mut self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureSettings2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaCaptureSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureSettings2Vtbl {
        unsafe extern "system" fn ConcurrentRecordAndPhotoSupported<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConcurrentRecordAndPhotoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConcurrentRecordAndPhotoSequenceSupported<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConcurrentRecordAndPhotoSequenceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraSoundRequiredForRegion<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraSoundRequiredForRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Horizontal35mmEquivalentFocalLength<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Horizontal35mmEquivalentFocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PitchOffsetDegrees<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PitchOffsetDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vertical35mmEquivalentFocalLength<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vertical35mmEquivalentFocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCategory<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioProcessing<Impl: IMediaCaptureSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureSettings2, BASE_OFFSET>(),
            ConcurrentRecordAndPhotoSupported: ConcurrentRecordAndPhotoSupported::<Impl, IMPL_OFFSET>,
            ConcurrentRecordAndPhotoSequenceSupported: ConcurrentRecordAndPhotoSequenceSupported::<Impl, IMPL_OFFSET>,
            CameraSoundRequiredForRegion: CameraSoundRequiredForRegion::<Impl, IMPL_OFFSET>,
            Horizontal35mmEquivalentFocalLength: Horizontal35mmEquivalentFocalLength::<Impl, IMPL_OFFSET>,
            PitchOffsetDegrees: PitchOffsetDegrees::<Impl, IMPL_OFFSET>,
            Vertical35mmEquivalentFocalLength: Vertical35mmEquivalentFocalLength::<Impl, IMPL_OFFSET>,
            MediaCategory: MediaCategory::<Impl, IMPL_OFFSET>,
            AudioProcessing: AudioProcessing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaCaptureSettings3Impl: Sized {
    fn Direct3D11Device(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>;
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureSettings3 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureSettings3";
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaCaptureSettings3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureSettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureSettings3Vtbl {
        unsafe extern "system" fn Direct3D11Device<Impl: IMediaCaptureSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3D11Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureSettings3, BASE_OFFSET>(),
            Direct3D11Device: Direct3D11Device::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaCaptureStaticsImpl: Sized {
    fn IsVideoProfileSupported(&mut self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn FindAllVideoProfiles(&mut self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindConcurrentProfiles(&mut self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindKnownVideoProfiles(&mut self, videodeviceid: &::windows::core::HSTRING, name: KnownVideoProfile) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaCaptureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureStaticsVtbl {
        unsafe extern "system" fn IsVideoProfileSupported<Impl: IMediaCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoProfileSupported(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllVideoProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllVideoProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConcurrentProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConcurrentProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindKnownVideoProfiles<Impl: IMediaCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindKnownVideoProfiles(&*(&videodeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), name) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureStatics, BASE_OFFSET>(),
            IsVideoProfileSupported: IsVideoProfileSupported::<Impl, IMPL_OFFSET>,
            FindAllVideoProfiles: FindAllVideoProfiles::<Impl, IMPL_OFFSET>,
            FindConcurrentProfiles: FindConcurrentProfiles::<Impl, IMPL_OFFSET>,
            FindKnownVideoProfiles: FindKnownVideoProfiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaCaptureStopResultImpl: Sized {
    fn LastFrame(&mut self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureStopResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaCaptureStopResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureStopResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureStopResultVtbl {
        unsafe extern "system" fn LastFrame<Impl: IMediaCaptureStopResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDuration<Impl: IMediaCaptureStopResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureStopResult, BASE_OFFSET>(),
            LastFrame: LastFrame::<Impl, IMPL_OFFSET>,
            RecordDuration: RecordDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureStopResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaCaptureVideoPreviewImpl: Sized {
    fn StartPreviewAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkIdAsync(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopPreviewAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureVideoPreview {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoPreview";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaCaptureVideoPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureVideoPreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVideoPreviewVtbl {
        unsafe extern "system" fn StartPreviewAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPreviewToCustomSinkAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPreviewToCustomSinkAsync(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType), &*(&custommediasink as *const <super::IMediaExtension as ::windows::core::Abi>::Abi as *const <super::IMediaExtension as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPreviewToCustomSinkIdAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn StopPreviewAsync<Impl: IMediaCaptureVideoPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopPreviewAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureVideoPreview, BASE_OFFSET>(),
            StartPreviewAsync: StartPreviewAsync::<Impl, IMPL_OFFSET>,
            StartPreviewToCustomSinkAsync: StartPreviewToCustomSinkAsync::<Impl, IMPL_OFFSET>,
            StartPreviewToCustomSinkIdAsync: StartPreviewToCustomSinkIdAsync::<Impl, IMPL_OFFSET>,
            StopPreviewAsync: StopPreviewAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureVideoPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaCaptureVideoProfileImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedPreviewMediaDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedRecordMediaDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedPhotoMediaDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn GetConcurrency(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfile";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaCaptureVideoProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureVideoProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVideoProfileVtbl {
        unsafe extern "system" fn Id<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPreviewMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPreviewMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedRecordMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedRecordMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPhotoMediaDescription<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPhotoMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConcurrency<Impl: IMediaCaptureVideoProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConcurrency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureVideoProfile, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            VideoDeviceId: VideoDeviceId::<Impl, IMPL_OFFSET>,
            SupportedPreviewMediaDescription: SupportedPreviewMediaDescription::<Impl, IMPL_OFFSET>,
            SupportedRecordMediaDescription: SupportedRecordMediaDescription::<Impl, IMPL_OFFSET>,
            SupportedPhotoMediaDescription: SupportedPhotoMediaDescription::<Impl, IMPL_OFFSET>,
            GetConcurrency: GetConcurrency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureVideoProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
pub trait IMediaCaptureVideoProfile2Impl: Sized {
    fn FrameSourceInfos(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfile2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfile2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl IMediaCaptureVideoProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureVideoProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVideoProfile2Vtbl {
        unsafe extern "system" fn FrameSourceInfos<Impl: IMediaCaptureVideoProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameSourceInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaCaptureVideoProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureVideoProfile2, BASE_OFFSET>(),
            FrameSourceInfos: FrameSourceInfos::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureVideoProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileMediaDescriptionImpl: Sized {
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
    fn FrameRate(&mut self) -> ::windows::core::Result<f64>;
    fn IsVariablePhotoSequenceSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsHdrVideoSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfileMediaDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCaptureVideoProfileMediaDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVideoProfileMediaDescriptionVtbl {
        unsafe extern "system" fn Width<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameRate<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVariablePhotoSequenceSupported<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVariablePhotoSequenceSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHdrVideoSupported<Impl: IMediaCaptureVideoProfileMediaDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHdrVideoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureVideoProfileMediaDescription, BASE_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            FrameRate: FrameRate::<Impl, IMPL_OFFSET>,
            IsVariablePhotoSequenceSupported: IsVariablePhotoSequenceSupported::<Impl, IMPL_OFFSET>,
            IsHdrVideoSupported: IsHdrVideoSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureVideoProfileMediaDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaCaptureVideoProfileMediaDescription2Impl: Sized {
    fn Subtype(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCaptureVideoProfileMediaDescription2 {
    const NAME: &'static str = "Windows.Media.Capture.IMediaCaptureVideoProfileMediaDescription2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaCaptureVideoProfileMediaDescription2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCaptureVideoProfileMediaDescription2Vtbl {
        unsafe extern "system" fn Subtype<Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaCaptureVideoProfileMediaDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCaptureVideoProfileMediaDescription2, BASE_OFFSET>(),
            Subtype: Subtype::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCaptureVideoProfileMediaDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOptionalReferencePhotoCapturedEventArgsImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn Context(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IOptionalReferencePhotoCapturedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOptionalReferencePhotoCapturedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOptionalReferencePhotoCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IOptionalReferencePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOptionalReferencePhotoCapturedEventArgs, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOptionalReferencePhotoCapturedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoCapturedEventArgsImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IPhotoCapturedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhotoCapturedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoCapturedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureTimeOffset<Impl: IPhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoCapturedEventArgs, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            CaptureTimeOffset: CaptureTimeOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoCapturedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhotoConfirmationCapturedEventArgsImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.IPhotoConfirmationCapturedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhotoConfirmationCapturedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoConfirmationCapturedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoConfirmationCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IPhotoConfirmationCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureTimeOffset<Impl: IPhotoConfirmationCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhotoConfirmationCapturedEventArgs, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            CaptureTimeOffset: CaptureTimeOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoConfirmationCapturedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IScreenCaptureImpl: Sized {
    fn AudioSource(&mut self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn VideoSource(&mut self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn IsAudioSuspended(&mut self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&mut self) -> ::windows::core::Result<bool>;
    fn SourceSuspensionChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceSuspensionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.IScreenCapture";
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl IScreenCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScreenCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScreenCaptureVtbl {
        unsafe extern "system" fn AudioSource<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoSource<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioSuspended<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAudioSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVideoSuspended<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceSuspensionChanged<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceSuspensionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceSuspensionChanged<Impl: IScreenCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceSuspensionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScreenCapture, BASE_OFFSET>(),
            AudioSource: AudioSource::<Impl, IMPL_OFFSET>,
            VideoSource: VideoSource::<Impl, IMPL_OFFSET>,
            IsAudioSuspended: IsAudioSuspended::<Impl, IMPL_OFFSET>,
            IsVideoSuspended: IsVideoSuspended::<Impl, IMPL_OFFSET>,
            SourceSuspensionChanged: SourceSuspensionChanged::<Impl, IMPL_OFFSET>,
            RemoveSourceSuspensionChanged: RemoveSourceSuspensionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScreenCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ScreenCapture>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScreenCaptureStatics {
    const NAME: &'static str = "Windows.Media.Capture.IScreenCaptureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScreenCaptureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScreenCaptureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScreenCaptureStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IScreenCaptureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScreenCaptureStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScreenCaptureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISourceSuspensionChangedEventArgsImpl: Sized {
    fn IsAudioSuspended(&mut self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.ISourceSuspensionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISourceSuspensionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISourceSuspensionChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISourceSuspensionChangedEventArgsVtbl {
        unsafe extern "system" fn IsAudioSuspended<Impl: ISourceSuspensionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAudioSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVideoSuspended<Impl: ISourceSuspensionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoSuspended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISourceSuspensionChangedEventArgs, BASE_OFFSET>(),
            IsAudioSuspended: IsAudioSuspended::<Impl, IMPL_OFFSET>,
            IsVideoSuspended: IsVideoSuspended::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISourceSuspensionChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IVideoStreamConfigurationImpl: Sized {
    fn InputProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn OutputProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.IVideoStreamConfiguration";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IVideoStreamConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStreamConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStreamConfigurationVtbl {
        unsafe extern "system" fn InputProperties<Impl: IVideoStreamConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputProperties<Impl: IVideoStreamConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStreamConfiguration, BASE_OFFSET>(),
            InputProperties: InputProperties::<Impl, IMPL_OFFSET>,
            OutputProperties: OutputProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStreamConfiguration as ::windows::core::Interface>::IID
    }
}

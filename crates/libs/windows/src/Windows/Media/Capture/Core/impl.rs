#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVariablePhotoCapturedEventArgsImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<super::CapturedFrame>;
    fn CaptureTimeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn UsedFrameControllerIndex(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn CapturedFrameControlValues(&mut self) -> ::windows::core::Result<super::CapturedFrameControlValues>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVariablePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVariablePhotoCapturedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariablePhotoCapturedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVariablePhotoCapturedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IVariablePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CaptureTimeOffset<Impl: IVariablePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsedFrameControllerIndex<Impl: IVariablePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedFrameControllerIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturedFrameControlValues<Impl: IVariablePhotoCapturedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturedFrameControlValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVariablePhotoCapturedEventArgs, BASE_OFFSET>(),
            Frame: Frame::<Impl, IMPL_OFFSET>,
            CaptureTimeOffset: CaptureTimeOffset::<Impl, IMPL_OFFSET>,
            UsedFrameControllerIndex: UsedFrameControllerIndex::<Impl, IMPL_OFFSET>,
            CapturedFrameControlValues: CapturedFrameControlValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVariablePhotoCapturedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVariablePhotoSequenceCaptureImpl: Sized {
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn PhotoCaptured(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoCaptured(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVariablePhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.Core.IVariablePhotoSequenceCapture";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVariablePhotoSequenceCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariablePhotoSequenceCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVariablePhotoSequenceCaptureVtbl {
        unsafe extern "system" fn StartAsync<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopAsync<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FinishAsync<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhotoCaptured<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhotoCaptured(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePhotoCaptured<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePhotoCaptured(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IVariablePhotoSequenceCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVariablePhotoSequenceCapture, BASE_OFFSET>(),
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
            PhotoCaptured: PhotoCaptured::<Impl, IMPL_OFFSET>,
            RemovePhotoCaptured: RemovePhotoCaptured::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVariablePhotoSequenceCapture as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVariablePhotoSequenceCapture2Impl: Sized {
    fn UpdateSettingsAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVariablePhotoSequenceCapture2 {
    const NAME: &'static str = "Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVariablePhotoSequenceCapture2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVariablePhotoSequenceCapture2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVariablePhotoSequenceCapture2Vtbl {
        unsafe extern "system" fn UpdateSettingsAsync<Impl: IVariablePhotoSequenceCapture2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateSettingsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVariablePhotoSequenceCapture2, BASE_OFFSET>(),
            UpdateSettingsAsync: UpdateSettingsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVariablePhotoSequenceCapture2 as ::windows::core::Interface>::IID
    }
}

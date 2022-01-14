#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStatics_Impl: Sized {
    fn Click(&mut self) -> ::windows::core::Result<u16>;
    fn BuzzContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn RumbleContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn Press(&mut self) -> ::windows::core::Result<u16>;
    fn Release(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownSimpleHapticsControllerWaveformsStatics {
    const NAME: &'static str = "Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownSimpleHapticsControllerWaveformsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownSimpleHapticsControllerWaveformsStatics_Vtbl {
        unsafe extern "system" fn Click<Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Click() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuzzContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuzzContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RumbleContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RumbleContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Press<Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Press() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Release<Impl: IKnownSimpleHapticsControllerWaveformsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownSimpleHapticsControllerWaveformsStatics, BASE_OFFSET>(),
            Click: Click::<Impl, IMPL_OFFSET>,
            BuzzContinuous: BuzzContinuous::<Impl, IMPL_OFFSET>,
            RumbleContinuous: RumbleContinuous::<Impl, IMPL_OFFSET>,
            Press: Press::<Impl, IMPL_OFFSET>,
            Release: Release::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownSimpleHapticsControllerWaveformsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStatics2_Impl: Sized {
    fn BrushContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn ChiselMarkerContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn EraserContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn Error(&mut self) -> ::windows::core::Result<u16>;
    fn GalaxyPenContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn Hover(&mut self) -> ::windows::core::Result<u16>;
    fn InkContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn MarkerContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn PencilContinuous(&mut self) -> ::windows::core::Result<u16>;
    fn Success(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownSimpleHapticsControllerWaveformsStatics2 {
    const NAME: &'static str = "Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl {
        unsafe extern "system" fn BrushContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrushContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChiselMarkerContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChiselMarkerContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraserContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GalaxyPenContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GalaxyPenContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hover<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InkContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkerContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkerContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PencilContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PencilContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Success<Impl: IKnownSimpleHapticsControllerWaveformsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Success() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownSimpleHapticsControllerWaveformsStatics2, BASE_OFFSET>(),
            BrushContinuous: BrushContinuous::<Impl, IMPL_OFFSET>,
            ChiselMarkerContinuous: ChiselMarkerContinuous::<Impl, IMPL_OFFSET>,
            EraserContinuous: EraserContinuous::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            GalaxyPenContinuous: GalaxyPenContinuous::<Impl, IMPL_OFFSET>,
            Hover: Hover::<Impl, IMPL_OFFSET>,
            InkContinuous: InkContinuous::<Impl, IMPL_OFFSET>,
            MarkerContinuous: MarkerContinuous::<Impl, IMPL_OFFSET>,
            PencilContinuous: PencilContinuous::<Impl, IMPL_OFFSET>,
            Success: Success::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownSimpleHapticsControllerWaveformsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISimpleHapticsController_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedFeedback(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>>;
    fn IsIntensitySupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPlayCountSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPlayDurationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsReplayPauseIntervalSupported(&mut self) -> ::windows::core::Result<bool>;
    fn StopFeedback(&mut self) -> ::windows::core::Result<()>;
    fn SendHapticFeedback(&mut self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackWithIntensity(&mut self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForDuration(&mut self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForPlayCount(&mut self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playcount: i32, replaypauseinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.ISimpleHapticsController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISimpleHapticsController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleHapticsController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleHapticsController_Vtbl {
        unsafe extern "system" fn Id<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedFeedback<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIntensitySupported<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIntensitySupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlayCountSupported<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlayCountSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlayDurationSupported<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlayDurationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReplayPauseIntervalSupported<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReplayPauseIntervalSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopFeedback<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopFeedback().into()
        }
        unsafe extern "system" fn SendHapticFeedback<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendHapticFeedback(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendHapticFeedbackWithIntensity<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackWithIntensity(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity).into()
        }
        unsafe extern "system" fn SendHapticFeedbackForDuration<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackForDuration(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity, &*(&playduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendHapticFeedbackForPlayCount<Impl: ISimpleHapticsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackForPlayCount(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity, playcount, &*(&replaypauseinterval as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleHapticsController, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SupportedFeedback: SupportedFeedback::<Impl, IMPL_OFFSET>,
            IsIntensitySupported: IsIntensitySupported::<Impl, IMPL_OFFSET>,
            IsPlayCountSupported: IsPlayCountSupported::<Impl, IMPL_OFFSET>,
            IsPlayDurationSupported: IsPlayDurationSupported::<Impl, IMPL_OFFSET>,
            IsReplayPauseIntervalSupported: IsReplayPauseIntervalSupported::<Impl, IMPL_OFFSET>,
            StopFeedback: StopFeedback::<Impl, IMPL_OFFSET>,
            SendHapticFeedback: SendHapticFeedback::<Impl, IMPL_OFFSET>,
            SendHapticFeedbackWithIntensity: SendHapticFeedbackWithIntensity::<Impl, IMPL_OFFSET>,
            SendHapticFeedbackForDuration: SendHapticFeedbackForDuration::<Impl, IMPL_OFFSET>,
            SendHapticFeedbackForPlayCount: SendHapticFeedbackForPlayCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleHapticsController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISimpleHapticsControllerFeedback_Impl: Sized {
    fn Waveform(&mut self) -> ::windows::core::Result<u16>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.ISimpleHapticsControllerFeedback";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISimpleHapticsControllerFeedback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleHapticsControllerFeedback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleHapticsControllerFeedback_Vtbl {
        unsafe extern "system" fn Waveform<Impl: ISimpleHapticsControllerFeedback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Waveform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: ISimpleHapticsControllerFeedback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISimpleHapticsControllerFeedback, BASE_OFFSET>(),
            Waveform: Waveform::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleHapticsControllerFeedback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDevice_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimpleHapticsController(&mut self) -> ::windows::core::Result<SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.IVibrationDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IVibrationDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVibrationDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVibrationDevice_Vtbl {
        unsafe extern "system" fn Id<Impl: IVibrationDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SimpleHapticsController<Impl: IVibrationDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVibrationDevice, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SimpleHapticsController: SimpleHapticsController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVibrationDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVibrationDeviceStatics_Impl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVibrationDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Haptics.IVibrationDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVibrationDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVibrationDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVibrationDeviceStatics_Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IVibrationDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IVibrationDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IVibrationDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: IVibrationDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IVibrationDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVibrationDeviceStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVibrationDeviceStatics as ::windows::core::Interface>::IID
    }
}

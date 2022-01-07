#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStaticsImpl: Sized {
    fn Click(&self) -> ::windows::core::Result<u16>;
    fn BuzzContinuous(&self) -> ::windows::core::Result<u16>;
    fn RumbleContinuous(&self) -> ::windows::core::Result<u16>;
    fn Press(&self) -> ::windows::core::Result<u16>;
    fn Release(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownSimpleHapticsControllerWaveformsStatics {
    const NAME: &'static str = "Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownSimpleHapticsControllerWaveformsStaticsVtbl {
    pub const fn new<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownSimpleHapticsControllerWaveformsStaticsVtbl {
        unsafe extern "system" fn Click<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Click() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuzzContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuzzContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RumbleContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RumbleContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Press<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Press() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Release<Impl: IKnownSimpleHapticsControllerWaveformsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownSimpleHapticsControllerWaveformsStatics>, base.5, Click::<Impl, OFFSET>, BuzzContinuous::<Impl, OFFSET>, RumbleContinuous::<Impl, OFFSET>, Press::<Impl, OFFSET>, Release::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStatics2Impl: Sized {
    fn BrushContinuous(&self) -> ::windows::core::Result<u16>;
    fn ChiselMarkerContinuous(&self) -> ::windows::core::Result<u16>;
    fn EraserContinuous(&self) -> ::windows::core::Result<u16>;
    fn Error(&self) -> ::windows::core::Result<u16>;
    fn GalaxyPenContinuous(&self) -> ::windows::core::Result<u16>;
    fn Hover(&self) -> ::windows::core::Result<u16>;
    fn InkContinuous(&self) -> ::windows::core::Result<u16>;
    fn MarkerContinuous(&self) -> ::windows::core::Result<u16>;
    fn PencilContinuous(&self) -> ::windows::core::Result<u16>;
    fn Success(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownSimpleHapticsControllerWaveformsStatics2 {
    const NAME: &'static str = "Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownSimpleHapticsControllerWaveformsStatics2Vtbl {
    pub const fn new<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownSimpleHapticsControllerWaveformsStatics2Vtbl {
        unsafe extern "system" fn BrushContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrushContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChiselMarkerContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChiselMarkerContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraserContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EraserContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GalaxyPenContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GalaxyPenContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hover<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InkContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InkContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkerContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarkerContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PencilContinuous<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PencilContinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Success<Impl: IKnownSimpleHapticsControllerWaveformsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Success() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownSimpleHapticsControllerWaveformsStatics2>, base.5, BrushContinuous::<Impl, OFFSET>, ChiselMarkerContinuous::<Impl, OFFSET>, EraserContinuous::<Impl, OFFSET>, Error::<Impl, OFFSET>, GalaxyPenContinuous::<Impl, OFFSET>, Hover::<Impl, OFFSET>, InkContinuous::<Impl, OFFSET>, MarkerContinuous::<Impl, OFFSET>, PencilContinuous::<Impl, OFFSET>, Success::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleHapticsControllerImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedFeedback(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>>;
    fn IsIntensitySupported(&self) -> ::windows::core::Result<bool>;
    fn IsPlayCountSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPlayDurationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReplayPauseIntervalSupported(&self) -> ::windows::core::Result<bool>;
    fn StopFeedback(&self) -> ::windows::core::Result<()>;
    fn SendHapticFeedback(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackWithIntensity(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForDuration(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForPlayCount(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playcount: i32, replaypauseinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.ISimpleHapticsController";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleHapticsControllerVtbl {
    pub const fn new<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISimpleHapticsControllerVtbl {
        unsafe extern "system" fn Id<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedFeedback<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedFeedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIntensitySupported<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIntensitySupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlayCountSupported<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPlayCountSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPlayDurationSupported<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPlayDurationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReplayPauseIntervalSupported<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReplayPauseIntervalSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopFeedback<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopFeedback().into()
        }
        unsafe extern "system" fn SendHapticFeedback<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendHapticFeedback(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendHapticFeedbackWithIntensity<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackWithIntensity(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity).into()
        }
        unsafe extern "system" fn SendHapticFeedbackForDuration<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackForDuration(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity, &*(&playduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendHapticFeedbackForPlayCount<Impl: ISimpleHapticsControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendHapticFeedbackForPlayCount(&*(&feedback as *const <SimpleHapticsControllerFeedback as ::windows::core::Abi>::Abi as *const <SimpleHapticsControllerFeedback as ::windows::core::DefaultType>::DefaultType), intensity, playcount, &*(&replaypauseinterval as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISimpleHapticsController>,
            base.5,
            Id::<Impl, OFFSET>,
            SupportedFeedback::<Impl, OFFSET>,
            IsIntensitySupported::<Impl, OFFSET>,
            IsPlayCountSupported::<Impl, OFFSET>,
            IsPlayDurationSupported::<Impl, OFFSET>,
            IsReplayPauseIntervalSupported::<Impl, OFFSET>,
            StopFeedback::<Impl, OFFSET>,
            SendHapticFeedback::<Impl, OFFSET>,
            SendHapticFeedbackWithIntensity::<Impl, OFFSET>,
            SendHapticFeedbackForDuration::<Impl, OFFSET>,
            SendHapticFeedbackForPlayCount::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleHapticsControllerFeedbackImpl: Sized {
    fn Waveform(&self) -> ::windows::core::Result<u16>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.ISimpleHapticsControllerFeedback";
}
#[cfg(feature = "implement_exclusive")]
impl ISimpleHapticsControllerFeedbackVtbl {
    pub const fn new<Impl: ISimpleHapticsControllerFeedbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISimpleHapticsControllerFeedbackVtbl {
        unsafe extern "system" fn Waveform<Impl: ISimpleHapticsControllerFeedbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Waveform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: ISimpleHapticsControllerFeedbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISimpleHapticsControllerFeedback>, base.5, Waveform::<Impl, OFFSET>, Duration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.IVibrationDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IVibrationDeviceVtbl {
    pub const fn new<Impl: IVibrationDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVibrationDeviceVtbl {
        unsafe extern "system" fn Id<Impl: IVibrationDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SimpleHapticsController<Impl: IVibrationDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVibrationDevice>, base.5, Id::<Impl, OFFSET>, SimpleHapticsController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVibrationDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Haptics.IVibrationDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVibrationDeviceStaticsVtbl {
    pub const fn new<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVibrationDeviceStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAsync<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IVibrationDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVibrationDeviceStatics>, base.5, RequestAccessAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDefaultAsync::<Impl, OFFSET>, FindAllAsync::<Impl, OFFSET>)
    }
}

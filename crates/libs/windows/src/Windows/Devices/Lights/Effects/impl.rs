#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapEffectImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SuggestedBitmapSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn BitmapRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBitmapEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>() -> ILampArrayBitmapEffectVtbl {
        unsafe extern "system" fn Duration<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDelay<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDelay<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDelay(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateInterval<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateInterval(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestedBitmapSize<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedBitmapSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapRequested<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBitmapRequested<Impl: ILampArrayBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBitmapRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILampArrayBitmapEffect>,
            ::windows::core::GetTrustLevel,
            Duration::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            StartDelay::<Impl, OFFSET>,
            SetStartDelay::<Impl, OFFSET>,
            UpdateInterval::<Impl, OFFSET>,
            SetUpdateInterval::<Impl, OFFSET>,
            SuggestedBitmapSize::<Impl, OFFSET>,
            BitmapRequested::<Impl, OFFSET>,
            RemoveBitmapRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBitmapEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBitmapEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBitmapEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapEffectFactoryImpl, const OFFSET: isize>() -> ILampArrayBitmapEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILampArrayBitmapEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&lamparray as *const <super::LampArray as ::windows::core::Abi>::Abi as *const <super::LampArray as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayBitmapEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapRequestedEventArgsImpl: Sized {
    fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn UpdateBitmap(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBitmapRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapRequestedEventArgsImpl, const OFFSET: isize>() -> ILampArrayBitmapRequestedEventArgsVtbl {
        unsafe extern "system" fn SinceStarted<Impl: ILampArrayBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SinceStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateBitmap<Impl: ILampArrayBitmapRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateBitmap(&*(&bitmap as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayBitmapRequestedEventArgs>, ::windows::core::GetTrustLevel, SinceStarted::<Impl, OFFSET>, UpdateBitmap::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBlinkEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AttackDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetAttackDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SustainDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetSustainDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecayDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDecayDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn RepetitionDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRepetitionDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBlinkEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBlinkEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>() -> ILampArrayBlinkEffectVtbl {
        unsafe extern "system" fn Color<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AttackDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttackDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttackDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttackDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SustainDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SustainDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSustainDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSustainDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DecayDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecayDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecayDuration<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecayDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RepetitionDelay<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepetitionDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetitionDelay<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepetitionDelay(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDelay<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDelay<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDelay(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Occurrences<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occurrences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOccurrences<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOccurrences(value).into()
        }
        unsafe extern "system" fn RepetitionMode<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepetitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetitionMode<Impl: ILampArrayBlinkEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepetitionMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILampArrayBlinkEffect>,
            ::windows::core::GetTrustLevel,
            Color::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            AttackDuration::<Impl, OFFSET>,
            SetAttackDuration::<Impl, OFFSET>,
            SustainDuration::<Impl, OFFSET>,
            SetSustainDuration::<Impl, OFFSET>,
            DecayDuration::<Impl, OFFSET>,
            SetDecayDuration::<Impl, OFFSET>,
            RepetitionDelay::<Impl, OFFSET>,
            SetRepetitionDelay::<Impl, OFFSET>,
            StartDelay::<Impl, OFFSET>,
            SetStartDelay::<Impl, OFFSET>,
            Occurrences::<Impl, OFFSET>,
            SetOccurrences::<Impl, OFFSET>,
            RepetitionMode::<Impl, OFFSET>,
            SetRepetitionMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBlinkEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBlinkEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBlinkEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBlinkEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBlinkEffectFactoryImpl, const OFFSET: isize>() -> ILampArrayBlinkEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILampArrayBlinkEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&lamparray as *const <super::LampArray as ::windows::core::Abi>::Abi as *const <super::LampArray as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayBlinkEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayColorRampEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn RampDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRampDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayColorRampEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayColorRampEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>() -> ILampArrayColorRampEffectVtbl {
        unsafe extern "system" fn Color<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RampDuration<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RampDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRampDuration<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRampDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDelay<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDelay<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDelay(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompletionBehavior<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompletionBehavior<Impl: ILampArrayColorRampEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletionBehavior(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILampArrayColorRampEffect>,
            ::windows::core::GetTrustLevel,
            Color::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            RampDuration::<Impl, OFFSET>,
            SetRampDuration::<Impl, OFFSET>,
            StartDelay::<Impl, OFFSET>,
            SetStartDelay::<Impl, OFFSET>,
            CompletionBehavior::<Impl, OFFSET>,
            SetCompletionBehavior::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayColorRampEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayColorRampEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayColorRampEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayColorRampEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayColorRampEffectFactoryImpl, const OFFSET: isize>() -> ILampArrayColorRampEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILampArrayColorRampEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&lamparray as *const <super::LampArray as ::windows::core::Abi>::Abi as *const <super::LampArray as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayColorRampEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayCustomEffectImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayCustomEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayCustomEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>() -> ILampArrayCustomEffectVtbl {
        unsafe extern "system" fn Duration<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateInterval<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateInterval(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateRequested<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateRequested<Impl: ILampArrayCustomEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdateRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayCustomEffect>, ::windows::core::GetTrustLevel, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, UpdateInterval::<Impl, OFFSET>, SetUpdateInterval::<Impl, OFFSET>, UpdateRequested::<Impl, OFFSET>, RemoveUpdateRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayCustomEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayCustomEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayCustomEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayCustomEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayCustomEffectFactoryImpl, const OFFSET: isize>() -> ILampArrayCustomEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILampArrayCustomEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&lamparray as *const <super::LampArray as ::windows::core::Abi>::Abi as *const <super::LampArray as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayCustomEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait ILampArrayEffectImpl: Sized {
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectImpl, const OFFSET: isize>() -> ILampArrayEffectVtbl {
        unsafe extern "system" fn ZIndex<Impl: ILampArrayEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: ILampArrayEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayEffect>, ::windows::core::GetTrustLevel, ZIndex::<Impl, OFFSET>, SetZIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayEffectPlaylistImpl: Sized {
    fn Append(&self, effect: &::core::option::Option<ILampArrayEffect>) -> ::windows::core::Result<()>;
    fn OverrideZIndex(&self, zindex: i32) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn EffectStartMode(&self) -> ::windows::core::Result<LampArrayEffectStartMode>;
    fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayEffectPlaylistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>() -> ILampArrayEffectPlaylistVtbl {
        unsafe extern "system" fn Append<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&effect as *const <ILampArrayEffect as ::windows::core::Abi>::Abi as *const <ILampArrayEffect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideZIndex<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideZIndex(zindex).into()
        }
        unsafe extern "system" fn Start<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn EffectStartMode<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectStartMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectStartMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffectStartMode<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LampArrayEffectStartMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffectStartMode(value).into()
        }
        unsafe extern "system" fn Occurrences<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occurrences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOccurrences<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOccurrences(value).into()
        }
        unsafe extern "system" fn RepetitionMode<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepetitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetitionMode<Impl: ILampArrayEffectPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepetitionMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILampArrayEffectPlaylist>,
            ::windows::core::GetTrustLevel,
            Append::<Impl, OFFSET>,
            OverrideZIndex::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            EffectStartMode::<Impl, OFFSET>,
            SetEffectStartMode::<Impl, OFFSET>,
            Occurrences::<Impl, OFFSET>,
            SetOccurrences::<Impl, OFFSET>,
            RepetitionMode::<Impl, OFFSET>,
            SetRepetitionMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayEffectPlaylistStaticsImpl: Sized {
    fn StartAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn StopAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn PauseAll(&self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayEffectPlaylistStatics {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayEffectPlaylistStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectPlaylistStaticsImpl, const OFFSET: isize>() -> ILampArrayEffectPlaylistStaticsVtbl {
        unsafe extern "system" fn StartAll<Impl: ILampArrayEffectPlaylistStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAll(&*(&value as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAll<Impl: ILampArrayEffectPlaylistStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAll(&*(&value as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseAll<Impl: ILampArrayEffectPlaylistStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseAll(&*(&value as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayEffectPlaylistStatics>, ::windows::core::GetTrustLevel, StartAll::<Impl, OFFSET>, StopAll::<Impl, OFFSET>, PauseAll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArraySolidEffectImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArraySolidEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArraySolidEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArraySolidEffectImpl, const OFFSET: isize>() -> ILampArraySolidEffectVtbl {
        unsafe extern "system" fn Color<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDelay<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDelay<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDelay(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompletionBehavior<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompletionBehavior<Impl: ILampArraySolidEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletionBehavior(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILampArraySolidEffect>,
            ::windows::core::GetTrustLevel,
            Color::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            StartDelay::<Impl, OFFSET>,
            SetStartDelay::<Impl, OFFSET>,
            CompletionBehavior::<Impl, OFFSET>,
            SetCompletionBehavior::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArraySolidEffectFactoryImpl: Sized {
    fn CreateInstance(&self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArraySolidEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArraySolidEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArraySolidEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArraySolidEffectFactoryImpl, const OFFSET: isize>() -> ILampArraySolidEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ILampArraySolidEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lamparray: ::windows::core::RawPtr, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&lamparray as *const <super::LampArray as ::windows::core::Abi>::Abi as *const <super::LampArray as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArraySolidEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayUpdateRequestedEventArgsImpl: Sized {
    fn SinceStarted(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetColor(&self, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetColorForIndex(&self, lampindex: i32, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetSingleColorForIndices(&self, desiredcolor: &super::super::super::UI::Color, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForIndices(&self, desiredcolors: &[<super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayUpdateRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>() -> ILampArrayUpdateRequestedEventArgsVtbl {
        unsafe extern "system" fn SinceStarted<Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SinceStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&desiredcolor as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetColorForIndex<Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorForIndex(lampindex, &*(&desiredcolor as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSingleColorForIndices<Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSingleColorForIndices(&*(&desiredcolor as *const <super::super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)).into()
        }
        unsafe extern "system" fn SetColorsForIndices<Impl: ILampArrayUpdateRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorsForIndices(::core::slice::from_raw_parts(::core::mem::transmute_copy(&desiredcolors), desiredColors_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILampArrayUpdateRequestedEventArgs>, ::windows::core::GetTrustLevel, SinceStarted::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, SetColorForIndex::<Impl, OFFSET>, SetSingleColorForIndices::<Impl, OFFSET>, SetColorsForIndices::<Impl, OFFSET>)
    }
}

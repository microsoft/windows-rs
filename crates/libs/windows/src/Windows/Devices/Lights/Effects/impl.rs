#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILampArrayBitmapEffectImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SuggestedBitmapSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn BitmapRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapEffect";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILampArrayBitmapEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayBitmapEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayBitmapEffect, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            StartDelay: StartDelay::<Impl, IMPL_OFFSET>,
            SetStartDelay: SetStartDelay::<Impl, IMPL_OFFSET>,
            UpdateInterval: UpdateInterval::<Impl, IMPL_OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Impl, IMPL_OFFSET>,
            SuggestedBitmapSize: SuggestedBitmapSize::<Impl, IMPL_OFFSET>,
            BitmapRequested: BitmapRequested::<Impl, IMPL_OFFSET>,
            RemoveBitmapRequested: RemoveBitmapRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayBitmapEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBitmapEffectFactoryImpl: Sized {
    fn CreateInstance(&mut self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBitmapEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBitmapEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBitmapEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayBitmapEffectFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayBitmapEffectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayBitmapEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ILampArrayBitmapRequestedEventArgsImpl: Sized {
    fn SinceStarted(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn UpdateBitmap(&mut self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ILampArrayBitmapRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBitmapRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayBitmapRequestedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayBitmapRequestedEventArgs, BASE_OFFSET>(),
            SinceStarted: SinceStarted::<Impl, IMPL_OFFSET>,
            UpdateBitmap: UpdateBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayBitmapRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampArrayBlinkEffectImpl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AttackDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetAttackDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SustainDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetSustainDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecayDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDecayDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn RepetitionDelay(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRepetitionDelay(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Occurrences(&mut self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&mut self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&mut self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBlinkEffect";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ILampArrayBlinkEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBlinkEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayBlinkEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayBlinkEffect, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            AttackDuration: AttackDuration::<Impl, IMPL_OFFSET>,
            SetAttackDuration: SetAttackDuration::<Impl, IMPL_OFFSET>,
            SustainDuration: SustainDuration::<Impl, IMPL_OFFSET>,
            SetSustainDuration: SetSustainDuration::<Impl, IMPL_OFFSET>,
            DecayDuration: DecayDuration::<Impl, IMPL_OFFSET>,
            SetDecayDuration: SetDecayDuration::<Impl, IMPL_OFFSET>,
            RepetitionDelay: RepetitionDelay::<Impl, IMPL_OFFSET>,
            SetRepetitionDelay: SetRepetitionDelay::<Impl, IMPL_OFFSET>,
            StartDelay: StartDelay::<Impl, IMPL_OFFSET>,
            SetStartDelay: SetStartDelay::<Impl, IMPL_OFFSET>,
            Occurrences: Occurrences::<Impl, IMPL_OFFSET>,
            SetOccurrences: SetOccurrences::<Impl, IMPL_OFFSET>,
            RepetitionMode: RepetitionMode::<Impl, IMPL_OFFSET>,
            SetRepetitionMode: SetRepetitionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayBlinkEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayBlinkEffectFactoryImpl: Sized {
    fn CreateInstance(&mut self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayBlinkEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayBlinkEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayBlinkEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayBlinkEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayBlinkEffectFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayBlinkEffectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayBlinkEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampArrayColorRampEffectImpl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn RampDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRampDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&mut self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&mut self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayColorRampEffect";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ILampArrayColorRampEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayColorRampEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayColorRampEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayColorRampEffect, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            RampDuration: RampDuration::<Impl, IMPL_OFFSET>,
            SetRampDuration: SetRampDuration::<Impl, IMPL_OFFSET>,
            StartDelay: StartDelay::<Impl, IMPL_OFFSET>,
            SetStartDelay: SetStartDelay::<Impl, IMPL_OFFSET>,
            CompletionBehavior: CompletionBehavior::<Impl, IMPL_OFFSET>,
            SetCompletionBehavior: SetCompletionBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayColorRampEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayColorRampEffectFactoryImpl: Sized {
    fn CreateInstance(&mut self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayColorRampEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayColorRampEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayColorRampEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayColorRampEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayColorRampEffectFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayColorRampEffectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayColorRampEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILampArrayCustomEffectImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateInterval(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetUpdateInterval(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn UpdateRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayCustomEffect";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILampArrayCustomEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayCustomEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayCustomEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayCustomEffect, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            UpdateInterval: UpdateInterval::<Impl, IMPL_OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Impl, IMPL_OFFSET>,
            UpdateRequested: UpdateRequested::<Impl, IMPL_OFFSET>,
            RemoveUpdateRequested: RemoveUpdateRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayCustomEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayCustomEffectFactoryImpl: Sized {
    fn CreateInstance(&mut self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArrayCustomEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayCustomEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayCustomEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayCustomEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayCustomEffectFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayCustomEffectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayCustomEffectFactory as ::windows::core::Interface>::IID
    }
}
pub trait ILampArrayEffectImpl: Sized {
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayEffect, BASE_OFFSET>(),
            ZIndex: ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex: SetZIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArrayEffectPlaylistImpl: Sized {
    fn Append(&mut self, effect: &::core::option::Option<ILampArrayEffect>) -> ::windows::core::Result<()>;
    fn OverrideZIndex(&mut self, zindex: i32) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn EffectStartMode(&mut self) -> ::windows::core::Result<LampArrayEffectStartMode>;
    fn SetEffectStartMode(&mut self, value: LampArrayEffectStartMode) -> ::windows::core::Result<()>;
    fn Occurrences(&mut self) -> ::windows::core::Result<i32>;
    fn SetOccurrences(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn RepetitionMode(&mut self) -> ::windows::core::Result<LampArrayRepetitionMode>;
    fn SetRepetitionMode(&mut self, value: LampArrayRepetitionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArrayEffectPlaylistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectPlaylistImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayEffectPlaylistVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayEffectPlaylist, BASE_OFFSET>(),
            Append: Append::<Impl, IMPL_OFFSET>,
            OverrideZIndex: OverrideZIndex::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            EffectStartMode: EffectStartMode::<Impl, IMPL_OFFSET>,
            SetEffectStartMode: SetEffectStartMode::<Impl, IMPL_OFFSET>,
            Occurrences: Occurrences::<Impl, IMPL_OFFSET>,
            SetOccurrences: SetOccurrences::<Impl, IMPL_OFFSET>,
            RepetitionMode: RepetitionMode::<Impl, IMPL_OFFSET>,
            SetRepetitionMode: SetRepetitionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayEffectPlaylist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILampArrayEffectPlaylistStaticsImpl: Sized {
    fn StartAll(&mut self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn StopAll(&mut self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
    fn PauseAll(&mut self, value: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayEffectPlaylistStatics {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILampArrayEffectPlaylistStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffectPlaylistStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayEffectPlaylistStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayEffectPlaylistStatics, BASE_OFFSET>(),
            StartAll: StartAll::<Impl, IMPL_OFFSET>,
            StopAll: StopAll::<Impl, IMPL_OFFSET>,
            PauseAll: PauseAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayEffectPlaylistStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampArraySolidEffectImpl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartDelay(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetStartDelay(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CompletionBehavior(&mut self) -> ::windows::core::Result<LampArrayEffectCompletionBehavior>;
    fn SetCompletionBehavior(&mut self, value: LampArrayEffectCompletionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArraySolidEffect";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ILampArraySolidEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArraySolidEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArraySolidEffectVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArraySolidEffect, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            StartDelay: StartDelay::<Impl, IMPL_OFFSET>,
            SetStartDelay: SetStartDelay::<Impl, IMPL_OFFSET>,
            CompletionBehavior: CompletionBehavior::<Impl, IMPL_OFFSET>,
            SetCompletionBehavior: SetCompletionBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArraySolidEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampArraySolidEffectFactoryImpl: Sized {
    fn CreateInstance(&mut self, lamparray: &::core::option::Option<super::LampArray>, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<LampArraySolidEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampArraySolidEffectFactory {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILampArraySolidEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArraySolidEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArraySolidEffectFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArraySolidEffectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArraySolidEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampArrayUpdateRequestedEventArgsImpl: Sized {
    fn SinceStarted(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetColor(&mut self, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetColorForIndex(&mut self, lampindex: i32, desiredcolor: &super::super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetSingleColorForIndices(&mut self, desiredcolor: &super::super::super::UI::Color, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForIndices(&mut self, desiredcolors: &[<super::super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ILampArrayUpdateRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayUpdateRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayUpdateRequestedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayUpdateRequestedEventArgs, BASE_OFFSET>(),
            SinceStarted: SinceStarted::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            SetColorForIndex: SetColorForIndex::<Impl, IMPL_OFFSET>,
            SetSingleColorForIndices: SetSingleColorForIndices::<Impl, IMPL_OFFSET>,
            SetColorsForIndices: SetColorsForIndices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayUpdateRequestedEventArgs as ::windows::core::Interface>::IID
    }
}

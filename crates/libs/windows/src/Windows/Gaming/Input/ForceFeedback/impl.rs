#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IConditionForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind>;
    fn SetParameters(&self, direction: &super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IConditionForceEffect";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IConditionForceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConditionForceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConditionForceEffectVtbl {
        unsafe extern "system" fn Kind<Impl: IConditionForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConditionForceEffectKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IConditionForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&direction as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConditionForceEffect>, ::windows::core::GetTrustLevel, Kind::<Impl, IMPL_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionForceEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConditionForceEffectFactoryImpl: Sized {
    fn CreateInstance(&self, effectkind: ConditionForceEffectKind) -> ::windows::core::Result<ConditionForceEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConditionForceEffectFactory {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IConditionForceEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConditionForceEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConditionForceEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IConditionForceEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effectkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConditionForceEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConditionForceEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IConstantForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IConstantForceEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IConstantForceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstantForceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstantForceEffectVtbl {
        unsafe extern "system" fn SetParameters<Impl: IConstantForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IConstantForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetParametersWithEnvelope(
                    &*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    attackgain,
                    sustaingain,
                    releasegain,
                    &*(&startdelay as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&attackduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&sustainduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&releaseduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    repeatcount,
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConstantForceEffect>, ::windows::core::GetTrustLevel, SetParameters::<Impl, IMPL_OFFSET>, SetParametersWithEnvelope::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstantForceEffect as ::windows::core::Interface>::IID
    }
}
pub trait IForceFeedbackEffectImpl: Sized {
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IForceFeedbackEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect";
}
impl IForceFeedbackEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForceFeedbackEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForceFeedbackEffectVtbl {
        unsafe extern "system" fn Gain<Impl: IForceFeedbackEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IForceFeedbackEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGain(value).into()
        }
        unsafe extern "system" fn State<Impl: IForceFeedbackEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IForceFeedbackEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IForceFeedbackEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IForceFeedbackEffect>, ::windows::core::GetTrustLevel, Gain::<Impl, IMPL_OFFSET>, SetGain::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForceFeedbackEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IForceFeedbackMotorImpl: Sized {
    fn AreEffectsPaused(&self) -> ::windows::core::Result<bool>;
    fn MasterGain(&self) -> ::windows::core::Result<f64>;
    fn SetMasterGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SupportedAxes(&self) -> ::windows::core::Result<ForceFeedbackEffectAxes>;
    fn LoadEffectAsync(&self, effect: &::core::option::Option<IForceFeedbackEffect>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>;
    fn PauseAllEffects(&self) -> ::windows::core::Result<()>;
    fn ResumeAllEffects(&self) -> ::windows::core::Result<()>;
    fn StopAllEffects(&self) -> ::windows::core::Result<()>;
    fn TryDisableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryEnableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryResetAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUnloadEffectAsync(&self, effect: &::core::option::Option<IForceFeedbackEffect>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IForceFeedbackMotorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForceFeedbackMotorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForceFeedbackMotorVtbl {
        unsafe extern "system" fn AreEffectsPaused<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreEffectsPaused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterGain<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MasterGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterGain<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterGain(value).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedAxes<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedAxes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadEffectAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadEffectAsync(&*(&effect as *const <IForceFeedbackEffect as ::windows::core::Abi>::Abi as *const <IForceFeedbackEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseAllEffects().into()
        }
        unsafe extern "system" fn ResumeAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeAllEffects().into()
        }
        unsafe extern "system" fn StopAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAllEffects().into()
        }
        unsafe extern "system" fn TryDisableAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnableAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryResetAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryResetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUnloadEffectAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUnloadEffectAsync(&*(&effect as *const <IForceFeedbackEffect as ::windows::core::Abi>::Abi as *const <IForceFeedbackEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IForceFeedbackMotor>,
            ::windows::core::GetTrustLevel,
            AreEffectsPaused::<Impl, IMPL_OFFSET>,
            MasterGain::<Impl, IMPL_OFFSET>,
            SetMasterGain::<Impl, IMPL_OFFSET>,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SupportedAxes::<Impl, IMPL_OFFSET>,
            LoadEffectAsync::<Impl, IMPL_OFFSET>,
            PauseAllEffects::<Impl, IMPL_OFFSET>,
            ResumeAllEffects::<Impl, IMPL_OFFSET>,
            StopAllEffects::<Impl, IMPL_OFFSET>,
            TryDisableAsync::<Impl, IMPL_OFFSET>,
            TryEnableAsync::<Impl, IMPL_OFFSET>,
            TryResetAsync::<Impl, IMPL_OFFSET>,
            TryUnloadEffectAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForceFeedbackMotor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IPeriodicForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind>;
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IPeriodicForceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeriodicForceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeriodicForceEffectVtbl {
        unsafe extern "system" fn Kind<Impl: IPeriodicForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PeriodicForceEffectKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IPeriodicForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), frequency, phase, bias, &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IPeriodicForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetParametersWithEnvelope(
                    &*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    frequency,
                    phase,
                    bias,
                    attackgain,
                    sustaingain,
                    releasegain,
                    &*(&startdelay as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&attackduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&sustainduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&releaseduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    repeatcount,
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPeriodicForceEffect>, ::windows::core::GetTrustLevel, Kind::<Impl, IMPL_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>, SetParametersWithEnvelope::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeriodicForceEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeriodicForceEffectFactoryImpl: Sized {
    fn CreateInstance(&self, effectkind: PeriodicForceEffectKind) -> ::windows::core::Result<PeriodicForceEffect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPeriodicForceEffectFactory {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPeriodicForceEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeriodicForceEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeriodicForceEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPeriodicForceEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effectkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPeriodicForceEffectFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeriodicForceEffectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IRampForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IRampForceEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IRampForceEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRampForceEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRampForceEffectVtbl {
        unsafe extern "system" fn SetParameters<Impl: IRampForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetParameters(
                    &*(&startvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&endvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IRampForceEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetParametersWithEnvelope(
                    &*(&startvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&endvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    attackgain,
                    sustaingain,
                    releasegain,
                    &*(&startdelay as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&attackduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&sustainduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    &*(&releaseduration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                    repeatcount,
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRampForceEffect>, ::windows::core::GetTrustLevel, SetParameters::<Impl, IMPL_OFFSET>, SetParametersWithEnvelope::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRampForceEffect as ::windows::core::Interface>::IID
    }
}

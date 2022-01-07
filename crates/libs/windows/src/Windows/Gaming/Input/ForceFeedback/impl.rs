#[cfg(feature = "implement_exclusive")]
pub trait IConditionForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind>;
    fn SetParameters(&self, direction: &super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IConditionForceEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IConditionForceEffectVtbl {
    pub const fn new<Impl: IConditionForceEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionForceEffectVtbl {
        unsafe extern "system" fn Kind<Impl: IConditionForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ConditionForceEffectKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IConditionForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&direction as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConditionForceEffect>, base.5, Kind::<Impl, OFFSET>, SetParameters::<Impl, OFFSET>)
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
    pub const fn new<Impl: IConditionForceEffectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConditionForceEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IConditionForceEffectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effectkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConditionForceEffectFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConstantForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IConstantForceEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IConstantForceEffectVtbl {
    pub const fn new<Impl: IConstantForceEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConstantForceEffectVtbl {
        unsafe extern "system" fn SetParameters<Impl: IConstantForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IConstantForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConstantForceEffect>, base.5, SetParameters::<Impl, OFFSET>, SetParametersWithEnvelope::<Impl, OFFSET>)
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
    pub const fn new<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IForceFeedbackEffectVtbl {
        unsafe extern "system" fn Gain<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGain(value).into()
        }
        unsafe extern "system" fn State<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IForceFeedbackEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IForceFeedbackEffect>, base.5, Gain::<Impl, OFFSET>, SetGain::<Impl, OFFSET>, State::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor";
}
#[cfg(feature = "implement_exclusive")]
impl IForceFeedbackMotorVtbl {
    pub const fn new<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IForceFeedbackMotorVtbl {
        unsafe extern "system" fn AreEffectsPaused<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreEffectsPaused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterGain<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MasterGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterGain<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMasterGain(value).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedAxes<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedAxes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadEffectAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadEffectAsync(&*(&effect as *const <IForceFeedbackEffect as ::windows::core::Abi>::Abi as *const <IForceFeedbackEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PauseAllEffects().into()
        }
        unsafe extern "system" fn ResumeAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResumeAllEffects().into()
        }
        unsafe extern "system" fn StopAllEffects<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopAllEffects().into()
        }
        unsafe extern "system" fn TryDisableAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryDisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnableAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryEnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryResetAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryResetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUnloadEffectAsync<Impl: IForceFeedbackMotorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IForceFeedbackMotor>,
            base.5,
            AreEffectsPaused::<Impl, OFFSET>,
            MasterGain::<Impl, OFFSET>,
            SetMasterGain::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SupportedAxes::<Impl, OFFSET>,
            LoadEffectAsync::<Impl, OFFSET>,
            PauseAllEffects::<Impl, OFFSET>,
            ResumeAllEffects::<Impl, OFFSET>,
            StopAllEffects::<Impl, OFFSET>,
            TryDisableAsync::<Impl, OFFSET>,
            TryEnableAsync::<Impl, OFFSET>,
            TryResetAsync::<Impl, OFFSET>,
            TryUnloadEffectAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeriodicForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind>;
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IPeriodicForceEffectVtbl {
    pub const fn new<Impl: IPeriodicForceEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPeriodicForceEffectVtbl {
        unsafe extern "system" fn Kind<Impl: IPeriodicForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PeriodicForceEffectKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IPeriodicForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&vector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), frequency, phase, bias, &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IPeriodicForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPeriodicForceEffect>, base.5, Kind::<Impl, OFFSET>, SetParameters::<Impl, OFFSET>, SetParametersWithEnvelope::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPeriodicForceEffectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPeriodicForceEffectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPeriodicForceEffectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(effectkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPeriodicForceEffectFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRampForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IRampForceEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IRampForceEffectVtbl {
    pub const fn new<Impl: IRampForceEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRampForceEffectVtbl {
        unsafe extern "system" fn SetParameters<Impl: IRampForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetParameters(
                    &*(&startvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&endvector as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetParametersWithEnvelope<Impl: IRampForceEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRampForceEffect>, base.5, SetParameters::<Impl, OFFSET>, SetParametersWithEnvelope::<Impl, OFFSET>)
    }
}

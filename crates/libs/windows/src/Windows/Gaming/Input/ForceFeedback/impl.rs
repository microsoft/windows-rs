#[cfg(feature = "implement_exclusive")]
pub trait IConditionForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind>;
    fn SetParameters(&self, direction: &super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConditionForceEffectFactoryImpl: Sized {
    fn CreateInstance(&self, effectkind: ConditionForceEffectKind) -> ::windows::core::Result<ConditionForceEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConstantForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
pub trait IForceFeedbackEffectImpl: Sized {
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
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
pub trait IPeriodicForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind>;
    fn SetParameters(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, vector: &super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeriodicForceEffectFactoryImpl: Sized {
    fn CreateInstance(&self, effectkind: PeriodicForceEffectKind) -> ::windows::core::Result<PeriodicForceEffect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRampForceEffectImpl: Sized + IForceFeedbackEffectImpl {
    fn SetParameters(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetParametersWithEnvelope(&self, startvector: &super::super::super::Foundation::Numerics::Vector3, endvector: &super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: &super::super::super::Foundation::TimeSpan, attackduration: &super::super::super::Foundation::TimeSpan, sustainduration: &super::super::super::Foundation::TimeSpan, releaseduration: &super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()>;
}

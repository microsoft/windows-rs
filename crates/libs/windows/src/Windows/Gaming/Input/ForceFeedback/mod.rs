#[doc(hidden)]
#[repr(transparent)]
pub struct IConditionForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_Vtbl;
}
impl ::core::clone::Clone for IConditionForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConditionForceEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConditionForceEffectKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConditionForceEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConditionForceEffectFactory {
    type Vtable = IConditionForceEffectFactory_Vtbl;
}
impl ::core::clone::Clone for IConditionForceEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConditionForceEffectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConstantForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_Vtbl;
}
impl ::core::clone::Clone for IConstantForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConstantForceEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct IForceFeedbackEffect(::windows::core::IUnknown);
impl IForceFeedbackEffect {
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IForceFeedbackEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IForceFeedbackEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForceFeedbackEffect {}
impl ::core::fmt::Debug for IForceFeedbackEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForceFeedbackEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
}
unsafe impl ::windows::core::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
impl ::core::clone::Clone for IForceFeedbackEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IForceFeedbackEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IForceFeedbackMotor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
}
impl ::core::clone::Clone for IForceFeedbackMotor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IForceFeedbackMotor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AreEffectsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadEffectAsync: usize,
    pub PauseAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnloadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnloadEffectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeriodicForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_Vtbl;
}
impl ::core::clone::Clone for IPeriodicForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPeriodicForceEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeriodicForceEffectKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeriodicForceEffectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeriodicForceEffectFactory {
    type Vtable = IPeriodicForceEffectFactory_Vtbl;
}
impl ::core::clone::Clone for IPeriodicForceEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPeriodicForceEffectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRampForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_Vtbl;
}
impl ::core::clone::Clone for IRampForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRampForceEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ConditionForceEffect(::windows::core::IUnknown);
impl ConditionForceEffect {
    pub fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind> {
        let this = &::windows::core::ComInterface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ConditionForceEffectKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IConditionForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::windows::core::Interface::as_raw(this), direction, positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows::core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ConditionForceEffect>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), effectkind, &mut result__).from_abi(result__)
        })
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ConditionForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConditionForceEffect {}
impl ::core::fmt::Debug for ConditionForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
impl ::core::clone::Clone for ConditionForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ConditionForceEffect {
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
::windows::imp::interface_hierarchy!(ConditionForceEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IForceFeedbackEffect> for ConditionForceEffect {}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ConstantForceEffect(::windows::core::IUnknown);
impl ConstantForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ConstantForceEffect, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::windows::core::Interface::as_raw(this), vector, duration).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::windows::core::Interface::as_raw(this), vector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for ConstantForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConstantForceEffect {}
impl ::core::fmt::Debug for ConstantForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConstantForceEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
impl ::core::clone::Clone for ConstantForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ConstantForceEffect {
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
::windows::imp::interface_hierarchy!(ConstantForceEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IForceFeedbackEffect> for ConstantForceEffect {}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ForceFeedbackMotor(::windows::core::IUnknown);
impl ForceFeedbackMotor {
    pub fn AreEffectsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AreEffectsPaused)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MasterGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MasterGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMasterGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMasterGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportedAxes(&self) -> ::windows::core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectAxes>();
            (::windows::core::Interface::vtable(this).SupportedAxes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadEffectAsync<P0>(&self, effect: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>
    where
        P0: ::windows::core::TryIntoParam<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>();
            (::windows::core::Interface::vtable(this).LoadEffectAsync)(::windows::core::Interface::as_raw(this), effect.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn PauseAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PauseAllEffects)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResumeAllEffects)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn StopAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopAllEffects)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryDisableAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryEnableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryEnableAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryResetAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryResetAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnloadEffectAsync<P0>(&self, effect: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows::core::TryIntoParam<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryUnloadEffectAsync)(::windows::core::Interface::as_raw(this), effect.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ForceFeedbackMotor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ForceFeedbackMotor {}
impl ::core::fmt::Debug for ForceFeedbackMotor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackMotor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
}
impl ::core::clone::Clone for ForceFeedbackMotor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ForceFeedbackMotor {
    const IID: ::windows::core::GUID = <IForceFeedbackMotor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
::windows::imp::interface_hierarchy!(ForceFeedbackMotor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct PeriodicForceEffect(::windows::core::IUnknown);
impl PeriodicForceEffect {
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind> {
        let this = &::windows::core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PeriodicForceEffectKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::windows::core::Interface::as_raw(this), vector, frequency, phase, bias, duration).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::windows::core::Interface::as_raw(this), vector, frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows::core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PeriodicForceEffect>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), effectkind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PeriodicForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeriodicForceEffect {}
impl ::core::fmt::Debug for PeriodicForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
impl ::core::clone::Clone for PeriodicForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PeriodicForceEffect {
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
::windows::imp::interface_hierarchy!(PeriodicForceEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IForceFeedbackEffect> for PeriodicForceEffect {}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct RampForceEffect(::windows::core::IUnknown);
impl RampForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RampForceEffect, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ForceFeedbackEffectState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::windows::core::Interface::as_raw(this), startvector, endvector, duration).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::windows::core::Interface::as_raw(this), startvector, endvector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
}
impl ::core::cmp::PartialEq for RampForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RampForceEffect {}
impl ::core::fmt::Debug for RampForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RampForceEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
impl ::core::clone::Clone for RampForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RampForceEffect {
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
::windows::imp::interface_hierarchy!(RampForceEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IForceFeedbackEffect> for RampForceEffect {}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl ::core::marker::Copy for ConditionForceEffectKind {}
impl ::core::clone::Clone for ConditionForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConditionForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ConditionForceEffectKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ConditionForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffectKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for ForceFeedbackEffectAxes {}
impl ::core::clone::Clone for ForceFeedbackEffectAxes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectAxes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ForceFeedbackEffectAxes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackEffectAxes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectAxes").field(&self.0).finish()
    }
}
impl ForceFeedbackEffectAxes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl ::core::marker::Copy for ForceFeedbackEffectState {}
impl ::core::clone::Clone for ForceFeedbackEffectState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ForceFeedbackEffectState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackEffectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for ForceFeedbackLoadEffectResult {}
impl ::core::clone::Clone for ForceFeedbackLoadEffectResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackLoadEffectResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ForceFeedbackLoadEffectResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackLoadEffectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackLoadEffectResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicForceEffectKind {}
impl ::core::clone::Clone for PeriodicForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeriodicForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PeriodicForceEffectKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PeriodicForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffectKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

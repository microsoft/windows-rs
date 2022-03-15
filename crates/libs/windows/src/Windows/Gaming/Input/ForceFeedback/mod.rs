#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ConditionForceEffect(::windows::core::IUnknown);
impl ConditionForceEffect {
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind> {
        let this = &::windows::core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__: ConditionForceEffectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConditionForceEffectKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, direction: Param0, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::core::mem::transmute_copy(this), direction.into_param().abi(), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows::core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<ConditionForceEffect>(result__)
        })
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConditionForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::core::IUnknown {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::core::IUnknown {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::core::IInspectable {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::core::IInspectable {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConditionForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: ConditionForceEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConditionForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConditionForceEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ConditionForceEffectKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConditionForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ConstantForceEffect(::windows::core::IUnknown);
impl ConstantForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConstantForceEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, duration: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::core::mem::transmute_copy(this), vector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: Param4, attackduration: Param5, sustainduration: Param6, releaseduration: Param7, repeatcount: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::core::mem::transmute_copy(this), vector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ConstantForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::core::IUnknown {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::core::IUnknown {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::core::IInspectable {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::core::IInspectable {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConstantForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: ConstantForceEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConstantForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConstantForceEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ForceFeedbackEffectAxes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackEffectAxes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectAxes").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ForceFeedbackEffectState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackEffectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ForceFeedbackLoadEffectResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackLoadEffectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackLoadEffectResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct ForceFeedbackMotor(::windows::core::IUnknown);
impl ForceFeedbackMotor {
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn AreEffectsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreEffectsPaused)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn MasterGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MasterGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetMasterGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMasterGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SupportedAxes(&self) -> ::windows::core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectAxes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedAxes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectAxes>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadEffectAsync)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn PauseAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PauseAllEffects)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn ResumeAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResumeAllEffects)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn StopAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopAllEffects)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryEnableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryEnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryResetAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryResetAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnloadEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUnloadEffectAsync)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for ForceFeedbackMotor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
    const IID: ::windows::core::GUID = <IForceFeedbackMotor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::core::IUnknown {
    fn from(value: ForceFeedbackMotor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::core::IUnknown {
    fn from(value: &ForceFeedbackMotor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::core::IInspectable {
    fn from(value: ForceFeedbackMotor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::core::IInspectable {
    fn from(value: &ForceFeedbackMotor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConditionForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConstantForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::core::IUnknown {
    fn from(value: IForceFeedbackEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::core::IUnknown {
    fn from(value: &IForceFeedbackEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::core::IInspectable {
    fn from(value: IForceFeedbackEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::core::IInspectable {
    fn from(value: &IForceFeedbackEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForceFeedbackEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AreEffectsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadEffectAsync: usize,
    pub PauseAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnloadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnloadEffectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeriodicForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRampForceEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
pub struct PeriodicForceEffect(::windows::core::IUnknown);
impl PeriodicForceEffect {
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__: PeriodicForceEffectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeriodicForceEffectKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, duration: Param4) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param9: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param10: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: Param7, attackduration: Param8, sustainduration: Param9, releaseduration: Param10, repeatcount: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows::core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<PeriodicForceEffect>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PeriodicForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::core::IUnknown {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::core::IUnknown {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::core::IInspectable {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::core::IInspectable {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PeriodicForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: PeriodicForceEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PeriodicForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &PeriodicForceEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PeriodicForceEffectKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeriodicForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
#[repr(transparent)]
pub struct RampForceEffect(::windows::core::IUnknown);
impl RampForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RampForceEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startvector: Param0, endvector: Param1, duration: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParameters)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        &self,
        startvector: Param0,
        endvector: Param1,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param5,
        attackduration: Param6,
        sustainduration: Param7,
        releaseduration: Param8,
        repeatcount: u32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParametersWithEnvelope)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
impl ::core::clone::Clone for RampForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows::core::GUID = <IForceFeedbackEffect as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
impl ::core::convert::From<RampForceEffect> for ::windows::core::IUnknown {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::core::IUnknown {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RampForceEffect> for ::windows::core::IInspectable {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::core::IInspectable {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RampForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: RampForceEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RampForceEffect> for IForceFeedbackEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &RampForceEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

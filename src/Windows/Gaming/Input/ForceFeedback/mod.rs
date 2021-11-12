#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConditionForceEffect(pub ::windows::core::IInspectable);
impl ConditionForceEffect {
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ConditionForceEffectKind> {
        let this = &::windows::core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__: ConditionForceEffectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConditionForceEffectKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, direction: Param0, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), direction.into_param().abi(), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows::core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<ConditionForceEffect>(result__)
        })
    }
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::core::IUnknown {
    fn from(value: ConditionForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::core::IUnknown {
    fn from(value: &ConditionForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConditionForceEffect> for ::windows::core::IInspectable {
    fn from(value: ConditionForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows::core::IInspectable {
    fn from(value: &ConditionForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ConditionForceEffect> for IForceFeedbackEffect {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for IForceFeedbackEffect {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &ConditionForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: ConditionForceEffectKind = ConditionForceEffectKind(0i32);
    pub const Damper: ConditionForceEffectKind = ConditionForceEffectKind(1i32);
    pub const Inertia: ConditionForceEffectKind = ConditionForceEffectKind(2i32);
    pub const Friction: ConditionForceEffectKind = ConditionForceEffectKind(3i32);
}
impl ::core::convert::From<i32> for ConditionForceEffectKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConditionForceEffectKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
}
impl ::windows::core::DefaultType for ConditionForceEffectKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConstantForceEffect(pub ::windows::core::IInspectable);
impl ConstantForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConstantForceEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, duration: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        &self,
        vector: Param0,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param4,
        attackduration: Param5,
        sustainduration: Param6,
        releaseduration: Param7,
        repeatcount: u32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::core::IUnknown {
    fn from(value: ConstantForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::core::IUnknown {
    fn from(value: &ConstantForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConstantForceEffect> for ::windows::core::IInspectable {
    fn from(value: ConstantForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows::core::IInspectable {
    fn from(value: &ConstantForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ConstantForceEffect> for IForceFeedbackEffect {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for IForceFeedbackEffect {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &ConstantForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(0u32);
    pub const X: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(1u32);
    pub const Y: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(2u32);
    pub const Z: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(4u32);
}
impl ::core::convert::From<u32> for ForceFeedbackEffectAxes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ForceFeedbackEffectAxes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
}
impl ::windows::core::DefaultType for ForceFeedbackEffectAxes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: ForceFeedbackEffectState = ForceFeedbackEffectState(0i32);
    pub const Running: ForceFeedbackEffectState = ForceFeedbackEffectState(1i32);
    pub const Paused: ForceFeedbackEffectState = ForceFeedbackEffectState(2i32);
    pub const Faulted: ForceFeedbackEffectState = ForceFeedbackEffectState(3i32);
}
impl ::core::convert::From<i32> for ForceFeedbackEffectState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ForceFeedbackEffectState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
}
impl ::windows::core::DefaultType for ForceFeedbackEffectState {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(0i32);
    pub const EffectStorageFull: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(1i32);
    pub const EffectNotSupported: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(2i32);
}
impl ::core::convert::From<i32> for ForceFeedbackLoadEffectResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ForceFeedbackLoadEffectResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
}
impl ::windows::core::DefaultType for ForceFeedbackLoadEffectResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ForceFeedbackMotor(pub ::windows::core::IInspectable);
impl ForceFeedbackMotor {
    pub fn AreEffectsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MasterGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMasterGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedAxes(&self) -> ::windows::core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectAxes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectAxes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>(result__)
        }
    }
    pub fn PauseAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ResumeAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn StopAllEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryDisableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryEnableAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryResetAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryUnloadEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
}
unsafe impl ::windows::core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
impl ::windows::core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::core::IUnknown {
    fn from(value: ForceFeedbackMotor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::core::IUnknown {
    fn from(value: &ForceFeedbackMotor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows::core::IInspectable {
    fn from(value: ForceFeedbackMotor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows::core::IInspectable {
    fn from(value: &ForceFeedbackMotor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IConditionForceEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ConditionForceEffectKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConditionForceEffectFactory {
    type Vtable = IConditionForceEffectFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectkind: ConditionForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConstantForceEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IForceFeedbackEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl IForceFeedbackEffect {
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::core::IUnknown {
    fn from(value: IForceFeedbackEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::core::IUnknown {
    fn from(value: &IForceFeedbackEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows::core::IInspectable {
    fn from(value: IForceFeedbackEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows::core::IInspectable {
    fn from(value: &IForceFeedbackEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ForceFeedbackEffectState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IForceFeedbackMotor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ForceFeedbackEffectAxes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeriodicForceEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PeriodicForceEffectKind) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeriodicForceEffectFactory {
    type Vtable = IPeriodicForceEffectFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectkind: PeriodicForceEffectKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRampForceEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        startvector: super::super::super::Foundation::Numerics::Vector3,
        endvector: super::super::super::Foundation::Numerics::Vector3,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: super::super::super::Foundation::TimeSpan,
        attackduration: super::super::super::Foundation::TimeSpan,
        sustainduration: super::super::super::Foundation::TimeSpan,
        releaseduration: super::super::super::Foundation::TimeSpan,
        repeatcount: u32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeriodicForceEffect(pub ::windows::core::IInspectable);
impl PeriodicForceEffect {
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<PeriodicForceEffectKind> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__: PeriodicForceEffectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeriodicForceEffectKind>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, duration: Param4) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param9: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param10: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(
        &self,
        vector: Param0,
        frequency: f32,
        phase: f32,
        bias: f32,
        attackgain: f32,
        sustaingain: f32,
        releasegain: f32,
        startdelay: Param7,
        attackduration: Param8,
        sustainduration: Param9,
        releaseduration: Param10,
        repeatcount: u32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), vector.into_param().abi(), frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows::core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effectkind, &mut result__).from_abi::<PeriodicForceEffect>(result__)
        })
    }
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::core::IUnknown {
    fn from(value: PeriodicForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::core::IUnknown {
    fn from(value: &PeriodicForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows::core::IInspectable {
    fn from(value: PeriodicForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows::core::IInspectable {
    fn from(value: &PeriodicForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PeriodicForceEffect> for IForceFeedbackEffect {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for IForceFeedbackEffect {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &PeriodicForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: PeriodicForceEffectKind = PeriodicForceEffectKind(0i32);
    pub const SineWave: PeriodicForceEffectKind = PeriodicForceEffectKind(1i32);
    pub const TriangleWave: PeriodicForceEffectKind = PeriodicForceEffectKind(2i32);
    pub const SawtoothWaveUp: PeriodicForceEffectKind = PeriodicForceEffectKind(3i32);
    pub const SawtoothWaveDown: PeriodicForceEffectKind = PeriodicForceEffectKind(4i32);
}
impl ::core::convert::From<i32> for PeriodicForceEffectKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PeriodicForceEffectKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
}
impl ::windows::core::DefaultType for PeriodicForceEffectKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RampForceEffect(pub ::windows::core::IInspectable);
impl RampForceEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RampForceEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__: ForceFeedbackEffectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startvector: Param0, endvector: Param1, duration: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetParametersWithEnvelope<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
        Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
        Param8: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>,
    >(
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
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), startvector.into_param().abi(), endvector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows::core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
impl ::windows::core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
impl ::core::convert::From<RampForceEffect> for ::windows::core::IUnknown {
    fn from(value: RampForceEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::core::IUnknown {
    fn from(value: &RampForceEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RampForceEffect> for ::windows::core::IInspectable {
    fn from(value: RampForceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows::core::IInspectable {
    fn from(value: &RampForceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RampForceEffect> for IForceFeedbackEffect {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for IForceFeedbackEffect {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IForceFeedbackEffect> for &RampForceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IForceFeedbackEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}

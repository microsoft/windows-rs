#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
}
impl ::core::clone::Clone for ICompositionConditionalValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionConditionalValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_Vtbl;
}
impl ::core::clone::Clone for ICompositionConditionalValueStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionConditionalValueStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x090c4b72_8467_4d0a_9065_ac46b80a5522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct ICompositionInteractionSource(::windows_core::IUnknown);
impl ICompositionInteractionSource {}
::windows_core::imp::interface_hierarchy!(ICompositionInteractionSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for ICompositionInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionInteractionSource {}
impl ::core::fmt::Debug for ICompositionInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionInteractionSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
}
unsafe impl ::windows_core::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_Vtbl;
}
impl ::core::clone::Clone for ICompositionInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionInteractionSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x043b2431_06e3_495a_ba54_409f0017fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionInteractionSourceCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
}
impl ::core::clone::Clone for ICompositionInteractionSourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionInteractionSourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionSourceConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
}
impl ::core::clone::Clone for IInteractionSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionSourceConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
}
impl ::core::clone::Clone for IInteractionTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InteractionSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MaxPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMaxPosition: usize,
    pub MaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MinPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMinPosition: usize,
    pub MinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScaleInertiaDecayRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScaleInertiaDecayRate: usize,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows_core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionYInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureScaleInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionBy: usize,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithAdditionalVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAnimation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAdditionalVelocity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTracker2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTracker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25769a3e_ce6d_448c_8386_92620d240756);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_Vtbl;
}
impl ::core::clone::Clone for IInteractionTracker3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTracker3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_Vtbl;
}
impl ::core::clone::Clone for IInteractionTracker4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTracker4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebd222bc_04af_4ac7_847d_06ea36e80a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionByWithOption: usize,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_Vtbl;
}
impl ::core::clone::Clone for IInteractionTracker5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTracker5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerCustomAnimationStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47d579b7_0985_5e99_b024_2f32c380c1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerIdleStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerIdleStateEnteredArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerIdleStateEnteredArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerIdleStateEnteredArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2e771ed_b803_5137_9435_1c96e48721e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaModifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaModifierFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaModifierFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x993818fe_c94e_4b86_87f3_922665ba46b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaMotion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaMotionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaMotionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaNaturalMotion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaNaturalMotionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaNaturalMotionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaRestingValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaRestingValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaRestingValueStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaRestingValueStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18ed4699_0745_4096_bcab_3a4e99569bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaStateEnteredArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub ModifiedRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ModifiedRestingPosition: usize,
    #[cfg(feature = "Foundation")]
    pub ModifiedRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModifiedRestingScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaStateEnteredArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaStateEnteredArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInertiaStateEnteredArgs3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInertiaStateEnteredArgs3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInteractingStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInteractingStateEnteredArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerInteractingStateEnteredArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerInteractingStateEnteredArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x509652d6_d488_59cd_819f_f52310295b11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct IInteractionTrackerOwner(::windows_core::IUnknown);
impl IInteractionTrackerOwner {
    pub fn CustomAnimationStateEntered<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerCustomAnimationStateEnteredArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CustomAnimationStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn IdleStateEntered<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerIdleStateEnteredArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).IdleStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn InertiaStateEntered<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerInertiaStateEnteredArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InertiaStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn InteractingStateEntered<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerInteractingStateEnteredArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InteractingStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn RequestIgnored<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerRequestIgnoredArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestIgnored)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn ValuesChanged<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTrackerValuesChangedArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ValuesChanged)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IInteractionTrackerOwner, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IInteractionTrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInteractionTrackerOwner {}
impl ::core::fmt::Debug for IInteractionTrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInteractionTrackerOwner").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
}
unsafe impl ::windows_core::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerOwner {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerRequestIgnoredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerRequestIgnoredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerRequestIgnoredArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35e53720_46b7_5cb0_b505_f3d6884a6163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, axismode: InteractionBindingAxisModes) -> ::windows_core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, result__: *mut InteractionBindingAxisModes) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerValuesChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerValuesChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerValuesChangedArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerVector2InertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerVector2InertiaModifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerVector2InertiaModifierFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerVector2InertiaModifierFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerVector2InertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerVector2InertiaNaturalMotion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl;
}
impl ::core::clone::Clone for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82001a48_09c0_434f_8189_141c66df362f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    TryRedirectForManipulation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSource2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa914893_a73c_414d_80d0_249bad2fbd93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub DeltaPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DeltaPosition: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocity: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaScaleModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSource3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd941ef2a_0d5c_4057_92d7_c9711533204f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSourceObjectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSourceObjectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSourceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x369965e1_8645_4f75_ba00_6479cd10c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_Vtbl;
}
impl ::core::clone::Clone for IVisualInteractionSourceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualInteractionSourceStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa979c032_5764_55e0_bc1f_0778786dcfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionConditionalValue(::windows_core::IUnknown);
impl CompositionConditionalValue {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<CompositionConditionalValue>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CompositionConditionalValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionConditionalValue {}
impl ::core::fmt::Debug for CompositionConditionalValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionConditionalValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})");
}
impl ::core::clone::Clone for CompositionConditionalValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositionConditionalValue {
    const IID: ::windows_core::GUID = <ICompositionConditionalValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
::windows_core::imp::interface_hierarchy!(CompositionConditionalValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for CompositionConditionalValue {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for CompositionConditionalValue {}
impl ::windows_core::CanTryInto<super::CompositionObject> for CompositionConditionalValue {}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionInteractionSourceCollection(::windows_core::IUnknown);
impl CompositionInteractionSourceCollection {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Count(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Count)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICompositionInteractionSource>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Remove<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICompositionInteractionSource>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn RemoveAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAll)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CompositionInteractionSourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionInteractionSourceCollection {}
impl ::core::fmt::Debug for CompositionInteractionSourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionInteractionSourceCollection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})");
}
impl ::core::clone::Clone for CompositionInteractionSourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositionInteractionSourceCollection {
    const IID: ::windows_core::GUID = <ICompositionInteractionSourceCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(CompositionInteractionSourceCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for CompositionInteractionSourceCollection {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for CompositionInteractionSourceCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> for CompositionInteractionSourceCollection {}
impl ::windows_core::CanTryInto<super::CompositionObject> for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionSourceConfiguration(::windows_core::IUnknown);
impl InteractionSourceConfiguration {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn PositionXSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionXSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionYSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for InteractionSourceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionSourceConfiguration {}
impl ::core::fmt::Debug for InteractionSourceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})");
}
impl ::core::clone::Clone for InteractionSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionSourceConfiguration {
    const IID: ::windows_core::GUID = <IInteractionSourceConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
::windows_core::imp::interface_hierarchy!(InteractionSourceConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionSourceConfiguration {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionSourceConfiguration {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTracker(::windows_core::IUnknown);
impl InteractionTracker {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn InteractionSources(&self) -> ::windows_core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSources)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionRoundingSuggested)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MaxPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMaxPosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MinPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMinPosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Owner(&self) -> ::windows_core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Owner)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionInertiaDecayRate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionInertiaDecayRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPositionInertiaDecayRate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionInertiaDecayRate)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScaleInertiaDecayRate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleInertiaDecayRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScaleInertiaDecayRate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::IReference<f32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleInertiaDecayRate)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustPositionXIfGreaterThanThreshold)(::windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustPositionYIfGreaterThanThreshold)(::windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionXInertiaModifiers<P0>(&self, modifiers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigurePositionXInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionYInertiaModifiers<P0>(&self, modifiers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigurePositionYInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureScaleInertiaModifiers<P0>(&self, modifiers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureScaleInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePosition)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionBy(&self, amount: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionBy)(::windows_core::Interface::as_raw(this), amount, &mut result__).from_abi(result__)
        }
    }
    pub fn TryUpdatePositionWithAnimation<P0>(&self, animation: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithAnimation)(::windows_core::Interface::as_raw(this), animation.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithAdditionalVelocity(&self, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithAdditionalVelocity)(::windows_core::Interface::as_raw(this), velocityinpixelspersecond, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScale(&self, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScale)(::windows_core::Interface::as_raw(this), value, centerpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAnimation<P0>(&self, animation: P0, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScaleWithAnimation)(::windows_core::Interface::as_raw(this), animation.try_into_param()?.abi(), centerpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAdditionalVelocity(&self, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScaleWithAdditionalVelocity)(::windows_core::Interface::as_raw(this), velocityinpercentpersecond, centerpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXInertiaModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointXInertiaModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYInertiaModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointYInertiaModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureVector2PositionInertiaModifiers<P0>(&self, modifiers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>,
    {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureVector2PositionInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption(&self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(::windows_core::Interface::as_raw(this), value, option, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionByWithOption(&self, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionByWithOption)(::windows_core::Interface::as_raw(this), amount, option, &mut result__).from_abi(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInertiaFromImpulse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption2(&self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(::windows_core::Interface::as_raw(this), value, option, posupdateoption, &mut result__).from_abi(result__)
        }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<InteractionTracker>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithOwner<P0, P1>(compositor: P0, owner: P1) -> ::windows_core::Result<InteractionTracker>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
        P1: ::windows_core::TryIntoParam<IInteractionTrackerOwner>,
    {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithOwner)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), owner.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn SetBindingMode<P0, P1>(boundtracker1: P0, boundtracker2: P1, axismode: InteractionBindingAxisModes) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTracker>,
    {
        Self::IInteractionTrackerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetBindingMode)(::windows_core::Interface::as_raw(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), axismode).ok() })
    }
    pub fn GetBindingMode<P0, P1>(boundtracker1: P0, boundtracker2: P1) -> ::windows_core::Result<InteractionBindingAxisModes>
    where
        P0: ::windows_core::IntoParam<InteractionTracker>,
        P1: ::windows_core::IntoParam<InteractionTracker>,
    {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBindingMode)(::windows_core::Interface::as_raw(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for InteractionTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTracker {}
impl ::core::fmt::Debug for InteractionTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTracker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTracker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})");
}
impl ::core::clone::Clone for InteractionTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTracker {
    const IID: ::windows_core::GUID = <IInteractionTracker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
::windows_core::imp::interface_hierarchy!(InteractionTracker, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTracker {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTracker {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTracker {}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerCustomAnimationStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerCustomAnimationStateEnteredArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})");
}
impl ::core::clone::Clone for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerCustomAnimationStateEnteredArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerCustomAnimationStateEnteredArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerCustomAnimationStateEnteredArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerIdleStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerIdleStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerIdleStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerIdleStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerIdleStateEnteredArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})");
}
impl ::core::clone::Clone for InteractionTrackerIdleStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerIdleStateEnteredArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerIdleStateEnteredArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerIdleStateEnteredArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaModifier(::windows_core::IUnknown);
impl InteractionTrackerInertiaModifier {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerInertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaModifier").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})");
}
impl ::core::clone::Clone for InteractionTrackerInertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInertiaModifier {
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaModifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaModifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerInertiaModifier {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerInertiaModifier {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaMotion(::windows_core::IUnknown);
impl InteractionTrackerInertiaMotion {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Motion(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Motion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMotion<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMotion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<InteractionTrackerInertiaMotion>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaMotion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})");
}
impl ::core::clone::Clone for InteractionTrackerInertiaMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInertiaMotion {
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaMotion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaMotion, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerInertiaMotion {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerInertiaMotion {}
impl ::windows_core::CanTryInto<InteractionTrackerInertiaModifier> for InteractionTrackerInertiaMotion {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaNaturalMotion(::windows_core::IUnknown);
impl InteractionTrackerInertiaNaturalMotion {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows_core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalMotion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ScalarNaturalMotionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNaturalMotion)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<InteractionTrackerInertiaNaturalMotion>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaNaturalMotion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})");
}
impl ::core::clone::Clone for InteractionTrackerInertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInertiaNaturalMotion {
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaNaturalMotion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaNaturalMotion, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerInertiaNaturalMotion {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerInertiaNaturalMotion {}
impl ::windows_core::CanTryInto<InteractionTrackerInertiaModifier> for InteractionTrackerInertiaNaturalMotion {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaRestingValue(::windows_core::IUnknown);
impl InteractionTrackerInertiaRestingValue {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RestingValue(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RestingValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRestingValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRestingValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<InteractionTrackerInertiaRestingValue>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaRestingValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaRestingValue {}
impl ::core::fmt::Debug for InteractionTrackerInertiaRestingValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaRestingValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})");
}
impl ::core::clone::Clone for InteractionTrackerInertiaRestingValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInertiaRestingValue {
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaRestingValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaRestingValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerInertiaRestingValue {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerInertiaRestingValue {}
impl ::windows_core::CanTryInto<InteractionTrackerInertiaModifier> for InteractionTrackerInertiaRestingValue {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ModifiedRestingPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModifiedRestingPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ModifiedRestingScale(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModifiedRestingScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInertiaFromImpulse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInertiaStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaStateEnteredArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})");
}
impl ::core::clone::Clone for InteractionTrackerInertiaStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInertiaStateEnteredArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaStateEnteredArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaStateEnteredArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInteractingStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInteractingStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInteractingStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInteractingStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInteractingStateEnteredArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})");
}
impl ::core::clone::Clone for InteractionTrackerInteractingStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerInteractingStateEnteredArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerInteractingStateEnteredArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerInteractingStateEnteredArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerRequestIgnoredArgs(::windows_core::IUnknown);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerRequestIgnoredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerRequestIgnoredArgs {}
impl ::core::fmt::Debug for InteractionTrackerRequestIgnoredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerRequestIgnoredArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})");
}
impl ::core::clone::Clone for InteractionTrackerRequestIgnoredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerRequestIgnoredArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerRequestIgnoredArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerRequestIgnoredArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerValuesChangedArgs(::windows_core::IUnknown);
impl InteractionTrackerValuesChangedArgs {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerValuesChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerValuesChangedArgs {}
impl ::core::fmt::Debug for InteractionTrackerValuesChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerValuesChangedArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})");
}
impl ::core::clone::Clone for InteractionTrackerValuesChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerValuesChangedArgs {
    const IID: ::windows_core::GUID = <IInteractionTrackerValuesChangedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerValuesChangedArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaModifier(::windows_core::IUnknown);
impl InteractionTrackerVector2InertiaModifier {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaModifier").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})");
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerVector2InertiaModifier {
    const IID: ::windows_core::GUID = <IInteractionTrackerVector2InertiaModifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerVector2InertiaModifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerVector2InertiaModifier {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaModifier {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(::windows_core::IUnknown);
impl InteractionTrackerVector2InertiaNaturalMotion {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows_core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalMotion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Vector2NaturalMotionAnimation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNaturalMotion)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<InteractionTrackerVector2InertiaNaturalMotion>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaNaturalMotion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})");
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InteractionTrackerVector2InertiaNaturalMotion {
    const IID: ::windows_core::GUID = <IInteractionTrackerVector2InertiaNaturalMotion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
::windows_core::imp::interface_hierarchy!(InteractionTrackerVector2InertiaNaturalMotion, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for InteractionTrackerVector2InertiaNaturalMotion {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaNaturalMotion {}
impl ::windows_core::CanTryInto<InteractionTrackerVector2InertiaModifier> for InteractionTrackerVector2InertiaNaturalMotion {}
impl ::windows_core::CanTryInto<super::CompositionObject> for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct VisualInteractionSource(::windows_core::IUnknown);
impl VisualInteractionSource {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn IsPositionXRailsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionXRailsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPositionXRailsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPositionYRailsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionYRailsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPositionYRailsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ManipulationRedirectionMode(&self) -> ::windows_core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationRedirectionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetManipulationRedirectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionXChainingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionXSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionYChainingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionYSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleChainingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleSourceMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation<P0>(&self, pointerpoint: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Input::PointerPoint>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TryRedirectForManipulation)(::windows_core::Interface::as_raw(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DeltaPosition(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeltaPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeltaScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeltaScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocity(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ScaleVelocity(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointXModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointYModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionXModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaPositionXModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionYModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaPositionYModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaScaleModifiers<P0>(&self, conditionalvalues: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaScaleModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.try_into_param()?.abi()).ok() }
    }
    pub fn PointerWheelConfig(&self) -> ::windows_core::Result<InteractionSourceConfiguration> {
        let this = &::windows_core::ComInterface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerWheelConfig)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create<P0>(source: P0) -> ::windows_core::Result<VisualInteractionSource>
    where
        P0: ::windows_core::TryIntoParam<super::Visual>,
    {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromIVisualElement<P0>(source: P0) -> ::windows_core::Result<VisualInteractionSource>
    where
        P0: ::windows_core::TryIntoParam<super::IVisualElement>,
    {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIVisualElement)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VisualInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualInteractionSource {}
impl ::core::fmt::Debug for VisualInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VisualInteractionSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})");
}
impl ::core::clone::Clone for VisualInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualInteractionSource {
    const IID: ::windows_core::GUID = <IVisualInteractionSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
::windows_core::imp::interface_hierarchy!(VisualInteractionSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for VisualInteractionSource {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for VisualInteractionSource {}
impl ::windows_core::CanTryInto<ICompositionInteractionSource> for VisualInteractionSource {}
impl ::windows_core::CanTryInto<super::CompositionObject> for VisualInteractionSource {}
unsafe impl ::core::marker::Send for VisualInteractionSource {}
unsafe impl ::core::marker::Sync for VisualInteractionSource {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
impl ::core::marker::Copy for InteractionBindingAxisModes {}
impl ::core::clone::Clone for InteractionBindingAxisModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionBindingAxisModes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionBindingAxisModes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionBindingAxisModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionBindingAxisModes").field(&self.0).finish()
    }
}
impl InteractionBindingAxisModes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionChainingMode {}
impl ::core::clone::Clone for InteractionChainingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionChainingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionChainingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionChainingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionChainingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionSourceMode {}
impl ::core::clone::Clone for InteractionSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionSourceMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionSourceRedirectionMode {}
impl ::core::clone::Clone for InteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionSourceRedirectionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerClampingOption {}
impl ::core::clone::Clone for InteractionTrackerClampingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerClampingOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionTrackerClampingOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionTrackerClampingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerClampingOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerPositionUpdateOption {}
impl ::core::clone::Clone for InteractionTrackerPositionUpdateOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerPositionUpdateOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InteractionTrackerPositionUpdateOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InteractionTrackerPositionUpdateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerPositionUpdateOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}
impl ::core::marker::Copy for VisualInteractionSourceRedirectionMode {}
impl ::core::clone::Clone for VisualInteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualInteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VisualInteractionSourceRedirectionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VisualInteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

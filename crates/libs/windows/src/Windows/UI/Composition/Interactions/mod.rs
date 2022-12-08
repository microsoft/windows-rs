#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionConditionalValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionConditionalValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x090c4b72_8467_4d0a_9065_ac46b80a5522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct ICompositionInteractionSource(::windows::core::IUnknown);
impl ICompositionInteractionSource {}
::windows::core::interface_hierarchy!(ICompositionInteractionSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICompositionInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionInteractionSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x043b2431_06e3_495a_ba54_409f0017fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionInteractionSourceCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionInteractionSourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionSourceConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionSourceConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InteractionSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MaxPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMaxPosition: usize,
    pub MaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MinPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMinPosition: usize,
    pub MinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScaleInertiaDecayRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScaleInertiaDecayRate: usize,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionYInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureScaleInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionBy: usize,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithAdditionalVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAnimation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAdditionalVelocity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTracker2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25769a3e_ce6d_448c_8386_92620d240756);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTracker3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTracker4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebd222bc_04af_4ac7_847d_06ea36e80a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionByWithOption: usize,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTracker5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d579b7_0985_5e99_b024_2f32c380c1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e771ed_b803_5137_9435_1c96e48721e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifierFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifierFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x993818fe_c94e_4b86_87f3_922665ba46b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ed4699_0745_4096_bcab_3a4e99569bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub ModifiedRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ModifiedRestingPosition: usize,
    #[cfg(feature = "Foundation")]
    pub ModifiedRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModifiedRestingScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509652d6_d488_59cd_819f_f52310295b11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct IInteractionTrackerOwner(::windows::core::IUnknown);
impl IInteractionTrackerOwner {
    pub fn CustomAnimationStateEntered(&self, sender: &InteractionTracker, args: &InteractionTrackerCustomAnimationStateEnteredArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CustomAnimationStateEntered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
    pub fn IdleStateEntered(&self, sender: &InteractionTracker, args: &InteractionTrackerIdleStateEnteredArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).IdleStateEntered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
    pub fn InertiaStateEntered(&self, sender: &InteractionTracker, args: &InteractionTrackerInertiaStateEnteredArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InertiaStateEntered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
    pub fn InteractingStateEntered(&self, sender: &InteractionTracker, args: &InteractionTrackerInteractingStateEnteredArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InteractingStateEntered)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
    pub fn RequestIgnored(&self, sender: &InteractionTracker, args: &InteractionTrackerRequestIgnoredArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RequestIgnored)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
    pub fn ValuesChanged(&self, sender: &InteractionTracker, args: &InteractionTrackerValuesChangedArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ValuesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
}
::windows::core::interface_hierarchy!(IInteractionTrackerOwner, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInteractionTrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerOwner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerRequestIgnoredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerRequestIgnoredArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35e53720_46b7_5cb0_b505_f3d6884a6163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, axismode: InteractionBindingAxisModes) -> ::windows::core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, result__: *mut InteractionBindingAxisModes) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerValuesChangedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerValuesChangedArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82001a48_09c0_434f_8189_141c66df362f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    TryRedirectForManipulation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa914893_a73c_414d_80d0_249bad2fbd93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub DeltaPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DeltaPosition: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocity: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaScaleModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSource3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd941ef2a_0d5c_4057_92d7_c9711533204f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSourceObjectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x369965e1_8645_4f75_ba00_6479cd10c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa979c032_5764_55e0_bc1f_0778786dcfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionConditionalValue(::windows::core::IUnknown);
impl CompositionConditionalValue {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Condition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCondition(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCondition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    #[doc(hidden)]
    pub fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CompositionConditionalValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
}
unsafe impl ::windows::core::Interface for CompositionConditionalValue {
    const IID: ::windows::core::GUID = <ICompositionConditionalValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
::windows::core::interface_hierarchy!(CompositionConditionalValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionConditionalValue> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionInteractionSourceCollection(::windows::core::IUnknown);
impl CompositionInteractionSourceCollection {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Count(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Count)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Add<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICompositionInteractionSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Add)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Remove<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICompositionInteractionSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn RemoveAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAll)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CompositionInteractionSourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for CompositionInteractionSourceCollection {
    const IID: ::windows::core::GUID = <ICompositionInteractionSourceCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionInteractionSourceCollection {
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
::windows::core::interface_hierarchy!(CompositionInteractionSourceCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionSourceConfiguration(::windows::core::IUnknown);
impl InteractionSourceConfiguration {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionXSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionXSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionYSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionYSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaleSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InteractionSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionSourceConfiguration {
    const IID: ::windows::core::GUID = <IInteractionSourceConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
::windows::core::interface_hierarchy!(InteractionSourceConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTracker(::windows::core::IUnknown);
impl InteractionTracker {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn InteractionSources(&self) -> ::windows::core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InteractionSources)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPositionRoundingSuggested)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MaxPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMaxPosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MaxScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxScale)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MinPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMinPosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMinPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MinScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMinScale)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalRestingPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalRestingScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Owner(&self) -> ::windows::core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Owner)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionInertiaDecayRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPositionInertiaDecayRate<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionInertiaDecayRate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionVelocityInPixelsPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScaleInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleInertiaDecayRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScaleInertiaDecayRate<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<f32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaleInertiaDecayRate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleVelocityInPercentPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AdjustPositionXIfGreaterThanThreshold)(::windows::core::Vtable::as_raw(this), adjustment, positionthreshold).ok() }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AdjustPositionYIfGreaterThanThreshold)(::windows::core::Vtable::as_raw(this), adjustment, positionthreshold).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionXInertiaModifiers<P0, E0>(&self, modifiers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigurePositionXInertiaModifiers)(::windows::core::Vtable::as_raw(this), modifiers.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionYInertiaModifiers<P0, E0>(&self, modifiers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigurePositionYInertiaModifiers)(::windows::core::Vtable::as_raw(this), modifiers.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureScaleInertiaModifiers<P0, E0>(&self, modifiers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureScaleInertiaModifiers)(::windows::core::Vtable::as_raw(this), modifiers.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePosition(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePosition)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionBy(&self, amount: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionBy)(::windows::core::Vtable::as_raw(this), amount, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryUpdatePositionWithAnimation<P0>(&self, animation: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionWithAnimation)(::windows::core::Vtable::as_raw(this), animation.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithAdditionalVelocity(&self, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionWithAdditionalVelocity)(::windows::core::Vtable::as_raw(this), velocityinpixelspersecond, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScale(&self, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdateScale)(::windows::core::Vtable::as_raw(this), value, centerpoint, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAnimation<P0>(&self, animation: P0, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdateScaleWithAnimation)(::windows::core::Vtable::as_raw(this), animation.into().abi(), centerpoint, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAdditionalVelocity(&self, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdateScaleWithAdditionalVelocity)(::windows::core::Vtable::as_raw(this), velocityinpercentpersecond, centerpoint, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXInertiaModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureCenterPointXInertiaModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYInertiaModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureCenterPointYInertiaModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureVector2PositionInertiaModifiers<P0, E0>(&self, modifiers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureVector2PositionInertiaModifiers)(::windows::core::Vtable::as_raw(this), modifiers.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption(&self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionWithOption)(::windows::core::Vtable::as_raw(this), value, option, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionByWithOption(&self, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionByWithOption)(::windows::core::Vtable::as_raw(this), amount, option, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInertiaFromImpulse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption2(&self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdatePositionWithOption)(::windows::core::Vtable::as_raw(this), value, option, posupdateoption, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithOwner<P0, E0>(compositor: &super::Compositor, owner: P0) -> ::windows::core::Result<InteractionTracker>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IInteractionTrackerOwner>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithOwner)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), owner.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetBindingMode(boundtracker1: &InteractionTracker, boundtracker2: &InteractionTracker, axismode: InteractionBindingAxisModes) -> ::windows::core::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetBindingMode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(boundtracker1), ::core::mem::transmute_copy(boundtracker2), axismode).ok() })
    }
    pub fn GetBindingMode(boundtracker1: &InteractionTracker, boundtracker2: &InteractionTracker) -> ::windows::core::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBindingMode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(boundtracker1), ::core::mem::transmute_copy(boundtracker2), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InteractionTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTracker {
    const IID: ::windows::core::GUID = <IInteractionTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
::windows::core::interface_hierarchy!(InteractionTracker, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTracker> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFromBinding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerCustomAnimationStateEnteredArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerIdleStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFromBinding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerIdleStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerIdleStateEnteredArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerIdleStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerIdleStateEnteredArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaModifier(::windows::core::IUnknown);
impl InteractionTrackerInertiaModifier {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaModifier {
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaModifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
::windows::core::interface_hierarchy!(InteractionTrackerInertiaModifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaMotion(::windows::core::IUnknown);
impl InteractionTrackerInertiaMotion {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Condition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCondition(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCondition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Motion(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Motion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMotion(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMotion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaMotion {
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
::windows::core::interface_hierarchy!(InteractionTrackerInertiaMotion, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::InParam<InteractionTrackerInertiaModifier> {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaNaturalMotion(::windows::core::IUnknown);
impl InteractionTrackerInertiaNaturalMotion {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Condition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCondition(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCondition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalMotion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::ScalarNaturalMotionAnimation>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNaturalMotion)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaNaturalMotion {
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaNaturalMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
::windows::core::interface_hierarchy!(InteractionTrackerInertiaNaturalMotion, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::InParam<InteractionTrackerInertiaModifier> {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaRestingValue(::windows::core::IUnknown);
impl InteractionTrackerInertiaRestingValue {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Condition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCondition(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCondition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RestingValue(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RestingValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRestingValue(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRestingValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaRestingValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaRestingValue {
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaRestingValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
::windows::core::interface_hierarchy!(InteractionTrackerInertiaRestingValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::core::InParam<InteractionTrackerInertiaModifier> {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ModifiedRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModifiedRestingPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ModifiedRestingScale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModifiedRestingScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalRestingPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalRestingScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionVelocityInPixelsPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleVelocityInPercentPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInertiaFromImpulse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFromBinding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerInertiaStateEnteredArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInteractingStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFromBinding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInteractingStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerInteractingStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerInteractingStateEnteredArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerRequestIgnoredArgs(::windows::core::IUnknown);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerRequestIgnoredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerRequestIgnoredArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerRequestIgnoredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerRequestIgnoredArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerValuesChangedArgs(::windows::core::IUnknown);
impl InteractionTrackerValuesChangedArgs {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerValuesChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerValuesChangedArgs {
    const IID: ::windows::core::GUID = <IInteractionTrackerValuesChangedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
::windows::core::interface_hierarchy!(InteractionTrackerValuesChangedArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaModifier(::windows::core::IUnknown);
impl InteractionTrackerVector2InertiaModifier {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaModifier {
    const IID: ::windows::core::GUID = <IInteractionTrackerVector2InertiaModifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
::windows::core::interface_hierarchy!(InteractionTrackerVector2InertiaModifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(::windows::core::IUnknown);
impl InteractionTrackerVector2InertiaNaturalMotion {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Condition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCondition(&self, value: &super::ExpressionAnimation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCondition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalMotion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Vector2NaturalMotionAnimation>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNaturalMotion)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    const IID: ::windows::core::GUID = <IInteractionTrackerVector2InertiaNaturalMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
::windows::core::interface_hierarchy!(InteractionTrackerVector2InertiaNaturalMotion, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::InParam<InteractionTrackerVector2InertiaModifier> {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct VisualInteractionSource(::windows::core::IUnknown);
impl VisualInteractionSource {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPositionXRailsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPositionXRailsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPositionYRailsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPositionYRailsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ManipulationRedirectionMode(&self) -> ::windows::core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationRedirectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetManipulationRedirectionMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PositionXChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionXChainingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionXChainingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionXSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionXSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PositionYChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionYChainingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionYChainingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionYSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPositionYSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScaleChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleChainingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaleChainingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleSourceMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaleSourceMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation(&self, pointerpoint: &super::super::Input::PointerPoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).TryRedirectForManipulation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pointerpoint)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DeltaPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeltaPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeltaScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeltaScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocity(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionVelocity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ScaleVelocity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleVelocity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureCenterPointXModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureCenterPointYModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionXModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureDeltaPositionXModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionYModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureDeltaPositionYModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaScaleModifiers<P0, E0>(&self, conditionalvalues: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureDeltaScaleModifiers)(::windows::core::Vtable::as_raw(this), conditionalvalues.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn PointerWheelConfig(&self) -> ::windows::core::Result<InteractionSourceConfiguration> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerWheelConfig)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create<P0>(source: P0) -> ::windows::core::Result<VisualInteractionSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), source.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromIVisualElement<P0, E0>(source: P0) -> ::windows::core::Result<VisualInteractionSource>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::IVisualElement>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromIVisualElement)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for VisualInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VisualInteractionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
}
unsafe impl ::windows::core::Interface for VisualInteractionSource {
    const IID: ::windows::core::GUID = <IVisualInteractionSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
::windows::core::interface_hierarchy!(VisualInteractionSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VisualInteractionSource> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ::windows::core::InParam<ICompositionInteractionSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
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
unsafe impl ::windows::core::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionBindingAxisModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionBindingAxisModes").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for InteractionChainingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionChainingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionChainingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for InteractionSourceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerClampingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerClampingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerPositionUpdateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerPositionUpdateOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualInteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

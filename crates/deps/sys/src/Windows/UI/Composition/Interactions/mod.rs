#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositionConditionalValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionInteractionSourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionConditionalValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionConditionalValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionInteractionSourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionSourceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTracker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTracker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTracker4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTracker5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifierFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerOwner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerRequestIgnoredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerValuesChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSourceObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
#[repr(transparent)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
#[repr(transparent)]
pub struct InteractionSourceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
#[repr(transparent)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
#[repr(transparent)]
pub struct InteractionTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
#[repr(transparent)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerIdleStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInertiaModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInertiaMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInertiaNaturalMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInertiaRestingValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInertiaStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerInteractingStateEnteredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
#[repr(transparent)]
pub struct InteractionTrackerRequestIgnoredArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerValuesChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}

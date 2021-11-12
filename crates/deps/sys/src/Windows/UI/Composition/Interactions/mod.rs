#![allow(non_snake_case, non_camel_case_types)]
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
pub struct InteractionBindingAxisModes(i32);
pub struct InteractionChainingMode(i32);
#[repr(transparent)]
pub struct InteractionSourceConfiguration(pub *mut ::core::ffi::c_void);
pub struct InteractionSourceMode(i32);
pub struct InteractionSourceRedirectionMode(i32);
#[repr(transparent)]
pub struct InteractionTracker(pub *mut ::core::ffi::c_void);
pub struct InteractionTrackerClampingOption(i32);
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
pub struct InteractionTrackerPositionUpdateOption(i32);
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
pub struct VisualInteractionSourceRedirectionMode(i32);

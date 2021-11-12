#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUIAnimationInterpolator(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationInterpolator2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationLoopIterationChangeHandler2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationManager(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationManager2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationManagerEventHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationManagerEventHandler2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationPrimitiveInterpolation(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationPriorityComparison(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationPriorityComparison2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationStoryboard(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationStoryboard2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationStoryboardEventHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationStoryboardEventHandler2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTimer(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTimerClientEventHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTimerEventHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTimerUpdateHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransition(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransition2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransitionFactory(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransitionFactory2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransitionLibrary(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationTransitionLibrary2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariable(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariable2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariableChangeHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariableChangeHandler2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariableCurveChangeHandler2(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariableIntegerChangeHandler(pub *mut ::core::ffi::c_void);
pub struct IUIAnimationVariableIntegerChangeHandler2(pub *mut ::core::ffi::c_void);
pub struct UIAnimationManager(i32);
pub struct UIAnimationManager2(i32);
pub struct UIAnimationTimer(i32);
pub struct UIAnimationTransitionFactory(i32);
pub struct UIAnimationTransitionFactory2(i32);
pub struct UIAnimationTransitionLibrary(i32);
pub struct UIAnimationTransitionLibrary2(i32);
pub struct UI_ANIMATION_DEPENDENCIES(i32);
pub struct UI_ANIMATION_IDLE_BEHAVIOR(i32);
pub struct UI_ANIMATION_KEYFRAME(i32);
pub struct UI_ANIMATION_MANAGER_STATUS(i32);
pub struct UI_ANIMATION_MODE(i32);
pub struct UI_ANIMATION_PRIORITY_EFFECT(i32);
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
pub struct UI_ANIMATION_REPEAT_MODE(i32);
pub struct UI_ANIMATION_ROUNDING_MODE(i32);
pub struct UI_ANIMATION_SCHEDULING_RESULT(i32);
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
pub struct UI_ANIMATION_SLOPE(i32);
pub struct UI_ANIMATION_STORYBOARD_STATUS(i32);
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(i32);
pub struct UI_ANIMATION_UPDATE_RESULT(i32);

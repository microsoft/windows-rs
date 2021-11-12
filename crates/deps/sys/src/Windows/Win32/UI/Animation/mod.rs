#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUIAnimationInterpolator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationInterpolator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationLoopIterationChangeHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationPrimitiveInterpolation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationStoryboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationStoryboard2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTimerClientEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTimerEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTimerUpdateHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariable2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariableCurveChangeHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UIAnimationManager(i32);
#[repr(C)]
pub struct UIAnimationManager2(i32);
#[repr(C)]
pub struct UIAnimationTimer(i32);
#[repr(C)]
pub struct UIAnimationTransitionFactory(i32);
#[repr(C)]
pub struct UIAnimationTransitionFactory2(i32);
#[repr(C)]
pub struct UIAnimationTransitionLibrary(i32);
#[repr(C)]
pub struct UIAnimationTransitionLibrary2(i32);
#[repr(transparent)]
pub struct UI_ANIMATION_DEPENDENCIES(pub u32);
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0u32);
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1u32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2u32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4u32);
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8u32);
#[repr(transparent)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
#[repr(C)]
pub struct UI_ANIMATION_KEYFRAME(i32);
#[repr(transparent)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
#[repr(transparent)]
pub struct UI_ANIMATION_MODE(pub i32);
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
#[repr(transparent)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
#[repr(transparent)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
#[repr(transparent)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
#[repr(transparent)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
#[repr(transparent)]
pub struct UI_ANIMATION_SLOPE(pub i32);
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
#[repr(transparent)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
#[repr(transparent)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
#[repr(transparent)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);

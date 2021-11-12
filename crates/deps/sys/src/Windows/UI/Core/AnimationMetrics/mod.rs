#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnimationDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AnimationEffect(pub i32);
impl AnimationEffect {
    pub const Expand: AnimationEffect = AnimationEffect(0i32);
    pub const Collapse: AnimationEffect = AnimationEffect(1i32);
    pub const Reposition: AnimationEffect = AnimationEffect(2i32);
    pub const FadeIn: AnimationEffect = AnimationEffect(3i32);
    pub const FadeOut: AnimationEffect = AnimationEffect(4i32);
    pub const AddToList: AnimationEffect = AnimationEffect(5i32);
    pub const DeleteFromList: AnimationEffect = AnimationEffect(6i32);
    pub const AddToGrid: AnimationEffect = AnimationEffect(7i32);
    pub const DeleteFromGrid: AnimationEffect = AnimationEffect(8i32);
    pub const AddToSearchGrid: AnimationEffect = AnimationEffect(9i32);
    pub const DeleteFromSearchGrid: AnimationEffect = AnimationEffect(10i32);
    pub const AddToSearchList: AnimationEffect = AnimationEffect(11i32);
    pub const DeleteFromSearchList: AnimationEffect = AnimationEffect(12i32);
    pub const ShowEdgeUI: AnimationEffect = AnimationEffect(13i32);
    pub const ShowPanel: AnimationEffect = AnimationEffect(14i32);
    pub const HideEdgeUI: AnimationEffect = AnimationEffect(15i32);
    pub const HidePanel: AnimationEffect = AnimationEffect(16i32);
    pub const ShowPopup: AnimationEffect = AnimationEffect(17i32);
    pub const HidePopup: AnimationEffect = AnimationEffect(18i32);
    pub const PointerDown: AnimationEffect = AnimationEffect(19i32);
    pub const PointerUp: AnimationEffect = AnimationEffect(20i32);
    pub const DragSourceStart: AnimationEffect = AnimationEffect(21i32);
    pub const DragSourceEnd: AnimationEffect = AnimationEffect(22i32);
    pub const TransitionContent: AnimationEffect = AnimationEffect(23i32);
    pub const Reveal: AnimationEffect = AnimationEffect(24i32);
    pub const Hide: AnimationEffect = AnimationEffect(25i32);
    pub const DragBetweenEnter: AnimationEffect = AnimationEffect(26i32);
    pub const DragBetweenLeave: AnimationEffect = AnimationEffect(27i32);
    pub const SwipeSelect: AnimationEffect = AnimationEffect(28i32);
    pub const SwipeDeselect: AnimationEffect = AnimationEffect(29i32);
    pub const SwipeReveal: AnimationEffect = AnimationEffect(30i32);
    pub const EnterPage: AnimationEffect = AnimationEffect(31i32);
    pub const TransitionPage: AnimationEffect = AnimationEffect(32i32);
    pub const CrossFade: AnimationEffect = AnimationEffect(33i32);
    pub const Peek: AnimationEffect = AnimationEffect(34i32);
    pub const UpdateBadge: AnimationEffect = AnimationEffect(35i32);
}
#[repr(transparent)]
pub struct AnimationEffectTarget(pub i32);
impl AnimationEffectTarget {
    pub const Primary: AnimationEffectTarget = AnimationEffectTarget(0i32);
    pub const Added: AnimationEffectTarget = AnimationEffectTarget(1i32);
    pub const Affected: AnimationEffectTarget = AnimationEffectTarget(2i32);
    pub const Background: AnimationEffectTarget = AnimationEffectTarget(3i32);
    pub const Content: AnimationEffectTarget = AnimationEffectTarget(4i32);
    pub const Deleted: AnimationEffectTarget = AnimationEffectTarget(5i32);
    pub const Deselected: AnimationEffectTarget = AnimationEffectTarget(6i32);
    pub const DragSource: AnimationEffectTarget = AnimationEffectTarget(7i32);
    pub const Hidden: AnimationEffectTarget = AnimationEffectTarget(8i32);
    pub const Incoming: AnimationEffectTarget = AnimationEffectTarget(9i32);
    pub const Outgoing: AnimationEffectTarget = AnimationEffectTarget(10i32);
    pub const Outline: AnimationEffectTarget = AnimationEffectTarget(11i32);
    pub const Remaining: AnimationEffectTarget = AnimationEffectTarget(12i32);
    pub const Revealed: AnimationEffectTarget = AnimationEffectTarget(13i32);
    pub const RowIn: AnimationEffectTarget = AnimationEffectTarget(14i32);
    pub const RowOut: AnimationEffectTarget = AnimationEffectTarget(15i32);
    pub const Selected: AnimationEffectTarget = AnimationEffectTarget(16i32);
    pub const Selection: AnimationEffectTarget = AnimationEffectTarget(17i32);
    pub const Shown: AnimationEffectTarget = AnimationEffectTarget(18i32);
    pub const Tapped: AnimationEffectTarget = AnimationEffectTarget(19i32);
}
#[repr(C)]
pub struct AnimationMetricsContract(i32);
#[repr(transparent)]
pub struct IAnimationDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpacityAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScaleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpacityAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyAnimationType(pub i32);
impl PropertyAnimationType {
    pub const Scale: PropertyAnimationType = PropertyAnimationType(0i32);
    pub const Translation: PropertyAnimationType = PropertyAnimationType(1i32);
    pub const Opacity: PropertyAnimationType = PropertyAnimationType(2i32);
}
#[repr(transparent)]
pub struct ScaleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TranslationAnimation(pub *mut ::core::ffi::c_void);

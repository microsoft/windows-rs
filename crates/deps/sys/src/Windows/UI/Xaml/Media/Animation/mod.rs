#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddDeleteThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BasicConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BeginStoryboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BounceEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CircleEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: Self = Self(0i32);
    pub const Filling: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ColorAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorKeyFrameCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommonNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectedAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: Self = Self(0i32);
    pub const OffsetY: Self = Self(1i32);
    pub const CrossFade: Self = Self(2i32);
    pub const Scale: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectedAnimationService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContinuumNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CubicEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DirectConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiscreteColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiscreteDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiscreteObjectKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiscretePointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleKeyFrameCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragItemThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragOverThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DrillInNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DrillInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DrillOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropTargetItemThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasingColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasingDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasingFunctionBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: Self = Self(0i32);
    pub const EaseIn: Self = Self(1i32);
    pub const EaseInOut: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EasingPointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EdgeUIThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElasticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EntranceNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EntranceThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExponentialEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FadeInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FadeOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: Self = Self(0i32);
    pub const Stop: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GravityConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAddDeleteThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackEaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBeginStoryboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBeginStoryboardStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceEaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICircleEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorKeyFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimationService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedAnimationServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICubicEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscreteColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscreteDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscreteObjectKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiscretePointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleKeyFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragItemThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragItemThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragOverThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragOverThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrillInNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrillInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrillInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrillOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrillOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingFunctionBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingFunctionBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingFunctionBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingPointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasingPointKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEdgeUIThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEdgeUIThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElasticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElasticEaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEntranceThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEntranceThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExponentialEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExponentialEaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFadeInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFadeInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFadeOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFadeOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeySpline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyTimeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyTimeHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearPointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationTransitionInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationTransitionInfoOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectKeyFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaneThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaneThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointKeyFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerDownThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerDownThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerUpThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerUpThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupThemeTransitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerEaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuadraticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuarticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuinticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReorderThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatBehaviorHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatBehaviorHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepositionThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepositionThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepositionThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepositionThemeTransition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepositionThemeTransitionStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISineEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplineColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplineColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplineDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplineDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplinePointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplinePointKeyFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitCloseThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitCloseThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitOpenThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitOpenThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoryboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoryboardStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISuppressNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeBackThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeBackThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeHintThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeHintThemeAnimationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelineFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeySpline(pub *mut ::core::ffi::c_void);
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct KeyTime {
    pub TimeSpan: super::super::super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for KeyTime {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for KeyTime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyTimeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LinearColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LinearDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LinearPointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ObjectAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ObjectKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ObjectKeyFrameCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaneThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointKeyFrameCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerDownThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerUpThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PopInThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PopOutThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PopupThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuadraticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuarticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuinticEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReorderThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct RepeatBehavior {
    pub Count: f64,
    pub Duration: super::super::super::super::Foundation::TimeSpan,
    pub Type: RepeatBehaviorType,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for RepeatBehavior {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for RepeatBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepeatBehaviorHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: Self = Self(0i32);
    pub const Duration: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
#[repr(transparent)]
pub struct RepositionThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RepositionThemeTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SineEase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: Self = Self(0i32);
    pub const FromLeft: Self = Self(1i32);
    pub const FromRight: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SlideNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplineColorKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplineDoubleKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplinePointKeyFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitCloseThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitOpenThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Storyboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuppressNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeBackThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeHintThemeAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Timeline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelineCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Transition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TransitionCollection(pub *mut ::core::ffi::c_void);

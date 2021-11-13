#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddDeleteThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AddDeleteThemeTransition {}
impl ::core::clone::Clone for AddDeleteThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackEase {}
impl ::core::clone::Clone for BackEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BasicConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BasicConnectedAnimationConfiguration {}
impl ::core::clone::Clone for BasicConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BeginStoryboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BeginStoryboard {}
impl ::core::clone::Clone for BeginStoryboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BounceEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BounceEase {}
impl ::core::clone::Clone for BounceEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CircleEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CircleEase {}
impl ::core::clone::Clone for CircleEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: Self = Self(0i32);
    pub const Filling: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
}
impl ::core::marker::Copy for ClockState {}
impl ::core::clone::Clone for ClockState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorAnimation {}
impl ::core::clone::Clone for ColorAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorAnimationUsingKeyFrames {}
impl ::core::clone::Clone for ColorAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorKeyFrame {}
impl ::core::clone::Clone for ColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorKeyFrameCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorKeyFrameCollection {}
impl ::core::clone::Clone for ColorKeyFrameCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommonNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommonNavigationTransitionInfo {}
impl ::core::clone::Clone for CommonNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConnectedAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConnectedAnimation {}
impl ::core::clone::Clone for ConnectedAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: Self = Self(0i32);
    pub const OffsetY: Self = Self(1i32);
    pub const CrossFade: Self = Self(2i32);
    pub const Scale: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectedAnimationComponent {}
impl ::core::clone::Clone for ConnectedAnimationComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConnectedAnimationConfiguration {}
impl ::core::clone::Clone for ConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConnectedAnimationService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConnectedAnimationService {}
impl ::core::clone::Clone for ConnectedAnimationService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentThemeTransition {}
impl ::core::clone::Clone for ContentThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContinuumNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContinuumNavigationTransitionInfo {}
impl ::core::clone::Clone for ContinuumNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CubicEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CubicEase {}
impl ::core::clone::Clone for CubicEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DirectConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DirectConnectedAnimationConfiguration {}
impl ::core::clone::Clone for DirectConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiscreteColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DiscreteColorKeyFrame {}
impl ::core::clone::Clone for DiscreteColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiscreteDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DiscreteDoubleKeyFrame {}
impl ::core::clone::Clone for DiscreteDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiscreteObjectKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DiscreteObjectKeyFrame {}
impl ::core::clone::Clone for DiscreteObjectKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiscretePointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DiscretePointKeyFrame {}
impl ::core::clone::Clone for DiscretePointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DoubleAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DoubleAnimation {}
impl ::core::clone::Clone for DoubleAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DoubleAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DoubleAnimationUsingKeyFrames {}
impl ::core::clone::Clone for DoubleAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DoubleKeyFrame {}
impl ::core::clone::Clone for DoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DoubleKeyFrameCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DoubleKeyFrameCollection {}
impl ::core::clone::Clone for DoubleKeyFrameCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragItemThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragItemThemeAnimation {}
impl ::core::clone::Clone for DragItemThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragOverThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragOverThemeAnimation {}
impl ::core::clone::Clone for DragOverThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DrillInNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DrillInNavigationTransitionInfo {}
impl ::core::clone::Clone for DrillInNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DrillInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DrillInThemeAnimation {}
impl ::core::clone::Clone for DrillInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DrillOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DrillOutThemeAnimation {}
impl ::core::clone::Clone for DrillOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DropTargetItemThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DropTargetItemThemeAnimation {}
impl ::core::clone::Clone for DropTargetItemThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasingColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasingColorKeyFrame {}
impl ::core::clone::Clone for EasingColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasingDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasingDoubleKeyFrame {}
impl ::core::clone::Clone for EasingDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasingFunctionBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasingFunctionBase {}
impl ::core::clone::Clone for EasingFunctionBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: Self = Self(0i32);
    pub const EaseIn: Self = Self(1i32);
    pub const EaseInOut: Self = Self(2i32);
}
impl ::core::marker::Copy for EasingMode {}
impl ::core::clone::Clone for EasingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasingPointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasingPointKeyFrame {}
impl ::core::clone::Clone for EasingPointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EdgeUIThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EdgeUIThemeTransition {}
impl ::core::clone::Clone for EdgeUIThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElasticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElasticEase {}
impl ::core::clone::Clone for ElasticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EntranceNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EntranceNavigationTransitionInfo {}
impl ::core::clone::Clone for EntranceNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EntranceThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EntranceThemeTransition {}
impl ::core::clone::Clone for EntranceThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExponentialEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExponentialEase {}
impl ::core::clone::Clone for ExponentialEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FadeInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FadeInThemeAnimation {}
impl ::core::clone::Clone for FadeInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FadeOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FadeOutThemeAnimation {}
impl ::core::clone::Clone for FadeOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: Self = Self(0i32);
    pub const Stop: Self = Self(1i32);
}
impl ::core::marker::Copy for FillBehavior {}
impl ::core::clone::Clone for FillBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GravityConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GravityConnectedAnimationConfiguration {}
impl ::core::clone::Clone for GravityConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAddDeleteThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAddDeleteThemeTransition {}
impl ::core::clone::Clone for IAddDeleteThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackEase {}
impl ::core::clone::Clone for IBackEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackEaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackEaseStatics {}
impl ::core::clone::Clone for IBackEaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBasicConnectedAnimationConfiguration {}
impl ::core::clone::Clone for IBasicConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBasicConnectedAnimationConfigurationFactory {}
impl ::core::clone::Clone for IBasicConnectedAnimationConfigurationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBeginStoryboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBeginStoryboard {}
impl ::core::clone::Clone for IBeginStoryboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBeginStoryboardStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBeginStoryboardStatics {}
impl ::core::clone::Clone for IBeginStoryboardStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceEase {}
impl ::core::clone::Clone for IBounceEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceEaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceEaseStatics {}
impl ::core::clone::Clone for IBounceEaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICircleEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICircleEase {}
impl ::core::clone::Clone for ICircleEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorAnimation {}
impl ::core::clone::Clone for IColorAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorAnimationStatics {}
impl ::core::clone::Clone for IColorAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorAnimationUsingKeyFrames {}
impl ::core::clone::Clone for IColorAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorAnimationUsingKeyFramesStatics {}
impl ::core::clone::Clone for IColorAnimationUsingKeyFramesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorKeyFrame {}
impl ::core::clone::Clone for IColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorKeyFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorKeyFrameFactory {}
impl ::core::clone::Clone for IColorKeyFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorKeyFrameStatics {}
impl ::core::clone::Clone for IColorKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommonNavigationTransitionInfo {}
impl ::core::clone::Clone for ICommonNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommonNavigationTransitionInfoStatics {}
impl ::core::clone::Clone for ICommonNavigationTransitionInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimation {}
impl ::core::clone::Clone for IConnectedAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimation2 {}
impl ::core::clone::Clone for IConnectedAnimation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimation3 {}
impl ::core::clone::Clone for IConnectedAnimation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimationConfiguration {}
impl ::core::clone::Clone for IConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimationConfigurationFactory {}
impl ::core::clone::Clone for IConnectedAnimationConfigurationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimationService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimationService {}
impl ::core::clone::Clone for IConnectedAnimationService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedAnimationServiceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedAnimationServiceStatics {}
impl ::core::clone::Clone for IConnectedAnimationServiceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentThemeTransition {}
impl ::core::clone::Clone for IContentThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentThemeTransitionStatics {}
impl ::core::clone::Clone for IContentThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContinuumNavigationTransitionInfo {}
impl ::core::clone::Clone for IContinuumNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContinuumNavigationTransitionInfoStatics {}
impl ::core::clone::Clone for IContinuumNavigationTransitionInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICubicEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICubicEase {}
impl ::core::clone::Clone for ICubicEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirectConnectedAnimationConfiguration {}
impl ::core::clone::Clone for IDirectConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirectConnectedAnimationConfigurationFactory {}
impl ::core::clone::Clone for IDirectConnectedAnimationConfigurationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscreteColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscreteColorKeyFrame {}
impl ::core::clone::Clone for IDiscreteColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscreteDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscreteDoubleKeyFrame {}
impl ::core::clone::Clone for IDiscreteDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscreteObjectKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscreteObjectKeyFrame {}
impl ::core::clone::Clone for IDiscreteObjectKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscretePointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscretePointKeyFrame {}
impl ::core::clone::Clone for IDiscretePointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleAnimation {}
impl ::core::clone::Clone for IDoubleAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleAnimationStatics {}
impl ::core::clone::Clone for IDoubleAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleAnimationUsingKeyFrames {}
impl ::core::clone::Clone for IDoubleAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleAnimationUsingKeyFramesStatics {}
impl ::core::clone::Clone for IDoubleAnimationUsingKeyFramesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleKeyFrame {}
impl ::core::clone::Clone for IDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleKeyFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleKeyFrameFactory {}
impl ::core::clone::Clone for IDoubleKeyFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDoubleKeyFrameStatics {}
impl ::core::clone::Clone for IDoubleKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragItemThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragItemThemeAnimation {}
impl ::core::clone::Clone for IDragItemThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragItemThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragItemThemeAnimationStatics {}
impl ::core::clone::Clone for IDragItemThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragOverThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragOverThemeAnimation {}
impl ::core::clone::Clone for IDragOverThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragOverThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragOverThemeAnimationStatics {}
impl ::core::clone::Clone for IDragOverThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDrillInNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDrillInNavigationTransitionInfo {}
impl ::core::clone::Clone for IDrillInNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDrillInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDrillInThemeAnimation {}
impl ::core::clone::Clone for IDrillInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDrillInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDrillInThemeAnimationStatics {}
impl ::core::clone::Clone for IDrillInThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDrillOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDrillOutThemeAnimation {}
impl ::core::clone::Clone for IDrillOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDrillOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDrillOutThemeAnimationStatics {}
impl ::core::clone::Clone for IDrillOutThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropTargetItemThemeAnimation {}
impl ::core::clone::Clone for IDropTargetItemThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropTargetItemThemeAnimationStatics {}
impl ::core::clone::Clone for IDropTargetItemThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingColorKeyFrame {}
impl ::core::clone::Clone for IEasingColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingColorKeyFrameStatics {}
impl ::core::clone::Clone for IEasingColorKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingDoubleKeyFrame {}
impl ::core::clone::Clone for IEasingDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingDoubleKeyFrameStatics {}
impl ::core::clone::Clone for IEasingDoubleKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingFunctionBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingFunctionBase {}
impl ::core::clone::Clone for IEasingFunctionBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingFunctionBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingFunctionBaseFactory {}
impl ::core::clone::Clone for IEasingFunctionBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingFunctionBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingFunctionBaseStatics {}
impl ::core::clone::Clone for IEasingFunctionBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingPointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingPointKeyFrame {}
impl ::core::clone::Clone for IEasingPointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasingPointKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasingPointKeyFrameStatics {}
impl ::core::clone::Clone for IEasingPointKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEdgeUIThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEdgeUIThemeTransition {}
impl ::core::clone::Clone for IEdgeUIThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEdgeUIThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEdgeUIThemeTransitionStatics {}
impl ::core::clone::Clone for IEdgeUIThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElasticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElasticEase {}
impl ::core::clone::Clone for IElasticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElasticEaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElasticEaseStatics {}
impl ::core::clone::Clone for IElasticEaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEntranceNavigationTransitionInfo {}
impl ::core::clone::Clone for IEntranceNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEntranceNavigationTransitionInfoStatics {}
impl ::core::clone::Clone for IEntranceNavigationTransitionInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEntranceThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEntranceThemeTransition {}
impl ::core::clone::Clone for IEntranceThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEntranceThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEntranceThemeTransitionStatics {}
impl ::core::clone::Clone for IEntranceThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExponentialEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExponentialEase {}
impl ::core::clone::Clone for IExponentialEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExponentialEaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExponentialEaseStatics {}
impl ::core::clone::Clone for IExponentialEaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFadeInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFadeInThemeAnimation {}
impl ::core::clone::Clone for IFadeInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFadeInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFadeInThemeAnimationStatics {}
impl ::core::clone::Clone for IFadeInThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFadeOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFadeOutThemeAnimation {}
impl ::core::clone::Clone for IFadeOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFadeOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFadeOutThemeAnimationStatics {}
impl ::core::clone::Clone for IFadeOutThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGravityConnectedAnimationConfiguration {}
impl ::core::clone::Clone for IGravityConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGravityConnectedAnimationConfiguration2 {}
impl ::core::clone::Clone for IGravityConnectedAnimationConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfigurationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGravityConnectedAnimationConfigurationFactory {}
impl ::core::clone::Clone for IGravityConnectedAnimationConfigurationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeySpline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeySpline {}
impl ::core::clone::Clone for IKeySpline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyTimeHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyTimeHelper {}
impl ::core::clone::Clone for IKeyTimeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyTimeHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyTimeHelperStatics {}
impl ::core::clone::Clone for IKeyTimeHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearColorKeyFrame {}
impl ::core::clone::Clone for ILinearColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearDoubleKeyFrame {}
impl ::core::clone::Clone for ILinearDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearPointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearPointKeyFrame {}
impl ::core::clone::Clone for ILinearPointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationThemeTransition {}
impl ::core::clone::Clone for INavigationThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationThemeTransitionStatics {}
impl ::core::clone::Clone for INavigationThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationTransitionInfo {}
impl ::core::clone::Clone for INavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationTransitionInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationTransitionInfoFactory {}
impl ::core::clone::Clone for INavigationTransitionInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationTransitionInfoOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationTransitionInfoOverrides {}
impl ::core::clone::Clone for INavigationTransitionInfoOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectAnimationUsingKeyFrames {}
impl ::core::clone::Clone for IObjectAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectAnimationUsingKeyFramesStatics {}
impl ::core::clone::Clone for IObjectAnimationUsingKeyFramesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectKeyFrame {}
impl ::core::clone::Clone for IObjectKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectKeyFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectKeyFrameFactory {}
impl ::core::clone::Clone for IObjectKeyFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectKeyFrameStatics {}
impl ::core::clone::Clone for IObjectKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaneThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaneThemeTransition {}
impl ::core::clone::Clone for IPaneThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaneThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaneThemeTransitionStatics {}
impl ::core::clone::Clone for IPaneThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointAnimation {}
impl ::core::clone::Clone for IPointAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointAnimationStatics {}
impl ::core::clone::Clone for IPointAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointAnimationUsingKeyFrames {}
impl ::core::clone::Clone for IPointAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFramesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointAnimationUsingKeyFramesStatics {}
impl ::core::clone::Clone for IPointAnimationUsingKeyFramesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointKeyFrame {}
impl ::core::clone::Clone for IPointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointKeyFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointKeyFrameFactory {}
impl ::core::clone::Clone for IPointKeyFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointKeyFrameStatics {}
impl ::core::clone::Clone for IPointKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerDownThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerDownThemeAnimation {}
impl ::core::clone::Clone for IPointerDownThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerDownThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerDownThemeAnimationStatics {}
impl ::core::clone::Clone for IPointerDownThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerUpThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerUpThemeAnimation {}
impl ::core::clone::Clone for IPointerUpThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerUpThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerUpThemeAnimationStatics {}
impl ::core::clone::Clone for IPointerUpThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopInThemeAnimation {}
impl ::core::clone::Clone for IPopInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopInThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopInThemeAnimationStatics {}
impl ::core::clone::Clone for IPopInThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopOutThemeAnimation {}
impl ::core::clone::Clone for IPopOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopOutThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopOutThemeAnimationStatics {}
impl ::core::clone::Clone for IPopOutThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupThemeTransition {}
impl ::core::clone::Clone for IPopupThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupThemeTransitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupThemeTransitionStatics {}
impl ::core::clone::Clone for IPopupThemeTransitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPowerEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPowerEase {}
impl ::core::clone::Clone for IPowerEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPowerEaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPowerEaseStatics {}
impl ::core::clone::Clone for IPowerEaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuadraticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuadraticEase {}
impl ::core::clone::Clone for IQuadraticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuarticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuarticEase {}
impl ::core::clone::Clone for IQuarticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuinticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuinticEase {}
impl ::core::clone::Clone for IQuinticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReorderThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReorderThemeTransition {}
impl ::core::clone::Clone for IReorderThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatBehaviorHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatBehaviorHelper {}
impl ::core::clone::Clone for IRepeatBehaviorHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatBehaviorHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatBehaviorHelperStatics {}
impl ::core::clone::Clone for IRepeatBehaviorHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepositionThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepositionThemeAnimation {}
impl ::core::clone::Clone for IRepositionThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepositionThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepositionThemeAnimationStatics {}
impl ::core::clone::Clone for IRepositionThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepositionThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepositionThemeTransition {}
impl ::core::clone::Clone for IRepositionThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepositionThemeTransition2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepositionThemeTransition2 {}
impl ::core::clone::Clone for IRepositionThemeTransition2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepositionThemeTransitionStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepositionThemeTransitionStatics2 {}
impl ::core::clone::Clone for IRepositionThemeTransitionStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISineEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISineEase {}
impl ::core::clone::Clone for ISineEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlideNavigationTransitionInfo {}
impl ::core::clone::Clone for ISlideNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlideNavigationTransitionInfo2 {}
impl ::core::clone::Clone for ISlideNavigationTransitionInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlideNavigationTransitionInfoStatics2 {}
impl ::core::clone::Clone for ISlideNavigationTransitionInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplineColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplineColorKeyFrame {}
impl ::core::clone::Clone for ISplineColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplineColorKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplineColorKeyFrameStatics {}
impl ::core::clone::Clone for ISplineColorKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplineDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplineDoubleKeyFrame {}
impl ::core::clone::Clone for ISplineDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplineDoubleKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplineDoubleKeyFrameStatics {}
impl ::core::clone::Clone for ISplineDoubleKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplinePointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplinePointKeyFrame {}
impl ::core::clone::Clone for ISplinePointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplinePointKeyFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplinePointKeyFrameStatics {}
impl ::core::clone::Clone for ISplinePointKeyFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitCloseThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitCloseThemeAnimation {}
impl ::core::clone::Clone for ISplitCloseThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitCloseThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitCloseThemeAnimationStatics {}
impl ::core::clone::Clone for ISplitCloseThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitOpenThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitOpenThemeAnimation {}
impl ::core::clone::Clone for ISplitOpenThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitOpenThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitOpenThemeAnimationStatics {}
impl ::core::clone::Clone for ISplitOpenThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStoryboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStoryboard {}
impl ::core::clone::Clone for IStoryboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStoryboardStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStoryboardStatics {}
impl ::core::clone::Clone for IStoryboardStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISuppressNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISuppressNavigationTransitionInfo {}
impl ::core::clone::Clone for ISuppressNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeBackThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeBackThemeAnimation {}
impl ::core::clone::Clone for ISwipeBackThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeBackThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeBackThemeAnimationStatics {}
impl ::core::clone::Clone for ISwipeBackThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeHintThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeHintThemeAnimation {}
impl ::core::clone::Clone for ISwipeHintThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeHintThemeAnimationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeHintThemeAnimationStatics {}
impl ::core::clone::Clone for ISwipeHintThemeAnimationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeline {}
impl ::core::clone::Clone for ITimeline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelineFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelineFactory {}
impl ::core::clone::Clone for ITimelineFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelineStatics {}
impl ::core::clone::Clone for ITimelineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransition {}
impl ::core::clone::Clone for ITransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransitionFactory {}
impl ::core::clone::Clone for ITransitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeySpline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeySpline {}
impl ::core::clone::Clone for KeySpline {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for KeyTimeHelper {}
impl ::core::clone::Clone for KeyTimeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LinearColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LinearColorKeyFrame {}
impl ::core::clone::Clone for LinearColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LinearDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LinearDoubleKeyFrame {}
impl ::core::clone::Clone for LinearDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LinearPointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LinearPointKeyFrame {}
impl ::core::clone::Clone for LinearPointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationThemeTransition {}
impl ::core::clone::Clone for NavigationThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationTransitionInfo {}
impl ::core::clone::Clone for NavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ObjectAnimationUsingKeyFrames {}
impl ::core::clone::Clone for ObjectAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ObjectKeyFrame {}
impl ::core::clone::Clone for ObjectKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ObjectKeyFrameCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ObjectKeyFrameCollection {}
impl ::core::clone::Clone for ObjectKeyFrameCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaneThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaneThemeTransition {}
impl ::core::clone::Clone for PaneThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointAnimation {}
impl ::core::clone::Clone for PointAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointAnimationUsingKeyFrames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointAnimationUsingKeyFrames {}
impl ::core::clone::Clone for PointAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointKeyFrame {}
impl ::core::clone::Clone for PointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointKeyFrameCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointKeyFrameCollection {}
impl ::core::clone::Clone for PointKeyFrameCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerDownThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerDownThemeAnimation {}
impl ::core::clone::Clone for PointerDownThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerUpThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerUpThemeAnimation {}
impl ::core::clone::Clone for PointerUpThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PopInThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PopInThemeAnimation {}
impl ::core::clone::Clone for PopInThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PopOutThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PopOutThemeAnimation {}
impl ::core::clone::Clone for PopOutThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PopupThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PopupThemeTransition {}
impl ::core::clone::Clone for PopupThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PowerEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PowerEase {}
impl ::core::clone::Clone for PowerEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuadraticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuadraticEase {}
impl ::core::clone::Clone for QuadraticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuarticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuarticEase {}
impl ::core::clone::Clone for QuarticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuinticEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuinticEase {}
impl ::core::clone::Clone for QuinticEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReorderThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReorderThemeTransition {}
impl ::core::clone::Clone for ReorderThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RepeatBehaviorHelper {}
impl ::core::clone::Clone for RepeatBehaviorHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: Self = Self(0i32);
    pub const Duration: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for RepeatBehaviorType {}
impl ::core::clone::Clone for RepeatBehaviorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepositionThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RepositionThemeAnimation {}
impl ::core::clone::Clone for RepositionThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepositionThemeTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RepositionThemeTransition {}
impl ::core::clone::Clone for RepositionThemeTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SineEase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SineEase {}
impl ::core::clone::Clone for SineEase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: Self = Self(0i32);
    pub const FromLeft: Self = Self(1i32);
    pub const FromRight: Self = Self(2i32);
}
impl ::core::marker::Copy for SlideNavigationTransitionEffect {}
impl ::core::clone::Clone for SlideNavigationTransitionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SlideNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SlideNavigationTransitionInfo {}
impl ::core::clone::Clone for SlideNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplineColorKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplineColorKeyFrame {}
impl ::core::clone::Clone for SplineColorKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplineDoubleKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplineDoubleKeyFrame {}
impl ::core::clone::Clone for SplineDoubleKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplinePointKeyFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplinePointKeyFrame {}
impl ::core::clone::Clone for SplinePointKeyFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitCloseThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitCloseThemeAnimation {}
impl ::core::clone::Clone for SplitCloseThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitOpenThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitOpenThemeAnimation {}
impl ::core::clone::Clone for SplitOpenThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Storyboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Storyboard {}
impl ::core::clone::Clone for Storyboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuppressNavigationTransitionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuppressNavigationTransitionInfo {}
impl ::core::clone::Clone for SuppressNavigationTransitionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeBackThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwipeBackThemeAnimation {}
impl ::core::clone::Clone for SwipeBackThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeHintThemeAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwipeHintThemeAnimation {}
impl ::core::clone::Clone for SwipeHintThemeAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Timeline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Timeline {}
impl ::core::clone::Clone for Timeline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelineCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelineCollection {}
impl ::core::clone::Clone for TimelineCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Transition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Transition {}
impl ::core::clone::Clone for Transition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TransitionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TransitionCollection {}
impl ::core::clone::Clone for TransitionCollection {
    fn clone(&self) -> Self {
        *self
    }
}

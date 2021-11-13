#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Composition_Core")]
pub mod Core;
#[cfg(feature = "UI_Composition_Desktop")]
pub mod Desktop;
#[cfg(feature = "UI_Composition_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "UI_Composition_Effects")]
pub mod Effects;
#[cfg(feature = "UI_Composition_Interactions")]
pub mod Interactions;
#[cfg(feature = "UI_Composition_Scenes")]
pub mod Scenes;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AmbientLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AmbientLight {}
impl ::core::clone::Clone for AmbientLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AnimationController {}
impl ::core::clone::Clone for AnimationController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationControllerProgressBehavior(pub i32);
impl AnimationControllerProgressBehavior {
    pub const Default: Self = Self(0i32);
    pub const IncludesDelayTime: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationControllerProgressBehavior {}
impl ::core::clone::Clone for AnimationControllerProgressBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationDelayBehavior(pub i32);
impl AnimationDelayBehavior {
    pub const SetInitialValueAfterDelay: Self = Self(0i32);
    pub const SetInitialValueBeforeDelay: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationDelayBehavior {}
impl ::core::clone::Clone for AnimationDelayBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Normal: Self = Self(0i32);
    pub const Reverse: Self = Self(1i32);
    pub const Alternate: Self = Self(2i32);
    pub const AlternateReverse: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationIterationBehavior(pub i32);
impl AnimationIterationBehavior {
    pub const Count: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for AnimationIterationBehavior {}
impl ::core::clone::Clone for AnimationIterationBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationPropertyAccessMode(pub i32);
impl AnimationPropertyAccessMode {
    pub const None: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const WriteOnly: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationPropertyAccessMode {}
impl ::core::clone::Clone for AnimationPropertyAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationPropertyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AnimationPropertyInfo {}
impl ::core::clone::Clone for AnimationPropertyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AnimationStopBehavior(pub i32);
impl AnimationStopBehavior {
    pub const LeaveCurrentValue: Self = Self(0i32);
    pub const SetToInitialValue: Self = Self(1i32);
    pub const SetToFinalValue: Self = Self(2i32);
}
impl ::core::marker::Copy for AnimationStopBehavior {}
impl ::core::clone::Clone for AnimationStopBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackEasingFunction {}
impl ::core::clone::Clone for BackEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BooleanKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BooleanKeyFrameAnimation {}
impl ::core::clone::Clone for BooleanKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BounceEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BounceEasingFunction {}
impl ::core::clone::Clone for BounceEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BounceScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BounceScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for BounceScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BounceVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BounceVector2NaturalMotionAnimation {}
impl ::core::clone::Clone for BounceVector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BounceVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BounceVector3NaturalMotionAnimation {}
impl ::core::clone::Clone for BounceVector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CircleEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CircleEasingFunction {}
impl ::core::clone::Clone for CircleEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorKeyFrameAnimation {}
impl ::core::clone::Clone for ColorKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionAnimation {}
impl ::core::clone::Clone for CompositionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionAnimationGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionAnimationGroup {}
impl ::core::clone::Clone for CompositionAnimationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBackdropBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionBackdropBrush {}
impl ::core::clone::Clone for CompositionBackdropBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBackfaceVisibility(pub i32);
impl CompositionBackfaceVisibility {
    pub const Inherit: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionBackfaceVisibility {}
impl ::core::clone::Clone for CompositionBackfaceVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBatchCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionBatchCompletedEventArgs {}
impl ::core::clone::Clone for CompositionBatchCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBatchTypes(pub u32);
impl CompositionBatchTypes {
    pub const None: Self = Self(0u32);
    pub const Animation: Self = Self(1u32);
    pub const Effect: Self = Self(2u32);
    pub const InfiniteAnimation: Self = Self(4u32);
    pub const AllAnimations: Self = Self(5u32);
}
impl ::core::marker::Copy for CompositionBatchTypes {}
impl ::core::clone::Clone for CompositionBatchTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBitmapInterpolationMode(pub i32);
impl CompositionBitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const MagLinearMinLinearMipLinear: Self = Self(2i32);
    pub const MagLinearMinLinearMipNearest: Self = Self(3i32);
    pub const MagLinearMinNearestMipLinear: Self = Self(4i32);
    pub const MagLinearMinNearestMipNearest: Self = Self(5i32);
    pub const MagNearestMinLinearMipLinear: Self = Self(6i32);
    pub const MagNearestMinLinearMipNearest: Self = Self(7i32);
    pub const MagNearestMinNearestMipLinear: Self = Self(8i32);
    pub const MagNearestMinNearestMipNearest: Self = Self(9i32);
}
impl ::core::marker::Copy for CompositionBitmapInterpolationMode {}
impl ::core::clone::Clone for CompositionBitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBorderMode(pub i32);
impl CompositionBorderMode {
    pub const Inherit: Self = Self(0i32);
    pub const Soft: Self = Self(1i32);
    pub const Hard: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionBorderMode {}
impl ::core::clone::Clone for CompositionBorderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionBrush {}
impl ::core::clone::Clone for CompositionBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionCapabilities {}
impl ::core::clone::Clone for CompositionCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionClip {}
impl ::core::clone::Clone for CompositionClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionColorBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionColorBrush {}
impl ::core::clone::Clone for CompositionColorBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionColorGradientStop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionColorGradientStop {}
impl ::core::clone::Clone for CompositionColorGradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionColorGradientStopCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionColorGradientStopCollection {}
impl ::core::clone::Clone for CompositionColorGradientStopCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionColorSpace(pub i32);
impl CompositionColorSpace {
    pub const Auto: Self = Self(0i32);
    pub const Hsl: Self = Self(1i32);
    pub const Rgb: Self = Self(2i32);
    pub const HslLinear: Self = Self(3i32);
    pub const RgbLinear: Self = Self(4i32);
}
impl ::core::marker::Copy for CompositionColorSpace {}
impl ::core::clone::Clone for CompositionColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionCommitBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionCommitBatch {}
impl ::core::clone::Clone for CompositionCommitBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionCompositeMode(pub i32);
impl CompositionCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const DestinationInvert: Self = Self(2i32);
    pub const MinBlend: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionCompositeMode {}
impl ::core::clone::Clone for CompositionCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionContainerShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionContainerShape {}
impl ::core::clone::Clone for CompositionContainerShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionDrawingSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionDrawingSurface {}
impl ::core::clone::Clone for CompositionDrawingSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionDropShadowSourcePolicy(pub i32);
impl CompositionDropShadowSourcePolicy {
    pub const Default: Self = Self(0i32);
    pub const InheritFromVisualContent: Self = Self(1i32);
}
impl ::core::marker::Copy for CompositionDropShadowSourcePolicy {}
impl ::core::clone::Clone for CompositionDropShadowSourcePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionEasingFunction {}
impl ::core::clone::Clone for CompositionEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEasingFunctionMode(pub i32);
impl CompositionEasingFunctionMode {
    pub const In: Self = Self(0i32);
    pub const Out: Self = Self(1i32);
    pub const InOut: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionEasingFunctionMode {}
impl ::core::clone::Clone for CompositionEasingFunctionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEffectBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionEffectBrush {}
impl ::core::clone::Clone for CompositionEffectBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionEffectFactory {}
impl ::core::clone::Clone for CompositionEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEffectFactoryLoadStatus(pub i32);
impl CompositionEffectFactoryLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const EffectTooComplex: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Other: Self = Self(-1i32);
}
impl ::core::marker::Copy for CompositionEffectFactoryLoadStatus {}
impl ::core::clone::Clone for CompositionEffectFactoryLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEffectSourceParameter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionEffectSourceParameter {}
impl ::core::clone::Clone for CompositionEffectSourceParameter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionEllipseGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionEllipseGeometry {}
impl ::core::clone::Clone for CompositionEllipseGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGeometricClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionGeometricClip {}
impl ::core::clone::Clone for CompositionGeometricClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionGeometry {}
impl ::core::clone::Clone for CompositionGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGetValueStatus(pub i32);
impl CompositionGetValueStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const TypeMismatch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionGetValueStatus {}
impl ::core::clone::Clone for CompositionGetValueStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionGradientBrush {}
impl ::core::clone::Clone for CompositionGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGradientExtendMode(pub i32);
impl CompositionGradientExtendMode {
    pub const Clamp: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
}
impl ::core::marker::Copy for CompositionGradientExtendMode {}
impl ::core::clone::Clone for CompositionGradientExtendMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionGraphicsDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionGraphicsDevice {}
impl ::core::clone::Clone for CompositionGraphicsDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionLight {}
impl ::core::clone::Clone for CompositionLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionLineGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionLineGeometry {}
impl ::core::clone::Clone for CompositionLineGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionLinearGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionLinearGradientBrush {}
impl ::core::clone::Clone for CompositionLinearGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionMappingMode(pub i32);
impl CompositionMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for CompositionMappingMode {}
impl ::core::clone::Clone for CompositionMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionMaskBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionMaskBrush {}
impl ::core::clone::Clone for CompositionMaskBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionMipmapSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionMipmapSurface {}
impl ::core::clone::Clone for CompositionMipmapSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionNineGridBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionNineGridBrush {}
impl ::core::clone::Clone for CompositionNineGridBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionObject {}
impl ::core::clone::Clone for CompositionObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionPath {}
impl ::core::clone::Clone for CompositionPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionPathGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionPathGeometry {}
impl ::core::clone::Clone for CompositionPathGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionProjectedShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionProjectedShadow {}
impl ::core::clone::Clone for CompositionProjectedShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionProjectedShadowCaster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionProjectedShadowCaster {}
impl ::core::clone::Clone for CompositionProjectedShadowCaster {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionProjectedShadowCasterCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionProjectedShadowCasterCollection {}
impl ::core::clone::Clone for CompositionProjectedShadowCasterCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionProjectedShadowReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionProjectedShadowReceiver {}
impl ::core::clone::Clone for CompositionProjectedShadowReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionProjectedShadowReceiverUnorderedCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionProjectedShadowReceiverUnorderedCollection {}
impl ::core::clone::Clone for CompositionProjectedShadowReceiverUnorderedCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionPropertySet {}
impl ::core::clone::Clone for CompositionPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionRadialGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionRadialGradientBrush {}
impl ::core::clone::Clone for CompositionRadialGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionRectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionRectangleGeometry {}
impl ::core::clone::Clone for CompositionRectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionRoundedRectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionRoundedRectangleGeometry {}
impl ::core::clone::Clone for CompositionRoundedRectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionScopedBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionScopedBatch {}
impl ::core::clone::Clone for CompositionScopedBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionShadow {}
impl ::core::clone::Clone for CompositionShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionShape {}
impl ::core::clone::Clone for CompositionShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionShapeCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionShapeCollection {}
impl ::core::clone::Clone for CompositionShapeCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionSpriteShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionSpriteShape {}
impl ::core::clone::Clone for CompositionSpriteShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionStretch(pub i32);
impl CompositionStretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStretch {}
impl ::core::clone::Clone for CompositionStretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionStrokeCap(pub i32);
impl CompositionStrokeCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStrokeCap {}
impl ::core::clone::Clone for CompositionStrokeCap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionStrokeDashArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionStrokeDashArray {}
impl ::core::clone::Clone for CompositionStrokeDashArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionStrokeLineJoin(pub i32);
impl CompositionStrokeLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const MiterOrBevel: Self = Self(3i32);
}
impl ::core::marker::Copy for CompositionStrokeLineJoin {}
impl ::core::clone::Clone for CompositionStrokeLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionSurfaceBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionSurfaceBrush {}
impl ::core::clone::Clone for CompositionSurfaceBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionTarget {}
impl ::core::clone::Clone for CompositionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionTransform {}
impl ::core::clone::Clone for CompositionTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionViewBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionViewBox {}
impl ::core::clone::Clone for CompositionViewBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionVirtualDrawingSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionVirtualDrawingSurface {}
impl ::core::clone::Clone for CompositionVirtualDrawingSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionVisualSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositionVisualSurface {}
impl ::core::clone::Clone for CompositionVisualSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Compositor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Compositor {}
impl ::core::clone::Clone for Compositor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContainerVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContainerVisual {}
impl ::core::clone::Clone for ContainerVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CubicBezierEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CubicBezierEasingFunction {}
impl ::core::clone::Clone for CubicBezierEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DelegatedInkTrailVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DelegatedInkTrailVisual {}
impl ::core::clone::Clone for DelegatedInkTrailVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DistantLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DistantLight {}
impl ::core::clone::Clone for DistantLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DropShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DropShadow {}
impl ::core::clone::Clone for DropShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElasticEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElasticEasingFunction {}
impl ::core::clone::Clone for ElasticEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExponentialEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExponentialEasingFunction {}
impl ::core::clone::Clone for ExponentialEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExpressionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExpressionAnimation {}
impl ::core::clone::Clone for ExpressionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAmbientLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAmbientLight {}
impl ::core::clone::Clone for IAmbientLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAmbientLight2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAmbientLight2 {}
impl ::core::clone::Clone for IAmbientLight2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnimationController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnimationController {}
impl ::core::clone::Clone for IAnimationController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnimationControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnimationControllerStatics {}
impl ::core::clone::Clone for IAnimationControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnimationObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnimationObject {}
impl ::core::clone::Clone for IAnimationObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnimationPropertyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnimationPropertyInfo {}
impl ::core::clone::Clone for IAnimationPropertyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnimationPropertyInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnimationPropertyInfo2 {}
impl ::core::clone::Clone for IAnimationPropertyInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackEasingFunction {}
impl ::core::clone::Clone for IBackEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBooleanKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBooleanKeyFrameAnimation {}
impl ::core::clone::Clone for IBooleanKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceEasingFunction {}
impl ::core::clone::Clone for IBounceEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for IBounceScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceVector2NaturalMotionAnimation {}
impl ::core::clone::Clone for IBounceVector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBounceVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBounceVector3NaturalMotionAnimation {}
impl ::core::clone::Clone for IBounceVector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICircleEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICircleEasingFunction {}
impl ::core::clone::Clone for ICircleEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorKeyFrameAnimation {}
impl ::core::clone::Clone for IColorKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimation {}
impl ::core::clone::Clone for ICompositionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimation2 {}
impl ::core::clone::Clone for ICompositionAnimation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimation3 {}
impl ::core::clone::Clone for ICompositionAnimation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimation4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimation4 {}
impl ::core::clone::Clone for ICompositionAnimation4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimationBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimationBase {}
impl ::core::clone::Clone for ICompositionAnimationBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimationFactory {}
impl ::core::clone::Clone for ICompositionAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionAnimationGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionAnimationGroup {}
impl ::core::clone::Clone for ICompositionAnimationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionBackdropBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionBackdropBrush {}
impl ::core::clone::Clone for ICompositionBackdropBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionBatchCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionBatchCompletedEventArgs {}
impl ::core::clone::Clone for ICompositionBatchCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionBrush {}
impl ::core::clone::Clone for ICompositionBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionBrushFactory {}
impl ::core::clone::Clone for ICompositionBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionCapabilities {}
impl ::core::clone::Clone for ICompositionCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionCapabilitiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionCapabilitiesStatics {}
impl ::core::clone::Clone for ICompositionCapabilitiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionClip {}
impl ::core::clone::Clone for ICompositionClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionClip2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionClip2 {}
impl ::core::clone::Clone for ICompositionClip2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionClipFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionClipFactory {}
impl ::core::clone::Clone for ICompositionClipFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionColorBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionColorBrush {}
impl ::core::clone::Clone for ICompositionColorBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionColorGradientStop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionColorGradientStop {}
impl ::core::clone::Clone for ICompositionColorGradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionColorGradientStopCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionColorGradientStopCollection {}
impl ::core::clone::Clone for ICompositionColorGradientStopCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionCommitBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionCommitBatch {}
impl ::core::clone::Clone for ICompositionCommitBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionContainerShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionContainerShape {}
impl ::core::clone::Clone for ICompositionContainerShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionDrawingSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionDrawingSurface {}
impl ::core::clone::Clone for ICompositionDrawingSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionDrawingSurface2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionDrawingSurface2 {}
impl ::core::clone::Clone for ICompositionDrawingSurface2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionDrawingSurfaceFactory {}
impl ::core::clone::Clone for ICompositionDrawingSurfaceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEasingFunction {}
impl ::core::clone::Clone for ICompositionEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEasingFunctionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEasingFunctionFactory {}
impl ::core::clone::Clone for ICompositionEasingFunctionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEasingFunctionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEasingFunctionStatics {}
impl ::core::clone::Clone for ICompositionEasingFunctionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEffectBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEffectBrush {}
impl ::core::clone::Clone for ICompositionEffectBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEffectFactory {}
impl ::core::clone::Clone for ICompositionEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEffectSourceParameter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEffectSourceParameter {}
impl ::core::clone::Clone for ICompositionEffectSourceParameter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEffectSourceParameterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEffectSourceParameterFactory {}
impl ::core::clone::Clone for ICompositionEffectSourceParameterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionEllipseGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionEllipseGeometry {}
impl ::core::clone::Clone for ICompositionEllipseGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGeometricClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGeometricClip {}
impl ::core::clone::Clone for ICompositionGeometricClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGeometry {}
impl ::core::clone::Clone for ICompositionGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGeometryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGeometryFactory {}
impl ::core::clone::Clone for ICompositionGeometryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGradientBrush {}
impl ::core::clone::Clone for ICompositionGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGradientBrush2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGradientBrush2 {}
impl ::core::clone::Clone for ICompositionGradientBrush2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGradientBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGradientBrushFactory {}
impl ::core::clone::Clone for ICompositionGradientBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGraphicsDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGraphicsDevice {}
impl ::core::clone::Clone for ICompositionGraphicsDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGraphicsDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGraphicsDevice2 {}
impl ::core::clone::Clone for ICompositionGraphicsDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGraphicsDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGraphicsDevice3 {}
impl ::core::clone::Clone for ICompositionGraphicsDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGraphicsDevice4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGraphicsDevice4 {}
impl ::core::clone::Clone for ICompositionGraphicsDevice4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLight {}
impl ::core::clone::Clone for ICompositionLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLight2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLight2 {}
impl ::core::clone::Clone for ICompositionLight2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLight3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLight3 {}
impl ::core::clone::Clone for ICompositionLight3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLightFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLightFactory {}
impl ::core::clone::Clone for ICompositionLightFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLineGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLineGeometry {}
impl ::core::clone::Clone for ICompositionLineGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionLinearGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionLinearGradientBrush {}
impl ::core::clone::Clone for ICompositionLinearGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionMaskBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionMaskBrush {}
impl ::core::clone::Clone for ICompositionMaskBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionMipmapSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionMipmapSurface {}
impl ::core::clone::Clone for ICompositionMipmapSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionNineGridBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionNineGridBrush {}
impl ::core::clone::Clone for ICompositionNineGridBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObject {}
impl ::core::clone::Clone for ICompositionObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObject2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObject2 {}
impl ::core::clone::Clone for ICompositionObject2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObject3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObject3 {}
impl ::core::clone::Clone for ICompositionObject3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObject4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObject4 {}
impl ::core::clone::Clone for ICompositionObject4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObjectFactory {}
impl ::core::clone::Clone for ICompositionObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionObjectStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionObjectStatics {}
impl ::core::clone::Clone for ICompositionObjectStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionPath {}
impl ::core::clone::Clone for ICompositionPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionPathFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionPathFactory {}
impl ::core::clone::Clone for ICompositionPathFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionPathGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionPathGeometry {}
impl ::core::clone::Clone for ICompositionPathGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadow {}
impl ::core::clone::Clone for ICompositionProjectedShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadowCaster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadowCaster {}
impl ::core::clone::Clone for ICompositionProjectedShadowCaster {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadowCasterCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadowCasterCollection {}
impl ::core::clone::Clone for ICompositionProjectedShadowCasterCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadowCasterCollectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadowCasterCollectionStatics {}
impl ::core::clone::Clone for ICompositionProjectedShadowCasterCollectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadowReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadowReceiver {}
impl ::core::clone::Clone for ICompositionProjectedShadowReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionProjectedShadowReceiverUnorderedCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionProjectedShadowReceiverUnorderedCollection {}
impl ::core::clone::Clone for ICompositionProjectedShadowReceiverUnorderedCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionPropertySet {}
impl ::core::clone::Clone for ICompositionPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionPropertySet2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionPropertySet2 {}
impl ::core::clone::Clone for ICompositionPropertySet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionRadialGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionRadialGradientBrush {}
impl ::core::clone::Clone for ICompositionRadialGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionRectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionRectangleGeometry {}
impl ::core::clone::Clone for ICompositionRectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionRoundedRectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionRoundedRectangleGeometry {}
impl ::core::clone::Clone for ICompositionRoundedRectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionScopedBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionScopedBatch {}
impl ::core::clone::Clone for ICompositionScopedBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionShadow {}
impl ::core::clone::Clone for ICompositionShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionShadowFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionShadowFactory {}
impl ::core::clone::Clone for ICompositionShadowFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionShape {}
impl ::core::clone::Clone for ICompositionShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionShapeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionShapeFactory {}
impl ::core::clone::Clone for ICompositionShapeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSpriteShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSpriteShape {}
impl ::core::clone::Clone for ICompositionSpriteShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSupportsSystemBackdrop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSupportsSystemBackdrop {}
impl ::core::clone::Clone for ICompositionSupportsSystemBackdrop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSurface {}
impl ::core::clone::Clone for ICompositionSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSurfaceBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSurfaceBrush {}
impl ::core::clone::Clone for ICompositionSurfaceBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSurfaceBrush2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSurfaceBrush2 {}
impl ::core::clone::Clone for ICompositionSurfaceBrush2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSurfaceBrush3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSurfaceBrush3 {}
impl ::core::clone::Clone for ICompositionSurfaceBrush3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionSurfaceFacade(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionSurfaceFacade {}
impl ::core::clone::Clone for ICompositionSurfaceFacade {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTarget {}
impl ::core::clone::Clone for ICompositionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionTargetFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTargetFactory {}
impl ::core::clone::Clone for ICompositionTargetFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTransform {}
impl ::core::clone::Clone for ICompositionTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionTransformFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTransformFactory {}
impl ::core::clone::Clone for ICompositionTransformFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionViewBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionViewBox {}
impl ::core::clone::Clone for ICompositionViewBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionVirtualDrawingSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionVirtualDrawingSurface {}
impl ::core::clone::Clone for ICompositionVirtualDrawingSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionVirtualDrawingSurfaceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionVirtualDrawingSurfaceFactory {}
impl ::core::clone::Clone for ICompositionVirtualDrawingSurfaceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionVisualSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionVisualSurface {}
impl ::core::clone::Clone for ICompositionVisualSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor {}
impl ::core::clone::Clone for ICompositor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor2 {}
impl ::core::clone::Clone for ICompositor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor3 {}
impl ::core::clone::Clone for ICompositor3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor4 {}
impl ::core::clone::Clone for ICompositor4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor5 {}
impl ::core::clone::Clone for ICompositor5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor6 {}
impl ::core::clone::Clone for ICompositor6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositor7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositor7 {}
impl ::core::clone::Clone for ICompositor7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorStatics {}
impl ::core::clone::Clone for ICompositorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorWithBlurredWallpaperBackdropBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorWithBlurredWallpaperBackdropBrush {}
impl ::core::clone::Clone for ICompositorWithBlurredWallpaperBackdropBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorWithProjectedShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorWithProjectedShadow {}
impl ::core::clone::Clone for ICompositorWithProjectedShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorWithRadialGradient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorWithRadialGradient {}
impl ::core::clone::Clone for ICompositorWithRadialGradient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorWithVisualSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorWithVisualSurface {}
impl ::core::clone::Clone for ICompositorWithVisualSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContainerVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContainerVisual {}
impl ::core::clone::Clone for IContainerVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContainerVisualFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContainerVisualFactory {}
impl ::core::clone::Clone for IContainerVisualFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICubicBezierEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICubicBezierEasingFunction {}
impl ::core::clone::Clone for ICubicBezierEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDelegatedInkTrailVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDelegatedInkTrailVisual {}
impl ::core::clone::Clone for IDelegatedInkTrailVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDelegatedInkTrailVisualStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDelegatedInkTrailVisualStatics {}
impl ::core::clone::Clone for IDelegatedInkTrailVisualStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDistantLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDistantLight {}
impl ::core::clone::Clone for IDistantLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDistantLight2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDistantLight2 {}
impl ::core::clone::Clone for IDistantLight2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropShadow {}
impl ::core::clone::Clone for IDropShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropShadow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropShadow2 {}
impl ::core::clone::Clone for IDropShadow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElasticEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElasticEasingFunction {}
impl ::core::clone::Clone for IElasticEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExponentialEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExponentialEasingFunction {}
impl ::core::clone::Clone for IExponentialEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExpressionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExpressionAnimation {}
impl ::core::clone::Clone for IExpressionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImplicitAnimationCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImplicitAnimationCollection {}
impl ::core::clone::Clone for IImplicitAnimationCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInsetClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInsetClip {}
impl ::core::clone::Clone for IInsetClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyFrameAnimation {}
impl ::core::clone::Clone for IKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyFrameAnimation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyFrameAnimation2 {}
impl ::core::clone::Clone for IKeyFrameAnimation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyFrameAnimation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyFrameAnimation3 {}
impl ::core::clone::Clone for IKeyFrameAnimation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyFrameAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyFrameAnimationFactory {}
impl ::core::clone::Clone for IKeyFrameAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILayerVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILayerVisual {}
impl ::core::clone::Clone for ILayerVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILayerVisual2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILayerVisual2 {}
impl ::core::clone::Clone for ILayerVisual2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearEasingFunction {}
impl ::core::clone::Clone for ILinearEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INaturalMotionAnimation {}
impl ::core::clone::Clone for INaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INaturalMotionAnimationFactory {}
impl ::core::clone::Clone for INaturalMotionAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathKeyFrameAnimation {}
impl ::core::clone::Clone for IPathKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointLight {}
impl ::core::clone::Clone for IPointLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointLight2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointLight2 {}
impl ::core::clone::Clone for IPointLight2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointLight3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointLight3 {}
impl ::core::clone::Clone for IPointLight3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPowerEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPowerEasingFunction {}
impl ::core::clone::Clone for IPowerEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuaternionKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuaternionKeyFrameAnimation {}
impl ::core::clone::Clone for IQuaternionKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectangleClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectangleClip {}
impl ::core::clone::Clone for IRectangleClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRedirectVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRedirectVisual {}
impl ::core::clone::Clone for IRedirectVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRenderingDeviceReplacedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRenderingDeviceReplacedEventArgs {}
impl ::core::clone::Clone for IRenderingDeviceReplacedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScalarKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScalarKeyFrameAnimation {}
impl ::core::clone::Clone for IScalarKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for IScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScalarNaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScalarNaturalMotionAnimationFactory {}
impl ::core::clone::Clone for IScalarNaturalMotionAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShapeVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShapeVisual {}
impl ::core::clone::Clone for IShapeVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISineEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISineEasingFunction {}
impl ::core::clone::Clone for ISineEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpotLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpotLight {}
impl ::core::clone::Clone for ISpotLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpotLight2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpotLight2 {}
impl ::core::clone::Clone for ISpotLight2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpotLight3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpotLight3 {}
impl ::core::clone::Clone for ISpotLight3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpringScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpringScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for ISpringScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpringVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpringVector2NaturalMotionAnimation {}
impl ::core::clone::Clone for ISpringVector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpringVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpringVector3NaturalMotionAnimation {}
impl ::core::clone::Clone for ISpringVector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpriteVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpriteVisual {}
impl ::core::clone::Clone for ISpriteVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpriteVisual2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpriteVisual2 {}
impl ::core::clone::Clone for ISpriteVisual2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStepEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStepEasingFunction {}
impl ::core::clone::Clone for IStepEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector2KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector2KeyFrameAnimation {}
impl ::core::clone::Clone for IVector2KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector2NaturalMotionAnimation {}
impl ::core::clone::Clone for IVector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector2NaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector2NaturalMotionAnimationFactory {}
impl ::core::clone::Clone for IVector2NaturalMotionAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector3KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector3KeyFrameAnimation {}
impl ::core::clone::Clone for IVector3KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector3NaturalMotionAnimation {}
impl ::core::clone::Clone for IVector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector3NaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector3NaturalMotionAnimationFactory {}
impl ::core::clone::Clone for IVector3NaturalMotionAnimationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector4KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector4KeyFrameAnimation {}
impl ::core::clone::Clone for IVector4KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisual {}
impl ::core::clone::Clone for IVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisual2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisual2 {}
impl ::core::clone::Clone for IVisual2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisual3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisual3 {}
impl ::core::clone::Clone for IVisual3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisual4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisual4 {}
impl ::core::clone::Clone for IVisual4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualCollection {}
impl ::core::clone::Clone for IVisualCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualElement {}
impl ::core::clone::Clone for IVisualElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualElement2 {}
impl ::core::clone::Clone for IVisualElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualFactory {}
impl ::core::clone::Clone for IVisualFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualUnorderedCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualUnorderedCollection {}
impl ::core::clone::Clone for IVisualUnorderedCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImplicitAnimationCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImplicitAnimationCollection {}
impl ::core::clone::Clone for ImplicitAnimationCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InitialValueExpressionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InitialValueExpressionCollection {}
impl ::core::clone::Clone for InitialValueExpressionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct InkTrailPoint {
    pub Point: super::super::Foundation::Point,
    pub Radius: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for InkTrailPoint {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for InkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InsetClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InsetClip {}
impl ::core::clone::Clone for InsetClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyFrameAnimation {}
impl ::core::clone::Clone for KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LayerVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LayerVisual {}
impl ::core::clone::Clone for LayerVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LinearEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LinearEasingFunction {}
impl ::core::clone::Clone for LinearEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NaturalMotionAnimation {}
impl ::core::clone::Clone for NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathKeyFrameAnimation {}
impl ::core::clone::Clone for PathKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointLight {}
impl ::core::clone::Clone for PointLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PowerEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PowerEasingFunction {}
impl ::core::clone::Clone for PowerEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuaternionKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuaternionKeyFrameAnimation {}
impl ::core::clone::Clone for QuaternionKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RectangleClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RectangleClip {}
impl ::core::clone::Clone for RectangleClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RedirectVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RedirectVisual {}
impl ::core::clone::Clone for RedirectVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RenderingDeviceReplacedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RenderingDeviceReplacedEventArgs {}
impl ::core::clone::Clone for RenderingDeviceReplacedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScalarKeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScalarKeyFrameAnimation {}
impl ::core::clone::Clone for ScalarKeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for ScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShapeVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShapeVisual {}
impl ::core::clone::Clone for ShapeVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SineEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SineEasingFunction {}
impl ::core::clone::Clone for SineEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpotLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpotLight {}
impl ::core::clone::Clone for SpotLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpringScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpringScalarNaturalMotionAnimation {}
impl ::core::clone::Clone for SpringScalarNaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpringVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpringVector2NaturalMotionAnimation {}
impl ::core::clone::Clone for SpringVector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpringVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpringVector3NaturalMotionAnimation {}
impl ::core::clone::Clone for SpringVector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpriteVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpriteVisual {}
impl ::core::clone::Clone for SpriteVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StepEasingFunction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StepEasingFunction {}
impl ::core::clone::Clone for StepEasingFunction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector2KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector2KeyFrameAnimation {}
impl ::core::clone::Clone for Vector2KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector2NaturalMotionAnimation {}
impl ::core::clone::Clone for Vector2NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector3KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector3KeyFrameAnimation {}
impl ::core::clone::Clone for Vector3KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector3NaturalMotionAnimation {}
impl ::core::clone::Clone for Vector3NaturalMotionAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector4KeyFrameAnimation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector4KeyFrameAnimation {}
impl ::core::clone::Clone for Vector4KeyFrameAnimation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Visual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Visual {}
impl ::core::clone::Clone for Visual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualCollection {}
impl ::core::clone::Clone for VisualCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualUnorderedCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualUnorderedCollection {}
impl ::core::clone::Clone for VisualUnorderedCollection {
    fn clone(&self) -> Self {
        *self
    }
}

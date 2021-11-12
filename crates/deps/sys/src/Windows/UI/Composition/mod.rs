#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(transparent)]
pub struct AnimationController(pub *mut ::core::ffi::c_void);
pub struct AnimationControllerProgressBehavior(i32);
pub struct AnimationDelayBehavior(i32);
pub struct AnimationDirection(i32);
pub struct AnimationIterationBehavior(i32);
pub struct AnimationPropertyAccessMode(i32);
#[repr(transparent)]
pub struct AnimationPropertyInfo(pub *mut ::core::ffi::c_void);
pub struct AnimationStopBehavior(i32);
#[repr(transparent)]
pub struct BackEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BooleanKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BounceEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BounceScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BounceVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BounceVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CircleEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionAnimationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionBackdropBrush(pub *mut ::core::ffi::c_void);
pub struct CompositionBackfaceVisibility(i32);
#[repr(transparent)]
pub struct CompositionBatchCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct CompositionBatchTypes(i32);
pub struct CompositionBitmapInterpolationMode(i32);
pub struct CompositionBorderMode(i32);
#[repr(transparent)]
pub struct CompositionBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionColorGradientStop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionColorGradientStopCollection(pub *mut ::core::ffi::c_void);
pub struct CompositionColorSpace(i32);
#[repr(transparent)]
pub struct CompositionCommitBatch(pub *mut ::core::ffi::c_void);
pub struct CompositionCompositeMode(i32);
#[repr(transparent)]
pub struct CompositionContainerShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionDrawingSurface(pub *mut ::core::ffi::c_void);
pub struct CompositionDropShadowSourcePolicy(i32);
#[repr(transparent)]
pub struct CompositionEasingFunction(pub *mut ::core::ffi::c_void);
pub struct CompositionEasingFunctionMode(i32);
#[repr(transparent)]
pub struct CompositionEffectBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionEffectFactory(pub *mut ::core::ffi::c_void);
pub struct CompositionEffectFactoryLoadStatus(i32);
#[repr(transparent)]
pub struct CompositionEffectSourceParameter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionEllipseGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionGeometricClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionGeometry(pub *mut ::core::ffi::c_void);
pub struct CompositionGetValueStatus(i32);
#[repr(transparent)]
pub struct CompositionGradientBrush(pub *mut ::core::ffi::c_void);
pub struct CompositionGradientExtendMode(i32);
#[repr(transparent)]
pub struct CompositionGraphicsDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionLineGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionLinearGradientBrush(pub *mut ::core::ffi::c_void);
pub struct CompositionMappingMode(i32);
#[repr(transparent)]
pub struct CompositionMaskBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionMipmapSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionNineGridBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionPathGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionProjectedShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionProjectedShadowCaster(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionProjectedShadowCasterCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionProjectedShadowReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionProjectedShadowReceiverUnorderedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionRadialGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionRoundedRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionScopedBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionShapeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionSpriteShape(pub *mut ::core::ffi::c_void);
pub struct CompositionStretch(i32);
pub struct CompositionStrokeCap(i32);
#[repr(transparent)]
pub struct CompositionStrokeDashArray(pub *mut ::core::ffi::c_void);
pub struct CompositionStrokeLineJoin(i32);
#[repr(transparent)]
pub struct CompositionSurfaceBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionViewBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionVirtualDrawingSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionVisualSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Compositor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContainerVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CubicBezierEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DelegatedInkTrailVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DistantLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElasticEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExponentialEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExpressionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAmbientLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAmbientLight2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationPropertyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationPropertyInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBooleanKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBounceVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICircleEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimationBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionAnimationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionBackdropBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionBatchCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionCapabilitiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionClip2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionClipFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionColorGradientStop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionColorGradientStopCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionCommitBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionContainerShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDrawingSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDrawingSurface2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEasingFunctionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEasingFunctionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEffectBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEffectSourceParameter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEffectSourceParameterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionEllipseGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGeometricClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGeometryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGradientBrush2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGradientBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGraphicsDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGraphicsDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGraphicsDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGraphicsDevice4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLight2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLight3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLightFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLineGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionLinearGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionMaskBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionMipmapSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionNineGridBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObject3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObject4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionObjectStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionPathFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionPathGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadowCaster(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadowCasterCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadowCasterCollectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadowReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionProjectedShadowReceiverUnorderedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionPropertySet2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionRadialGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionRoundedRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionScopedBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionShadowFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionShapeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSpriteShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSupportsSystemBackdrop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSurfaceBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSurfaceBrush2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSurfaceBrush3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionSurfaceFacade(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTargetFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTransformFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionViewBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionVirtualDrawingSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionVirtualDrawingSurfaceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionVisualSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositor7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorWithBlurredWallpaperBackdropBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorWithProjectedShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorWithRadialGradient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorWithVisualSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContainerVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContainerVisualFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICubicBezierEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDelegatedInkTrailVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDelegatedInkTrailVisualStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDistantLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDistantLight2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropShadow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElasticEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExponentialEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExpressionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImplicitAnimationCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInsetClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyFrameAnimation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyFrameAnimation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyFrameAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayerVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayerVisual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointLight2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointLight3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuaternionKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectangleClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRedirectVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRenderingDeviceReplacedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScalarKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScalarNaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShapeVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISineEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpotLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpotLight2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpotLight3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpringScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpringVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpringVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpriteVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpriteVisual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStepEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector2KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector2NaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector3KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector3NaturalMotionAnimationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector4KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisual3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisual4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualUnorderedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImplicitAnimationCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InitialValueExpressionCollection(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
pub struct InkTrailPoint(i32);
#[repr(transparent)]
pub struct InsetClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LayerVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LinearEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuaternionKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RectangleClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RedirectVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RenderingDeviceReplacedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScalarKeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShapeVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SineEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpotLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpringScalarNaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpringVector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpringVector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpriteVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StepEasingFunction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector2KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector2NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector3KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector3NaturalMotionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector4KeyFrameAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Visual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualUnorderedCollection(pub *mut ::core::ffi::c_void);

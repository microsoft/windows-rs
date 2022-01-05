#[cfg(feature = "implement_exclusive")]
pub trait IAmbientLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAmbientLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationControllerImpl: Sized {
    fn PlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn SetPlaybackRate(&self, value: f32) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<f32>;
    fn SetProgress(&self, value: f32) -> ::windows::core::Result<()>;
    fn ProgressBehavior(&self) -> ::windows::core::Result<AnimationControllerProgressBehavior>;
    fn SetProgressBehavior(&self, value: AnimationControllerProgressBehavior) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationControllerStaticsImpl: Sized {
    fn MaxPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn MinPlaybackRate(&self) -> ::windows::core::Result<f32>;
}
pub trait IAnimationObjectImpl: Sized {
    fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationPropertyInfoImpl: Sized {
    fn AccessMode(&self) -> ::windows::core::Result<AnimationPropertyAccessMode>;
    fn SetAccessMode(&self, value: AnimationPropertyAccessMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationPropertyInfo2Impl: Sized {
    fn GetResolvedCompositionObject(&self) -> ::windows::core::Result<CompositionObject>;
    fn GetResolvedCompositionObjectProperty(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Amplitude(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBooleanKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Bounces(&self) -> ::windows::core::Result<i32>;
    fn Bounciness(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceScalarNaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceVector2NaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBounceVector3NaturalMotionAnimationImpl: Sized {
    fn Acceleration(&self) -> ::windows::core::Result<f32>;
    fn SetAcceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn Restitution(&self) -> ::windows::core::Result<f32>;
    fn SetRestitution(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICircleEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorKeyFrameAnimationImpl: Sized {
    fn InterpolationColorSpace(&self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationColorSpace(&self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::Color, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationImpl: Sized {
    fn ClearAllParameters(&self) -> ::windows::core::Result<()>;
    fn ClearParameter(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetColorParameter(&self, key: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn SetMatrix3x2Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn SetMatrix4x4Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn SetQuaternionParameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SetReferenceParameter(&self, key: &::windows::core::HSTRING, compositionobject: &::core::option::Option<CompositionObject>) -> ::windows::core::Result<()>;
    fn SetScalarParameter(&self, key: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn SetVector2Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SetVector3Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetVector4Parameter(&self, key: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimation2Impl: Sized {
    fn SetBooleanParameter(&self, key: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTarget(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimation3Impl: Sized {
    fn InitialValueExpressions(&self) -> ::windows::core::Result<InitialValueExpressionCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimation4Impl: Sized {
    fn SetExpressionReferenceParameter(&self, parametername: &::windows::core::HSTRING, source: &::core::option::Option<IAnimationObject>) -> ::windows::core::Result<()>;
}
pub trait ICompositionAnimationBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionAnimationGroupImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn Remove(&self, value: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBackdropBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBatchCompletedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionBrushFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionCapabilitiesImpl: Sized {
    fn AreEffectsSupported(&self) -> ::windows::core::Result<bool>;
    fn AreEffectsFast(&self) -> ::windows::core::Result<bool>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionCapabilities, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionCapabilitiesStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CompositionCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionClipImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionClip2Impl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionClipFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorGradientStopImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<f32>;
    fn SetOffset(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionColorGradientStopCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionCommitBatchImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsEnded(&self) -> ::windows::core::Result<bool>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionContainerShapeImpl: Sized {
    fn Shapes(&self) -> ::windows::core::Result<CompositionShapeCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDrawingSurfaceImpl: Sized {
    fn AlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDrawingSurface2Impl: Sized {
    fn SizeInt32(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Resize(&self, sizepixels: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Scroll(&self, offset: &super::super::Graphics::PointInt32) -> ::windows::core::Result<()>;
    fn ScrollRect(&self, offset: &super::super::Graphics::PointInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollWithClip(&self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn ScrollRectWithClip(&self, offset: &super::super::Graphics::PointInt32, cliprect: &super::super::Graphics::RectInt32, scrollrect: &super::super::Graphics::RectInt32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDrawingSurfaceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEasingFunctionStaticsImpl: Sized {
    fn CreateCubicBezierEasingFunction(&self, owner: &::core::option::Option<Compositor>, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateLinearEasingFunction(&self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreateStepEasingFunction(&self, owner: &::core::option::Option<Compositor>) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&self, owner: &::core::option::Option<Compositor>, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateBackEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, amplitude: f32) -> ::windows::core::Result<BackEasingFunction>;
    fn CreateBounceEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, bounces: i32, bounciness: f32) -> ::windows::core::Result<BounceEasingFunction>;
    fn CreateCircleEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<CircleEasingFunction>;
    fn CreateElasticEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, oscillations: i32, springiness: f32) -> ::windows::core::Result<ElasticEasingFunction>;
    fn CreateExponentialEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, exponent: f32) -> ::windows::core::Result<ExponentialEasingFunction>;
    fn CreatePowerEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode, power: f32) -> ::windows::core::Result<PowerEasingFunction>;
    fn CreateSineEasingFunction(&self, owner: &::core::option::Option<Compositor>, mode: CompositionEasingFunctionMode) -> ::windows::core::Result<SineEasingFunction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectBrushImpl: Sized {
    fn GetSourceParameter(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionBrush>;
    fn SetSourceParameter(&self, name: &::windows::core::HSTRING, source: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectFactoryImpl: Sized {
    fn CreateBrush(&self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn LoadStatus(&self) -> ::windows::core::Result<CompositionEffectFactoryLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectSourceParameterImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEffectSourceParameterFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<CompositionEffectSourceParameter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionEllipseGeometryImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenter(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometricClipImpl: Sized {
    fn Geometry(&self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn ViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometryImpl: Sized {
    fn TrimEnd(&self) -> ::windows::core::Result<f32>;
    fn SetTrimEnd(&self, value: f32) -> ::windows::core::Result<()>;
    fn TrimOffset(&self) -> ::windows::core::Result<f32>;
    fn SetTrimOffset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TrimStart(&self) -> ::windows::core::Result<f32>;
    fn SetTrimStart(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGeometryFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGradientBrushImpl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ColorStops(&self) -> ::windows::core::Result<CompositionColorGradientStopCollection>;
    fn ExtendMode(&self) -> ::windows::core::Result<CompositionGradientExtendMode>;
    fn SetExtendMode(&self, value: CompositionGradientExtendMode) -> ::windows::core::Result<()>;
    fn InterpolationSpace(&self) -> ::windows::core::Result<CompositionColorSpace>;
    fn SetInterpolationSpace(&self, value: CompositionColorSpace) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGradientBrush2Impl: Sized {
    fn MappingMode(&self) -> ::windows::core::Result<CompositionMappingMode>;
    fn SetMappingMode(&self, value: CompositionMappingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGradientBrushFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGraphicsDeviceImpl: Sized {
    fn CreateDrawingSurface(&self, sizepixels: &super::super::Foundation::Size, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn RenderingDeviceReplaced(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CompositionGraphicsDevice, RenderingDeviceReplacedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRenderingDeviceReplaced(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGraphicsDevice2Impl: Sized {
    fn CreateDrawingSurface2(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionDrawingSurface>;
    fn CreateVirtualDrawingSurface(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionVirtualDrawingSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGraphicsDevice3Impl: Sized {
    fn CreateMipmapSurface(&self, sizepixels: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode) -> ::windows::core::Result<CompositionMipmapSurface>;
    fn Trim(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionGraphicsDevice4Impl: Sized {
    fn CaptureAsync(&self, capturevisual: &::core::option::Option<Visual>, size: &super::super::Graphics::SizeInt32, pixelformat: super::super::Graphics::DirectX::DirectXPixelFormat, alphamode: super::super::Graphics::DirectX::DirectXAlphaMode, sdrboost: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ICompositionSurface>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLightImpl: Sized {
    fn Targets(&self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLight2Impl: Sized {
    fn ExclusionsFromTargets(&self) -> ::windows::core::Result<VisualUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLight3Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLightFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLineGeometryImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStart(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEnd(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionLinearGradientBrushImpl: Sized {
    fn EndPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEndPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn StartPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetStartPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionMaskBrushImpl: Sized {
    fn Mask(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionMipmapSurfaceImpl: Sized {
    fn LevelCount(&self) -> ::windows::core::Result<u32>;
    fn AlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXAlphaMode>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SizeInt32(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn GetDrawingSurfaceForLevel(&self, level: u32) -> ::windows::core::Result<CompositionDrawingSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionNineGridBrushImpl: Sized {
    fn BottomInset(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn IsCenterHollow(&self) -> ::windows::core::Result<bool>;
    fn SetIsCenterHollow(&self, value: bool) -> ::windows::core::Result<()>;
    fn LeftInset(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetRightInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSource(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn TopInset(&self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopInsetScale(&self) -> ::windows::core::Result<f32>;
    fn SetTopInsetScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn SetInsets(&self, inset: f32) -> ::windows::core::Result<()>;
    fn SetInsetsWithValues(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
    fn SetInsetScales(&self, scale: f32) -> ::windows::core::Result<()>;
    fn SetInsetScalesWithValues(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectImpl: Sized {
    fn Compositor(&self) -> ::windows::core::Result<Compositor>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Properties(&self) -> ::windows::core::Result<CompositionPropertySet>;
    fn StartAnimation(&self, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObject2Impl: Sized {
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ImplicitAnimations(&self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn SetImplicitAnimations(&self, value: &::core::option::Option<ImplicitAnimationCollection>) -> ::windows::core::Result<()>;
    fn StartAnimationGroup(&self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimationGroup(&self, value: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObject3Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObject4Impl: Sized {
    fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<AnimationController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionObjectStaticsImpl: Sized {
    fn StartAnimationWithIAnimationObject(&self, target: &::core::option::Option<IAnimationObject>, propertyname: &::windows::core::HSTRING, animation: &::core::option::Option<CompositionAnimation>) -> ::windows::core::Result<()>;
    fn StartAnimationGroupWithIAnimationObject(&self, target: &::core::option::Option<IAnimationObject>, animation: &::core::option::Option<ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPathImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPathFactoryImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::super::Graphics::IGeometrySource2D>) -> ::windows::core::Result<CompositionPath>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPathGeometryImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<CompositionPath>;
    fn SetPath(&self, value: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowImpl: Sized {
    fn BlurRadiusMultiplier(&self) -> ::windows::core::Result<f32>;
    fn SetBlurRadiusMultiplier(&self, value: f32) -> ::windows::core::Result<()>;
    fn Casters(&self) -> ::windows::core::Result<CompositionProjectedShadowCasterCollection>;
    fn LightSource(&self) -> ::windows::core::Result<CompositionLight>;
    fn SetLightSource(&self, value: &::core::option::Option<CompositionLight>) -> ::windows::core::Result<()>;
    fn MaxBlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetMaxBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn MinBlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetMinBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn Receivers(&self) -> ::windows::core::Result<CompositionProjectedShadowReceiverUnorderedCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterImpl: Sized {
    fn Brush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn CastingVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetCastingVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn InsertBelow(&self, newcaster: &::core::option::Option<CompositionProjectedShadowCaster>, reference: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn Remove(&self, caster: &::core::option::Option<CompositionProjectedShadowCaster>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowCasterCollectionStaticsImpl: Sized {
    fn MaxRespectedCasters(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowReceiverImpl: Sized {
    fn ReceivingVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetReceivingVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionProjectedShadowReceiverUnorderedCollectionImpl: Sized {
    fn Add(&self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Remove(&self, value: &::core::option::Option<CompositionProjectedShadowReceiver>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPropertySetImpl: Sized {
    fn InsertColor(&self, propertyname: &::windows::core::HSTRING, value: &super::Color) -> ::windows::core::Result<()>;
    fn InsertMatrix3x2(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn InsertMatrix4x4(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn InsertQuaternion(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertScalar(&self, propertyname: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn InsertVector2(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertVector3(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertVector4(&self, propertyname: &::windows::core::HSTRING, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn TryGetColor(&self, propertyname: &::windows::core::HSTRING, value: &mut super::Color) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix3x2(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetMatrix4x4(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetQuaternion(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetScalar(&self, propertyname: &::windows::core::HSTRING, value: &mut f32) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector2(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector3(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<CompositionGetValueStatus>;
    fn TryGetVector4(&self, propertyname: &::windows::core::HSTRING, value: &mut super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionPropertySet2Impl: Sized {
    fn InsertBoolean(&self, propertyname: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn TryGetBoolean(&self, propertyname: &::windows::core::HSTRING, value: &mut bool) -> ::windows::core::Result<CompositionGetValueStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionRadialGradientBrushImpl: Sized {
    fn EllipseCenter(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseCenter(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn EllipseRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetEllipseRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn GradientOriginOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetGradientOriginOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionRectangleGeometryImpl: Sized {
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionRoundedRectangleGeometryImpl: Sized {
    fn CornerRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCornerRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionScopedBatchImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsEnded(&self) -> ::windows::core::Result<bool>;
    fn End(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Suspend(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CompositionBatchCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShadowImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShadowFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShapeImpl: Sized {
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionShapeFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSpriteShapeImpl: Sized {
    fn FillBrush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetFillBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Geometry(&self) -> ::windows::core::Result<CompositionGeometry>;
    fn SetGeometry(&self, value: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<()>;
    fn IsStrokeNonScaling(&self) -> ::windows::core::Result<bool>;
    fn SetIsStrokeNonScaling(&self, value: bool) -> ::windows::core::Result<()>;
    fn StrokeBrush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetStrokeBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn StrokeDashArray(&self) -> ::windows::core::Result<CompositionStrokeDashArray>;
    fn StrokeDashCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeDashCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeDashOffset(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeDashOffset(&self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeEndCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeEndCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeLineJoin(&self) -> ::windows::core::Result<CompositionStrokeLineJoin>;
    fn SetStrokeLineJoin(&self, value: CompositionStrokeLineJoin) -> ::windows::core::Result<()>;
    fn StrokeMiterLimit(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeMiterLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn StrokeStartCap(&self) -> ::windows::core::Result<CompositionStrokeCap>;
    fn SetStrokeStartCap(&self, value: CompositionStrokeCap) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f32>;
    fn SetStrokeThickness(&self, value: f32) -> ::windows::core::Result<()>;
}
pub trait ICompositionSupportsSystemBackdropImpl: Sized {
    fn SystemBackdrop(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
pub trait ICompositionSurfaceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrushImpl: Sized {
    fn BitmapInterpolationMode(&self) -> ::windows::core::Result<CompositionBitmapInterpolationMode>;
    fn SetBitmapInterpolationMode(&self, value: CompositionBitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn Surface(&self) -> ::windows::core::Result<ICompositionSurface>;
    fn SetSurface(&self, value: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrush2Impl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionSurfaceBrush3Impl: Sized {
    fn SnapToPixels(&self) -> ::windows::core::Result<bool>;
    fn SetSnapToPixels(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait ICompositionSurfaceFacadeImpl: Sized {
    fn GetRealSurface(&self) -> ::windows::core::Result<ICompositionSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetImpl: Sized {
    fn Root(&self) -> ::windows::core::Result<Visual>;
    fn SetRoot(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransformImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTransformFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionViewBoxImpl: Sized {
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetHorizontalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<CompositionStretch>;
    fn SetStretch(&self, value: CompositionStretch) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f32>;
    fn SetVerticalAlignmentRatio(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionVirtualDrawingSurfaceImpl: Sized {
    fn Trim(&self, rects: &[<super::super::Graphics::RectInt32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionVirtualDrawingSurfaceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionVisualSurfaceImpl: Sized {
    fn SourceVisual(&self) -> ::windows::core::Result<Visual>;
    fn SetSourceVisual(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn SourceOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceOffset(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn SourceSize(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSourceSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorImpl: Sized {
    fn CreateColorKeyFrameAnimation(&self) -> ::windows::core::Result<ColorKeyFrameAnimation>;
    fn CreateColorBrush(&self) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateColorBrushWithColor(&self, color: &super::Color) -> ::windows::core::Result<CompositionColorBrush>;
    fn CreateContainerVisual(&self) -> ::windows::core::Result<ContainerVisual>;
    fn CreateCubicBezierEasingFunction(&self, controlpoint1: &super::super::Foundation::Numerics::Vector2, controlpoint2: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<CubicBezierEasingFunction>;
    fn CreateEffectFactory(&self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateEffectFactoryWithProperties(&self, graphicseffect: &::core::option::Option<super::super::Graphics::Effects::IGraphicsEffect>, animatableproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<CompositionEffectFactory>;
    fn CreateExpressionAnimation(&self) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateExpressionAnimationWithExpression(&self, expression: &::windows::core::HSTRING) -> ::windows::core::Result<ExpressionAnimation>;
    fn CreateInsetClip(&self) -> ::windows::core::Result<InsetClip>;
    fn CreateInsetClipWithInsets(&self, leftinset: f32, topinset: f32, rightinset: f32, bottominset: f32) -> ::windows::core::Result<InsetClip>;
    fn CreateLinearEasingFunction(&self) -> ::windows::core::Result<LinearEasingFunction>;
    fn CreatePropertySet(&self) -> ::windows::core::Result<CompositionPropertySet>;
    fn CreateQuaternionKeyFrameAnimation(&self) -> ::windows::core::Result<QuaternionKeyFrameAnimation>;
    fn CreateScalarKeyFrameAnimation(&self) -> ::windows::core::Result<ScalarKeyFrameAnimation>;
    fn CreateScopedBatch(&self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionScopedBatch>;
    fn CreateSpriteVisual(&self) -> ::windows::core::Result<SpriteVisual>;
    fn CreateSurfaceBrush(&self) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateSurfaceBrushWithSurface(&self, surface: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<CompositionSurfaceBrush>;
    fn CreateTargetForCurrentView(&self) -> ::windows::core::Result<CompositionTarget>;
    fn CreateVector2KeyFrameAnimation(&self) -> ::windows::core::Result<Vector2KeyFrameAnimation>;
    fn CreateVector3KeyFrameAnimation(&self) -> ::windows::core::Result<Vector3KeyFrameAnimation>;
    fn CreateVector4KeyFrameAnimation(&self) -> ::windows::core::Result<Vector4KeyFrameAnimation>;
    fn GetCommitBatch(&self, batchtype: CompositionBatchTypes) -> ::windows::core::Result<CompositionCommitBatch>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor2Impl: Sized {
    fn CreateAmbientLight(&self) -> ::windows::core::Result<AmbientLight>;
    fn CreateAnimationGroup(&self) -> ::windows::core::Result<CompositionAnimationGroup>;
    fn CreateBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
    fn CreateDistantLight(&self) -> ::windows::core::Result<DistantLight>;
    fn CreateDropShadow(&self) -> ::windows::core::Result<DropShadow>;
    fn CreateImplicitAnimationCollection(&self) -> ::windows::core::Result<ImplicitAnimationCollection>;
    fn CreateLayerVisual(&self) -> ::windows::core::Result<LayerVisual>;
    fn CreateMaskBrush(&self) -> ::windows::core::Result<CompositionMaskBrush>;
    fn CreateNineGridBrush(&self) -> ::windows::core::Result<CompositionNineGridBrush>;
    fn CreatePointLight(&self) -> ::windows::core::Result<PointLight>;
    fn CreateSpotLight(&self) -> ::windows::core::Result<SpotLight>;
    fn CreateStepEasingFunction(&self) -> ::windows::core::Result<StepEasingFunction>;
    fn CreateStepEasingFunctionWithStepCount(&self, stepcount: i32) -> ::windows::core::Result<StepEasingFunction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor3Impl: Sized {
    fn CreateHostBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor4Impl: Sized {
    fn CreateColorGradientStop(&self) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateColorGradientStopWithOffsetAndColor(&self, offset: f32, color: &super::Color) -> ::windows::core::Result<CompositionColorGradientStop>;
    fn CreateLinearGradientBrush(&self) -> ::windows::core::Result<CompositionLinearGradientBrush>;
    fn CreateSpringScalarAnimation(&self) -> ::windows::core::Result<SpringScalarNaturalMotionAnimation>;
    fn CreateSpringVector2Animation(&self) -> ::windows::core::Result<SpringVector2NaturalMotionAnimation>;
    fn CreateSpringVector3Animation(&self) -> ::windows::core::Result<SpringVector3NaturalMotionAnimation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor5Impl: Sized {
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn SetGlobalPlaybackRate(&self, value: f32) -> ::windows::core::Result<()>;
    fn CreateBounceScalarAnimation(&self) -> ::windows::core::Result<BounceScalarNaturalMotionAnimation>;
    fn CreateBounceVector2Animation(&self) -> ::windows::core::Result<BounceVector2NaturalMotionAnimation>;
    fn CreateBounceVector3Animation(&self) -> ::windows::core::Result<BounceVector3NaturalMotionAnimation>;
    fn CreateContainerShape(&self) -> ::windows::core::Result<CompositionContainerShape>;
    fn CreateEllipseGeometry(&self) -> ::windows::core::Result<CompositionEllipseGeometry>;
    fn CreateLineGeometry(&self) -> ::windows::core::Result<CompositionLineGeometry>;
    fn CreatePathGeometry(&self) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathGeometryWithPath(&self, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<CompositionPathGeometry>;
    fn CreatePathKeyFrameAnimation(&self) -> ::windows::core::Result<PathKeyFrameAnimation>;
    fn CreateRectangleGeometry(&self) -> ::windows::core::Result<CompositionRectangleGeometry>;
    fn CreateRoundedRectangleGeometry(&self) -> ::windows::core::Result<CompositionRoundedRectangleGeometry>;
    fn CreateShapeVisual(&self) -> ::windows::core::Result<ShapeVisual>;
    fn CreateSpriteShape(&self) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateSpriteShapeWithGeometry(&self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionSpriteShape>;
    fn CreateViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn RequestCommitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor6Impl: Sized {
    fn CreateGeometricClip(&self) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateGeometricClipWithGeometry(&self, geometry: &::core::option::Option<CompositionGeometry>) -> ::windows::core::Result<CompositionGeometricClip>;
    fn CreateRedirectVisual(&self) -> ::windows::core::Result<RedirectVisual>;
    fn CreateRedirectVisualWithSourceVisual(&self, source: &::core::option::Option<Visual>) -> ::windows::core::Result<RedirectVisual>;
    fn CreateBooleanKeyFrameAnimation(&self) -> ::windows::core::Result<BooleanKeyFrameAnimation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositor7Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn CreateAnimationPropertyInfo(&self) -> ::windows::core::Result<AnimationPropertyInfo>;
    fn CreateRectangleClip(&self) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSides(&self, left: f32, top: f32, right: f32, bottom: f32) -> ::windows::core::Result<RectangleClip>;
    fn CreateRectangleClipWithSidesAndRadius(&self, left: f32, top: f32, right: f32, bottom: f32, topleftradius: &super::super::Foundation::Numerics::Vector2, toprightradius: &super::super::Foundation::Numerics::Vector2, bottomrightradius: &super::super::Foundation::Numerics::Vector2, bottomleftradius: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<RectangleClip>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorStaticsImpl: Sized {
    fn MaxGlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
    fn MinGlobalPlaybackRate(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithBlurredWallpaperBackdropBrushImpl: Sized {
    fn TryCreateBlurredWallpaperBackdropBrush(&self) -> ::windows::core::Result<CompositionBackdropBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithProjectedShadowImpl: Sized {
    fn CreateProjectedShadowCaster(&self) -> ::windows::core::Result<CompositionProjectedShadowCaster>;
    fn CreateProjectedShadow(&self) -> ::windows::core::Result<CompositionProjectedShadow>;
    fn CreateProjectedShadowReceiver(&self) -> ::windows::core::Result<CompositionProjectedShadowReceiver>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithRadialGradientImpl: Sized {
    fn CreateRadialGradientBrush(&self) -> ::windows::core::Result<CompositionRadialGradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositorWithVisualSurfaceImpl: Sized {
    fn CreateVisualSurface(&self) -> ::windows::core::Result<CompositionVisualSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerVisualImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<VisualCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerVisualFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICubicBezierEasingFunctionImpl: Sized {
    fn ControlPoint1(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn ControlPoint2(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDelegatedInkTrailVisualImpl: Sized {
    fn AddTrailPoints(&self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn AddTrailPointsWithPrediction(&self, inkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType], predictedinkpoints: &[<InkTrailPoint as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()>;
    fn StartNewTrail(&self, color: &super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDelegatedInkTrailVisualStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<Compositor>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
    fn CreateForSwapChain(&self, compositor: &::core::option::Option<Compositor>, swapchain: &::core::option::Option<ICompositionSurface>) -> ::windows::core::Result<DelegatedInkTrailVisual>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDistantLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDistantLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropShadowImpl: Sized {
    fn BlurRadius(&self) -> ::windows::core::Result<f32>;
    fn SetBlurRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Mask(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetMask(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropShadow2Impl: Sized {
    fn SourcePolicy(&self) -> ::windows::core::Result<CompositionDropShadowSourcePolicy>;
    fn SetSourcePolicy(&self, value: CompositionDropShadowSourcePolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElasticEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Oscillations(&self) -> ::windows::core::Result<i32>;
    fn Springiness(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExponentialEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Exponent(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpressionAnimationImpl: Sized {
    fn Expression(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExpression(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImplicitAnimationCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IInsetClipImpl: Sized {
    fn BottomInset(&self) -> ::windows::core::Result<f32>;
    fn SetBottomInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn LeftInset(&self) -> ::windows::core::Result<f32>;
    fn SetLeftInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightInset(&self) -> ::windows::core::Result<f32>;
    fn SetRightInset(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopInset(&self) -> ::windows::core::Result<f32>;
    fn SetTopInset(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimationImpl: Sized {
    fn DelayTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IterationBehavior(&self) -> ::windows::core::Result<AnimationIterationBehavior>;
    fn SetIterationBehavior(&self, value: AnimationIterationBehavior) -> ::windows::core::Result<()>;
    fn IterationCount(&self) -> ::windows::core::Result<i32>;
    fn SetIterationCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn KeyFrameCount(&self) -> ::windows::core::Result<i32>;
    fn StopBehavior(&self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrame(&self, normalizedprogresskey: f32, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn InsertExpressionKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &::windows::core::HSTRING, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimation2Impl: Sized {
    fn Direction(&self) -> ::windows::core::Result<AnimationDirection>;
    fn SetDirection(&self, value: AnimationDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimation3Impl: Sized {
    fn DelayBehavior(&self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyFrameAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILayerVisualImpl: Sized {
    fn Effect(&self) -> ::windows::core::Result<CompositionEffectBrush>;
    fn SetEffect(&self, value: &::core::option::Option<CompositionEffectBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayerVisual2Impl: Sized {
    fn Shadow(&self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearEasingFunctionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait INaturalMotionAnimationImpl: Sized {
    fn DelayBehavior(&self) -> ::windows::core::Result<AnimationDelayBehavior>;
    fn SetDelayBehavior(&self, value: AnimationDelayBehavior) -> ::windows::core::Result<()>;
    fn DelayTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDelayTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StopBehavior(&self) -> ::windows::core::Result<AnimationStopBehavior>;
    fn SetStopBehavior(&self, value: AnimationStopBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPathKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, path: &::core::option::Option<CompositionPath>, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointLightImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::Color>;
    fn SetColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn ConstantAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointLight2Impl: Sized {
    fn Intensity(&self) -> ::windows::core::Result<f32>;
    fn SetIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointLight3Impl: Sized {
    fn MinAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
    fn Power(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuaternionKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Quaternion, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleClipImpl: Sized {
    fn Bottom(&self) -> ::windows::core::Result<f32>;
    fn SetBottom(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomLeftRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomLeftRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BottomRightRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetBottomRightRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn Left(&self) -> ::windows::core::Result<f32>;
    fn SetLeft(&self, value: f32) -> ::windows::core::Result<()>;
    fn Right(&self) -> ::windows::core::Result<f32>;
    fn SetRight(&self, value: f32) -> ::windows::core::Result<()>;
    fn Top(&self) -> ::windows::core::Result<f32>;
    fn SetTop(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopLeftRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopLeftRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TopRightRadius(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetTopRightRadius(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRedirectVisualImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Visual>;
    fn SetSource(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderingDeviceReplacedEventArgsImpl: Sized {
    fn GraphicsDevice(&self) -> ::windows::core::Result<CompositionGraphicsDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarKeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: f32) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: f32, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarNaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<f32>;
    fn SetInitialVelocity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarNaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeVisualImpl: Sized {
    fn Shapes(&self) -> ::windows::core::Result<CompositionShapeCollection>;
    fn ViewBox(&self) -> ::windows::core::Result<CompositionViewBox>;
    fn SetViewBox(&self, value: &::core::option::Option<CompositionViewBox>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISineEasingFunctionImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CompositionEasingFunctionMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpotLightImpl: Sized {
    fn ConstantAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetConstantAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn CoordinateSpace(&self) -> ::windows::core::Result<Visual>;
    fn SetCoordinateSpace(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InnerConeAngle(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn InnerConeColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetInnerConeColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn LinearAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetLinearAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn OuterConeAngle(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetOuterConeColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn QuadraticAttenuation(&self) -> ::windows::core::Result<f32>;
    fn SetQuadraticAttenuation(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpotLight2Impl: Sized {
    fn InnerConeIntensity(&self) -> ::windows::core::Result<f32>;
    fn SetInnerConeIntensity(&self, value: f32) -> ::windows::core::Result<()>;
    fn OuterConeIntensity(&self) -> ::windows::core::Result<f32>;
    fn SetOuterConeIntensity(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpotLight3Impl: Sized {
    fn MinAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMinAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn MaxAttenuationCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetMaxAttenuationCutoff(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpringScalarNaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpringVector2NaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpringVector3NaturalMotionAnimationImpl: Sized {
    fn DampingRatio(&self) -> ::windows::core::Result<f32>;
    fn SetDampingRatio(&self, value: f32) -> ::windows::core::Result<()>;
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPeriod(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpriteVisualImpl: Sized {
    fn Brush(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetBrush(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpriteVisual2Impl: Sized {
    fn Shadow(&self) -> ::windows::core::Result<CompositionShadow>;
    fn SetShadow(&self, value: &::core::option::Option<CompositionShadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStepEasingFunctionImpl: Sized {
    fn FinalStep(&self) -> ::windows::core::Result<i32>;
    fn SetFinalStep(&self, value: i32) -> ::windows::core::Result<()>;
    fn InitialStep(&self) -> ::windows::core::Result<i32>;
    fn SetInitialStep(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsFinalStepSingleFrame(&self) -> ::windows::core::Result<bool>;
    fn SetIsFinalStepSingleFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsInitialStepSingleFrame(&self) -> ::windows::core::Result<bool>;
    fn SetIsInitialStepSingleFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn StepCount(&self) -> ::windows::core::Result<i32>;
    fn SetStepCount(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector2KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector2, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector2NaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector2>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetInitialVelocity(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector2NaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector3, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3NaturalMotionAnimationImpl: Sized {
    fn FinalValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetFinalValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>;
    fn SetInitialValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Numerics::Vector3>>) -> ::windows::core::Result<()>;
    fn InitialVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetInitialVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3NaturalMotionAnimationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVector4KeyFrameAnimationImpl: Sized {
    fn InsertKeyFrame(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn InsertKeyFrameWithEasingFunction(&self, normalizedprogresskey: f32, value: &super::super::Foundation::Numerics::Vector4, easingfunction: &::core::option::Option<CompositionEasingFunction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualImpl: Sized {
    fn AnchorPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetAnchorPoint(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn BackfaceVisibility(&self) -> ::windows::core::Result<CompositionBackfaceVisibility>;
    fn SetBackfaceVisibility(&self, value: CompositionBackfaceVisibility) -> ::windows::core::Result<()>;
    fn BorderMode(&self) -> ::windows::core::Result<CompositionBorderMode>;
    fn SetBorderMode(&self, value: CompositionBorderMode) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Clip(&self) -> ::windows::core::Result<CompositionClip>;
    fn SetClip(&self, value: &::core::option::Option<CompositionClip>) -> ::windows::core::Result<()>;
    fn CompositeMode(&self) -> ::windows::core::Result<CompositionCompositeMode>;
    fn SetCompositeMode(&self, value: CompositionCompositeMode) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetOffset(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<ContainerVisual>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetSize(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisual2Impl: Sized {
    fn ParentForTransform(&self) -> ::windows::core::Result<Visual>;
    fn SetParentForTransform(&self, value: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RelativeOffsetAdjustment(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativeOffsetAdjustment(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeSizeAdjustment(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn SetRelativeSizeAdjustment(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisual3Impl: Sized {
    fn IsHitTestVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisual4Impl: Sized {
    fn IsPixelSnappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn InsertAbove(&self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtBottom(&self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertAtTop(&self, newchild: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn InsertBelow(&self, newchild: &::core::option::Option<Visual>, sibling: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&self, child: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}
pub trait IVisualElementImpl: Sized {}
pub trait IVisualElement2Impl: Sized {
    fn GetVisualInternal(&self) -> ::windows::core::Result<Visual>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualUnorderedCollectionImpl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, newvisual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn Remove(&self, visual: &::core::option::Option<Visual>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
}

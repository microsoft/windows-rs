#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushImpl: Sized {
    fn BackgroundSource(&self) -> ::windows::core::Result<AcrylicBackgroundSource>;
    fn SetBackgroundSource(&self, value: AcrylicBackgroundSource) -> ::windows::core::Result<()>;
    fn TintColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetTintColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TintOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetTintOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn TintTransitionDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTintTransitionDuration(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrush2Impl: Sized {
    fn TintLuminosityOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetTintLuminosityOpacity(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AcrylicBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStaticsImpl: Sized {
    fn BackgroundSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintOpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TintTransitionDurationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcrylicBrushStatics2Impl: Sized {
    fn TintLuminosityOpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcSegmentImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f64>;
    fn SetRotationAngle(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsLargeArc(&self) -> ::windows::core::Result<bool>;
    fn SetIsLargeArc(&self, value: bool) -> ::windows::core::Result<()>;
    fn SweepDirection(&self) -> ::windows::core::Result<SweepDirection>;
    fn SetSweepDirection(&self, value: SweepDirection) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IArcSegmentStaticsImpl: Sized {
    fn PointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SizeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationAngleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsLargeArcProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SweepDirectionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBezierSegmentImpl: Sized {
    fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point3(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint3(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBezierSegmentStaticsImpl: Sized {
    fn Point1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point3Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBitmapCacheImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushImpl: Sized {
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Transform(&self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn RelativeTransform(&self) -> ::windows::core::Result<Transform>;
    fn SetRelativeTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Brush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushOverrides2Impl: Sized {
    fn PopulatePropertyInfoOverride(&self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushStaticsImpl: Sized {
    fn OpacityProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RelativeTransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheModeImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICacheModeFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CacheMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
    fn SkewX(&self) -> ::windows::core::Result<f64>;
    fn SetSkewX(&self, value: f64) -> ::windows::core::Result<()>;
    fn SkewY(&self) -> ::windows::core::Result<f64>;
    fn SetSkewY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f64>;
    fn SetRotation(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateX(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateY(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SkewYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TranslateYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetStaticsImpl: Sized {
    fn Rendering(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendering(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SurfaceContentsLost(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSurfaceContentsLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionTargetStatics3Impl: Sized {
    fn Rendered(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<RenderedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRendered(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEllipseGeometryImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RadiusX(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RadiusY(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEllipseGeometryStaticsImpl: Sized {
    fn CenterProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyFactoryImpl: Sized {
    fn CreateInstanceWithName(&self, familyname: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontFamilyStatics2Impl: Sized {
    fn XamlAutoFontFamily(&self) -> ::windows::core::Result<FontFamily>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformImpl: Sized {
    fn Inverse(&self) -> ::windows::core::Result<GeneralTransform>;
    fn TransformPoint(&self, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn TryTransform(&self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GeneralTransform>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneralTransformOverridesImpl: Sized {
    fn InverseCore(&self) -> ::windows::core::Result<GeneralTransform>;
    fn TryTransformCore(&self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBoundsCore(&self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryImpl: Sized {
    fn Transform(&self) -> ::windows::core::Result<Transform>;
    fn SetTransform(&self, value: &::core::option::Option<Transform>) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryGroupImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<GeometryCollection>;
    fn SetChildren(&self, value: &::core::option::Option<GeometryCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryGroupStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeometryStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<Geometry>;
    fn StandardFlatteningTolerance(&self) -> ::windows::core::Result<f64>;
    fn TransformProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushImpl: Sized {
    fn SpreadMethod(&self) -> ::windows::core::Result<GradientSpreadMethod>;
    fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows::core::Result<()>;
    fn MappingMode(&self) -> ::windows::core::Result<BrushMappingMode>;
    fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows::core::Result<()>;
    fn ColorInterpolationMode(&self) -> ::windows::core::Result<ColorInterpolationMode>;
    fn SetColorInterpolationMode(&self, value: ColorInterpolationMode) -> ::windows::core::Result<()>;
    fn GradientStops(&self) -> ::windows::core::Result<GradientStopCollection>;
    fn SetGradientStops(&self, value: &::core::option::Option<GradientStopCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientBrushStaticsImpl: Sized {
    fn SpreadMethodProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn MappingModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ColorInterpolationModeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GradientStopsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStopImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<f64>;
    fn SetOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGradientStopStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn OffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageBrushImpl: Sized {
    fn ImageSource(&self) -> ::windows::core::Result<ImageSource>;
    fn SetImageSource(&self, value: &::core::option::Option<ImageSource>) -> ::windows::core::Result<()>;
    fn ImageFailed(&self, handler: &::core::option::Option<super::ExceptionRoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImageOpened(&self, handler: &::core::option::Option<super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageOpened(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageBrushStaticsImpl: Sized {
    fn ImageSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IImageSourceFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILineGeometryImpl: Sized {
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineGeometryStaticsImpl: Sized {
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineSegmentImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineSegmentStaticsImpl: Sized {
    fn PointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushImpl: Sized {
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn EndPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetEndPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushFactoryImpl: Sized {
    fn CreateInstanceWithGradientStopCollectionAndAngle(&self, gradientstopcollection: &::core::option::Option<GradientStopCollection>, angle: f64) -> ::windows::core::Result<LinearGradientBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILinearGradientBrushStaticsImpl: Sized {
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn EndPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSourceLoadCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LoadedImageSourceLoadStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSurfaceImpl: Sized {
    fn DecodedPhysicalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn DecodedSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn NaturalSize(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn LoadCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<LoadedImageSurface, LoadedImageSourceLoadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoadedImageSurfaceStaticsImpl: Sized {
    fn StartLoadFromUriWithSize(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromUri(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStreamWithSize(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>, desiredmaxsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<LoadedImageSurface>;
    fn StartLoadFromStream(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<LoadedImageSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DProjectionImpl: Sized {
    fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D>;
    fn SetProjectionMatrix(&self, value: &Media3D::Matrix3D) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DProjectionStaticsImpl: Sized {
    fn ProjectionMatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixHelperStaticsImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<Matrix>;
    fn FromElements(&self, m11: f64, m12: f64, m21: f64, m22: f64, offsetx: f64, offsety: f64) -> ::windows::core::Result<Matrix>;
    fn GetIsIdentity(&self, target: &Matrix) -> ::windows::core::Result<bool>;
    fn Transform(&self, target: &Matrix, point: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransformImpl: Sized {
    fn Matrix(&self) -> ::windows::core::Result<Matrix>;
    fn SetMatrix(&self, value: &Matrix) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrixTransformStaticsImpl: Sized {
    fn MatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsThumbnailRequestedEventArgsImpl: Sized {
    fn SetThumbnailImage(&self, source: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPartialMediaFailureDetectedEventArgsImpl: Sized {
    fn StreamKind(&self) -> ::windows::core::Result<super::super::super::Media::Playback::FailedMediaStreamKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPartialMediaFailureDetectedEventArgs2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFigureImpl: Sized {
    fn Segments(&self) -> ::windows::core::Result<PathSegmentCollection>;
    fn SetSegments(&self, value: &::core::option::Option<PathSegmentCollection>) -> ::windows::core::Result<()>;
    fn StartPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetStartPoint(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsClosed(&self) -> ::windows::core::Result<bool>;
    fn SetIsClosed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFilled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFilled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFigureStaticsImpl: Sized {
    fn SegmentsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StartPointProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsClosedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsFilledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathGeometryImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<FillRule>;
    fn SetFillRule(&self, value: FillRule) -> ::windows::core::Result<()>;
    fn Figures(&self) -> ::windows::core::Result<PathFigureCollection>;
    fn SetFigures(&self, value: &::core::option::Option<PathFigureCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathGeometryStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn FiguresProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegmentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPathSegmentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaneProjectionImpl: Sized {
    fn LocalOffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
    fn LocalOffsetZ(&self) -> ::windows::core::Result<f64>;
    fn SetLocalOffsetZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationX(&self) -> ::windows::core::Result<f64>;
    fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationY(&self) -> ::windows::core::Result<f64>;
    fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterOfRotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetCenterOfRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
    fn GlobalOffsetZ(&self) -> ::windows::core::Result<f64>;
    fn SetGlobalOffsetZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn ProjectionMatrix(&self) -> ::windows::core::Result<Media3D::Matrix3D>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaneProjectionStaticsImpl: Sized {
    fn LocalOffsetXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn LocalOffsetZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RotationZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterOfRotationZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GlobalOffsetZProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ProjectionMatrixProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyBezierSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyBezierSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyLineSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyLineSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyQuadraticBezierSegmentImpl: Sized {
    fn Points(&self) -> ::windows::core::Result<PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolyQuadraticBezierSegmentStaticsImpl: Sized {
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IProjectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Projection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticBezierSegmentImpl: Sized {
    fn Point1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint1(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Point2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetPoint2(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IQuadraticBezierSegmentStaticsImpl: Sized {
    fn Point1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Point2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRateChangedRoutedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleGeometryImpl: Sized {
    fn Rect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleGeometryStaticsImpl: Sized {
    fn RectProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderedEventArgsImpl: Sized {
    fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRenderingEventArgsImpl: Sized {
    fn RenderingTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBackgroundBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBackgroundBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrushImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBorderBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBorderBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn TargetTheme(&self) -> ::windows::core::Result<super::ApplicationTheme>;
    fn SetTargetTheme(&self, value: super::ApplicationTheme) -> ::windows::core::Result<()>;
    fn AlwaysUseFallback(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysUseFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RevealBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevealBrushStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TargetThemeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlwaysUseFallbackProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StateProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn SetState(&self, element: &::core::option::Option<super::UIElement>, value: RevealBrushState) -> ::windows::core::Result<()>;
    fn GetState(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<RevealBrushState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn Angle(&self) -> ::windows::core::Result<f64>;
    fn SetAngle(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRotateTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShadowImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IShadowFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransformImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn AngleX(&self) -> ::windows::core::Result<f64>;
    fn SetAngleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn AngleY(&self) -> ::windows::core::Result<f64>;
    fn SetAngleY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISkewTransformStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AngleYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushFactoryImpl: Sized {
    fn CreateInstanceWithColor(&self, color: &super::super::Color) -> ::windows::core::Result<SolidColorBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISolidColorBrushStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadowImpl: Sized {
    fn Receivers(&self) -> ::windows::core::Result<super::UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThemeShadowFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThemeShadow>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushImpl: Sized {
    fn AlignmentX(&self) -> ::windows::core::Result<AlignmentX>;
    fn SetAlignmentX(&self, value: AlignmentX) -> ::windows::core::Result<()>;
    fn AlignmentY(&self) -> ::windows::core::Result<AlignmentY>;
    fn SetAlignmentY(&self, value: AlignmentY) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<Stretch>;
    fn SetStretch(&self, value: Stretch) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TileBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileBrushStaticsImpl: Sized {
    fn AlignmentXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AlignmentYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerRoutedEventArgsImpl: Sized {
    fn Marker(&self) -> ::windows::core::Result<TimelineMarker>;
    fn SetMarker(&self, value: &::core::option::Option<TimelineMarker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelineMarkerStaticsImpl: Sized {
    fn TimeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn TextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformGroupImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<TransformCollection>;
    fn SetChildren(&self, value: &::core::option::Option<TransformCollection>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<Matrix>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformGroupStaticsImpl: Sized {
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransformImpl: Sized {
    fn X(&self) -> ::windows::core::Result<f64>;
    fn SetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y(&self) -> ::windows::core::Result<f64>;
    fn SetY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITranslateTransformStaticsImpl: Sized {
    fn XProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn YProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStaticsImpl: Sized {
    fn FindElementsInHostCoordinatesPoint(&self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindElementsInHostCoordinatesRect(&self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesPoint(&self, intersectingpoint: &super::super::super::Foundation::Point, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn FindAllElementsInHostCoordinatesRect(&self, intersectingrect: &super::super::super::Foundation::Rect, subtree: &::core::option::Option<super::UIElement>, includeallelements: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<super::UIElement>>;
    fn GetChild(&self, reference: &::core::option::Option<super::DependencyObject>, childindex: i32) -> ::windows::core::Result<super::DependencyObject>;
    fn GetChildrenCount(&self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn GetParent(&self, reference: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn DisconnectChildrenRecursive(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStatics2Impl: Sized {
    fn GetOpenPopups(&self, window: &::core::option::Option<super::Window>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTreeHelperStatics3Impl: Sized {
    fn GetOpenPopupsForXamlRoot(&self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseImpl: Sized {
    fn FallbackColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetFallbackColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlCompositionBrushBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseOverridesImpl: Sized {
    fn OnConnected(&self) -> ::windows::core::Result<()>;
    fn OnDisconnected(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseProtectedImpl: Sized {
    fn CompositionBrush(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
    fn SetCompositionBrush(&self, value: &::core::option::Option<super::super::Composition::CompositionBrush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlCompositionBrushBaseStaticsImpl: Sized {
    fn FallbackColorProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlLight>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightOverridesImpl: Sized {
    fn GetId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnConnected(&self, newelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn OnDisconnected(&self, oldelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightProtectedImpl: Sized {
    fn CompositionLight(&self) -> ::windows::core::Result<super::super::Composition::CompositionLight>;
    fn SetCompositionLight(&self, value: &::core::option::Option<super::super::Composition::CompositionLight>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlLightStaticsImpl: Sized {
    fn AddTargetElement(&self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn RemoveTargetElement(&self, lightid: &::windows::core::HSTRING, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddTargetBrush(&self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
    fn RemoveTargetBrush(&self, lightid: &::windows::core::HSTRING, brush: &::core::option::Option<Brush>) -> ::windows::core::Result<()>;
}

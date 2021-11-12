#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Xaml_Media_Animation")]
pub mod Animation;
#[cfg(feature = "UI_Xaml_Media_Imaging")]
pub mod Imaging;
#[cfg(feature = "UI_Xaml_Media_Media3D")]
pub mod Media3D;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AcrylicBackgroundSource(pub i32);
impl AcrylicBackgroundSource {
    pub const HostBackdrop: Self = Self(0i32);
    pub const Backdrop: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AcrylicBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ArcSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioCategory(pub i32);
impl AudioCategory {
    pub const Other: Self = Self(0i32);
    pub const ForegroundOnlyMedia: Self = Self(1i32);
    pub const BackgroundCapableMedia: Self = Self(2i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
#[repr(transparent)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
#[repr(transparent)]
pub struct BezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Brush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BrushCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CacheMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CompositeTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EllipseGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
#[repr(transparent)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
#[repr(transparent)]
pub struct FontFamily(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeneralTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeometryCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeometryGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GradientStop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GradientStopCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcrylicBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcrylicBrush2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcrylicBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcrylicBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcrylicBrushStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IArcSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IArcSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBezierSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrushOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICacheMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICacheModeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositeTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositeTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTargetStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionTargetStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEllipseGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEllipseGeometryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontFamily(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontFamilyFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontFamilyStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeneralTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeneralTransformFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeneralTransformOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeometryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeometryGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeometryGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeometryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGradientBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGradientBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGradientStop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGradientStopStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineGeometryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearGradientBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILinearGradientBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadedImageSourceLoadCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadedImageSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadedImageSurfaceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrix3DProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrix3DProjectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrixHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrixHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrixTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrixTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathFigure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathFigureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathGeometryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathSegmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaneProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaneProjectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyBezierSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyLineSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyLineSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProjectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuadraticBezierSegmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRateChangedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectangleGeometryStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRenderedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRenderingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBackgroundBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBackgroundBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBorderBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBorderBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRevealBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRotateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRotateTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScaleTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScaleTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShadowFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISkewTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISkewTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISolidColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISolidColorBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISolidColorBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThemeShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThemeShadowFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileBrushFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelineMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelineMarkerRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelineMarkerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITranslateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITranslateTransformStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeHelperStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeHelperStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlCompositionBrushBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlLight(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlLightFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlLightOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlLightProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlLightStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LinearGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoadedImageSourceLoadCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
#[repr(transparent)]
pub struct LoadedImageSurface(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Matrix(i32);
#[repr(transparent)]
pub struct Matrix3DProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MatrixHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MatrixTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MediaElementState(pub i32);
impl MediaElementState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
#[repr(transparent)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PartialMediaFailureDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathFigure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathFigureCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathSegmentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PlaneProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PolyBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PolyLineSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PolyQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Projection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuadraticBezierSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RateChangedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RateChangedRoutedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RenderedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RenderingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevealBackgroundBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevealBorderBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevealBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: Self = Self(0i32);
    pub const PointerOver: Self = Self(1i32);
    pub const Pressed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct RotateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScaleTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Shadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SkewTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SolidColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
#[repr(transparent)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
#[repr(transparent)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
#[repr(transparent)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
#[repr(transparent)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ThemeShadow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelineMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelineMarkerCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Transform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TransformCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TransformGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TranslateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualTreeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlCompositionBrushBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlLight(pub *mut ::core::ffi::c_void);

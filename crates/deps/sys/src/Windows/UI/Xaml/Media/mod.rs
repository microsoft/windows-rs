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
impl ::core::marker::Copy for AcrylicBackgroundSource {}
impl ::core::clone::Clone for AcrylicBackgroundSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AcrylicBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AcrylicBrush {}
impl ::core::clone::Clone for AcrylicBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentX {}
impl ::core::clone::Clone for AlignmentX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentY {}
impl ::core::clone::Clone for AlignmentY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ArcSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ArcSegment {}
impl ::core::clone::Clone for ArcSegment {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AudioCategory {}
impl ::core::clone::Clone for AudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceType(pub i32);
impl AudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDeviceType {}
impl ::core::clone::Clone for AudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BezierSegment {}
impl ::core::clone::Clone for BezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapCache {}
impl ::core::clone::Clone for BitmapCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Brush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Brush {}
impl ::core::clone::Clone for Brush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BrushCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BrushCollection {}
impl ::core::clone::Clone for BrushCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for BrushMappingMode {}
impl ::core::clone::Clone for BrushMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CacheMode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CacheMode {}
impl ::core::clone::Clone for CacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorInterpolationMode {}
impl ::core::clone::Clone for ColorInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositeTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositeTransform {}
impl ::core::clone::Clone for CompositeTransform {
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
pub struct DoubleCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DoubleCollection {}
impl ::core::clone::Clone for DoubleCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementCompositeMode {}
impl ::core::clone::Clone for ElementCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EllipseGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EllipseGeometry {}
impl ::core::clone::Clone for EllipseGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
impl ::core::marker::Copy for FastPlayFallbackBehaviour {}
impl ::core::clone::Clone for FastPlayFallbackBehaviour {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
impl ::core::marker::Copy for FillRule {}
impl ::core::clone::Clone for FillRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontFamily(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FontFamily {}
impl ::core::clone::Clone for FontFamily {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeneralTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeneralTransform {}
impl ::core::clone::Clone for GeneralTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geometry {}
impl ::core::clone::Clone for Geometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeometryCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeometryCollection {}
impl ::core::clone::Clone for GeometryCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeometryGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeometryGroup {}
impl ::core::clone::Clone for GeometryGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GradientBrush {}
impl ::core::clone::Clone for GradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for GradientSpreadMethod {}
impl ::core::clone::Clone for GradientSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GradientStop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GradientStop {}
impl ::core::clone::Clone for GradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GradientStopCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GradientStopCollection {}
impl ::core::clone::Clone for GradientStopCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcrylicBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcrylicBrush {}
impl ::core::clone::Clone for IAcrylicBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcrylicBrush2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcrylicBrush2 {}
impl ::core::clone::Clone for IAcrylicBrush2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcrylicBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcrylicBrushFactory {}
impl ::core::clone::Clone for IAcrylicBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcrylicBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcrylicBrushStatics {}
impl ::core::clone::Clone for IAcrylicBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcrylicBrushStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcrylicBrushStatics2 {}
impl ::core::clone::Clone for IAcrylicBrushStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IArcSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IArcSegment {}
impl ::core::clone::Clone for IArcSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IArcSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IArcSegmentStatics {}
impl ::core::clone::Clone for IArcSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBezierSegment {}
impl ::core::clone::Clone for IBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBezierSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBezierSegmentStatics {}
impl ::core::clone::Clone for IBezierSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapCache {}
impl ::core::clone::Clone for IBitmapCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrush {}
impl ::core::clone::Clone for IBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrushFactory {}
impl ::core::clone::Clone for IBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrushOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrushOverrides2 {}
impl ::core::clone::Clone for IBrushOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrushStatics {}
impl ::core::clone::Clone for IBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICacheMode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICacheMode {}
impl ::core::clone::Clone for ICacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICacheModeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICacheModeFactory {}
impl ::core::clone::Clone for ICacheModeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositeTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositeTransform {}
impl ::core::clone::Clone for ICompositeTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositeTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositeTransformStatics {}
impl ::core::clone::Clone for ICompositeTransformStatics {
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
pub struct ICompositionTargetStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTargetStatics {}
impl ::core::clone::Clone for ICompositionTargetStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionTargetStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionTargetStatics3 {}
impl ::core::clone::Clone for ICompositionTargetStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEllipseGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEllipseGeometry {}
impl ::core::clone::Clone for IEllipseGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEllipseGeometryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEllipseGeometryStatics {}
impl ::core::clone::Clone for IEllipseGeometryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontFamily(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontFamily {}
impl ::core::clone::Clone for IFontFamily {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontFamilyFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontFamilyFactory {}
impl ::core::clone::Clone for IFontFamilyFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontFamilyStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontFamilyStatics2 {}
impl ::core::clone::Clone for IFontFamilyStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeneralTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeneralTransform {}
impl ::core::clone::Clone for IGeneralTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeneralTransformFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeneralTransformFactory {}
impl ::core::clone::Clone for IGeneralTransformFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeneralTransformOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeneralTransformOverrides {}
impl ::core::clone::Clone for IGeneralTransformOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometry {}
impl ::core::clone::Clone for IGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeometryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometryFactory {}
impl ::core::clone::Clone for IGeometryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeometryGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometryGroup {}
impl ::core::clone::Clone for IGeometryGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeometryGroupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometryGroupStatics {}
impl ::core::clone::Clone for IGeometryGroupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeometryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeometryStatics {}
impl ::core::clone::Clone for IGeometryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGradientBrush {}
impl ::core::clone::Clone for IGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGradientBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGradientBrushFactory {}
impl ::core::clone::Clone for IGradientBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGradientBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGradientBrushStatics {}
impl ::core::clone::Clone for IGradientBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGradientStop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGradientStop {}
impl ::core::clone::Clone for IGradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGradientStopStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGradientStopStatics {}
impl ::core::clone::Clone for IGradientStopStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageBrush {}
impl ::core::clone::Clone for IImageBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageBrushStatics {}
impl ::core::clone::Clone for IImageBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageSource {}
impl ::core::clone::Clone for IImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageSourceFactory {}
impl ::core::clone::Clone for IImageSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineGeometry {}
impl ::core::clone::Clone for ILineGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineGeometryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineGeometryStatics {}
impl ::core::clone::Clone for ILineGeometryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineSegment {}
impl ::core::clone::Clone for ILineSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineSegmentStatics {}
impl ::core::clone::Clone for ILineSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearGradientBrush {}
impl ::core::clone::Clone for ILinearGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearGradientBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearGradientBrushFactory {}
impl ::core::clone::Clone for ILinearGradientBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILinearGradientBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILinearGradientBrushStatics {}
impl ::core::clone::Clone for ILinearGradientBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoadedImageSourceLoadCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoadedImageSourceLoadCompletedEventArgs {}
impl ::core::clone::Clone for ILoadedImageSourceLoadCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoadedImageSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoadedImageSurface {}
impl ::core::clone::Clone for ILoadedImageSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoadedImageSurfaceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoadedImageSurfaceStatics {}
impl ::core::clone::Clone for ILoadedImageSurfaceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrix3DProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrix3DProjection {}
impl ::core::clone::Clone for IMatrix3DProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrix3DProjectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrix3DProjectionStatics {}
impl ::core::clone::Clone for IMatrix3DProjectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrixHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrixHelper {}
impl ::core::clone::Clone for IMatrixHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrixHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrixHelperStatics {}
impl ::core::clone::Clone for IMatrixHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrixTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrixTransform {}
impl ::core::clone::Clone for IMatrixTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMatrixTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMatrixTransformStatics {}
impl ::core::clone::Clone for IMatrixTransformStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsThumbnailRequestedEventArgs {}
impl ::core::clone::Clone for IMediaTransportControlsThumbnailRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPartialMediaFailureDetectedEventArgs {}
impl ::core::clone::Clone for IPartialMediaFailureDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPartialMediaFailureDetectedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPartialMediaFailureDetectedEventArgs2 {}
impl ::core::clone::Clone for IPartialMediaFailureDetectedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathFigure(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathFigure {}
impl ::core::clone::Clone for IPathFigure {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathFigureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathFigureStatics {}
impl ::core::clone::Clone for IPathFigureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathGeometry {}
impl ::core::clone::Clone for IPathGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathGeometryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathGeometryStatics {}
impl ::core::clone::Clone for IPathGeometryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathSegment {}
impl ::core::clone::Clone for IPathSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathSegmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathSegmentFactory {}
impl ::core::clone::Clone for IPathSegmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaneProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaneProjection {}
impl ::core::clone::Clone for IPlaneProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaneProjectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaneProjectionStatics {}
impl ::core::clone::Clone for IPlaneProjectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyBezierSegment {}
impl ::core::clone::Clone for IPolyBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyBezierSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyBezierSegmentStatics {}
impl ::core::clone::Clone for IPolyBezierSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyLineSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyLineSegment {}
impl ::core::clone::Clone for IPolyLineSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyLineSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyLineSegmentStatics {}
impl ::core::clone::Clone for IPolyLineSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyQuadraticBezierSegment {}
impl ::core::clone::Clone for IPolyQuadraticBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyQuadraticBezierSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyQuadraticBezierSegmentStatics {}
impl ::core::clone::Clone for IPolyQuadraticBezierSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProjection {}
impl ::core::clone::Clone for IProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProjectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProjectionFactory {}
impl ::core::clone::Clone for IProjectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuadraticBezierSegment {}
impl ::core::clone::Clone for IQuadraticBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IQuadraticBezierSegmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuadraticBezierSegmentStatics {}
impl ::core::clone::Clone for IQuadraticBezierSegmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRateChangedRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRateChangedRoutedEventArgs {}
impl ::core::clone::Clone for IRateChangedRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectangleGeometry {}
impl ::core::clone::Clone for IRectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectangleGeometryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectangleGeometryStatics {}
impl ::core::clone::Clone for IRectangleGeometryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRenderedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRenderedEventArgs {}
impl ::core::clone::Clone for IRenderedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRenderingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRenderingEventArgs {}
impl ::core::clone::Clone for IRenderingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBackgroundBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBackgroundBrush {}
impl ::core::clone::Clone for IRevealBackgroundBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBackgroundBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBackgroundBrushFactory {}
impl ::core::clone::Clone for IRevealBackgroundBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBorderBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBorderBrush {}
impl ::core::clone::Clone for IRevealBorderBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBorderBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBorderBrushFactory {}
impl ::core::clone::Clone for IRevealBorderBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBrush {}
impl ::core::clone::Clone for IRevealBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBrushFactory {}
impl ::core::clone::Clone for IRevealBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRevealBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRevealBrushStatics {}
impl ::core::clone::Clone for IRevealBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRotateTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRotateTransform {}
impl ::core::clone::Clone for IRotateTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRotateTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRotateTransformStatics {}
impl ::core::clone::Clone for IRotateTransformStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScaleTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScaleTransform {}
impl ::core::clone::Clone for IScaleTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScaleTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScaleTransformStatics {}
impl ::core::clone::Clone for IScaleTransformStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShadow {}
impl ::core::clone::Clone for IShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShadowFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShadowFactory {}
impl ::core::clone::Clone for IShadowFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISkewTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISkewTransform {}
impl ::core::clone::Clone for ISkewTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISkewTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISkewTransformStatics {}
impl ::core::clone::Clone for ISkewTransformStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISolidColorBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISolidColorBrush {}
impl ::core::clone::Clone for ISolidColorBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISolidColorBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISolidColorBrushFactory {}
impl ::core::clone::Clone for ISolidColorBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISolidColorBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISolidColorBrushStatics {}
impl ::core::clone::Clone for ISolidColorBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThemeShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThemeShadow {}
impl ::core::clone::Clone for IThemeShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThemeShadowFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThemeShadowFactory {}
impl ::core::clone::Clone for IThemeShadowFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileBrush {}
impl ::core::clone::Clone for ITileBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileBrushFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileBrushFactory {}
impl ::core::clone::Clone for ITileBrushFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileBrushStatics {}
impl ::core::clone::Clone for ITileBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelineMarker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelineMarker {}
impl ::core::clone::Clone for ITimelineMarker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelineMarkerRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelineMarkerRoutedEventArgs {}
impl ::core::clone::Clone for ITimelineMarkerRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelineMarkerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelineMarkerStatics {}
impl ::core::clone::Clone for ITimelineMarkerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransform {}
impl ::core::clone::Clone for ITransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransformFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransformFactory {}
impl ::core::clone::Clone for ITransformFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransformGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransformGroup {}
impl ::core::clone::Clone for ITransformGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransformGroupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransformGroupStatics {}
impl ::core::clone::Clone for ITransformGroupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITranslateTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITranslateTransform {}
impl ::core::clone::Clone for ITranslateTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITranslateTransformStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITranslateTransformStatics {}
impl ::core::clone::Clone for ITranslateTransformStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeHelper {}
impl ::core::clone::Clone for IVisualTreeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeHelperStatics {}
impl ::core::clone::Clone for IVisualTreeHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeHelperStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeHelperStatics2 {}
impl ::core::clone::Clone for IVisualTreeHelperStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeHelperStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeHelperStatics3 {}
impl ::core::clone::Clone for IVisualTreeHelperStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlCompositionBrushBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlCompositionBrushBase {}
impl ::core::clone::Clone for IXamlCompositionBrushBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlCompositionBrushBaseFactory {}
impl ::core::clone::Clone for IXamlCompositionBrushBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlCompositionBrushBaseOverrides {}
impl ::core::clone::Clone for IXamlCompositionBrushBaseOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlCompositionBrushBaseProtected {}
impl ::core::clone::Clone for IXamlCompositionBrushBaseProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlCompositionBrushBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlCompositionBrushBaseStatics {}
impl ::core::clone::Clone for IXamlCompositionBrushBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlLight {}
impl ::core::clone::Clone for IXamlLight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlLightFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlLightFactory {}
impl ::core::clone::Clone for IXamlLightFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlLightOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlLightOverrides {}
impl ::core::clone::Clone for IXamlLightOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlLightProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlLightProtected {}
impl ::core::clone::Clone for IXamlLightProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlLightStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlLightStatics {}
impl ::core::clone::Clone for IXamlLightStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageBrush {}
impl ::core::clone::Clone for ImageBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageSource {}
impl ::core::clone::Clone for ImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineGeometry {}
impl ::core::clone::Clone for LineGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineSegment {}
impl ::core::clone::Clone for LineSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LinearGradientBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LinearGradientBrush {}
impl ::core::clone::Clone for LinearGradientBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoadedImageSourceLoadCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoadedImageSourceLoadCompletedEventArgs {}
impl ::core::clone::Clone for LoadedImageSourceLoadCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LoadedImageSourceLoadStatus {}
impl ::core::clone::Clone for LoadedImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoadedImageSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoadedImageSurface {}
impl ::core::clone::Clone for LoadedImageSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl ::core::marker::Copy for Matrix {}
impl ::core::clone::Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Matrix3DProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Matrix3DProjection {}
impl ::core::clone::Clone for Matrix3DProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MatrixHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MatrixHelper {}
impl ::core::clone::Clone for MatrixHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MatrixTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MatrixTransform {}
impl ::core::clone::Clone for MatrixTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCanPlayResponse(pub i32);
impl MediaCanPlayResponse {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCanPlayResponse {}
impl ::core::clone::Clone for MediaCanPlayResponse {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for MediaElementState {}
impl ::core::clone::Clone for MediaElementState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTransportControlsThumbnailRequestedEventArgs {}
impl ::core::clone::Clone for MediaTransportControlsThumbnailRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PartialMediaFailureDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PartialMediaFailureDetectedEventArgs {}
impl ::core::clone::Clone for PartialMediaFailureDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathFigure(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathFigure {}
impl ::core::clone::Clone for PathFigure {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathFigureCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathFigureCollection {}
impl ::core::clone::Clone for PathFigureCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathGeometry {}
impl ::core::clone::Clone for PathGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathSegment {}
impl ::core::clone::Clone for PathSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathSegmentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathSegmentCollection {}
impl ::core::clone::Clone for PathSegmentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for PenLineCap {}
impl ::core::clone::Clone for PenLineCap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
impl ::core::marker::Copy for PenLineJoin {}
impl ::core::clone::Clone for PenLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaneProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaneProjection {}
impl ::core::clone::Clone for PlaneProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointCollection {}
impl ::core::clone::Clone for PointCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolyBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PolyBezierSegment {}
impl ::core::clone::Clone for PolyBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolyLineSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PolyLineSegment {}
impl ::core::clone::Clone for PolyLineSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PolyQuadraticBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PolyQuadraticBezierSegment {}
impl ::core::clone::Clone for PolyQuadraticBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Projection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Projection {}
impl ::core::clone::Clone for Projection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuadraticBezierSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuadraticBezierSegment {}
impl ::core::clone::Clone for QuadraticBezierSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RateChangedRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RateChangedRoutedEventArgs {}
impl ::core::clone::Clone for RateChangedRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RateChangedRoutedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RateChangedRoutedEventHandler {}
impl ::core::clone::Clone for RateChangedRoutedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RectangleGeometry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RectangleGeometry {}
impl ::core::clone::Clone for RectangleGeometry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RenderedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RenderedEventArgs {}
impl ::core::clone::Clone for RenderedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RenderingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RenderingEventArgs {}
impl ::core::clone::Clone for RenderingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevealBackgroundBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RevealBackgroundBrush {}
impl ::core::clone::Clone for RevealBackgroundBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevealBorderBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RevealBorderBrush {}
impl ::core::clone::Clone for RevealBorderBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevealBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RevealBrush {}
impl ::core::clone::Clone for RevealBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RevealBrushState(pub i32);
impl RevealBrushState {
    pub const Normal: Self = Self(0i32);
    pub const PointerOver: Self = Self(1i32);
    pub const Pressed: Self = Self(2i32);
}
impl ::core::marker::Copy for RevealBrushState {}
impl ::core::clone::Clone for RevealBrushState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RotateTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RotateTransform {}
impl ::core::clone::Clone for RotateTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScaleTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScaleTransform {}
impl ::core::clone::Clone for ScaleTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Shadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Shadow {}
impl ::core::clone::Clone for Shadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SkewTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SkewTransform {}
impl ::core::clone::Clone for SkewTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SolidColorBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SolidColorBrush {}
impl ::core::clone::Clone for SolidColorBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Stereo3DVideoPackingMode(pub i32);
impl Stereo3DVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for Stereo3DVideoPackingMode {}
impl ::core::clone::Clone for Stereo3DVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Stereo3DVideoRenderMode(pub i32);
impl Stereo3DVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for Stereo3DVideoRenderMode {}
impl ::core::clone::Clone for Stereo3DVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for Stretch {}
impl ::core::clone::Clone for Stretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
impl ::core::marker::Copy for StyleSimulations {}
impl ::core::clone::Clone for StyleSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for SweepDirection {}
impl ::core::clone::Clone for SweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThemeShadow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ThemeShadow {}
impl ::core::clone::Clone for ThemeShadow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileBrush {}
impl ::core::clone::Clone for TileBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelineMarker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelineMarker {}
impl ::core::clone::Clone for TimelineMarker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelineMarkerCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelineMarkerCollection {}
impl ::core::clone::Clone for TimelineMarkerCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelineMarkerRoutedEventArgs {}
impl ::core::clone::Clone for TimelineMarkerRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelineMarkerRoutedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelineMarkerRoutedEventHandler {}
impl ::core::clone::Clone for TimelineMarkerRoutedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Transform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Transform {}
impl ::core::clone::Clone for Transform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TransformCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TransformCollection {}
impl ::core::clone::Clone for TransformCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TransformGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TransformGroup {}
impl ::core::clone::Clone for TransformGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TranslateTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TranslateTransform {}
impl ::core::clone::Clone for TranslateTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualTreeHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualTreeHelper {}
impl ::core::clone::Clone for VisualTreeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlCompositionBrushBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlCompositionBrushBase {}
impl ::core::clone::Clone for XamlCompositionBrushBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlLight(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlLight {}
impl ::core::clone::Clone for XamlLight {
    fn clone(&self) -> Self {
        *self
    }
}

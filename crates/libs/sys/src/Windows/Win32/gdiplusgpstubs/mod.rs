#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpAdjustableArrowCap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpBitmap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpBrush(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpBrushType = super::BrushType;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpCachedBitmap(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpCoordinateSpace = super::CoordinateSpace;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpCustomLineCap(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpDashCap = super::DashCap;
#[cfg(feature = "gdiplusenums")]
pub type GpDashStyle = super::DashStyle;
#[cfg(feature = "gdiplusenums")]
pub type GpFillMode = super::FillMode;
#[cfg(feature = "gdiplusenums")]
pub type GpFlushIntention = super::FlushIntention;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpFont(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpFontCollection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpFontFamily(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpGraphics(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpHatch(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpHatchStyle = super::HatchStyle;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpImage(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpImageAttributes(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpLineCap = super::LineCap;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpLineGradient(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpLineJoin = super::LineJoin;
#[cfg(feature = "gdiplusmatrix")]
pub type GpMatrix = super::Matrix;
#[cfg(feature = "gdiplusenums")]
pub type GpMatrixOrder = super::MatrixOrder;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpMetafile(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpPath(pub u8);
#[cfg(feature = "gdiplustypes")]
pub type GpPathData = super::PathData;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpPathGradient(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpPathIterator(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpPen(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpPenAlignment = super::PenAlignment;
#[cfg(feature = "gdiplusenums")]
pub type GpPenType = super::PenType;
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
pub type GpPoint = super::Point;
#[cfg(feature = "gdiplustypes")]
pub type GpPointF = super::PointF;
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
pub type GpRect = super::Rect;
#[cfg(feature = "gdiplustypes")]
pub type GpRectF = super::RectF;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpRegion(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpSolidFill(pub u8);
#[cfg(feature = "gdiplustypes")]
pub type GpStatus = super::Status;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpStringFormat(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GpTexture(pub u8);
#[cfg(feature = "gdiplusenums")]
pub type GpUnit = super::Unit;
#[cfg(feature = "gdiplusenums")]
pub type GpWrapMode = super::WrapMode;

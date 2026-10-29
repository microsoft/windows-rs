#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathArc(path: *mut super::GpPath, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathArc(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathArc(path as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathArcI(path: *mut super::GpPath, x: super::INT, y: super::INT, width: super::INT, height: super::INT, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathArcI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathArcI(path as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathBezier(path: *mut super::GpPath, x1: super::REAL, y1: super::REAL, x2: super::REAL, y2: super::REAL, x3: super::REAL, y3: super::REAL, x4: super::REAL, y4: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBezier(path : *mut super::GpPath, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL, x3 : super::REAL, y3 : super::REAL, x4 : super::REAL, y4 : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathBezier(path as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathBezierI(path: *mut super::GpPath, x1: super::INT, y1: super::INT, x2: super::INT, y2: super::INT, x3: super::INT, y3: super::INT, x4: super::INT, y4: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBezierI(path : *mut super::GpPath, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT, x3 : super::INT, y3 : super::INT, x4 : super::INT, y4 : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathBezierI(path as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathBeziers(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBeziers(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathBeziers(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathBeziersI(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBeziersI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathBeziersI(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathClosedCurve(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathClosedCurve(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathClosedCurve2(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathClosedCurve2(path as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathClosedCurve2I(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathClosedCurve2I(path as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathClosedCurveI(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurveI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathClosedCurveI(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurve(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathCurve(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurve2(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathCurve2(path as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurve2I(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathCurve2I(path as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurve3(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT, offset: super::INT, numberofsegments: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve3(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathCurve3(path as _, points, count, offset, numberofsegments, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurve3I(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT, offset: super::INT, numberofsegments: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve3I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathCurve3I(path as _, points, count, offset, numberofsegments, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathCurveI(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurveI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathCurveI(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathEllipse(path: *mut super::GpPath, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathEllipse(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathEllipse(path as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathEllipseI(path: *mut super::GpPath, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathEllipseI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathEllipseI(path as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathLine(path: *mut super::GpPath, x1: super::REAL, y1: super::REAL, x2: super::REAL, y2: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine(path : *mut super::GpPath, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathLine(path as _, x1, y1, x2, y2) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathLine2(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathLine2(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathLine2I(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathLine2I(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathLineI(path: *mut super::GpPath, x1: super::INT, y1: super::INT, x2: super::INT, y2: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLineI(path : *mut super::GpPath, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathLineI(path as _, x1, y1, x2, y2) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathPath(path: *mut super::GpPath, addingpath: *const super::GpPath, connect: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPath(path : *mut super::GpPath, addingpath : *const super::GpPath, connect : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipAddPathPath(path as _, addingpath, connect.into()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathPie(path: *mut super::GpPath, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPie(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathPie(path as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathPieI(path: *mut super::GpPath, x: super::INT, y: super::INT, width: super::INT, height: super::INT, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPieI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathPieI(path as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathPolygon(path: *mut super::GpPath, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPolygon(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathPolygon(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathPolygonI(path: *mut super::GpPath, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPolygonI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathPolygonI(path as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipAddPathRectangle(path: *mut super::GpPath, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangle(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipAddPathRectangle(path as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathRectangleI(path: *mut super::GpPath, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangleI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathRectangleI(path as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathRectangles(path: *mut super::GpPath, rects: *const super::GpRectF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangles(path : *mut super::GpPath, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathRectangles(path as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipAddPathRectanglesI(path: *mut super::GpPath, rects: *const super::GpRect, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectanglesI(path : *mut super::GpPath, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
    unsafe { GdipAddPathRectanglesI(path as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipAddPathString(path: *mut super::GpPath, string: *const super::WCHAR, length: super::INT, family: *const super::GpFontFamily, style: super::INT, emsize: super::REAL, layoutrect: *const super::RectF, format: *const super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathString(path : *mut super::GpPath, string : *const super::WCHAR, length : super::INT, family : *const super::GpFontFamily, style : super::INT, emsize : super::REAL, layoutrect : *const super::RectF, format : *const super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipAddPathString(path as _, string, length, family, style, emsize, layoutrect, format) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipAddPathStringI(path: *mut super::GpPath, string: *const super::WCHAR, length: super::INT, family: *const super::GpFontFamily, style: super::INT, emsize: super::REAL, layoutrect: *const super::Rect, format: *const super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathStringI(path : *mut super::GpPath, string : *const super::WCHAR, length : super::INT, family : *const super::GpFontFamily, style : super::INT, emsize : super::REAL, layoutrect : *const super::Rect, format : *const super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipAddPathStringI(path as _, string, length, family, style, emsize, layoutrect, format) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipBeginContainer(graphics: *mut super::GpGraphics, dstrect: *const super::GpRectF, srcrect: *const super::GpRectF, unit: super::GpUnit, state: *mut super::GraphicsContainer) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainer(graphics : *mut super::GpGraphics, dstrect : *const super::GpRectF, srcrect : *const super::GpRectF, unit : super::GpUnit, state : *mut super::GraphicsContainer) -> super::GpStatus);
    unsafe { GdipBeginContainer(graphics as _, dstrect, srcrect, unit, state as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipBeginContainer2(graphics: *mut super::GpGraphics, state: *mut super::GraphicsContainer) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainer2(graphics : *mut super::GpGraphics, state : *mut super::GraphicsContainer) -> super::GpStatus);
    unsafe { GdipBeginContainer2(graphics as _, state as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBeginContainerI(graphics: *mut super::GpGraphics, dstrect: *const super::GpRect, srcrect: *const super::GpRect, unit: super::GpUnit, state: *mut super::GraphicsContainer) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainerI(graphics : *mut super::GpGraphics, dstrect : *const super::GpRect, srcrect : *const super::GpRect, unit : super::GpUnit, state : *mut super::GraphicsContainer) -> super::GpStatus);
    unsafe { GdipBeginContainerI(graphics as _, dstrect, srcrect, unit, state as _) }
}
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GdipBitmapApplyEffect(bitmap: *mut super::GpBitmap, effect: *mut super::CGpEffect, roi: *mut super::RECT, useauxdata: bool, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapApplyEffect(bitmap : *mut super::GpBitmap, effect : *mut super::CGpEffect, roi : *mut super::RECT, useauxdata : windows_core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut super::INT) -> super::GpStatus);
    unsafe { GdipBitmapApplyEffect(bitmap as _, effect as _, roi as _, useauxdata.into(), auxdata as _, auxdatasize as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapConvertFormat(pinputbitmap: *mut super::GpBitmap, format: super::PixelFormat, dithertype: super::DitherType, palettetype: super::PaletteType, palette: *mut super::ColorPalette, alphathresholdpercent: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapConvertFormat(pinputbitmap : *mut super::GpBitmap, format : super::PixelFormat, dithertype : super::DitherType, palettetype : super::PaletteType, palette : *mut super::ColorPalette, alphathresholdpercent : super::REAL) -> super::GpStatus);
    unsafe { GdipBitmapConvertFormat(pinputbitmap as _, format, dithertype, palettetype, palette as _, alphathresholdpercent) }
}
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GdipBitmapCreateApplyEffect(inputbitmaps: *mut *mut super::GpBitmap, numinputs: super::INT, effect: *mut super::CGpEffect, roi: *mut super::RECT, outputrect: *mut super::RECT, outputbitmap: *mut *mut super::GpBitmap, useauxdata: bool, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapCreateApplyEffect(inputbitmaps : *mut *mut super::GpBitmap, numinputs : super::INT, effect : *mut super::CGpEffect, roi : *mut super::RECT, outputrect : *mut super::RECT, outputbitmap : *mut *mut super::GpBitmap, useauxdata : windows_core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut super::INT) -> super::GpStatus);
    unsafe { GdipBitmapCreateApplyEffect(inputbitmaps as _, numinputs, effect as _, roi as _, outputrect as _, outputbitmap as _, useauxdata.into(), auxdata as _, auxdatasize as _) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapGetHistogram(bitmap: *mut super::GpBitmap, format: super::HistogramFormat, numberofentries: super::UINT, channel0: *mut super::UINT, channel1: *mut super::UINT, channel2: *mut super::UINT, channel3: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogram(bitmap : *mut super::GpBitmap, format : super::HistogramFormat, numberofentries : super::UINT, channel0 : *mut super::UINT, channel1 : *mut super::UINT, channel2 : *mut super::UINT, channel3 : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipBitmapGetHistogram(bitmap as _, format, numberofentries, channel0 as _, channel1 as _, channel2 as _, channel3 as _) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapGetHistogramSize(format: super::HistogramFormat, numberofentries: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogramSize(format : super::HistogramFormat, numberofentries : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipBitmapGetHistogramSize(format, numberofentries as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapGetPixel(bitmap: *mut super::GpBitmap, x: super::INT, y: super::INT, color: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetPixel(bitmap : *mut super::GpBitmap, x : super::INT, y : super::INT, color : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipBitmapGetPixel(bitmap as _, x, y, color as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapLockBits(bitmap: *mut super::GpBitmap, rect: *const super::GpRect, flags: super::UINT, format: super::PixelFormat, lockedbitmapdata: *mut super::BitmapData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapLockBits(bitmap : *mut super::GpBitmap, rect : *const super::GpRect, flags : super::UINT, format : super::PixelFormat, lockedbitmapdata : *mut super::BitmapData) -> super::GpStatus);
    unsafe { GdipBitmapLockBits(bitmap as _, rect, flags, format, lockedbitmapdata as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapSetPixel(bitmap: *mut super::GpBitmap, x: super::INT, y: super::INT, color: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapSetPixel(bitmap : *mut super::GpBitmap, x : super::INT, y : super::INT, color : super::ARGB) -> super::GpStatus);
    unsafe { GdipBitmapSetPixel(bitmap as _, x, y, color) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipBitmapSetResolution(bitmap: *mut super::GpBitmap, xdpi: super::REAL, ydpi: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapSetResolution(bitmap : *mut super::GpBitmap, xdpi : super::REAL, ydpi : super::REAL) -> super::GpStatus);
    unsafe { GdipBitmapSetResolution(bitmap as _, xdpi, ydpi) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipBitmapUnlockBits(bitmap: *mut super::GpBitmap, lockedbitmapdata: *mut super::BitmapData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapUnlockBits(bitmap : *mut super::GpBitmap, lockedbitmapdata : *mut super::BitmapData) -> super::GpStatus);
    unsafe { GdipBitmapUnlockBits(bitmap as _, lockedbitmapdata as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipClearPathMarkers(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipClearPathMarkers(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipClearPathMarkers(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneBitmapArea(x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, format: super::PixelFormat, srcbitmap: *mut super::GpBitmap, dstbitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBitmapArea(x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, format : super::PixelFormat, srcbitmap : *mut super::GpBitmap, dstbitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCloneBitmapArea(x, y, width, height, format, srcbitmap as _, dstbitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCloneBitmapAreaI(x: super::INT, y: super::INT, width: super::INT, height: super::INT, format: super::PixelFormat, srcbitmap: *mut super::GpBitmap, dstbitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBitmapAreaI(x : super::INT, y : super::INT, width : super::INT, height : super::INT, format : super::PixelFormat, srcbitmap : *mut super::GpBitmap, dstbitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCloneBitmapAreaI(x, y, width, height, format, srcbitmap as _, dstbitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneBrush(brush: *mut super::GpBrush, clonebrush: *mut *mut super::GpBrush) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBrush(brush : *mut super::GpBrush, clonebrush : *mut *mut super::GpBrush) -> super::GpStatus);
    unsafe { GdipCloneBrush(brush as _, clonebrush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneCustomLineCap(customcap: *mut super::GpCustomLineCap, clonedcap: *mut *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneCustomLineCap(customcap : *mut super::GpCustomLineCap, clonedcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipCloneCustomLineCap(customcap as _, clonedcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneFont(font: *mut super::GpFont, clonefont: *mut *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneFont(font : *mut super::GpFont, clonefont : *mut *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipCloneFont(font as _, clonefont as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneFontFamily(fontfamily: *mut super::GpFontFamily, clonedfontfamily: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneFontFamily(fontfamily : *mut super::GpFontFamily, clonedfontfamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipCloneFontFamily(fontfamily as _, clonedfontfamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneImage(image: *mut super::GpImage, cloneimage: *mut *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneImage(image : *mut super::GpImage, cloneimage : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipCloneImage(image as _, cloneimage as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneImageAttributes(imageattr: *const super::GpImageAttributes, cloneimageattr: *mut *mut super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneImageAttributes(imageattr : *const super::GpImageAttributes, cloneimageattr : *mut *mut super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipCloneImageAttributes(imageattr, cloneimageattr as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneMatrix(matrix: *mut super::GpMatrix, clonematrix: *mut *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneMatrix(matrix : *mut super::GpMatrix, clonematrix : *mut *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipCloneMatrix(matrix as _, clonematrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipClonePath(path: *mut super::GpPath, clonepath: *mut *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipClonePath(path : *mut super::GpPath, clonepath : *mut *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipClonePath(path as _, clonepath as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipClonePen(pen: *mut super::GpPen, clonepen: *mut *mut super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipClonePen(pen : *mut super::GpPen, clonepen : *mut *mut super::GpPen) -> super::GpStatus);
    unsafe { GdipClonePen(pen as _, clonepen as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneRegion(region: *mut super::GpRegion, cloneregion: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneRegion(region : *mut super::GpRegion, cloneregion : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCloneRegion(region as _, cloneregion as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCloneStringFormat(format: *const super::GpStringFormat, newformat: *mut *mut super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneStringFormat(format : *const super::GpStringFormat, newformat : *mut *mut super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipCloneStringFormat(format, newformat as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipClosePathFigure(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipClosePathFigure(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipClosePathFigure(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipClosePathFigures(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipClosePathFigures(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipClosePathFigures(path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCombineRegionPath(region: *mut super::GpRegion, path: *mut super::GpPath, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionPath(region : *mut super::GpRegion, path : *mut super::GpPath, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipCombineRegionPath(region as _, path as _, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCombineRegionRect(region: *mut super::GpRegion, rect: *const super::GpRectF, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRect(region : *mut super::GpRegion, rect : *const super::GpRectF, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipCombineRegionRect(region as _, rect, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCombineRegionRectI(region: *mut super::GpRegion, rect: *const super::GpRect, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRectI(region : *mut super::GpRegion, rect : *const super::GpRect, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipCombineRegionRectI(region as _, rect, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCombineRegionRegion(region: *mut super::GpRegion, region2: *mut super::GpRegion, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRegion(region : *mut super::GpRegion, region2 : *mut super::GpRegion, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipCombineRegionRegion(region as _, region2 as _, combinemode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipComment(graphics: *mut super::GpGraphics, sizedata: super::UINT, data: *const super::BYTE) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipComment(graphics : *mut super::GpGraphics, sizedata : super::UINT, data : *const super::BYTE) -> super::GpStatus);
    unsafe { GdipComment(graphics as _, sizedata, data) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipConvertToEmfPlus(refgraphics: *const super::GpGraphics, metafile: *mut super::GpMetafile, conversionfailureflag: *mut super::INT, emftype: super::EmfType, description: *const super::WCHAR, out_metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlus(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipConvertToEmfPlus(refgraphics, metafile as _, conversionfailureflag as _, emftype, description, out_metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipConvertToEmfPlusToFile(refgraphics: *const super::GpGraphics, metafile: *mut super::GpMetafile, conversionfailureflag: *mut super::INT, filename: *const super::WCHAR, emftype: super::EmfType, description: *const super::WCHAR, out_metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToFile(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, filename : *const super::WCHAR, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipConvertToEmfPlusToFile(refgraphics, metafile as _, conversionfailureflag as _, filename, emftype, description, out_metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipConvertToEmfPlusToStream<P3>(refgraphics: *const super::GpGraphics, metafile: *mut super::GpMetafile, conversionfailureflag: *mut super::INT, stream: P3, emftype: super::EmfType, description: *const super::WCHAR, out_metafile: *mut *mut super::GpMetafile) -> super::GpStatus
where
    P3: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToStream(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, stream : *mut core::ffi::c_void, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipConvertToEmfPlusToStream(refgraphics, metafile as _, conversionfailureflag as _, stream.param().abi(), emftype, description, out_metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateAdjustableArrowCap(height: super::REAL, width: super::REAL, isfilled: bool, cap: *mut *mut super::GpAdjustableArrowCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateAdjustableArrowCap(height : super::REAL, width : super::REAL, isfilled : windows_core::BOOL, cap : *mut *mut super::GpAdjustableArrowCap) -> super::GpStatus);
    unsafe { GdipCreateAdjustableArrowCap(height, width, isfilled.into(), cap as _) }
}
#[cfg(all(feature = "ddraw", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromDirectDrawSurface<P0>(surface: P0, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus
where
    P0: windows_core::Param<super::IDirectDrawSurface7>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromDirectDrawSurface(surface : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromDirectDrawSurface(surface.param().abi(), bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromFile(filename: *const super::WCHAR, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFile(filename : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromFile(filename, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromFileICM(filename: *const super::WCHAR, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFileICM(filename : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromFileICM(filename, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromGdiDib(gdibitmapinfo: *const super::BITMAPINFO, gdibitmapdata: *mut core::ffi::c_void, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGdiDib(gdibitmapinfo : *const super::BITMAPINFO, gdibitmapdata : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromGdiDib(gdibitmapinfo, gdibitmapdata as _, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromGraphics(width: super::INT, height: super::INT, target: *mut super::GpGraphics, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGraphics(width : super::INT, height : super::INT, target : *mut super::GpGraphics, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromGraphics(width, height, target as _, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromHBITMAP(hbm: super::HBITMAP, hpal: super::HPALETTE, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHBITMAP(hbm : super::HBITMAP, hpal : super::HPALETTE, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromHBITMAP(hbm as _, hpal as _, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromHICON(hicon: super::HICON, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHICON(hicon : super::HICON, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromHICON(hicon as _, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromResource(hinstance: super::HINSTANCE, lpbitmapname: *const super::WCHAR, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromResource(hinstance : super::HINSTANCE, lpbitmapname : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromResource(hinstance as _, lpbitmapname, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromScan0(width: super::INT, height: super::INT, stride: super::INT, format: super::PixelFormat, scan0: Option<*const super::BYTE>, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromScan0(width : super::INT, height : super::INT, stride : super::INT, format : super::PixelFormat, scan0 : *const super::BYTE, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromScan0(width, height, stride, format, scan0.unwrap_or(core::mem::zeroed()) as _, bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromStream<P0>(stream: P0, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStream(stream : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromStream(stream.param().abi(), bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
#[inline]
pub unsafe fn GdipCreateBitmapFromStreamICM<P0>(stream: P0, bitmap: *mut *mut super::GpBitmap) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStreamICM(stream : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipCreateBitmapFromStreamICM(stream.param().abi(), bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateCachedBitmap(bitmap: *mut super::GpBitmap, graphics: *mut super::GpGraphics, cachedbitmap: *mut *mut super::GpCachedBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateCachedBitmap(bitmap : *mut super::GpBitmap, graphics : *mut super::GpGraphics, cachedbitmap : *mut *mut super::GpCachedBitmap) -> super::GpStatus);
    unsafe { GdipCreateCachedBitmap(bitmap as _, graphics as _, cachedbitmap as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateCustomLineCap(fillpath: *mut super::GpPath, strokepath: *mut super::GpPath, basecap: super::GpLineCap, baseinset: super::REAL, customcap: *mut *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateCustomLineCap(fillpath : *mut super::GpPath, strokepath : *mut super::GpPath, basecap : super::GpLineCap, baseinset : super::REAL, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipCreateCustomLineCap(fillpath as _, strokepath as _, basecap, baseinset, customcap as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateFont(fontfamily: *const super::GpFontFamily, emsize: super::REAL, style: super::INT, unit: super::Unit, font: *mut *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFont(fontfamily : *const super::GpFontFamily, emsize : super::REAL, style : super::INT, unit : super::Unit, font : *mut *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipCreateFont(fontfamily, emsize, style, unit, font as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateFontFamilyFromName(name: *const super::WCHAR, fontcollection: *mut super::GpFontCollection, fontfamily: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFamilyFromName(name : *const super::WCHAR, fontcollection : *mut super::GpFontCollection, fontfamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipCreateFontFamilyFromName(name, fontcollection as _, fontfamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateFontFromDC(hdc: super::HDC, font: *mut *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromDC(hdc : super::HDC, font : *mut *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipCreateFontFromDC(hdc as _, font as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontA(hdc: super::HDC, logfont: *const super::LOGFONTA, font: *mut *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontA(hdc : super::HDC, logfont : *const super::LOGFONTA, font : *mut *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipCreateFontFromLogfontA(hdc as _, logfont, font as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontW(hdc: super::HDC, logfont: *const super::LOGFONTW, font: *mut *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontW(hdc : super::HDC, logfont : *const super::LOGFONTW, font : *mut *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipCreateFontFromLogfontW(hdc as _, logfont, font as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateFromHDC(hdc: super::HDC, graphics: *mut *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHDC(hdc : super::HDC, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipCreateFromHDC(hdc as _, graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateFromHDC2(hdc: super::HDC, hdevice: super::HANDLE, graphics: *mut *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHDC2(hdc : super::HDC, hdevice : super::HANDLE, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipCreateFromHDC2(hdc as _, hdevice as _, graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateFromHWND(hwnd: super::HWND, graphics: *mut *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHWND(hwnd : super::HWND, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipCreateFromHWND(hwnd as _, graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateFromHWNDICM(hwnd: super::HWND, graphics: *mut *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHWNDICM(hwnd : super::HWND, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipCreateFromHWNDICM(hwnd as _, graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateHBITMAPFromBitmap(bitmap: *mut super::GpBitmap, hbmreturn: *mut super::HBITMAP, background: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHBITMAPFromBitmap(bitmap : *mut super::GpBitmap, hbmreturn : *mut super::HBITMAP, background : super::ARGB) -> super::GpStatus);
    unsafe { GdipCreateHBITMAPFromBitmap(bitmap as _, hbmreturn as _, background) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateHICONFromBitmap(bitmap: *mut super::GpBitmap, hbmreturn: *mut super::HICON) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHICONFromBitmap(bitmap : *mut super::GpBitmap, hbmreturn : *mut super::HICON) -> super::GpStatus);
    unsafe { GdipCreateHICONFromBitmap(bitmap as _, hbmreturn as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GdipCreateHalftonePalette() -> super::HPALETTE {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHalftonePalette() -> super::HPALETTE);
    unsafe { GdipCreateHalftonePalette() }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateHatchBrush(hatchstyle: super::GpHatchStyle, forecol: super::ARGB, backcol: super::ARGB, brush: *mut *mut super::GpHatch) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHatchBrush(hatchstyle : super::GpHatchStyle, forecol : super::ARGB, backcol : super::ARGB, brush : *mut *mut super::GpHatch) -> super::GpStatus);
    unsafe { GdipCreateHatchBrush(hatchstyle, forecol, backcol, brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateImageAttributes(imageattr: *mut *mut super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateImageAttributes(imageattr : *mut *mut super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipCreateImageAttributes(imageattr as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateLineBrush(point1: *const super::GpPointF, point2: *const super::GpPointF, color1: super::ARGB, color2: super::ARGB, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrush(point1 : *const super::GpPointF, point2 : *const super::GpPointF, color1 : super::ARGB, color2 : super::ARGB, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrush(point1, point2, color1, color2, wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateLineBrushFromRect(rect: *const super::GpRectF, color1: super::ARGB, color2: super::ARGB, mode: super::LinearGradientMode, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRect(rect : *const super::GpRectF, color1 : super::ARGB, color2 : super::ARGB, mode : super::LinearGradientMode, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrushFromRect(rect, color1, color2, mode, wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectI(rect: *const super::GpRect, color1: super::ARGB, color2: super::ARGB, mode: super::LinearGradientMode, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectI(rect : *const super::GpRect, color1 : super::ARGB, color2 : super::ARGB, mode : super::LinearGradientMode, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrushFromRectI(rect, color1, color2, mode, wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngle(rect: *const super::GpRectF, color1: super::ARGB, color2: super::ARGB, angle: super::REAL, isanglescalable: bool, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngle(rect : *const super::GpRectF, color1 : super::ARGB, color2 : super::ARGB, angle : super::REAL, isanglescalable : windows_core::BOOL, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrushFromRectWithAngle(rect, color1, color2, angle, isanglescalable.into(), wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngleI(rect: *const super::GpRect, color1: super::ARGB, color2: super::ARGB, angle: super::REAL, isanglescalable: bool, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngleI(rect : *const super::GpRect, color1 : super::ARGB, color2 : super::ARGB, angle : super::REAL, isanglescalable : windows_core::BOOL, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrushFromRectWithAngleI(rect, color1, color2, angle, isanglescalable.into(), wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateLineBrushI(point1: *const super::GpPoint, point2: *const super::GpPoint, color1: super::ARGB, color2: super::ARGB, wrapmode: super::GpWrapMode, linegradient: *mut *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushI(point1 : *const super::GpPoint, point2 : *const super::GpPoint, color1 : super::ARGB, color2 : super::ARGB, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipCreateLineBrushI(point1, point2, color1, color2, wrapmode, linegradient as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateMatrix(matrix: *mut *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix(matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipCreateMatrix(matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateMatrix2(m11: super::REAL, m12: super::REAL, m21: super::REAL, m22: super::REAL, dx: super::REAL, dy: super::REAL, matrix: *mut *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix2(m11 : super::REAL, m12 : super::REAL, m21 : super::REAL, m22 : super::REAL, dx : super::REAL, dy : super::REAL, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipCreateMatrix2(m11, m12, m21, m22, dx, dy, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateMatrix3(rect: *const super::GpRectF, dstplg: *const super::GpPointF, matrix: *mut *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix3(rect : *const super::GpRectF, dstplg : *const super::GpPointF, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipCreateMatrix3(rect, dstplg, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateMatrix3I(rect: *const super::GpRect, dstplg: *const super::GpPoint, matrix: *mut *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix3I(rect : *const super::GpRect, dstplg : *const super::GpPoint, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipCreateMatrix3I(rect, dstplg, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipCreateMetafileFromEmf(hemf: super::HENHMETAFILE, deleteemf: bool, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromEmf(hemf : super::HENHMETAFILE, deleteemf : windows_core::BOOL, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipCreateMetafileFromEmf(hemf as _, deleteemf.into(), metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateMetafileFromFile(file: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromFile(file : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipCreateMetafileFromFile(file, metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
#[inline]
pub unsafe fn GdipCreateMetafileFromStream<P0>(stream: P0, metafile: *mut *mut super::GpMetafile) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromStream(stream : *mut core::ffi::c_void, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipCreateMetafileFromStream(stream.param().abi(), metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateMetafileFromWmf(hwmf: super::HMETAFILE, deletewmf: bool, wmfplaceablefileheader: *const super::WmfPlaceableFileHeader, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmf(hwmf : super::HMETAFILE, deletewmf : windows_core::BOOL, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipCreateMetafileFromWmf(hwmf as _, deletewmf.into(), wmfplaceablefileheader, metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateMetafileFromWmfFile(file: *const super::WCHAR, wmfplaceablefileheader: *const super::WmfPlaceableFileHeader, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmfFile(file : *const super::WCHAR, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipCreateMetafileFromWmfFile(file, wmfplaceablefileheader, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreatePath(brushmode: super::GpFillMode, path: *mut *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath(brushmode : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipCreatePath(brushmode, path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreatePath2(param0: *const super::GpPointF, param1: *const super::BYTE, param2: super::INT, param3: super::GpFillMode, path: *mut *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath2(param0 : *const super::GpPointF, param1 : *const super::BYTE, param2 : super::INT, param3 : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipCreatePath2(param0, param1, param2, param3, path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreatePath2I(param0: *const super::GpPoint, param1: *const super::BYTE, param2: super::INT, param3: super::GpFillMode, path: *mut *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath2I(param0 : *const super::GpPoint, param1 : *const super::BYTE, param2 : super::INT, param3 : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipCreatePath2I(param0, param1, param2, param3, path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreatePathGradient(points: *const super::GpPointF, count: super::INT, wrapmode: super::GpWrapMode, polygradient: *mut *mut super::GpPathGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradient(points : *const super::GpPointF, count : super::INT, wrapmode : super::GpWrapMode, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
    unsafe { GdipCreatePathGradient(points, count, wrapmode, polygradient as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreatePathGradientFromPath(path: *const super::GpPath, polygradient: *mut *mut super::GpPathGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradientFromPath(path : *const super::GpPath, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
    unsafe { GdipCreatePathGradientFromPath(path, polygradient as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreatePathGradientI(points: *const super::GpPoint, count: super::INT, wrapmode: super::GpWrapMode, polygradient: *mut *mut super::GpPathGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradientI(points : *const super::GpPoint, count : super::INT, wrapmode : super::GpWrapMode, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
    unsafe { GdipCreatePathGradientI(points, count, wrapmode, polygradient as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreatePathIter(iterator: *mut *mut super::GpPathIterator, path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathIter(iterator : *mut *mut super::GpPathIterator, path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipCreatePathIter(iterator as _, path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreatePen1(color: super::ARGB, width: super::REAL, unit: super::GpUnit, pen: *mut *mut super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePen1(color : super::ARGB, width : super::REAL, unit : super::GpUnit, pen : *mut *mut super::GpPen) -> super::GpStatus);
    unsafe { GdipCreatePen1(color, width, unit, pen as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreatePen2(brush: *mut super::GpBrush, width: super::REAL, unit: super::GpUnit, pen: *mut *mut super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePen2(brush : *mut super::GpBrush, width : super::REAL, unit : super::GpUnit, pen : *mut *mut super::GpPen) -> super::GpStatus);
    unsafe { GdipCreatePen2(brush as _, width, unit, pen as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateRegion(region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegion(region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegion(region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GdipCreateRegionHrgn(hrgn: super::HRGN, region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionHrgn(hrgn : super::HRGN, region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegionHrgn(hrgn as _, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateRegionPath(path: *mut super::GpPath, region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionPath(path : *mut super::GpPath, region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegionPath(path as _, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateRegionRect(rect: *const super::GpRectF, region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRect(rect : *const super::GpRectF, region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegionRect(rect, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateRegionRectI(rect: *const super::GpRect, region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRectI(rect : *const super::GpRect, region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegionRectI(rect, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateRegionRgnData(regiondata: *const super::BYTE, size: super::INT, region: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRgnData(regiondata : *const super::BYTE, size : super::INT, region : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipCreateRegionRgnData(regiondata, size, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateSolidFill(color: super::ARGB, brush: *mut *mut super::GpSolidFill) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateSolidFill(color : super::ARGB, brush : *mut *mut super::GpSolidFill) -> super::GpStatus);
    unsafe { GdipCreateSolidFill(color, brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipCreateStreamOnFile(filename: *const super::WCHAR, access: super::UINT, stream: *mut Option<super::IStream>) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateStreamOnFile(filename : *const super::WCHAR, access : super::UINT, stream : *mut *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipCreateStreamOnFile(filename, access, stream as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateStringFormat(formatattributes: super::INT, language: super::LANGID, format: *mut *mut super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateStringFormat(formatattributes : super::INT, language : super::LANGID, format : *mut *mut super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipCreateStringFormat(formatattributes, language, format as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateTexture(image: *mut super::GpImage, wrapmode: super::GpWrapMode, texture: *mut *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture(image : *mut super::GpImage, wrapmode : super::GpWrapMode, texture : *mut *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipCreateTexture(image as _, wrapmode, texture as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateTexture2(image: *mut super::GpImage, wrapmode: super::GpWrapMode, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, texture: *mut *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture2(image : *mut super::GpImage, wrapmode : super::GpWrapMode, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, texture : *mut *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipCreateTexture2(image as _, wrapmode, x, y, width, height, texture as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateTexture2I(image: *mut super::GpImage, wrapmode: super::GpWrapMode, x: super::INT, y: super::INT, width: super::INT, height: super::INT, texture: *mut *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture2I(image : *mut super::GpImage, wrapmode : super::GpWrapMode, x : super::INT, y : super::INT, width : super::INT, height : super::INT, texture : *mut *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipCreateTexture2I(image as _, wrapmode, x, y, width, height, texture as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipCreateTextureIA(image: *mut super::GpImage, imageattributes: *const super::GpImageAttributes, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, texture: *mut *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTextureIA(image : *mut super::GpImage, imageattributes : *const super::GpImageAttributes, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, texture : *mut *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipCreateTextureIA(image as _, imageattributes, x, y, width, height, texture as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipCreateTextureIAI(image: *mut super::GpImage, imageattributes: *const super::GpImageAttributes, x: super::INT, y: super::INT, width: super::INT, height: super::INT, texture: *mut *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTextureIAI(image : *mut super::GpImage, imageattributes : *const super::GpImageAttributes, x : super::INT, y : super::INT, width : super::INT, height : super::INT, texture : *mut *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipCreateTextureIAI(image as _, imageattributes, x, y, width, height, texture as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteBrush(brush: *mut super::GpBrush) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteBrush(brush : *mut super::GpBrush) -> super::GpStatus);
    unsafe { GdipDeleteBrush(brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteCachedBitmap(cachedbitmap: *mut super::GpCachedBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteCachedBitmap(cachedbitmap : *mut super::GpCachedBitmap) -> super::GpStatus);
    unsafe { GdipDeleteCachedBitmap(cachedbitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteCustomLineCap(customcap: *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteCustomLineCap(customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipDeleteCustomLineCap(customcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteFont(font: *mut super::GpFont) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteFont(font : *mut super::GpFont) -> super::GpStatus);
    unsafe { GdipDeleteFont(font as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteFontFamily(fontfamily: *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteFontFamily(fontfamily : *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipDeleteFontFamily(fontfamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteGraphics(graphics: *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteGraphics(graphics : *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipDeleteGraphics(graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteMatrix(matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteMatrix(matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipDeleteMatrix(matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeletePath(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePath(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipDeletePath(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeletePathIter(iterator: *mut super::GpPathIterator) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePathIter(iterator : *mut super::GpPathIterator) -> super::GpStatus);
    unsafe { GdipDeletePathIter(iterator as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeletePen(pen: *mut super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePen(pen : *mut super::GpPen) -> super::GpStatus);
    unsafe { GdipDeletePen(pen as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeletePrivateFontCollection(fontcollection: *mut *mut super::GpFontCollection) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePrivateFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
    unsafe { GdipDeletePrivateFontCollection(fontcollection as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteRegion(region: *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteRegion(region : *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipDeleteRegion(region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDeleteStringFormat(format: *mut super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteStringFormat(format : *mut super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipDeleteStringFormat(format as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDisposeImage(image: *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDisposeImage(image : *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipDisposeImage(image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDisposeImageAttributes(imageattr: *mut super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDisposeImageAttributes(imageattr : *mut super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipDisposeImageAttributes(imageattr as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawArc(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawArc(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawArc(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawArcI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::INT, y: super::INT, width: super::INT, height: super::INT, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawArcI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawArcI(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawBezier(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x1: super::REAL, y1: super::REAL, x2: super::REAL, y2: super::REAL, x3: super::REAL, y3: super::REAL, x4: super::REAL, y4: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBezier(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL, x3 : super::REAL, y3 : super::REAL, x4 : super::REAL, y4 : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawBezier(graphics as _, pen as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawBezierI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x1: super::INT, y1: super::INT, x2: super::INT, y2: super::INT, x3: super::INT, y3: super::INT, x4: super::INT, y4: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBezierI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT, x3 : super::INT, y3 : super::INT, x4 : super::INT, y4 : super::INT) -> super::GpStatus);
    unsafe { GdipDrawBezierI(graphics as _, pen as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawBeziers(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBeziers(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawBeziers(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawBeziersI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBeziersI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawBeziersI(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCachedBitmap(graphics: *mut super::GpGraphics, cachedbitmap: *mut super::GpCachedBitmap, x: super::INT, y: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCachedBitmap(graphics : *mut super::GpGraphics, cachedbitmap : *mut super::GpCachedBitmap, x : super::INT, y : super::INT) -> super::GpStatus);
    unsafe { GdipDrawCachedBitmap(graphics as _, cachedbitmap as _, x, y) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawClosedCurve(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawClosedCurve(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawClosedCurve2(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawClosedCurve2(graphics as _, pen as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawClosedCurve2I(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawClosedCurve2I(graphics as _, pen as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawClosedCurveI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurveI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawClosedCurveI(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurve(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawCurve(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurve2(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve2(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawCurve2(graphics as _, pen as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurve2I(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve2I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawCurve2I(graphics as _, pen as _, points, count, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurve3(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT, offset: super::INT, numberofsegments: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve3(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawCurve3(graphics as _, pen as _, points, count, offset, numberofsegments, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurve3I(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT, offset: super::INT, numberofsegments: super::INT, tension: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve3I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawCurve3I(graphics as _, pen as _, points, count, offset, numberofsegments, tension) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawCurveI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurveI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawCurveI(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawDriverString(graphics: *mut super::GpGraphics, text: *const super::UINT16, length: super::INT, font: *const super::GpFont, brush: *const super::GpBrush, positions: *const super::PointF, flags: super::INT, matrix: *const super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawDriverString(graphics : *mut super::GpGraphics, text : *const super::UINT16, length : super::INT, font : *const super::GpFont, brush : *const super::GpBrush, positions : *const super::PointF, flags : super::INT, matrix : *const super::GpMatrix) -> super::GpStatus);
    unsafe { GdipDrawDriverString(graphics as _, text, length, font, brush, positions, flags, matrix) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawEllipse(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawEllipse(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawEllipse(graphics as _, pen as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawEllipseI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawEllipseI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipDrawEllipseI(graphics as _, pen as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawImage(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::REAL, y: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImage(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawImage(graphics as _, image as _, x, y) }
}
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawImageFX(graphics: *mut super::GpGraphics, image: *mut super::GpImage, source: *mut super::GpRectF, xform: *mut super::GpMatrix, effect: *mut super::CGpEffect, imageattributes: *mut super::GpImageAttributes, srcunit: super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageFX(graphics : *mut super::GpGraphics, image : *mut super::GpImage, source : *mut super::GpRectF, xform : *mut super::GpMatrix, effect : *mut super::CGpEffect, imageattributes : *mut super::GpImageAttributes, srcunit : super::GpUnit) -> super::GpStatus);
    unsafe { GdipDrawImageFX(graphics as _, image as _, source as _, xform as _, effect as _, imageattributes as _, srcunit) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImageI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::INT, y: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT) -> super::GpStatus);
    unsafe { GdipDrawImageI(graphics as _, image as _, x, y) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawImagePointRect(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::REAL, y: super::REAL, srcx: super::REAL, srcy: super::REAL, srcwidth: super::REAL, srcheight: super::REAL, srcunit: super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit) -> super::GpStatus);
    unsafe { GdipDrawImagePointRect(graphics as _, image as _, x, y, srcx, srcy, srcwidth, srcheight, srcunit) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImagePointRectI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::INT, y: super::INT, srcx: super::INT, srcy: super::INT, srcwidth: super::INT, srcheight: super::INT, srcunit: super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit) -> super::GpStatus);
    unsafe { GdipDrawImagePointRectI(graphics as _, image as _, x, y, srcx, srcy, srcwidth, srcheight, srcunit) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImagePoints(graphics: *mut super::GpGraphics, image: *mut super::GpImage, dstpoints: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePoints(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstpoints : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawImagePoints(graphics as _, image as _, dstpoints, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImagePointsI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, dstpoints: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstpoints : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawImagePointsI(graphics as _, image as _, dstpoints, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImagePointsRect(graphics: *mut super::GpGraphics, image: *mut super::GpImage, points: *const super::GpPointF, count: super::INT, srcx: super::REAL, srcy: super::REAL, srcwidth: super::REAL, srcheight: super::REAL, srcunit: super::GpUnit, imageattributes: *const super::GpImageAttributes, callback: super::DrawImageAbort, callbackdata: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, points : *const super::GpPointF, count : super::INT, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipDrawImagePointsRect(graphics as _, image as _, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImagePointsRectI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, points: *const super::GpPoint, count: super::INT, srcx: super::INT, srcy: super::INT, srcwidth: super::INT, srcheight: super::INT, srcunit: super::GpUnit, imageattributes: *const super::GpImageAttributes, callback: super::DrawImageAbort, callbackdata: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, points : *const super::GpPoint, count : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipDrawImagePointsRectI(graphics as _, image as _, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawImageRect(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawImageRect(graphics as _, image as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImageRectI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipDrawImageRectI(graphics as _, image as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawImageRectRect(graphics: *mut super::GpGraphics, image: *mut super::GpImage, dstx: super::REAL, dsty: super::REAL, dstwidth: super::REAL, dstheight: super::REAL, srcx: super::REAL, srcy: super::REAL, srcwidth: super::REAL, srcheight: super::REAL, srcunit: super::GpUnit, imageattributes: *const super::GpImageAttributes, callback: super::DrawImageAbort, callbackdata: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstx : super::REAL, dsty : super::REAL, dstwidth : super::REAL, dstheight : super::REAL, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipDrawImageRectRect(graphics as _, image as _, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawImageRectRectI(graphics: *mut super::GpGraphics, image: *mut super::GpImage, dstx: super::INT, dsty: super::INT, dstwidth: super::INT, dstheight: super::INT, srcx: super::INT, srcy: super::INT, srcwidth: super::INT, srcheight: super::INT, srcunit: super::GpUnit, imageattributes: *const super::GpImageAttributes, callback: super::DrawImageAbort, callbackdata: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstx : super::INT, dsty : super::INT, dstwidth : super::INT, dstheight : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipDrawImageRectRectI(graphics as _, image as _, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawLine(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x1: super::REAL, y1: super::REAL, x2: super::REAL, y2: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLine(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawLine(graphics as _, pen as _, x1, y1, x2, y2) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawLineI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x1: super::INT, y1: super::INT, x2: super::INT, y2: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLineI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT) -> super::GpStatus);
    unsafe { GdipDrawLineI(graphics as _, pen as _, x1, y1, x2, y2) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawLines(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLines(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawLines(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawLinesI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLinesI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawLinesI(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawPath(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPath(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipDrawPath(graphics as _, pen as _, path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawPie(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPie(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawPie(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawPieI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::INT, y: super::INT, width: super::INT, height: super::INT, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPieI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawPieI(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawPolygon(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPolygon(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawPolygon(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawPolygonI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPolygonI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawPolygonI(graphics as _, pen as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipDrawRectangle(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangle(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipDrawRectangle(graphics as _, pen as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawRectangleI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangleI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipDrawRectangleI(graphics as _, pen as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawRectangles(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, rects: *const super::GpRectF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangles(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawRectangles(graphics as _, pen as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipDrawRectanglesI(graphics: *mut super::GpGraphics, pen: *mut super::GpPen, rects: *const super::GpRect, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectanglesI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
    unsafe { GdipDrawRectanglesI(graphics as _, pen as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipDrawString(graphics: *mut super::GpGraphics, string: *const super::WCHAR, length: super::INT, font: *const super::GpFont, layoutrect: *const super::RectF, stringformat: *const super::GpStringFormat, brush: *const super::GpBrush) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawString(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, brush : *const super::GpBrush) -> super::GpStatus);
    unsafe { GdipDrawString(graphics as _, string, length, font, layoutrect, stringformat, brush) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GdipEmfToWmfBits(hemf: super::HENHMETAFILE, cbdata16: super::UINT, pdata16: Option<super::LPBYTE>, imapmode: super::INT, eflags: super::INT) -> super::UINT {
    windows_core::link!("gdiplus.dll" "system" fn GdipEmfToWmfBits(hemf : super::HENHMETAFILE, cbdata16 : super::UINT, pdata16 : super::LPBYTE, imapmode : super::INT, eflags : super::INT) -> super::UINT);
    unsafe { GdipEmfToWmfBits(hemf as _, cbdata16, pdata16.unwrap_or(core::mem::zeroed()) as _, imapmode, eflags) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipEndContainer(graphics: *mut super::GpGraphics, state: super::GraphicsContainer) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEndContainer(graphics : *mut super::GpGraphics, state : super::GraphicsContainer) -> super::GpStatus);
    unsafe { GdipEndContainer(graphics as _, state) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoint(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoint: *const super::PointF, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoint(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::PointF, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestPoint(graphics as _, metafile, destpoint, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoint: *const super::Point, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::Point, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestPointI(graphics as _, metafile, destpoint, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoints(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoints: *const super::PointF, count: super::INT, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoints(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::PointF, count : super::INT, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestPoints(graphics as _, metafile, destpoints, count, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointsI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoints: *const super::Point, count: super::INT, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointsI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::Point, count : super::INT, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestPointsI(graphics as _, metafile, destpoints, count, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRect(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destrect: *const super::RectF, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRect(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::RectF, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestRect(graphics as _, metafile, destrect, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRectI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destrect: *const super::Rect, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRectI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::Rect, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileDestRectI(graphics as _, metafile, destrect, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoint(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoint: *const super::PointF, srcrect: *const super::RectF, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoint(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::PointF, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestPoint(graphics as _, metafile, destpoint, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoint: *const super::Point, srcrect: *const super::Rect, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::Point, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestPointI(graphics as _, metafile, destpoint, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoints(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoints: *const super::PointF, count: super::INT, srcrect: *const super::RectF, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoints(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::PointF, count : super::INT, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestPoints(graphics as _, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointsI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destpoints: *const super::Point, count: super::INT, srcrect: *const super::Rect, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointsI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::Point, count : super::INT, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestPointsI(graphics as _, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRect(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destrect: *const super::RectF, srcrect: *const super::RectF, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRect(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::RectF, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestRect(graphics as _, metafile, destrect, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRectI(graphics: *mut super::GpGraphics, metafile: *const super::GpMetafile, destrect: *const super::Rect, srcrect: *const super::Rect, srcunit: super::Unit, callback: super::EnumerateMetafileProc, callbackdata: *mut core::ffi::c_void, imageattributes: *const super::GpImageAttributes) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRectI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::Rect, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
    unsafe { GdipEnumerateMetafileSrcRectDestRectI(graphics as _, metafile, destrect, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillClosedCurve(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillClosedCurve(graphics as _, brush as _, points, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillClosedCurve2(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPointF, count: super::INT, tension: super::REAL, fillmode: super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT, tension : super::REAL, fillmode : super::GpFillMode) -> super::GpStatus);
    unsafe { GdipFillClosedCurve2(graphics as _, brush as _, points, count, tension, fillmode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillClosedCurve2I(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPoint, count: super::INT, tension: super::REAL, fillmode: super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2I(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT, tension : super::REAL, fillmode : super::GpFillMode) -> super::GpStatus);
    unsafe { GdipFillClosedCurve2I(graphics as _, brush as _, points, count, tension, fillmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillClosedCurveI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurveI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillClosedCurveI(graphics as _, brush as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFillEllipse(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillEllipse(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipFillEllipse(graphics as _, brush as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillEllipseI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillEllipseI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipFillEllipseI(graphics as _, brush as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFillPath(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPath(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipFillPath(graphics as _, brush as _, path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFillPie(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPie(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipFillPie(graphics as _, brush as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillPieI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::INT, y: super::INT, width: super::INT, height: super::INT, startangle: super::REAL, sweepangle: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPieI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
    unsafe { GdipFillPieI(graphics as _, brush as _, x, y, width, height, startangle, sweepangle) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillPolygon(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPointF, count: super::INT, fillmode: super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT, fillmode : super::GpFillMode) -> super::GpStatus);
    unsafe { GdipFillPolygon(graphics as _, brush as _, points, count, fillmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillPolygon2(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon2(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillPolygon2(graphics as _, brush as _, points, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillPolygon2I(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon2I(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillPolygon2I(graphics as _, brush as _, points, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillPolygonI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, points: *const super::GpPoint, count: super::INT, fillmode: super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygonI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT, fillmode : super::GpFillMode) -> super::GpStatus);
    unsafe { GdipFillPolygonI(graphics as _, brush as _, points, count, fillmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFillRectangle(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangle(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
    unsafe { GdipFillRectangle(graphics as _, brush as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillRectangleI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, x: super::INT, y: super::INT, width: super::INT, height: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangleI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
    unsafe { GdipFillRectangleI(graphics as _, brush as _, x, y, width, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillRectangles(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, rects: *const super::GpRectF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangles(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillRectangles(graphics as _, brush as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFillRectanglesI(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, rects: *const super::GpRect, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectanglesI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
    unsafe { GdipFillRectanglesI(graphics as _, brush as _, rects, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFillRegion(graphics: *mut super::GpGraphics, brush: *mut super::GpBrush, region: *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRegion(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, region : *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipFillRegion(graphics as _, brush as _, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFindFirstImageItem(image: *mut super::GpImage, item: *mut super::ImageItemData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFindFirstImageItem(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
    unsafe { GdipFindFirstImageItem(image as _, item as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipFindNextImageItem(image: *mut super::GpImage, item: *mut super::ImageItemData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFindNextImageItem(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
    unsafe { GdipFindNextImageItem(image as _, item as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFlattenPath(path: *mut super::GpPath, matrix: *mut super::GpMatrix, flatness: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFlattenPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
    unsafe { GdipFlattenPath(path as _, matrix as _, flatness) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipFlush(graphics: *mut super::GpGraphics, intention: super::GpFlushIntention) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipFlush(graphics : *mut super::GpGraphics, intention : super::GpFlushIntention) -> super::GpStatus);
    unsafe { GdipFlush(graphics as _, intention) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapFillState(cap: *mut super::GpAdjustableArrowCap, fillstate: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapFillState(cap : *mut super::GpAdjustableArrowCap, fillstate : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipGetAdjustableArrowCapFillState(cap as _, fillstate as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapHeight(cap: *mut super::GpAdjustableArrowCap, height: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapHeight(cap : *mut super::GpAdjustableArrowCap, height : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetAdjustableArrowCapHeight(cap as _, height as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapMiddleInset(cap: *mut super::GpAdjustableArrowCap, middleinset: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapMiddleInset(cap : *mut super::GpAdjustableArrowCap, middleinset : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetAdjustableArrowCapMiddleInset(cap as _, middleinset as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapWidth(cap: *mut super::GpAdjustableArrowCap, width: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapWidth(cap : *mut super::GpAdjustableArrowCap, width : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetAdjustableArrowCapWidth(cap as _, width as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipGetAllPropertyItems(image: *mut super::GpImage, totalbuffersize: super::UINT, numproperties: super::UINT, allitems: *mut super::PropertyItem) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAllPropertyItems(image : *mut super::GpImage, totalbuffersize : super::UINT, numproperties : super::UINT, allitems : *mut super::PropertyItem) -> super::GpStatus);
    unsafe { GdipGetAllPropertyItems(image as _, totalbuffersize, numproperties, allitems as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetBrushType(brush: *mut super::GpBrush, r#type: *mut super::GpBrushType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetBrushType(brush : *mut super::GpBrush, r#type : *mut super::GpBrushType) -> super::GpStatus);
    unsafe { GdipGetBrushType(brush as _, r#type as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetCellAscent(family: *const super::GpFontFamily, style: super::INT, cellascent: *mut super::UINT16) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCellAscent(family : *const super::GpFontFamily, style : super::INT, cellascent : *mut super::UINT16) -> super::GpStatus);
    unsafe { GdipGetCellAscent(family, style, cellascent as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetCellDescent(family: *const super::GpFontFamily, style: super::INT, celldescent: *mut super::UINT16) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCellDescent(family : *const super::GpFontFamily, style : super::INT, celldescent : *mut super::UINT16) -> super::GpStatus);
    unsafe { GdipGetCellDescent(family, style, celldescent as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetClip(graphics: *mut super::GpGraphics, region: *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClip(graphics : *mut super::GpGraphics, region : *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipGetClip(graphics as _, region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetClipBounds(graphics: *mut super::GpGraphics, rect: *mut super::GpRectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClipBounds(graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
    unsafe { GdipGetClipBounds(graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetClipBoundsI(graphics: *mut super::GpGraphics, rect: *mut super::GpRect) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClipBoundsI(graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
    unsafe { GdipGetClipBoundsI(graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCompositingMode(graphics: *mut super::GpGraphics, compositingmode: *mut super::CompositingMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCompositingMode(graphics : *mut super::GpGraphics, compositingmode : *mut super::CompositingMode) -> super::GpStatus);
    unsafe { GdipGetCompositingMode(graphics as _, compositingmode as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCompositingQuality(graphics: *mut super::GpGraphics, compositingquality: *mut super::CompositingQuality) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCompositingQuality(graphics : *mut super::GpGraphics, compositingquality : *mut super::CompositingQuality) -> super::GpStatus);
    unsafe { GdipGetCompositingQuality(graphics as _, compositingquality as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseCap(customcap: *mut super::GpCustomLineCap, basecap: *mut super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseCap(customcap : *mut super::GpCustomLineCap, basecap : *mut super::GpLineCap) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapBaseCap(customcap as _, basecap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseInset(customcap: *mut super::GpCustomLineCap, inset: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseInset(customcap : *mut super::GpCustomLineCap, inset : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapBaseInset(customcap as _, inset as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeCaps(customcap: *mut super::GpCustomLineCap, startcap: *mut super::GpLineCap, endcap: *mut super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeCaps(customcap : *mut super::GpCustomLineCap, startcap : *mut super::GpLineCap, endcap : *mut super::GpLineCap) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapStrokeCaps(customcap as _, startcap as _, endcap as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeJoin(customcap: *mut super::GpCustomLineCap, linejoin: *mut super::GpLineJoin) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeJoin(customcap : *mut super::GpCustomLineCap, linejoin : *mut super::GpLineJoin) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapStrokeJoin(customcap as _, linejoin as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapType(customcap: *mut super::GpCustomLineCap, captype: *mut super::CustomLineCapType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapType(customcap : *mut super::GpCustomLineCap, captype : *mut super::CustomLineCapType) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapType(customcap as _, captype as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetCustomLineCapWidthScale(customcap: *mut super::GpCustomLineCap, widthscale: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapWidthScale(customcap : *mut super::GpCustomLineCap, widthscale : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetCustomLineCapWidthScale(customcap as _, widthscale as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipGetDC(graphics: *mut super::GpGraphics, hdc: *mut super::HDC) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDC(graphics : *mut super::GpGraphics, hdc : *mut super::HDC) -> super::GpStatus);
    unsafe { GdipGetDC(graphics as _, hdc as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetDpiX(graphics: *mut super::GpGraphics, dpi: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDpiX(graphics : *mut super::GpGraphics, dpi : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetDpiX(graphics as _, dpi as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetDpiY(graphics: *mut super::GpGraphics, dpi: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDpiY(graphics : *mut super::GpGraphics, dpi : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetDpiY(graphics as _, dpi as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetEmHeight(family: *const super::GpFontFamily, style: super::INT, emheight: *mut super::UINT16) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEmHeight(family : *const super::GpFontFamily, style : super::INT, emheight : *mut super::UINT16) -> super::GpStatus);
    unsafe { GdipGetEmHeight(family, style, emheight as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetEncoderParameterList(image: *mut super::GpImage, clsidencoder: *const super::CLSID, size: super::UINT, buffer: *mut super::EncoderParameters) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterList(image : *mut super::GpImage, clsidencoder : *const super::CLSID, size : super::UINT, buffer : *mut super::EncoderParameters) -> super::GpStatus);
    unsafe { GdipGetEncoderParameterList(image as _, clsidencoder, size, buffer as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetEncoderParameterListSize(image: *mut super::GpImage, clsidencoder: *const super::CLSID, size: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterListSize(image : *mut super::GpImage, clsidencoder : *const super::CLSID, size : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetEncoderParameterListSize(image as _, clsidencoder, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetFamily(font: *mut super::GpFont, family: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFamily(font : *mut super::GpFont, family : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipGetFamily(font as _, family as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetFamilyName(family: *const super::GpFontFamily, name: windows_core::PWSTR, language: super::LANGID) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFamilyName(family : *const super::GpFontFamily, name : windows_core::PWSTR, language : super::LANGID) -> super::GpStatus);
    unsafe { GdipGetFamilyName(family, name, language) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyCount(fontcollection: *mut super::GpFontCollection, numfound: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyCount(fontcollection : *mut super::GpFontCollection, numfound : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetFontCollectionFamilyCount(fontcollection as _, numfound as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyList(fontcollection: *const super::GpFontCollection, numsought: super::INT, gpfamilies: *mut *mut super::GpFontFamily, numfound: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyList(fontcollection : *const super::GpFontCollection, numsought : super::INT, gpfamilies : *mut *mut super::GpFontFamily, numfound : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetFontCollectionFamilyList(fontcollection, numsought, gpfamilies as _, numfound as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetFontHeight(font: *const super::GpFont, graphics: *const super::GpGraphics, height: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontHeight(font : *const super::GpFont, graphics : *const super::GpGraphics, height : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetFontHeight(font, graphics, height as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetFontHeightGivenDPI(font: *const super::GpFont, dpi: super::REAL, height: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontHeightGivenDPI(font : *const super::GpFont, dpi : super::REAL, height : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetFontHeightGivenDPI(font, dpi, height as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetFontSize(font: *mut super::GpFont, size: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontSize(font : *mut super::GpFont, size : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetFontSize(font as _, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetFontStyle(font: *mut super::GpFont, style: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontStyle(font : *mut super::GpFont, style : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetFontStyle(font as _, style as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetFontUnit(font: *mut super::GpFont, unit: *mut super::Unit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontUnit(font : *mut super::GpFont, unit : *mut super::Unit) -> super::GpStatus);
    unsafe { GdipGetFontUnit(font as _, unit as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetGenericFontFamilyMonospace(nativefamily: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilyMonospace(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipGetGenericFontFamilyMonospace(nativefamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetGenericFontFamilySansSerif(nativefamily: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySansSerif(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipGetGenericFontFamilySansSerif(nativefamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetGenericFontFamilySerif(nativefamily: *mut *mut super::GpFontFamily) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySerif(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
    unsafe { GdipGetGenericFontFamilySerif(nativefamily as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetHatchBackgroundColor(brush: *mut super::GpHatch, backcol: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchBackgroundColor(brush : *mut super::GpHatch, backcol : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetHatchBackgroundColor(brush as _, backcol as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetHatchForegroundColor(brush: *mut super::GpHatch, forecol: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchForegroundColor(brush : *mut super::GpHatch, forecol : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetHatchForegroundColor(brush as _, forecol as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetHatchStyle(brush: *mut super::GpHatch, hatchstyle: *mut super::GpHatchStyle) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchStyle(brush : *mut super::GpHatch, hatchstyle : *mut super::GpHatchStyle) -> super::GpStatus);
    unsafe { GdipGetHatchStyle(brush as _, hatchstyle as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipGetHemfFromMetafile(metafile: *mut super::GpMetafile, hemf: *mut super::HENHMETAFILE) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHemfFromMetafile(metafile : *mut super::GpMetafile, hemf : *mut super::HENHMETAFILE) -> super::GpStatus);
    unsafe { GdipGetHemfFromMetafile(metafile as _, hemf as _) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageAttributesAdjustedPalette(imageattr: *mut super::GpImageAttributes, colorpalette: *mut super::ColorPalette, coloradjusttype: super::ColorAdjustType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageAttributesAdjustedPalette(imageattr : *mut super::GpImageAttributes, colorpalette : *mut super::ColorPalette, coloradjusttype : super::ColorAdjustType) -> super::GpStatus);
    unsafe { GdipGetImageAttributesAdjustedPalette(imageattr as _, colorpalette as _, coloradjusttype) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageBounds(image: *mut super::GpImage, srcrect: *mut super::GpRectF, srcunit: *mut super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageBounds(image : *mut super::GpImage, srcrect : *mut super::GpRectF, srcunit : *mut super::GpUnit) -> super::GpStatus);
    unsafe { GdipGetImageBounds(image as _, srcrect as _, srcunit as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipGetImageDecoders(numdecoders: super::UINT, size: super::UINT, decoders: *mut super::ImageCodecInfo) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDecoders(numdecoders : super::UINT, size : super::UINT, decoders : *mut super::ImageCodecInfo) -> super::GpStatus);
    unsafe { GdipGetImageDecoders(numdecoders, size, decoders as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageDecodersSize(numdecoders: *mut super::UINT, size: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDecodersSize(numdecoders : *mut super::UINT, size : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetImageDecodersSize(numdecoders as _, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageDimension(image: *mut super::GpImage, width: *mut super::REAL, height: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDimension(image : *mut super::GpImage, width : *mut super::REAL, height : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetImageDimension(image as _, width as _, height as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipGetImageEncoders(numencoders: super::UINT, size: super::UINT, encoders: *mut super::ImageCodecInfo) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageEncoders(numencoders : super::UINT, size : super::UINT, encoders : *mut super::ImageCodecInfo) -> super::GpStatus);
    unsafe { GdipGetImageEncoders(numencoders, size, encoders as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageEncodersSize(numencoders: *mut super::UINT, size: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageEncodersSize(numencoders : *mut super::UINT, size : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetImageEncodersSize(numencoders as _, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageFlags(image: *mut super::GpImage, flags: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageFlags(image : *mut super::GpImage, flags : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetImageFlags(image as _, flags as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageGraphicsContext(image: *mut super::GpImage, graphics: *mut *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageGraphicsContext(image : *mut super::GpImage, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipGetImageGraphicsContext(image as _, graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageHeight(image: *mut super::GpImage, height: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageHeight(image : *mut super::GpImage, height : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetImageHeight(image as _, height as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageHorizontalResolution(image: *mut super::GpImage, resolution: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageHorizontalResolution(image : *mut super::GpImage, resolution : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetImageHorizontalResolution(image as _, resolution as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageItemData(image: *mut super::GpImage, item: *mut super::ImageItemData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageItemData(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
    unsafe { GdipGetImageItemData(image as _, item as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImagePalette(image: *mut super::GpImage, palette: *mut super::ColorPalette, size: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePalette(image : *mut super::GpImage, palette : *mut super::ColorPalette, size : super::INT) -> super::GpStatus);
    unsafe { GdipGetImagePalette(image as _, palette as _, size) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImagePaletteSize(image: *mut super::GpImage, size: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePaletteSize(image : *mut super::GpImage, size : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetImagePaletteSize(image as _, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImagePixelFormat(image: *mut super::GpImage, format: *mut super::PixelFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePixelFormat(image : *mut super::GpImage, format : *mut super::PixelFormat) -> super::GpStatus);
    unsafe { GdipGetImagePixelFormat(image as _, format as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageRawFormat(image: *mut super::GpImage, format: *mut windows_core::GUID) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageRawFormat(image : *mut super::GpImage, format : *mut windows_core::GUID) -> super::GpStatus);
    unsafe { GdipGetImageRawFormat(image as _, format as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageThumbnail(image: *mut super::GpImage, thumbwidth: super::UINT, thumbheight: super::UINT, thumbimage: *mut *mut super::GpImage, callback: super::GetThumbnailImageAbort, callbackdata: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageThumbnail(image : *mut super::GpImage, thumbwidth : super::UINT, thumbheight : super::UINT, thumbimage : *mut *mut super::GpImage, callback : super::GetThumbnailImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipGetImageThumbnail(image as _, thumbwidth, thumbheight, thumbimage as _, callback, callbackdata as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageType(image: *mut super::GpImage, r#type: *mut super::ImageType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageType(image : *mut super::GpImage, r#type : *mut super::ImageType) -> super::GpStatus);
    unsafe { GdipGetImageType(image as _, r#type as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetImageVerticalResolution(image: *mut super::GpImage, resolution: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageVerticalResolution(image : *mut super::GpImage, resolution : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetImageVerticalResolution(image as _, resolution as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetImageWidth(image: *mut super::GpImage, width: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageWidth(image : *mut super::GpImage, width : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetImageWidth(image as _, width as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetInterpolationMode(graphics: *mut super::GpGraphics, interpolationmode: *mut super::InterpolationMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetInterpolationMode(graphics : *mut super::GpGraphics, interpolationmode : *mut super::InterpolationMode) -> super::GpStatus);
    unsafe { GdipGetInterpolationMode(graphics as _, interpolationmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLineBlend(brush: *mut super::GpLineGradient, blend: *mut super::REAL, positions: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineBlend(brush : *mut super::GpLineGradient, blend : *mut super::REAL, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetLineBlend(brush as _, blend as _, positions as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLineBlendCount(brush: *mut super::GpLineGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineBlendCount(brush : *mut super::GpLineGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetLineBlendCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetLineColors(brush: *mut super::GpLineGradient, colors: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineColors(brush : *mut super::GpLineGradient, colors : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetLineColors(brush as _, colors as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetLineGammaCorrection(brush: *mut super::GpLineGradient, usegammacorrection: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineGammaCorrection(brush : *mut super::GpLineGradient, usegammacorrection : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipGetLineGammaCorrection(brush as _, usegammacorrection as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLinePresetBlend(brush: *mut super::GpLineGradient, blend: *mut super::ARGB, positions: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlend(brush : *mut super::GpLineGradient, blend : *mut super::ARGB, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetLinePresetBlend(brush as _, blend as _, positions as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLinePresetBlendCount(brush: *mut super::GpLineGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlendCount(brush : *mut super::GpLineGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetLinePresetBlendCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetLineRect(brush: *mut super::GpLineGradient, rect: *mut super::GpRectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineRect(brush : *mut super::GpLineGradient, rect : *mut super::GpRectF) -> super::GpStatus);
    unsafe { GdipGetLineRect(brush as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLineRectI(brush: *mut super::GpLineGradient, rect: *mut super::GpRect) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineRectI(brush : *mut super::GpLineGradient, rect : *mut super::GpRect) -> super::GpStatus);
    unsafe { GdipGetLineRectI(brush as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetLineSpacing(family: *const super::GpFontFamily, style: super::INT, linespacing: *mut super::UINT16) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineSpacing(family : *const super::GpFontFamily, style : super::INT, linespacing : *mut super::UINT16) -> super::GpStatus);
    unsafe { GdipGetLineSpacing(family, style, linespacing as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetLineTransform(brush: *mut super::GpLineGradient, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineTransform(brush : *mut super::GpLineGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetLineTransform(brush as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetLineWrapMode(brush: *mut super::GpLineGradient, wrapmode: *mut super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineWrapMode(brush : *mut super::GpLineGradient, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipGetLineWrapMode(brush as _, wrapmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipGetLogFontA(font: *mut super::GpFont, graphics: *mut super::GpGraphics, logfonta: *mut super::LOGFONTA) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLogFontA(font : *mut super::GpFont, graphics : *mut super::GpGraphics, logfonta : *mut super::LOGFONTA) -> super::GpStatus);
    unsafe { GdipGetLogFontA(font as _, graphics as _, logfonta as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn GdipGetLogFontW(font: *mut super::GpFont, graphics: *mut super::GpGraphics, logfontw: *mut super::LOGFONTW) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLogFontW(font : *mut super::GpFont, graphics : *mut super::GpGraphics, logfontw : *mut super::LOGFONTW) -> super::GpStatus);
    unsafe { GdipGetLogFontW(font as _, graphics as _, logfontw as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetMatrixElements(matrix: *const super::GpMatrix, matrixout: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMatrixElements(matrix : *const super::GpMatrix, matrixout : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetMatrixElements(matrix, matrixout as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetMetafileDownLevelRasterizationLimit(metafile: *const super::GpMetafile, metafilerasterizationlimitdpi: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileDownLevelRasterizationLimit(metafile : *const super::GpMetafile, metafilerasterizationlimitdpi : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetMetafileDownLevelRasterizationLimit(metafile, metafilerasterizationlimitdpi as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromEmf(hemf: super::HENHMETAFILE, header: *mut super::MetafileHeader) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromEmf(hemf : super::HENHMETAFILE, header : *mut super::MetafileHeader) -> super::GpStatus);
    unsafe { GdipGetMetafileHeaderFromEmf(hemf as _, header as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromFile(filename: *const super::WCHAR, header: *mut super::MetafileHeader) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromFile(filename : *const super::WCHAR, header : *mut super::MetafileHeader) -> super::GpStatus);
    unsafe { GdipGetMetafileHeaderFromFile(filename, header as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromMetafile(metafile: *mut super::GpMetafile, header: *mut super::MetafileHeader) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromMetafile(metafile : *mut super::GpMetafile, header : *mut super::MetafileHeader) -> super::GpStatus);
    unsafe { GdipGetMetafileHeaderFromMetafile(metafile as _, header as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromStream<P0>(stream: P0, header: *mut super::MetafileHeader) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromStream(stream : *mut core::ffi::c_void, header : *mut super::MetafileHeader) -> super::GpStatus);
    unsafe { GdipGetMetafileHeaderFromStream(stream.param().abi(), header as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromWmf(hwmf: super::HMETAFILE, wmfplaceablefileheader: *const super::WmfPlaceableFileHeader, header: *mut super::MetafileHeader) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromWmf(hwmf : super::HMETAFILE, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, header : *mut super::MetafileHeader) -> super::GpStatus);
    unsafe { GdipGetMetafileHeaderFromWmf(hwmf as _, wmfplaceablefileheader, header as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetNearestColor(graphics: *mut super::GpGraphics, argb: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetNearestColor(graphics : *mut super::GpGraphics, argb : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetNearestColor(graphics as _, argb as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPageScale(graphics: *mut super::GpGraphics, scale: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPageScale(graphics : *mut super::GpGraphics, scale : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetPageScale(graphics as _, scale as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPageUnit(graphics: *mut super::GpGraphics, unit: *mut super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPageUnit(graphics : *mut super::GpGraphics, unit : *mut super::GpUnit) -> super::GpStatus);
    unsafe { GdipGetPageUnit(graphics as _, unit as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathData(path: *mut super::GpPath, pathdata: *mut super::GpPathData) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathData(path : *mut super::GpPath, pathdata : *mut super::GpPathData) -> super::GpStatus);
    unsafe { GdipGetPathData(path as _, pathdata as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathFillMode(path: *mut super::GpPath, fillmode: *mut super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathFillMode(path : *mut super::GpPath, fillmode : *mut super::GpFillMode) -> super::GpStatus);
    unsafe { GdipGetPathFillMode(path as _, fillmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientBlend(brush: *mut super::GpPathGradient, blend: *mut super::REAL, positions: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlend(brush : *mut super::GpPathGradient, blend : *mut super::REAL, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientBlend(brush as _, blend as _, positions as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientBlendCount(brush: *mut super::GpPathGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlendCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientBlendCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientCenterColor(brush: *mut super::GpPathGradient, colors: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterColor(brush : *mut super::GpPathGradient, colors : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetPathGradientCenterColor(brush as _, colors as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientCenterPoint(brush: *mut super::GpPathGradient, points: *mut super::GpPointF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPoint(brush : *mut super::GpPathGradient, points : *mut super::GpPointF) -> super::GpStatus);
    unsafe { GdipGetPathGradientCenterPoint(brush as _, points as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientCenterPointI(brush: *mut super::GpPathGradient, points: *mut super::GpPoint) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPointI(brush : *mut super::GpPathGradient, points : *mut super::GpPoint) -> super::GpStatus);
    unsafe { GdipGetPathGradientCenterPointI(brush as _, points as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientFocusScales(brush: *mut super::GpPathGradient, xscale: *mut super::REAL, yscale: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientFocusScales(brush : *mut super::GpPathGradient, xscale : *mut super::REAL, yscale : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetPathGradientFocusScales(brush as _, xscale as _, yscale as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientGammaCorrection(brush: *mut super::GpPathGradient, usegammacorrection: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientGammaCorrection(brush : *mut super::GpPathGradient, usegammacorrection : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipGetPathGradientGammaCorrection(brush as _, usegammacorrection as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientPath(brush: *mut super::GpPathGradient, path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPath(brush : *mut super::GpPathGradient, path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipGetPathGradientPath(brush as _, path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientPointCount(brush: *mut super::GpPathGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPointCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientPointCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlend(brush: *mut super::GpPathGradient, blend: *mut super::ARGB, positions: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlend(brush : *mut super::GpPathGradient, blend : *mut super::ARGB, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientPresetBlend(brush as _, blend as _, positions as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlendCount(brush: *mut super::GpPathGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlendCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientPresetBlendCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientRect(brush: *mut super::GpPathGradient, rect: *mut super::GpRectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientRect(brush : *mut super::GpPathGradient, rect : *mut super::GpRectF) -> super::GpStatus);
    unsafe { GdipGetPathGradientRect(brush as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientRectI(brush: *mut super::GpPathGradient, rect: *mut super::GpRect) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientRectI(brush : *mut super::GpPathGradient, rect : *mut super::GpRect) -> super::GpStatus);
    unsafe { GdipGetPathGradientRectI(brush as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorCount(brush: *mut super::GpPathGradient, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientSurroundColorCount(brush as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorsWithCount(brush: *const super::GpPathGradient, color: *mut super::ARGB, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorsWithCount(brush : *const super::GpPathGradient, color : *mut super::ARGB, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPathGradientSurroundColorsWithCount(brush, color as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientTransform(brush: *mut super::GpPathGradient, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetPathGradientTransform(brush as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathGradientWrapMode(brush: *mut super::GpPathGradient, wrapmode: *mut super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientWrapMode(brush : *mut super::GpPathGradient, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipGetPathGradientWrapMode(brush as _, wrapmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathLastPoint(path: *mut super::GpPath, lastpoint: *mut super::GpPointF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathLastPoint(path : *mut super::GpPath, lastpoint : *mut super::GpPointF) -> super::GpStatus);
    unsafe { GdipGetPathLastPoint(path as _, lastpoint as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathPoints(param0: *mut super::GpPath, points: *mut super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathPoints(param0 : *mut super::GpPath, points : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPathPoints(param0 as _, points as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathPointsI(param0: *mut super::GpPath, points: *mut super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathPointsI(param0 : *mut super::GpPath, points : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPathPointsI(param0 as _, points as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathTypes(path: *const super::GpPath, types: *mut super::BYTE, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathTypes(path : *const super::GpPath, types : *mut super::BYTE, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPathTypes(path, types as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPathWorldBounds(path: *mut super::GpPath, bounds: *mut super::GpRectF, matrix: *const super::GpMatrix, pen: *const super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathWorldBounds(path : *mut super::GpPath, bounds : *mut super::GpRectF, matrix : *const super::GpMatrix, pen : *const super::GpPen) -> super::GpStatus);
    unsafe { GdipGetPathWorldBounds(path as _, bounds as _, matrix, pen) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPathWorldBoundsI(path: *mut super::GpPath, bounds: *mut super::GpRect, matrix: *const super::GpMatrix, pen: *const super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathWorldBoundsI(path : *mut super::GpPath, bounds : *mut super::GpRect, matrix : *const super::GpMatrix, pen : *const super::GpPen) -> super::GpStatus);
    unsafe { GdipGetPathWorldBoundsI(path as _, bounds as _, matrix, pen) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenBrushFill(pen: *mut super::GpPen, brush: *mut *mut super::GpBrush) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenBrushFill(pen : *mut super::GpPen, brush : *mut *mut super::GpBrush) -> super::GpStatus);
    unsafe { GdipGetPenBrushFill(pen as _, brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenColor(pen: *mut super::GpPen, argb: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenColor(pen : *mut super::GpPen, argb : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetPenColor(pen as _, argb as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPenCompoundArray(pen: *mut super::GpPen, dash: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCompoundArray(pen : *mut super::GpPen, dash : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPenCompoundArray(pen as _, dash as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPenCompoundCount(pen: *mut super::GpPen, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCompoundCount(pen : *mut super::GpPen, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPenCompoundCount(pen as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenCustomEndCap(pen: *mut super::GpPen, customcap: *mut *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCustomEndCap(pen : *mut super::GpPen, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipGetPenCustomEndCap(pen as _, customcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenCustomStartCap(pen: *mut super::GpPen, customcap: *mut *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCustomStartCap(pen : *mut super::GpPen, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipGetPenCustomStartCap(pen as _, customcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPenDashArray(pen: *mut super::GpPen, dash: *mut super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashArray(pen : *mut super::GpPen, dash : *mut super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipGetPenDashArray(pen as _, dash as _, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenDashCap197819(pen: *mut super::GpPen, dashcap: *mut super::GpDashCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashCap197819(pen : *mut super::GpPen, dashcap : *mut super::GpDashCap) -> super::GpStatus);
    unsafe { GdipGetPenDashCap197819(pen as _, dashcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPenDashCount(pen: *mut super::GpPen, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashCount(pen : *mut super::GpPen, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPenDashCount(pen as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenDashOffset(pen: *mut super::GpPen, offset: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashOffset(pen : *mut super::GpPen, offset : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetPenDashOffset(pen as _, offset as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenDashStyle(pen: *mut super::GpPen, dashstyle: *mut super::GpDashStyle) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashStyle(pen : *mut super::GpPen, dashstyle : *mut super::GpDashStyle) -> super::GpStatus);
    unsafe { GdipGetPenDashStyle(pen as _, dashstyle as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenEndCap(pen: *mut super::GpPen, endcap: *mut super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenEndCap(pen : *mut super::GpPen, endcap : *mut super::GpLineCap) -> super::GpStatus);
    unsafe { GdipGetPenEndCap(pen as _, endcap as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenFillType(pen: *mut super::GpPen, r#type: *mut super::GpPenType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenFillType(pen : *mut super::GpPen, r#type : *mut super::GpPenType) -> super::GpStatus);
    unsafe { GdipGetPenFillType(pen as _, r#type as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenLineJoin(pen: *mut super::GpPen, linejoin: *mut super::GpLineJoin) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenLineJoin(pen : *mut super::GpPen, linejoin : *mut super::GpLineJoin) -> super::GpStatus);
    unsafe { GdipGetPenLineJoin(pen as _, linejoin as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenMiterLimit(pen: *mut super::GpPen, miterlimit: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenMiterLimit(pen : *mut super::GpPen, miterlimit : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetPenMiterLimit(pen as _, miterlimit as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenMode(pen: *mut super::GpPen, penmode: *mut super::GpPenAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenMode(pen : *mut super::GpPen, penmode : *mut super::GpPenAlignment) -> super::GpStatus);
    unsafe { GdipGetPenMode(pen as _, penmode as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenStartCap(pen: *mut super::GpPen, startcap: *mut super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenStartCap(pen : *mut super::GpPen, startcap : *mut super::GpLineCap) -> super::GpStatus);
    unsafe { GdipGetPenStartCap(pen as _, startcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenTransform(pen: *mut super::GpPen, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenTransform(pen : *mut super::GpPen, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetPenTransform(pen as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenUnit(pen: *mut super::GpPen, unit: *mut super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenUnit(pen : *mut super::GpPen, unit : *mut super::GpUnit) -> super::GpStatus);
    unsafe { GdipGetPenUnit(pen as _, unit as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPenWidth(pen: *mut super::GpPen, width: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenWidth(pen : *mut super::GpPen, width : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetPenWidth(pen as _, width as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetPixelOffsetMode(graphics: *mut super::GpGraphics, pixeloffsetmode: *mut super::PixelOffsetMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPixelOffsetMode(graphics : *mut super::GpGraphics, pixeloffsetmode : *mut super::PixelOffsetMode) -> super::GpStatus);
    unsafe { GdipGetPixelOffsetMode(graphics as _, pixeloffsetmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPointCount(path: *mut super::GpPath, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPointCount(path : *mut super::GpPath, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetPointCount(path as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPropertyCount(image: *mut super::GpImage, numofproperty: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyCount(image : *mut super::GpImage, numofproperty : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetPropertyCount(image as _, numofproperty as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipGetPropertyIdList(image: *mut super::GpImage, numofproperty: super::UINT, list: *mut super::PROPID) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyIdList(image : *mut super::GpImage, numofproperty : super::UINT, list : *mut super::PROPID) -> super::GpStatus);
    unsafe { GdipGetPropertyIdList(image as _, numofproperty, list as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipGetPropertyItem(image: *mut super::GpImage, propid: super::PROPID, propsize: super::UINT, buffer: *mut super::PropertyItem) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyItem(image : *mut super::GpImage, propid : super::PROPID, propsize : super::UINT, buffer : *mut super::PropertyItem) -> super::GpStatus);
    unsafe { GdipGetPropertyItem(image as _, propid, propsize, buffer as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipGetPropertyItemSize(image: *mut super::GpImage, propid: super::PROPID, size: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyItemSize(image : *mut super::GpImage, propid : super::PROPID, size : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetPropertyItemSize(image as _, propid, size as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetPropertySize(image: *mut super::GpImage, totalbuffersize: *mut super::UINT, numproperties: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertySize(image : *mut super::GpImage, totalbuffersize : *mut super::UINT, numproperties : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetPropertySize(image as _, totalbuffersize as _, numproperties as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetRegionBounds(region: *mut super::GpRegion, graphics: *mut super::GpGraphics, rect: *mut super::GpRectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionBounds(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
    unsafe { GdipGetRegionBounds(region as _, graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionBoundsI(region: *mut super::GpRegion, graphics: *mut super::GpGraphics, rect: *mut super::GpRect) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionBoundsI(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
    unsafe { GdipGetRegionBoundsI(region as _, graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionData(region: *mut super::GpRegion, buffer: *mut super::BYTE, buffersize: super::UINT, sizefilled: Option<*mut super::UINT>) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionData(region : *mut super::GpRegion, buffer : *mut super::BYTE, buffersize : super::UINT, sizefilled : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetRegionData(region as _, buffer as _, buffersize, sizefilled.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionDataSize(region: *mut super::GpRegion, buffersize: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionDataSize(region : *mut super::GpRegion, buffersize : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetRegionDataSize(region as _, buffersize as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GdipGetRegionHRgn(region: *mut super::GpRegion, graphics: *mut super::GpGraphics, hrgn: *mut super::HRGN) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionHRgn(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, hrgn : *mut super::HRGN) -> super::GpStatus);
    unsafe { GdipGetRegionHRgn(region as _, graphics as _, hrgn as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionScans(region: *mut super::GpRegion, rects: *mut super::GpRectF, count: *mut super::INT, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScans(region : *mut super::GpRegion, rects : *mut super::GpRectF, count : *mut super::INT, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetRegionScans(region as _, rects as _, count as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionScansCount(region: *mut super::GpRegion, count: *mut super::UINT, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScansCount(region : *mut super::GpRegion, count : *mut super::UINT, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetRegionScansCount(region as _, count as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRegionScansI(region: *mut super::GpRegion, rects: *mut super::GpRect, count: *mut super::INT, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScansI(region : *mut super::GpRegion, rects : *mut super::GpRect, count : *mut super::INT, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetRegionScansI(region as _, rects as _, count as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetRenderingOrigin(graphics: *mut super::GpGraphics, x: *mut super::INT, y: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRenderingOrigin(graphics : *mut super::GpGraphics, x : *mut super::INT, y : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetRenderingOrigin(graphics as _, x as _, y as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetSmoothingMode(graphics: *mut super::GpGraphics, smoothingmode: *mut super::SmoothingMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetSmoothingMode(graphics : *mut super::GpGraphics, smoothingmode : *mut super::SmoothingMode) -> super::GpStatus);
    unsafe { GdipGetSmoothingMode(graphics as _, smoothingmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetSolidFillColor(brush: *mut super::GpSolidFill, color: *mut super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetSolidFillColor(brush : *mut super::GpSolidFill, color : *mut super::ARGB) -> super::GpStatus);
    unsafe { GdipGetSolidFillColor(brush as _, color as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetStringFormatAlign(format: *const super::GpStringFormat, align: *mut super::StringAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatAlign(format : *const super::GpStringFormat, align : *mut super::StringAlignment) -> super::GpStatus);
    unsafe { GdipGetStringFormatAlign(format, align as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatDigitSubstitution(format: *const super::GpStringFormat, language: *mut super::LANGID, substitute: *mut super::StringDigitSubstitute) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatDigitSubstitution(format : *const super::GpStringFormat, language : *mut super::LANGID, substitute : *mut super::StringDigitSubstitute) -> super::GpStatus);
    unsafe { GdipGetStringFormatDigitSubstitution(format, language as _, substitute as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatFlags(format: *const super::GpStringFormat, flags: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatFlags(format : *const super::GpStringFormat, flags : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetStringFormatFlags(format, flags as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatHotkeyPrefix(format: *const super::GpStringFormat, hotkeyprefix: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatHotkeyPrefix(format : *const super::GpStringFormat, hotkeyprefix : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetStringFormatHotkeyPrefix(format, hotkeyprefix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetStringFormatLineAlign(format: *const super::GpStringFormat, align: *mut super::StringAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatLineAlign(format : *const super::GpStringFormat, align : *mut super::StringAlignment) -> super::GpStatus);
    unsafe { GdipGetStringFormatLineAlign(format, align as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatMeasurableCharacterRangeCount(format: *const super::GpStringFormat, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatMeasurableCharacterRangeCount(format : *const super::GpStringFormat, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetStringFormatMeasurableCharacterRangeCount(format, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatTabStopCount(format: *const super::GpStringFormat, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStopCount(format : *const super::GpStringFormat, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipGetStringFormatTabStopCount(format, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetStringFormatTabStops(format: *const super::GpStringFormat, count: super::INT, firsttaboffset: *mut super::REAL, tabstops: *mut super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStops(format : *const super::GpStringFormat, count : super::INT, firsttaboffset : *mut super::REAL, tabstops : *mut super::REAL) -> super::GpStatus);
    unsafe { GdipGetStringFormatTabStops(format, count, firsttaboffset as _, tabstops as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetStringFormatTrimming(format: *const super::GpStringFormat, trimming: *mut super::StringTrimming) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTrimming(format : *const super::GpStringFormat, trimming : *mut super::StringTrimming) -> super::GpStatus);
    unsafe { GdipGetStringFormatTrimming(format, trimming as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetTextContrast(graphics: *mut super::GpGraphics, contrast: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextContrast(graphics : *mut super::GpGraphics, contrast : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipGetTextContrast(graphics as _, contrast as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetTextRenderingHint(graphics: *mut super::GpGraphics, mode: *mut super::TextRenderingHint) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextRenderingHint(graphics : *mut super::GpGraphics, mode : *mut super::TextRenderingHint) -> super::GpStatus);
    unsafe { GdipGetTextRenderingHint(graphics as _, mode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetTextureImage(brush: *mut super::GpTexture, image: *mut *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureImage(brush : *mut super::GpTexture, image : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipGetTextureImage(brush as _, image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetTextureTransform(brush: *mut super::GpTexture, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureTransform(brush : *mut super::GpTexture, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetTextureTransform(brush as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetTextureWrapMode(brush: *mut super::GpTexture, wrapmode: *mut super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureWrapMode(brush : *mut super::GpTexture, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipGetTextureWrapMode(brush as _, wrapmode as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetVisibleClipBounds(graphics: *mut super::GpGraphics, rect: *mut super::GpRectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBounds(graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
    unsafe { GdipGetVisibleClipBounds(graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetVisibleClipBoundsI(graphics: *mut super::GpGraphics, rect: *mut super::GpRect) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBoundsI(graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
    unsafe { GdipGetVisibleClipBoundsI(graphics as _, rect as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGetWorldTransform(graphics: *mut super::GpGraphics, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetWorldTransform(graphics : *mut super::GpGraphics, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipGetWorldTransform(graphics as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGraphicsClear(graphics: *mut super::GpGraphics, color: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipGraphicsClear(graphics : *mut super::GpGraphics, color : super::ARGB) -> super::GpStatus);
    unsafe { GdipGraphicsClear(graphics as _, color) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipGraphicsSetAbort<P1>(pgraphics: *mut super::GpGraphics, piabort: P1) -> super::GpStatus
where
    P1: windows_core::Param<super::GdiplusAbort>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipGraphicsSetAbort(pgraphics : *mut super::GpGraphics, piabort : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipGraphicsSetAbort(pgraphics as _, piabort.param().abi()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipImageForceValidation(image: *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageForceValidation(image : *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipImageForceValidation(image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipImageGetFrameCount(image: *mut super::GpImage, dimensionid: *const windows_core::GUID, count: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameCount(image : *mut super::GpImage, dimensionid : *const windows_core::GUID, count : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipImageGetFrameCount(image as _, dimensionid, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsCount(image: *mut super::GpImage, count: *mut super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsCount(image : *mut super::GpImage, count : *mut super::UINT) -> super::GpStatus);
    unsafe { GdipImageGetFrameDimensionsCount(image as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsList(image: *mut super::GpImage, dimensionids: *mut windows_core::GUID, count: super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsList(image : *mut super::GpImage, dimensionids : *mut windows_core::GUID, count : super::UINT) -> super::GpStatus);
    unsafe { GdipImageGetFrameDimensionsList(image as _, dimensionids as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipImageRotateFlip(image: *mut super::GpImage, rftype: super::RotateFlipType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageRotateFlip(image : *mut super::GpImage, rftype : super::RotateFlipType) -> super::GpStatus);
    unsafe { GdipImageRotateFlip(image as _, rftype) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipImageSelectActiveFrame(image: *mut super::GpImage, dimensionid: *const windows_core::GUID, frameindex: super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageSelectActiveFrame(image : *mut super::GpImage, dimensionid : *const windows_core::GUID, frameindex : super::UINT) -> super::GpStatus);
    unsafe { GdipImageSelectActiveFrame(image as _, dimensionid, frameindex) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipImageSetAbort<P1>(pimage: *mut super::GpImage, piabort: P1) -> super::GpStatus
where
    P1: windows_core::Param<super::GdiplusAbort>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipImageSetAbort(pimage : *mut super::GpImage, piabort : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipImageSetAbort(pimage as _, piabort.param().abi()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipInitializePalette(palette: *mut super::ColorPalette, palettetype: super::PaletteType, optimalcolors: super::INT, usetransparentcolor: bool, bitmap: *mut super::GpBitmap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipInitializePalette(palette : *mut super::ColorPalette, palettetype : super::PaletteType, optimalcolors : super::INT, usetransparentcolor : windows_core::BOOL, bitmap : *mut super::GpBitmap) -> super::GpStatus);
    unsafe { GdipInitializePalette(palette as _, palettetype, optimalcolors, usetransparentcolor.into(), bitmap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipInvertMatrix(matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipInvertMatrix(matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipInvertMatrix(matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsClipEmpty(graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsClipEmpty(graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsClipEmpty(graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsEmptyRegion(region: *mut super::GpRegion, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsEmptyRegion(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsEmptyRegion(region as _, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsEqualRegion(region: *mut super::GpRegion, region2: *mut super::GpRegion, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsEqualRegion(region : *mut super::GpRegion, region2 : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsEqualRegion(region as _, region2 as _, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsInfiniteRegion(region: *mut super::GpRegion, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsInfiniteRegion(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsInfiniteRegion(region as _, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsMatrixEqual(matrix: *const super::GpMatrix, matrix2: *const super::GpMatrix, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixEqual(matrix : *const super::GpMatrix, matrix2 : *const super::GpMatrix, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsMatrixEqual(matrix, matrix2, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsMatrixIdentity(matrix: *const super::GpMatrix, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixIdentity(matrix : *const super::GpMatrix, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsMatrixIdentity(matrix, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsMatrixInvertible(matrix: *const super::GpMatrix, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixInvertible(matrix : *const super::GpMatrix, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsMatrixInvertible(matrix, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPoint(path: *mut super::GpPath, x: super::REAL, y: super::REAL, pen: *mut super::GpPen, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPoint(path : *mut super::GpPath, x : super::REAL, y : super::REAL, pen : *mut super::GpPen, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsOutlineVisiblePathPoint(path as _, x, y, pen as _, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPointI(path: *mut super::GpPath, x: super::INT, y: super::INT, pen: *mut super::GpPen, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPointI(path : *mut super::GpPath, x : super::INT, y : super::INT, pen : *mut super::GpPen, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsOutlineVisiblePathPointI(path as _, x, y, pen as _, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsStyleAvailable(family: *const super::GpFontFamily, style: super::INT, isstyleavailable: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsStyleAvailable(family : *const super::GpFontFamily, style : super::INT, isstyleavailable : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsStyleAvailable(family, style, isstyleavailable as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisibleClipEmpty(graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleClipEmpty(graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleClipEmpty(graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisiblePathPoint(path: *mut super::GpPath, x: super::REAL, y: super::REAL, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPoint(path : *mut super::GpPath, x : super::REAL, y : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisiblePathPoint(path as _, x, y, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsVisiblePathPointI(path: *mut super::GpPath, x: super::INT, y: super::INT, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPointI(path : *mut super::GpPath, x : super::INT, y : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisiblePathPointI(path as _, x, y, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisiblePoint(graphics: *mut super::GpGraphics, x: super::REAL, y: super::REAL, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePoint(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisiblePoint(graphics as _, x, y, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsVisiblePointI(graphics: *mut super::GpGraphics, x: super::INT, y: super::INT, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePointI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisiblePointI(graphics as _, x, y, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisibleRect(graphics: *mut super::GpGraphics, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRect(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRect(graphics as _, x, y, width, height, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsVisibleRectI(graphics: *mut super::GpGraphics, x: super::INT, y: super::INT, width: super::INT, height: super::INT, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRectI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, width : super::INT, height : super::INT, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRectI(graphics as _, x, y, width, height, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisibleRegionPoint(region: *mut super::GpRegion, x: super::REAL, y: super::REAL, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPoint(region : *mut super::GpRegion, x : super::REAL, y : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRegionPoint(region as _, x, y, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsVisibleRegionPointI(region: *mut super::GpRegion, x: super::INT, y: super::INT, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPointI(region : *mut super::GpRegion, x : super::INT, y : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRegionPointI(region as _, x, y, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipIsVisibleRegionRect(region: *mut super::GpRegion, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRect(region : *mut super::GpRegion, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRegionRect(region as _, x, y, width, height, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipIsVisibleRegionRectI(region: *mut super::GpRegion, x: super::INT, y: super::INT, width: super::INT, height: super::INT, graphics: *mut super::GpGraphics, result: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRectI(region : *mut super::GpRegion, x : super::INT, y : super::INT, width : super::INT, height : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipIsVisibleRegionRectI(region as _, x, y, width, height, graphics as _, result as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipLoadImageFromFile(filename: *const super::WCHAR, image: *mut *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromFile(filename : *const super::WCHAR, image : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipLoadImageFromFile(filename, image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipLoadImageFromFileICM(filename: *const super::WCHAR, image: *mut *mut super::GpImage) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromFileICM(filename : *const super::WCHAR, image : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipLoadImageFromFileICM(filename, image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
#[inline]
pub unsafe fn GdipLoadImageFromStream<P0>(stream: P0, image: *mut *mut super::GpImage) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromStream(stream : *mut core::ffi::c_void, image : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipLoadImageFromStream(stream.param().abi(), image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
#[inline]
pub unsafe fn GdipLoadImageFromStreamICM<P0>(stream: P0, image: *mut *mut super::GpImage) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromStreamICM(stream : *mut core::ffi::c_void, image : *mut *mut super::GpImage) -> super::GpStatus);
    unsafe { GdipLoadImageFromStreamICM(stream.param().abi(), image as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipMeasureCharacterRanges(graphics: *mut super::GpGraphics, string: *const super::WCHAR, length: super::INT, font: *const super::GpFont, layoutrect: *const super::RectF, stringformat: *const super::GpStringFormat, regioncount: super::INT, regions: *mut *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureCharacterRanges(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, regioncount : super::INT, regions : *mut *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipMeasureCharacterRanges(graphics as _, string, length, font, layoutrect, stringformat, regioncount, regions as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipMeasureDriverString(graphics: *mut super::GpGraphics, text: *const super::UINT16, length: super::INT, font: *const super::GpFont, positions: *const super::PointF, flags: super::INT, matrix: *const super::GpMatrix, boundingbox: *mut super::RectF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureDriverString(graphics : *mut super::GpGraphics, text : *const super::UINT16, length : super::INT, font : *const super::GpFont, positions : *const super::PointF, flags : super::INT, matrix : *const super::GpMatrix, boundingbox : *mut super::RectF) -> super::GpStatus);
    unsafe { GdipMeasureDriverString(graphics as _, text, length, font, positions, flags, matrix, boundingbox as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipMeasureString(graphics: *mut super::GpGraphics, string: *const super::WCHAR, length: super::INT, font: *const super::GpFont, layoutrect: *const super::RectF, stringformat: *const super::GpStringFormat, boundingbox: *mut super::RectF, codepointsfitted: *mut super::INT, linesfilled: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureString(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, boundingbox : *mut super::RectF, codepointsfitted : *mut super::INT, linesfilled : *mut super::INT) -> super::GpStatus);
    unsafe { GdipMeasureString(graphics as _, string, length, font, layoutrect, stringformat, boundingbox as _, codepointsfitted as _, linesfilled as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyLineTransform(brush: *mut super::GpLineGradient, matrix: *const super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyLineTransform(brush : *mut super::GpLineGradient, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyLineTransform(brush as _, matrix, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyMatrix(matrix: *mut super::GpMatrix, matrix2: *mut super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyMatrix(matrix : *mut super::GpMatrix, matrix2 : *mut super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyMatrix(matrix as _, matrix2 as _, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyPathGradientTransform(brush: *mut super::GpPathGradient, matrix: *const super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyPathGradientTransform(brush as _, matrix, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyPenTransform(pen: *mut super::GpPen, matrix: *const super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyPenTransform(pen : *mut super::GpPen, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyPenTransform(pen as _, matrix, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyTextureTransform(brush: *mut super::GpTexture, matrix: *const super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyTextureTransform(brush : *mut super::GpTexture, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyTextureTransform(brush as _, matrix, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipMultiplyWorldTransform(graphics: *mut super::GpGraphics, matrix: *const super::GpMatrix, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyWorldTransform(graphics : *mut super::GpGraphics, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipMultiplyWorldTransform(graphics as _, matrix, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipNewInstalledFontCollection(fontcollection: *mut *mut super::GpFontCollection) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipNewInstalledFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
    unsafe { GdipNewInstalledFontCollection(fontcollection as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipNewPrivateFontCollection(fontcollection: *mut *mut super::GpFontCollection) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipNewPrivateFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
    unsafe { GdipNewPrivateFontCollection(fontcollection as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterCopyData(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, points: *mut super::GpPointF, types: *mut super::BYTE, startindex: super::INT, endindex: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterCopyData(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, points : *mut super::GpPointF, types : *mut super::BYTE, startindex : super::INT, endindex : super::INT) -> super::GpStatus);
    unsafe { GdipPathIterCopyData(iterator as _, resultcount as _, points as _, types as _, startindex, endindex) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterEnumerate(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, points: *mut super::GpPointF, types: *mut super::BYTE, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterEnumerate(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, points : *mut super::GpPointF, types : *mut super::BYTE, count : super::INT) -> super::GpStatus);
    unsafe { GdipPathIterEnumerate(iterator as _, resultcount as _, points as _, types as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterGetCount(iterator: *mut super::GpPathIterator, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterGetCount(iterator : *mut super::GpPathIterator, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipPathIterGetCount(iterator as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterGetSubpathCount(iterator: *mut super::GpPathIterator, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterGetSubpathCount(iterator : *mut super::GpPathIterator, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipPathIterGetSubpathCount(iterator as _, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipPathIterHasCurve(iterator: *mut super::GpPathIterator, hascurve: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterHasCurve(iterator : *mut super::GpPathIterator, hascurve : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipPathIterHasCurve(iterator as _, hascurve as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipPathIterIsValid(iterator: *mut super::GpPathIterator, valid: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterIsValid(iterator : *mut super::GpPathIterator, valid : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipPathIterIsValid(iterator as _, valid as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterNextMarker(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, startindex: *mut super::INT, endindex: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextMarker(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, startindex : *mut super::INT, endindex : *mut super::INT) -> super::GpStatus);
    unsafe { GdipPathIterNextMarker(iterator as _, resultcount as _, startindex as _, endindex as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterNextMarkerPath(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextMarkerPath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipPathIterNextMarkerPath(iterator as _, resultcount as _, path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterNextPathType(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, pathtype: *mut super::BYTE, startindex: *mut super::INT, endindex: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextPathType(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, pathtype : *mut super::BYTE, startindex : *mut super::INT, endindex : *mut super::INT) -> super::GpStatus);
    unsafe { GdipPathIterNextPathType(iterator as _, resultcount as _, pathtype as _, startindex as _, endindex as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterNextSubpath(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, startindex: *mut super::INT, endindex: *mut super::INT, isclosed: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, startindex : *mut super::INT, endindex : *mut super::INT, isclosed : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipPathIterNextSubpath(iterator as _, resultcount as _, startindex as _, endindex as _, isclosed as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPathIterNextSubpathPath(iterator: *mut super::GpPathIterator, resultcount: *mut super::INT, path: *mut super::GpPath, isclosed: *mut windows_core::BOOL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpathPath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, path : *mut super::GpPath, isclosed : *mut windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipPathIterNextSubpathPath(iterator as _, resultcount as _, path as _, isclosed as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipPathIterRewind(iterator: *mut super::GpPathIterator) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterRewind(iterator : *mut super::GpPathIterator) -> super::GpStatus);
    unsafe { GdipPathIterRewind(iterator as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPlayMetafileRecord(metafile: *const super::GpMetafile, recordtype: super::EmfPlusRecordType, flags: super::UINT, datasize: super::UINT, data: *const super::BYTE) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPlayMetafileRecord(metafile : *const super::GpMetafile, recordtype : super::EmfPlusRecordType, flags : super::UINT, datasize : super::UINT, data : *const super::BYTE) -> super::GpStatus);
    unsafe { GdipPlayMetafileRecord(metafile, recordtype, flags, datasize, data) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipPrivateAddFontFile(fontcollection: *mut super::GpFontCollection, filename: *const super::WCHAR) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPrivateAddFontFile(fontcollection : *mut super::GpFontCollection, filename : *const super::WCHAR) -> super::GpStatus);
    unsafe { GdipPrivateAddFontFile(fontcollection as _, filename) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipPrivateAddMemoryFont(fontcollection: *mut super::GpFontCollection, memory: *const core::ffi::c_void, length: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipPrivateAddMemoryFont(fontcollection : *mut super::GpFontCollection, memory : *const core::ffi::c_void, length : super::INT) -> super::GpStatus);
    unsafe { GdipPrivateAddMemoryFont(fontcollection as _, memory, length) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafile(referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRectF, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafile(referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafile(referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafileFileName(filename: *const super::WCHAR, referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRectF, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileName(filename : *const super::WCHAR, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafileFileName(filename, referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafileFileNameI(filename: *const super::WCHAR, referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRect, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileNameI(filename : *const super::WCHAR, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafileFileNameI(filename, referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafileI(referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRect, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileI(referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafileI(referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafileStream<P0>(stream: P0, referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRectF, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileStream(stream : *mut core::ffi::c_void, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafileStream(stream.param().abi(), referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GdipRecordMetafileStreamI<P0>(stream: P0, referencehdc: super::HDC, r#type: super::EmfType, framerect: *const super::GpRect, frameunit: super::MetafileFrameUnit, description: *const super::WCHAR, metafile: *mut *mut super::GpMetafile) -> super::GpStatus
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileStreamI(stream : *mut core::ffi::c_void, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
    unsafe { GdipRecordMetafileStreamI(stream.param().abi(), referencehdc as _, r#type, framerect, frameunit, description, metafile as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
#[inline]
pub unsafe fn GdipReleaseDC(graphics: *mut super::GpGraphics, hdc: super::HDC) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipReleaseDC(graphics : *mut super::GpGraphics, hdc : super::HDC) -> super::GpStatus);
    unsafe { GdipReleaseDC(graphics as _, hdc as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipRemovePropertyItem(image: *mut super::GpImage, propid: super::PROPID) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRemovePropertyItem(image : *mut super::GpImage, propid : super::PROPID) -> super::GpStatus);
    unsafe { GdipRemovePropertyItem(image as _, propid) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetClip(graphics: *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetClip(graphics : *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipResetClip(graphics as _) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetImageAttributes(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetImageAttributes(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType) -> super::GpStatus);
    unsafe { GdipResetImageAttributes(imageattr as _, r#type) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetLineTransform(brush: *mut super::GpLineGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetLineTransform(brush : *mut super::GpLineGradient) -> super::GpStatus);
    unsafe { GdipResetLineTransform(brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetPageTransform(graphics: *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPageTransform(graphics : *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipResetPageTransform(graphics as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetPath(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPath(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipResetPath(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetPathGradientTransform(brush: *mut super::GpPathGradient) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPathGradientTransform(brush : *mut super::GpPathGradient) -> super::GpStatus);
    unsafe { GdipResetPathGradientTransform(brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetPenTransform(pen: *mut super::GpPen) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPenTransform(pen : *mut super::GpPen) -> super::GpStatus);
    unsafe { GdipResetPenTransform(pen as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetTextureTransform(brush: *mut super::GpTexture) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetTextureTransform(brush : *mut super::GpTexture) -> super::GpStatus);
    unsafe { GdipResetTextureTransform(brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipResetWorldTransform(graphics: *mut super::GpGraphics) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetWorldTransform(graphics : *mut super::GpGraphics) -> super::GpStatus);
    unsafe { GdipResetWorldTransform(graphics as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRestoreGraphics(graphics: *mut super::GpGraphics, state: super::GraphicsState) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRestoreGraphics(graphics : *mut super::GpGraphics, state : super::GraphicsState) -> super::GpStatus);
    unsafe { GdipRestoreGraphics(graphics as _, state) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipReversePath(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipReversePath(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipReversePath(path as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotateLineTransform(brush: *mut super::GpLineGradient, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateLineTransform(brush : *mut super::GpLineGradient, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotateLineTransform(brush as _, angle, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotateMatrix(matrix: *mut super::GpMatrix, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateMatrix(matrix : *mut super::GpMatrix, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotateMatrix(matrix as _, angle, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotatePathGradientTransform(brush: *mut super::GpPathGradient, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotatePathGradientTransform(brush : *mut super::GpPathGradient, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotatePathGradientTransform(brush as _, angle, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotatePenTransform(pen: *mut super::GpPen, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotatePenTransform(pen : *mut super::GpPen, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotatePenTransform(pen as _, angle, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotateTextureTransform(brush: *mut super::GpTexture, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateTextureTransform(brush : *mut super::GpTexture, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotateTextureTransform(brush as _, angle, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipRotateWorldTransform(graphics: *mut super::GpGraphics, angle: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateWorldTransform(graphics : *mut super::GpGraphics, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipRotateWorldTransform(graphics as _, angle, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSaveAdd(image: *mut super::GpImage, encoderparams: *const super::EncoderParameters) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveAdd(image : *mut super::GpImage, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
    unsafe { GdipSaveAdd(image as _, encoderparams) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSaveAddImage(image: *mut super::GpImage, newimage: *mut super::GpImage, encoderparams: *const super::EncoderParameters) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveAddImage(image : *mut super::GpImage, newimage : *mut super::GpImage, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
    unsafe { GdipSaveAddImage(image as _, newimage as _, encoderparams) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSaveGraphics(graphics: *mut super::GpGraphics, state: *mut super::GraphicsState) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveGraphics(graphics : *mut super::GpGraphics, state : *mut super::GraphicsState) -> super::GpStatus);
    unsafe { GdipSaveGraphics(graphics as _, state as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GdipSaveImageToFile(image: *mut super::GpImage, filename: *const super::WCHAR, clsidencoder: *const super::CLSID, encoderparams: *const super::EncoderParameters) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveImageToFile(image : *mut super::GpImage, filename : *const super::WCHAR, clsidencoder : *const super::CLSID, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
    unsafe { GdipSaveImageToFile(image as _, filename, clsidencoder, encoderparams) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "objidlbase", feature = "shared"))]
#[inline]
pub unsafe fn GdipSaveImageToStream<P1>(image: *mut super::GpImage, stream: P1, clsidencoder: *const super::CLSID, encoderparams: *const super::EncoderParameters) -> super::GpStatus
where
    P1: windows_core::Param<super::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveImageToStream(image : *mut super::GpImage, stream : *mut core::ffi::c_void, clsidencoder : *const super::CLSID, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
    unsafe { GdipSaveImageToStream(image as _, stream.param().abi(), clsidencoder, encoderparams) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScaleLineTransform(brush: *mut super::GpLineGradient, sx: super::REAL, sy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleLineTransform(brush : *mut super::GpLineGradient, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScaleLineTransform(brush as _, sx, sy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScaleMatrix(matrix: *mut super::GpMatrix, scalex: super::REAL, scaley: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleMatrix(matrix : *mut super::GpMatrix, scalex : super::REAL, scaley : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScaleMatrix(matrix as _, scalex, scaley, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScalePathGradientTransform(brush: *mut super::GpPathGradient, sx: super::REAL, sy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScalePathGradientTransform(brush : *mut super::GpPathGradient, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScalePathGradientTransform(brush as _, sx, sy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScalePenTransform(pen: *mut super::GpPen, sx: super::REAL, sy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScalePenTransform(pen : *mut super::GpPen, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScalePenTransform(pen as _, sx, sy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScaleTextureTransform(brush: *mut super::GpTexture, sx: super::REAL, sy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleTextureTransform(brush : *mut super::GpTexture, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScaleTextureTransform(brush as _, sx, sy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipScaleWorldTransform(graphics: *mut super::GpGraphics, sx: super::REAL, sy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleWorldTransform(graphics : *mut super::GpGraphics, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipScaleWorldTransform(graphics as _, sx, sy, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapFillState(cap: *mut super::GpAdjustableArrowCap, fillstate: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapFillState(cap : *mut super::GpAdjustableArrowCap, fillstate : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetAdjustableArrowCapFillState(cap as _, fillstate.into()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapHeight(cap: *mut super::GpAdjustableArrowCap, height: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapHeight(cap : *mut super::GpAdjustableArrowCap, height : super::REAL) -> super::GpStatus);
    unsafe { GdipSetAdjustableArrowCapHeight(cap as _, height) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapMiddleInset(cap: *mut super::GpAdjustableArrowCap, middleinset: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapMiddleInset(cap : *mut super::GpAdjustableArrowCap, middleinset : super::REAL) -> super::GpStatus);
    unsafe { GdipSetAdjustableArrowCapMiddleInset(cap as _, middleinset) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapWidth(cap: *mut super::GpAdjustableArrowCap, width: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapWidth(cap : *mut super::GpAdjustableArrowCap, width : super::REAL) -> super::GpStatus);
    unsafe { GdipSetAdjustableArrowCapWidth(cap as _, width) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetClipGraphics(graphics: *mut super::GpGraphics, srcgraphics: *mut super::GpGraphics, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipGraphics(graphics : *mut super::GpGraphics, srcgraphics : *mut super::GpGraphics, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipGraphics(graphics as _, srcgraphics as _, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GdipSetClipHrgn(graphics: *mut super::GpGraphics, hrgn: super::HRGN, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipHrgn(graphics : *mut super::GpGraphics, hrgn : super::HRGN, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipHrgn(graphics as _, hrgn as _, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetClipPath(graphics: *mut super::GpGraphics, path: *mut super::GpPath, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipPath(graphics : *mut super::GpGraphics, path : *mut super::GpPath, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipPath(graphics as _, path as _, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetClipRect(graphics: *mut super::GpGraphics, x: super::REAL, y: super::REAL, width: super::REAL, height: super::REAL, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRect(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipRect(graphics as _, x, y, width, height, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetClipRectI(graphics: *mut super::GpGraphics, x: super::INT, y: super::INT, width: super::INT, height: super::INT, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRectI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, width : super::INT, height : super::INT, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipRectI(graphics as _, x, y, width, height, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetClipRegion(graphics: *mut super::GpGraphics, region: *mut super::GpRegion, combinemode: super::CombineMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRegion(graphics : *mut super::GpGraphics, region : *mut super::GpRegion, combinemode : super::CombineMode) -> super::GpStatus);
    unsafe { GdipSetClipRegion(graphics as _, region as _, combinemode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCompositingMode(graphics: *mut super::GpGraphics, compositingmode: super::CompositingMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCompositingMode(graphics : *mut super::GpGraphics, compositingmode : super::CompositingMode) -> super::GpStatus);
    unsafe { GdipSetCompositingMode(graphics as _, compositingmode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCompositingQuality(graphics: *mut super::GpGraphics, compositingquality: super::CompositingQuality) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCompositingQuality(graphics : *mut super::GpGraphics, compositingquality : super::CompositingQuality) -> super::GpStatus);
    unsafe { GdipSetCompositingQuality(graphics as _, compositingquality) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseCap(customcap: *mut super::GpCustomLineCap, basecap: super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseCap(customcap : *mut super::GpCustomLineCap, basecap : super::GpLineCap) -> super::GpStatus);
    unsafe { GdipSetCustomLineCapBaseCap(customcap as _, basecap) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseInset(customcap: *mut super::GpCustomLineCap, inset: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseInset(customcap : *mut super::GpCustomLineCap, inset : super::REAL) -> super::GpStatus);
    unsafe { GdipSetCustomLineCapBaseInset(customcap as _, inset) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeCaps(customcap: *mut super::GpCustomLineCap, startcap: super::GpLineCap, endcap: super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeCaps(customcap : *mut super::GpCustomLineCap, startcap : super::GpLineCap, endcap : super::GpLineCap) -> super::GpStatus);
    unsafe { GdipSetCustomLineCapStrokeCaps(customcap as _, startcap, endcap) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeJoin(customcap: *mut super::GpCustomLineCap, linejoin: super::GpLineJoin) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeJoin(customcap : *mut super::GpCustomLineCap, linejoin : super::GpLineJoin) -> super::GpStatus);
    unsafe { GdipSetCustomLineCapStrokeJoin(customcap as _, linejoin) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetCustomLineCapWidthScale(customcap: *mut super::GpCustomLineCap, widthscale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapWidthScale(customcap : *mut super::GpCustomLineCap, widthscale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetCustomLineCapWidthScale(customcap as _, widthscale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetEmpty(region: *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetEmpty(region : *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipSetEmpty(region as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesCachedBackground(imageattr: *mut super::GpImageAttributes, enableflag: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesCachedBackground(imageattr : *mut super::GpImageAttributes, enableflag : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetImageAttributesCachedBackground(imageattr as _, enableflag.into()) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesColorKeys(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, colorlow: super::ARGB, colorhigh: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorKeys(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, colorlow : super::ARGB, colorhigh : super::ARGB) -> super::GpStatus);
    unsafe { GdipSetImageAttributesColorKeys(imageattr as _, r#type, enableflag.into(), colorlow, colorhigh) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesColorMatrix(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, colormatrix: *const super::ColorMatrix, graymatrix: *const super::ColorMatrix, flags: super::ColorMatrixFlags) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorMatrix(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, colormatrix : *const super::ColorMatrix, graymatrix : *const super::ColorMatrix, flags : super::ColorMatrixFlags) -> super::GpStatus);
    unsafe { GdipSetImageAttributesColorMatrix(imageattr as _, r#type, enableflag.into(), colormatrix, graymatrix, flags) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesGamma(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, gamma: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesGamma(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, gamma : super::REAL) -> super::GpStatus);
    unsafe { GdipSetImageAttributesGamma(imageattr as _, r#type, enableflag.into(), gamma) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesNoOp(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesNoOp(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetImageAttributesNoOp(imageattr as _, r#type, enableflag.into()) }
}
#[cfg(all(feature = "gdipluscolor", feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannel(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, channelflags: super::ColorChannelFlags) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannel(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, channelflags : super::ColorChannelFlags) -> super::GpStatus);
    unsafe { GdipSetImageAttributesOutputChannel(imageattr as _, r#type, enableflag.into(), channelflags) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannelColorProfile(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, colorprofilefilename: *const super::WCHAR) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannelColorProfile(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, colorprofilefilename : *const super::WCHAR) -> super::GpStatus);
    unsafe { GdipSetImageAttributesOutputChannelColorProfile(imageattr as _, r#type, enableflag.into(), colorprofilefilename) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesThreshold(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType, enableflag: bool, threshold: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesThreshold(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_core::BOOL, threshold : super::REAL) -> super::GpStatus);
    unsafe { GdipSetImageAttributesThreshold(imageattr as _, r#type, enableflag.into(), threshold) }
}
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesToIdentity(imageattr: *mut super::GpImageAttributes, r#type: super::ColorAdjustType) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesToIdentity(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType) -> super::GpStatus);
    unsafe { GdipSetImageAttributesToIdentity(imageattr as _, r#type) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetImageAttributesWrapMode(imageattr: *mut super::GpImageAttributes, wrap: super::WrapMode, argb: super::ARGB, clamp: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesWrapMode(imageattr : *mut super::GpImageAttributes, wrap : super::WrapMode, argb : super::ARGB, clamp : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetImageAttributesWrapMode(imageattr as _, wrap, argb, clamp.into()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetImagePalette(image: *mut super::GpImage, palette: *const super::ColorPalette) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImagePalette(image : *mut super::GpImage, palette : *const super::ColorPalette) -> super::GpStatus);
    unsafe { GdipSetImagePalette(image as _, palette) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetInfinite(region: *mut super::GpRegion) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetInfinite(region : *mut super::GpRegion) -> super::GpStatus);
    unsafe { GdipSetInfinite(region as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetInterpolationMode(graphics: *mut super::GpGraphics, interpolationmode: super::InterpolationMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetInterpolationMode(graphics : *mut super::GpGraphics, interpolationmode : super::InterpolationMode) -> super::GpStatus);
    unsafe { GdipSetInterpolationMode(graphics as _, interpolationmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetLineBlend(brush: *mut super::GpLineGradient, blend: *const super::REAL, positions: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineBlend(brush : *mut super::GpLineGradient, blend : *const super::REAL, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetLineBlend(brush as _, blend, positions, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineColors(brush: *mut super::GpLineGradient, color1: super::ARGB, color2: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineColors(brush : *mut super::GpLineGradient, color1 : super::ARGB, color2 : super::ARGB) -> super::GpStatus);
    unsafe { GdipSetLineColors(brush as _, color1, color2) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineGammaCorrection(brush: *mut super::GpLineGradient, usegammacorrection: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineGammaCorrection(brush : *mut super::GpLineGradient, usegammacorrection : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetLineGammaCorrection(brush as _, usegammacorrection.into()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineLinearBlend(brush: *mut super::GpLineGradient, focus: super::REAL, scale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineLinearBlend(brush : *mut super::GpLineGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetLineLinearBlend(brush as _, focus, scale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetLinePresetBlend(brush: *mut super::GpLineGradient, blend: *const super::ARGB, positions: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLinePresetBlend(brush : *mut super::GpLineGradient, blend : *const super::ARGB, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetLinePresetBlend(brush as _, blend, positions, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineSigmaBlend(brush: *mut super::GpLineGradient, focus: super::REAL, scale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineSigmaBlend(brush : *mut super::GpLineGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetLineSigmaBlend(brush as _, focus, scale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineTransform(brush: *mut super::GpLineGradient, matrix: *const super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineTransform(brush : *mut super::GpLineGradient, matrix : *const super::GpMatrix) -> super::GpStatus);
    unsafe { GdipSetLineTransform(brush as _, matrix) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetLineWrapMode(brush: *mut super::GpLineGradient, wrapmode: super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineWrapMode(brush : *mut super::GpLineGradient, wrapmode : super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipSetLineWrapMode(brush as _, wrapmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetMatrixElements(matrix: *mut super::GpMatrix, m11: super::REAL, m12: super::REAL, m21: super::REAL, m22: super::REAL, dx: super::REAL, dy: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetMatrixElements(matrix : *mut super::GpMatrix, m11 : super::REAL, m12 : super::REAL, m21 : super::REAL, m22 : super::REAL, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
    unsafe { GdipSetMatrixElements(matrix as _, m11, m12, m21, m22, dx, dy) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetMetafileDownLevelRasterizationLimit(metafile: *mut super::GpMetafile, metafilerasterizationlimitdpi: super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetMetafileDownLevelRasterizationLimit(metafile : *mut super::GpMetafile, metafilerasterizationlimitdpi : super::UINT) -> super::GpStatus);
    unsafe { GdipSetMetafileDownLevelRasterizationLimit(metafile as _, metafilerasterizationlimitdpi) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPageScale(graphics: *mut super::GpGraphics, scale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPageScale(graphics : *mut super::GpGraphics, scale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPageScale(graphics as _, scale) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPageUnit(graphics: *mut super::GpGraphics, unit: super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPageUnit(graphics : *mut super::GpGraphics, unit : super::GpUnit) -> super::GpStatus);
    unsafe { GdipSetPageUnit(graphics as _, unit) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathFillMode(path: *mut super::GpPath, fillmode: super::GpFillMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathFillMode(path : *mut super::GpPath, fillmode : super::GpFillMode) -> super::GpStatus);
    unsafe { GdipSetPathFillMode(path as _, fillmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPathGradientBlend(brush: *mut super::GpPathGradient, blend: *const super::REAL, positions: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientBlend(brush : *mut super::GpPathGradient, blend : *const super::REAL, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetPathGradientBlend(brush as _, blend, positions, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientCenterColor(brush: *mut super::GpPathGradient, colors: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterColor(brush : *mut super::GpPathGradient, colors : super::ARGB) -> super::GpStatus);
    unsafe { GdipSetPathGradientCenterColor(brush as _, colors) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientCenterPoint(brush: *mut super::GpPathGradient, points: *const super::GpPointF) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPoint(brush : *mut super::GpPathGradient, points : *const super::GpPointF) -> super::GpStatus);
    unsafe { GdipSetPathGradientCenterPoint(brush as _, points) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPathGradientCenterPointI(brush: *mut super::GpPathGradient, points: *const super::GpPoint) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPointI(brush : *mut super::GpPathGradient, points : *const super::GpPoint) -> super::GpStatus);
    unsafe { GdipSetPathGradientCenterPointI(brush as _, points) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientFocusScales(brush: *mut super::GpPathGradient, xscale: super::REAL, yscale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientFocusScales(brush : *mut super::GpPathGradient, xscale : super::REAL, yscale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPathGradientFocusScales(brush as _, xscale, yscale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientGammaCorrection(brush: *mut super::GpPathGradient, usegammacorrection: bool) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientGammaCorrection(brush : *mut super::GpPathGradient, usegammacorrection : windows_core::BOOL) -> super::GpStatus);
    unsafe { GdipSetPathGradientGammaCorrection(brush as _, usegammacorrection.into()) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientLinearBlend(brush: *mut super::GpPathGradient, focus: super::REAL, scale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientLinearBlend(brush : *mut super::GpPathGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPathGradientLinearBlend(brush as _, focus, scale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientPath(brush: *mut super::GpPathGradient, path: *const super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientPath(brush : *mut super::GpPathGradient, path : *const super::GpPath) -> super::GpStatus);
    unsafe { GdipSetPathGradientPath(brush as _, path) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPathGradientPresetBlend(brush: *mut super::GpPathGradient, blend: *const super::ARGB, positions: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientPresetBlend(brush : *mut super::GpPathGradient, blend : *const super::ARGB, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetPathGradientPresetBlend(brush as _, blend, positions, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientSigmaBlend(brush: *mut super::GpPathGradient, focus: super::REAL, scale: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientSigmaBlend(brush : *mut super::GpPathGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPathGradientSigmaBlend(brush as _, focus, scale) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPathGradientSurroundColorsWithCount(brush: *mut super::GpPathGradient, color: *const super::ARGB, count: *mut super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientSurroundColorsWithCount(brush : *mut super::GpPathGradient, color : *const super::ARGB, count : *mut super::INT) -> super::GpStatus);
    unsafe { GdipSetPathGradientSurroundColorsWithCount(brush as _, color, count as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientTransform(brush: *mut super::GpPathGradient, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipSetPathGradientTransform(brush as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathGradientWrapMode(brush: *mut super::GpPathGradient, wrapmode: super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientWrapMode(brush : *mut super::GpPathGradient, wrapmode : super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipSetPathGradientWrapMode(brush as _, wrapmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPathMarker(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathMarker(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipSetPathMarker(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenBrushFill(pen: *mut super::GpPen, brush: *mut super::GpBrush) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenBrushFill(pen : *mut super::GpPen, brush : *mut super::GpBrush) -> super::GpStatus);
    unsafe { GdipSetPenBrushFill(pen as _, brush as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenColor(pen: *mut super::GpPen, argb: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenColor(pen : *mut super::GpPen, argb : super::ARGB) -> super::GpStatus);
    unsafe { GdipSetPenColor(pen as _, argb) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPenCompoundArray(pen: *mut super::GpPen, dash: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCompoundArray(pen : *mut super::GpPen, dash : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetPenCompoundArray(pen as _, dash, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenCustomEndCap(pen: *mut super::GpPen, customcap: *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCustomEndCap(pen : *mut super::GpPen, customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipSetPenCustomEndCap(pen as _, customcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenCustomStartCap(pen: *mut super::GpPen, customcap: *mut super::GpCustomLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCustomStartCap(pen : *mut super::GpPen, customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
    unsafe { GdipSetPenCustomStartCap(pen as _, customcap as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetPenDashArray(pen: *mut super::GpPen, dash: *const super::REAL, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashArray(pen : *mut super::GpPen, dash : *const super::REAL, count : super::INT) -> super::GpStatus);
    unsafe { GdipSetPenDashArray(pen as _, dash, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenDashCap197819(pen: *mut super::GpPen, dashcap: super::GpDashCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashCap197819(pen : *mut super::GpPen, dashcap : super::GpDashCap) -> super::GpStatus);
    unsafe { GdipSetPenDashCap197819(pen as _, dashcap) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenDashOffset(pen: *mut super::GpPen, offset: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashOffset(pen : *mut super::GpPen, offset : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPenDashOffset(pen as _, offset) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenDashStyle(pen: *mut super::GpPen, dashstyle: super::GpDashStyle) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashStyle(pen : *mut super::GpPen, dashstyle : super::GpDashStyle) -> super::GpStatus);
    unsafe { GdipSetPenDashStyle(pen as _, dashstyle) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenEndCap(pen: *mut super::GpPen, endcap: super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenEndCap(pen : *mut super::GpPen, endcap : super::GpLineCap) -> super::GpStatus);
    unsafe { GdipSetPenEndCap(pen as _, endcap) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenLineCap197819(pen: *mut super::GpPen, startcap: super::GpLineCap, endcap: super::GpLineCap, dashcap: super::GpDashCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenLineCap197819(pen : *mut super::GpPen, startcap : super::GpLineCap, endcap : super::GpLineCap, dashcap : super::GpDashCap) -> super::GpStatus);
    unsafe { GdipSetPenLineCap197819(pen as _, startcap, endcap, dashcap) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenLineJoin(pen: *mut super::GpPen, linejoin: super::GpLineJoin) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenLineJoin(pen : *mut super::GpPen, linejoin : super::GpLineJoin) -> super::GpStatus);
    unsafe { GdipSetPenLineJoin(pen as _, linejoin) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenMiterLimit(pen: *mut super::GpPen, miterlimit: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenMiterLimit(pen : *mut super::GpPen, miterlimit : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPenMiterLimit(pen as _, miterlimit) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenMode(pen: *mut super::GpPen, penmode: super::GpPenAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenMode(pen : *mut super::GpPen, penmode : super::GpPenAlignment) -> super::GpStatus);
    unsafe { GdipSetPenMode(pen as _, penmode) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenStartCap(pen: *mut super::GpPen, startcap: super::GpLineCap) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenStartCap(pen : *mut super::GpPen, startcap : super::GpLineCap) -> super::GpStatus);
    unsafe { GdipSetPenStartCap(pen as _, startcap) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenTransform(pen: *mut super::GpPen, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenTransform(pen : *mut super::GpPen, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipSetPenTransform(pen as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenUnit(pen: *mut super::GpPen, unit: super::GpUnit) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenUnit(pen : *mut super::GpPen, unit : super::GpUnit) -> super::GpStatus);
    unsafe { GdipSetPenUnit(pen as _, unit) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPenWidth(pen: *mut super::GpPen, width: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenWidth(pen : *mut super::GpPen, width : super::REAL) -> super::GpStatus);
    unsafe { GdipSetPenWidth(pen as _, width) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetPixelOffsetMode(graphics: *mut super::GpGraphics, pixeloffsetmode: super::PixelOffsetMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPixelOffsetMode(graphics : *mut super::GpGraphics, pixeloffsetmode : super::PixelOffsetMode) -> super::GpStatus);
    unsafe { GdipSetPixelOffsetMode(graphics as _, pixeloffsetmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn GdipSetPropertyItem(image: *mut super::GpImage, item: *const super::PropertyItem) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPropertyItem(image : *mut super::GpImage, item : *const super::PropertyItem) -> super::GpStatus);
    unsafe { GdipSetPropertyItem(image as _, item) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetRenderingOrigin(graphics: *mut super::GpGraphics, x: super::INT, y: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetRenderingOrigin(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT) -> super::GpStatus);
    unsafe { GdipSetRenderingOrigin(graphics as _, x, y) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetSmoothingMode(graphics: *mut super::GpGraphics, smoothingmode: super::SmoothingMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetSmoothingMode(graphics : *mut super::GpGraphics, smoothingmode : super::SmoothingMode) -> super::GpStatus);
    unsafe { GdipSetSmoothingMode(graphics as _, smoothingmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetSolidFillColor(brush: *mut super::GpSolidFill, color: super::ARGB) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetSolidFillColor(brush : *mut super::GpSolidFill, color : super::ARGB) -> super::GpStatus);
    unsafe { GdipSetSolidFillColor(brush as _, color) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetStringFormatAlign(format: *mut super::GpStringFormat, align: super::StringAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatAlign(format : *mut super::GpStringFormat, align : super::StringAlignment) -> super::GpStatus);
    unsafe { GdipSetStringFormatAlign(format as _, align) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetStringFormatDigitSubstitution(format: *mut super::GpStringFormat, language: super::LANGID, substitute: super::StringDigitSubstitute) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatDigitSubstitution(format : *mut super::GpStringFormat, language : super::LANGID, substitute : super::StringDigitSubstitute) -> super::GpStatus);
    unsafe { GdipSetStringFormatDigitSubstitution(format as _, language, substitute) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetStringFormatFlags(format: *mut super::GpStringFormat, flags: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatFlags(format : *mut super::GpStringFormat, flags : super::INT) -> super::GpStatus);
    unsafe { GdipSetStringFormatFlags(format as _, flags) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetStringFormatHotkeyPrefix(format: *mut super::GpStringFormat, hotkeyprefix: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatHotkeyPrefix(format : *mut super::GpStringFormat, hotkeyprefix : super::INT) -> super::GpStatus);
    unsafe { GdipSetStringFormatHotkeyPrefix(format as _, hotkeyprefix) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetStringFormatLineAlign(format: *mut super::GpStringFormat, align: super::StringAlignment) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatLineAlign(format : *mut super::GpStringFormat, align : super::StringAlignment) -> super::GpStatus);
    unsafe { GdipSetStringFormatLineAlign(format as _, align) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetStringFormatMeasurableCharacterRanges(format: *mut super::GpStringFormat, rangecount: super::INT, ranges: *const super::CharacterRange) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatMeasurableCharacterRanges(format : *mut super::GpStringFormat, rangecount : super::INT, ranges : *const super::CharacterRange) -> super::GpStatus);
    unsafe { GdipSetStringFormatMeasurableCharacterRanges(format as _, rangecount, ranges) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetStringFormatTabStops(format: *mut super::GpStringFormat, firsttaboffset: super::REAL, count: super::INT, tabstops: *const super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatTabStops(format : *mut super::GpStringFormat, firsttaboffset : super::REAL, count : super::INT, tabstops : *const super::REAL) -> super::GpStatus);
    unsafe { GdipSetStringFormatTabStops(format as _, firsttaboffset, count, tabstops) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetStringFormatTrimming(format: *mut super::GpStringFormat, trimming: super::StringTrimming) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatTrimming(format : *mut super::GpStringFormat, trimming : super::StringTrimming) -> super::GpStatus);
    unsafe { GdipSetStringFormatTrimming(format as _, trimming) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetTextContrast(graphics: *mut super::GpGraphics, contrast: super::UINT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextContrast(graphics : *mut super::GpGraphics, contrast : super::UINT) -> super::GpStatus);
    unsafe { GdipSetTextContrast(graphics as _, contrast) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetTextRenderingHint(graphics: *mut super::GpGraphics, mode: super::TextRenderingHint) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextRenderingHint(graphics : *mut super::GpGraphics, mode : super::TextRenderingHint) -> super::GpStatus);
    unsafe { GdipSetTextRenderingHint(graphics as _, mode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetTextureTransform(brush: *mut super::GpTexture, matrix: *const super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextureTransform(brush : *mut super::GpTexture, matrix : *const super::GpMatrix) -> super::GpStatus);
    unsafe { GdipSetTextureTransform(brush as _, matrix) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetTextureWrapMode(brush: *mut super::GpTexture, wrapmode: super::GpWrapMode) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextureWrapMode(brush : *mut super::GpTexture, wrapmode : super::GpWrapMode) -> super::GpStatus);
    unsafe { GdipSetTextureWrapMode(brush as _, wrapmode) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipSetWorldTransform(graphics: *mut super::GpGraphics, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetWorldTransform(graphics : *mut super::GpGraphics, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipSetWorldTransform(graphics as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipShearMatrix(matrix: *mut super::GpMatrix, shearx: super::REAL, sheary: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipShearMatrix(matrix : *mut super::GpMatrix, shearx : super::REAL, sheary : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipShearMatrix(matrix as _, shearx, sheary, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipStartPathFigure(path: *mut super::GpPath) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipStartPathFigure(path : *mut super::GpPath) -> super::GpStatus);
    unsafe { GdipStartPathFigure(path as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipStringFormatGetGenericDefault(format: *mut *mut super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericDefault(format : *mut *mut super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipStringFormatGetGenericDefault(format as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipStringFormatGetGenericTypographic(format: *mut *mut super::GpStringFormat) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericTypographic(format : *mut *mut super::GpStringFormat) -> super::GpStatus);
    unsafe { GdipStringFormatGetGenericTypographic(format as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTestControl(control: super::GpTestControlEnum, param: *mut core::ffi::c_void) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTestControl(control : super::GpTestControlEnum, param : *mut core::ffi::c_void) -> super::GpStatus);
    unsafe { GdipTestControl(control, param as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTransformMatrixPoints(matrix: *mut super::GpMatrix, pts: *mut super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformMatrixPoints(matrix : *mut super::GpMatrix, pts : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipTransformMatrixPoints(matrix as _, pts as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTransformMatrixPointsI(matrix: *mut super::GpMatrix, pts: *mut super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformMatrixPointsI(matrix : *mut super::GpMatrix, pts : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipTransformMatrixPointsI(matrix as _, pts as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTransformPath(path: *mut super::GpPath, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipTransformPath(path as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTransformPoints(graphics: *mut super::GpGraphics, destspace: super::GpCoordinateSpace, srcspace: super::GpCoordinateSpace, points: *mut super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPoints(graphics : *mut super::GpGraphics, destspace : super::GpCoordinateSpace, srcspace : super::GpCoordinateSpace, points : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipTransformPoints(graphics as _, destspace, srcspace, points as _, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTransformPointsI(graphics: *mut super::GpGraphics, destspace: super::GpCoordinateSpace, srcspace: super::GpCoordinateSpace, points: *mut super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPointsI(graphics : *mut super::GpGraphics, destspace : super::GpCoordinateSpace, srcspace : super::GpCoordinateSpace, points : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipTransformPointsI(graphics as _, destspace, srcspace, points as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTransformRegion(region: *mut super::GpRegion, matrix: *mut super::GpMatrix) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformRegion(region : *mut super::GpRegion, matrix : *mut super::GpMatrix) -> super::GpStatus);
    unsafe { GdipTransformRegion(region as _, matrix as _) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateClip(graphics: *mut super::GpGraphics, dx: super::REAL, dy: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateClip(graphics : *mut super::GpGraphics, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
    unsafe { GdipTranslateClip(graphics as _, dx, dy) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTranslateClipI(graphics: *mut super::GpGraphics, dx: super::INT, dy: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateClipI(graphics : *mut super::GpGraphics, dx : super::INT, dy : super::INT) -> super::GpStatus);
    unsafe { GdipTranslateClipI(graphics as _, dx, dy) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateLineTransform(brush: *mut super::GpLineGradient, dx: super::REAL, dy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateLineTransform(brush : *mut super::GpLineGradient, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslateLineTransform(brush as _, dx, dy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateMatrix(matrix: *mut super::GpMatrix, offsetx: super::REAL, offsety: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateMatrix(matrix : *mut super::GpMatrix, offsetx : super::REAL, offsety : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslateMatrix(matrix as _, offsetx, offsety, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslatePathGradientTransform(brush: *mut super::GpPathGradient, dx: super::REAL, dy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslatePathGradientTransform(brush : *mut super::GpPathGradient, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslatePathGradientTransform(brush as _, dx, dy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslatePenTransform(pen: *mut super::GpPen, dx: super::REAL, dy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslatePenTransform(pen : *mut super::GpPen, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslatePenTransform(pen as _, dx, dy, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateRegion(region: *mut super::GpRegion, dx: super::REAL, dy: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateRegion(region : *mut super::GpRegion, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
    unsafe { GdipTranslateRegion(region as _, dx, dy) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipTranslateRegionI(region: *mut super::GpRegion, dx: super::INT, dy: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateRegionI(region : *mut super::GpRegion, dx : super::INT, dy : super::INT) -> super::GpStatus);
    unsafe { GdipTranslateRegionI(region as _, dx, dy) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateTextureTransform(brush: *mut super::GpTexture, dx: super::REAL, dy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateTextureTransform(brush : *mut super::GpTexture, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslateTextureTransform(brush as _, dx, dy, order) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipTranslateWorldTransform(graphics: *mut super::GpGraphics, dx: super::REAL, dy: super::REAL, order: super::GpMatrixOrder) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateWorldTransform(graphics : *mut super::GpGraphics, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
    unsafe { GdipTranslateWorldTransform(graphics as _, dx, dy, order) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipVectorTransformMatrixPoints(matrix: *mut super::GpMatrix, pts: *mut super::GpPointF, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPoints(matrix : *mut super::GpMatrix, pts : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
    unsafe { GdipVectorTransformMatrixPoints(matrix as _, pts as _, count) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipVectorTransformMatrixPointsI(matrix: *mut super::GpMatrix, pts: *mut super::GpPoint, count: super::INT) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPointsI(matrix : *mut super::GpMatrix, pts : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
    unsafe { GdipVectorTransformMatrixPointsI(matrix as _, pts as _, count) }
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipWarpPath(path: *mut super::GpPath, matrix: *mut super::GpMatrix, points: *const super::GpPointF, count: super::INT, srcx: super::REAL, srcy: super::REAL, srcwidth: super::REAL, srcheight: super::REAL, warpmode: super::WarpMode, flatness: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipWarpPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix, points : *const super::GpPointF, count : super::INT, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, warpmode : super::WarpMode, flatness : super::REAL) -> super::GpStatus);
    unsafe { GdipWarpPath(path as _, matrix as _, points, count, srcx, srcy, srcwidth, srcheight, warpmode, flatness) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipWidenPath(nativepath: *mut super::GpPath, pen: *mut super::GpPen, matrix: *mut super::GpMatrix, flatness: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipWidenPath(nativepath : *mut super::GpPath, pen : *mut super::GpPen, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
    unsafe { GdipWidenPath(nativepath as _, pen as _, matrix as _, flatness) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
#[inline]
pub unsafe fn GdipWindingModeOutline(path: *mut super::GpPath, matrix: *mut super::GpMatrix, flatness: super::REAL) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdipWindingModeOutline(path : *mut super::GpPath, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
    unsafe { GdipWindingModeOutline(path as _, matrix as _, flatness) }
}
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdiplusNotificationHook(token: *mut super::ULONG_PTR) -> super::GpStatus {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusNotificationHook(token : *mut super::ULONG_PTR) -> super::GpStatus);
    unsafe { GdiplusNotificationHook(token as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GdiplusNotificationUnhook(token: super::ULONG_PTR) {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusNotificationUnhook(token : super::ULONG_PTR));
    unsafe { GdiplusNotificationUnhook(token) }
}

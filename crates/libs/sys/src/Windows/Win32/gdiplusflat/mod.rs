#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathArc(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathArcI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathBezier(path : *mut super::GpPath, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL, x3 : super::REAL, y3 : super::REAL, x4 : super::REAL, y4 : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathBezierI(path : *mut super::GpPath, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT, x3 : super::INT, y3 : super::INT, x4 : super::INT, y4 : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathBeziers(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathBeziersI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurveI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurve(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurve2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurve2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurve3(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurve3I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathCurveI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathEllipse(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathEllipseI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathLine(path : *mut super::GpPath, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathLine2(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathLine2I(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathLineI(path : *mut super::GpPath, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathPath(path : *mut super::GpPath, addingpath : *const super::GpPath, connect : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathPie(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathPieI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathPolygon(path : *mut super::GpPath, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathPolygonI(path : *mut super::GpPath, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathRectangle(path : *mut super::GpPath, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathRectangleI(path : *mut super::GpPath, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathRectangles(path : *mut super::GpPath, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathRectanglesI(path : *mut super::GpPath, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathString(path : *mut super::GpPath, string : *const super::WCHAR, length : super::INT, family : *const super::GpFontFamily, style : super::INT, emsize : super::REAL, layoutrect : *const super::RectF, format : *const super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipAddPathStringI(path : *mut super::GpPath, string : *const super::WCHAR, length : super::INT, family : *const super::GpFontFamily, style : super::INT, emsize : super::REAL, layoutrect : *const super::Rect, format : *const super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBeginContainer(graphics : *mut super::GpGraphics, dstrect : *const super::GpRectF, srcrect : *const super::GpRectF, unit : super::GpUnit, state : *mut super::GraphicsContainer) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBeginContainer2(graphics : *mut super::GpGraphics, state : *mut super::GraphicsContainer) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBeginContainerI(graphics : *mut super::GpGraphics, dstrect : *const super::GpRect, srcrect : *const super::GpRect, unit : super::GpUnit, state : *mut super::GraphicsContainer) -> super::GpStatus);
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapApplyEffect(bitmap : *mut super::GpBitmap, effect : *mut super::CGpEffect, roi : *mut super::RECT, useauxdata : windows_sys::core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapConvertFormat(pinputbitmap : *mut super::GpBitmap, format : super::PixelFormat, dithertype : super::DitherType, palettetype : super::PaletteType, palette : *mut super::ColorPalette, alphathresholdpercent : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapCreateApplyEffect(inputbitmaps : *mut *mut super::GpBitmap, numinputs : super::INT, effect : *mut super::CGpEffect, roi : *mut super::RECT, outputrect : *mut super::RECT, outputbitmap : *mut *mut super::GpBitmap, useauxdata : windows_sys::core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogram(bitmap : *mut super::GpBitmap, format : super::HistogramFormat, numberofentries : super::UINT, channel0 : *mut super::UINT, channel1 : *mut super::UINT, channel2 : *mut super::UINT, channel3 : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogramSize(format : super::HistogramFormat, numberofentries : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapGetPixel(bitmap : *mut super::GpBitmap, x : super::INT, y : super::INT, color : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapLockBits(bitmap : *mut super::GpBitmap, rect : *const super::GpRect, flags : super::UINT, format : super::PixelFormat, lockedbitmapdata : *mut super::BitmapData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapSetPixel(bitmap : *mut super::GpBitmap, x : super::INT, y : super::INT, color : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapSetResolution(bitmap : *mut super::GpBitmap, xdpi : super::REAL, ydpi : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipBitmapUnlockBits(bitmap : *mut super::GpBitmap, lockedbitmapdata : *mut super::BitmapData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipClearPathMarkers(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneBitmapArea(x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, format : super::PixelFormat, srcbitmap : *mut super::GpBitmap, dstbitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneBitmapAreaI(x : super::INT, y : super::INT, width : super::INT, height : super::INT, format : super::PixelFormat, srcbitmap : *mut super::GpBitmap, dstbitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneBrush(brush : *mut super::GpBrush, clonebrush : *mut *mut super::GpBrush) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneCustomLineCap(customcap : *mut super::GpCustomLineCap, clonedcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneFont(font : *mut super::GpFont, clonefont : *mut *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneFontFamily(fontfamily : *mut super::GpFontFamily, clonedfontfamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneImage(image : *mut super::GpImage, cloneimage : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneImageAttributes(imageattr : *const super::GpImageAttributes, cloneimageattr : *mut *mut super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneMatrix(matrix : *mut super::GpMatrix, clonematrix : *mut *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipClonePath(path : *mut super::GpPath, clonepath : *mut *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipClonePen(pen : *mut super::GpPen, clonepen : *mut *mut super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneRegion(region : *mut super::GpRegion, cloneregion : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCloneStringFormat(format : *const super::GpStringFormat, newformat : *mut *mut super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipClosePathFigure(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipClosePathFigures(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCombineRegionPath(region : *mut super::GpRegion, path : *mut super::GpPath, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCombineRegionRect(region : *mut super::GpRegion, rect : *const super::GpRectF, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCombineRegionRectI(region : *mut super::GpRegion, rect : *const super::GpRect, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCombineRegionRegion(region : *mut super::GpRegion, region2 : *mut super::GpRegion, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipComment(graphics : *mut super::GpGraphics, sizedata : super::UINT, data : *const super::BYTE) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlus(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToFile(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, filename : *const super::WCHAR, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToStream(refgraphics : *const super::GpGraphics, metafile : *mut super::GpMetafile, conversionfailureflag : *mut super::INT, stream : *mut core::ffi::c_void, emftype : super::EmfType, description : *const super::WCHAR, out_metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateAdjustableArrowCap(height : super::REAL, width : super::REAL, isfilled : windows_sys::core::BOOL, cap : *mut *mut super::GpAdjustableArrowCap) -> super::GpStatus);
#[cfg(all(feature = "ddraw", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromDirectDrawSurface(surface : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFile(filename : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFileICM(filename : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGdiDib(gdibitmapinfo : *const super::BITMAPINFO, gdibitmapdata : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGraphics(width : super::INT, height : super::INT, target : *mut super::GpGraphics, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHBITMAP(hbm : super::HBITMAP, hpal : super::HPALETTE, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHICON(hicon : super::HICON, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromResource(hinstance : super::HINSTANCE, lpbitmapname : *const super::WCHAR, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromScan0(width : super::INT, height : super::INT, stride : super::INT, format : super::PixelFormat, scan0 : *const super::BYTE, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStream(stream : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStreamICM(stream : *mut core::ffi::c_void, bitmap : *mut *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateCachedBitmap(bitmap : *mut super::GpBitmap, graphics : *mut super::GpGraphics, cachedbitmap : *mut *mut super::GpCachedBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateCustomLineCap(fillpath : *mut super::GpPath, strokepath : *mut super::GpPath, basecap : super::GpLineCap, baseinset : super::REAL, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFont(fontfamily : *const super::GpFontFamily, emsize : super::REAL, style : super::INT, unit : super::Unit, font : *mut *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFontFamilyFromName(name : *const super::WCHAR, fontcollection : *mut super::GpFontCollection, fontfamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFontFromDC(hdc : super::HDC, font : *mut *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontA(hdc : super::HDC, logfont : *const super::LOGFONTA, font : *mut *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontW(hdc : super::HDC, logfont : *const super::LOGFONTW, font : *mut *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFromHDC(hdc : super::HDC, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFromHDC2(hdc : super::HDC, hdevice : super::HANDLE, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFromHWND(hwnd : super::HWND, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateFromHWNDICM(hwnd : super::HWND, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateHBITMAPFromBitmap(bitmap : *mut super::GpBitmap, hbmreturn : *mut super::HBITMAP, background : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateHICONFromBitmap(bitmap : *mut super::GpBitmap, hbmreturn : *mut super::HICON) -> super::GpStatus);
#[cfg(feature = "windef")]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateHalftonePalette() -> super::HPALETTE);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateHatchBrush(hatchstyle : super::GpHatchStyle, forecol : super::ARGB, backcol : super::ARGB, brush : *mut *mut super::GpHatch) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateImageAttributes(imageattr : *mut *mut super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrush(point1 : *const super::GpPointF, point2 : *const super::GpPointF, color1 : super::ARGB, color2 : super::ARGB, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRect(rect : *const super::GpRectF, color1 : super::ARGB, color2 : super::ARGB, mode : super::LinearGradientMode, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectI(rect : *const super::GpRect, color1 : super::ARGB, color2 : super::ARGB, mode : super::LinearGradientMode, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngle(rect : *const super::GpRectF, color1 : super::ARGB, color2 : super::ARGB, angle : super::REAL, isanglescalable : windows_sys::core::BOOL, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngleI(rect : *const super::GpRect, color1 : super::ARGB, color2 : super::ARGB, angle : super::REAL, isanglescalable : windows_sys::core::BOOL, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateLineBrushI(point1 : *const super::GpPoint, point2 : *const super::GpPoint, color1 : super::ARGB, color2 : super::ARGB, wrapmode : super::GpWrapMode, linegradient : *mut *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMatrix(matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMatrix2(m11 : super::REAL, m12 : super::REAL, m21 : super::REAL, m22 : super::REAL, dx : super::REAL, dy : super::REAL, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMatrix3(rect : *const super::GpRectF, dstplg : *const super::GpPointF, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMatrix3I(rect : *const super::GpRect, dstplg : *const super::GpPoint, matrix : *mut *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromEmf(hemf : super::HENHMETAFILE, deleteemf : windows_sys::core::BOOL, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromFile(file : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromStream(stream : *mut core::ffi::c_void, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "minwindef", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmf(hwmf : super::HMETAFILE, deletewmf : windows_sys::core::BOOL, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmfFile(file : *const super::WCHAR, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePath(brushmode : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePath2(param0 : *const super::GpPointF, param1 : *const super::BYTE, param2 : super::INT, param3 : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePath2I(param0 : *const super::GpPoint, param1 : *const super::BYTE, param2 : super::INT, param3 : super::GpFillMode, path : *mut *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePathGradient(points : *const super::GpPointF, count : super::INT, wrapmode : super::GpWrapMode, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePathGradientFromPath(path : *const super::GpPath, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePathGradientI(points : *const super::GpPoint, count : super::INT, wrapmode : super::GpWrapMode, polygradient : *mut *mut super::GpPathGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePathIter(iterator : *mut *mut super::GpPathIterator, path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePen1(color : super::ARGB, width : super::REAL, unit : super::GpUnit, pen : *mut *mut super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreatePen2(brush : *mut super::GpBrush, width : super::REAL, unit : super::GpUnit, pen : *mut *mut super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegion(region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegionHrgn(hrgn : super::HRGN, region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegionPath(path : *mut super::GpPath, region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegionRect(rect : *const super::GpRectF, region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegionRectI(rect : *const super::GpRect, region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateRegionRgnData(regiondata : *const super::BYTE, size : super::INT, region : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateSolidFill(color : super::ARGB, brush : *mut *mut super::GpSolidFill) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateStreamOnFile(filename : *const super::WCHAR, access : super::UINT, stream : *mut *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateStringFormat(formatattributes : super::INT, language : super::LANGID, format : *mut *mut super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateTexture(image : *mut super::GpImage, wrapmode : super::GpWrapMode, texture : *mut *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateTexture2(image : *mut super::GpImage, wrapmode : super::GpWrapMode, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, texture : *mut *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateTexture2I(image : *mut super::GpImage, wrapmode : super::GpWrapMode, x : super::INT, y : super::INT, width : super::INT, height : super::INT, texture : *mut *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateTextureIA(image : *mut super::GpImage, imageattributes : *const super::GpImageAttributes, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, texture : *mut *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateTextureIAI(image : *mut super::GpImage, imageattributes : *const super::GpImageAttributes, x : super::INT, y : super::INT, width : super::INT, height : super::INT, texture : *mut *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteBrush(brush : *mut super::GpBrush) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteCachedBitmap(cachedbitmap : *mut super::GpCachedBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteCustomLineCap(customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteFont(font : *mut super::GpFont) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteFontFamily(fontfamily : *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteGraphics(graphics : *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteMatrix(matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeletePath(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeletePathIter(iterator : *mut super::GpPathIterator) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeletePen(pen : *mut super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeletePrivateFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteRegion(region : *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteStringFormat(format : *mut super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDisposeImage(image : *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDisposeImageAttributes(imageattr : *mut super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawArc(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawArcI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawBezier(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL, x3 : super::REAL, y3 : super::REAL, x4 : super::REAL, y4 : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawBezierI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT, x3 : super::INT, y3 : super::INT, x4 : super::INT, y4 : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawBeziers(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawBeziersI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCachedBitmap(graphics : *mut super::GpGraphics, cachedbitmap : *mut super::GpCachedBitmap, x : super::INT, y : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawClosedCurveI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurve(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurve2(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurve2I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurve3(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurve3I(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT, offset : super::INT, numberofsegments : super::INT, tension : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawCurveI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawDriverString(graphics : *mut super::GpGraphics, text : *const super::UINT16, length : super::INT, font : *const super::GpFont, brush : *const super::GpBrush, positions : *const super::PointF, flags : super::INT, matrix : *const super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawEllipse(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawEllipseI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImage(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdipluseffects", feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageFX(graphics : *mut super::GpGraphics, image : *mut super::GpImage, source : *mut super::GpRectF, xform : *mut super::GpMatrix, effect : *mut super::CGpEffect, imageattributes : *mut super::GpImageAttributes, srcunit : super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePointRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePointRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePoints(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstpoints : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePointsI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstpoints : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, points : *const super::GpPointF, count : super::INT, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, points : *const super::GpPoint, count : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageRectRect(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstx : super::REAL, dsty : super::REAL, dstwidth : super::REAL, dstheight : super::REAL, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawImageRectRectI(graphics : *mut super::GpGraphics, image : *mut super::GpImage, dstx : super::INT, dsty : super::INT, dstwidth : super::INT, dstheight : super::INT, srcx : super::INT, srcy : super::INT, srcwidth : super::INT, srcheight : super::INT, srcunit : super::GpUnit, imageattributes : *const super::GpImageAttributes, callback : super::DrawImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawLine(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::REAL, y1 : super::REAL, x2 : super::REAL, y2 : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawLineI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x1 : super::INT, y1 : super::INT, x2 : super::INT, y2 : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawLines(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawLinesI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawPath(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawPie(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawPieI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawPolygon(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawPolygonI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawRectangle(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawRectangleI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawRectangles(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawRectanglesI(graphics : *mut super::GpGraphics, pen : *mut super::GpPen, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipDrawString(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, brush : *const super::GpBrush) -> super::GpStatus);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEmfToWmfBits(hemf : super::HENHMETAFILE, cbdata16 : super::UINT, pdata16 : super::LPBYTE, imapmode : super::INT, eflags : super::INT) -> super::UINT);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEndContainer(graphics : *mut super::GpGraphics, state : super::GraphicsContainer) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoint(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::PointF, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::Point, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoints(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::PointF, count : super::INT, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointsI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::Point, count : super::INT, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRect(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::RectF, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRectI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::Rect, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoint(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::PointF, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoint : *const super::Point, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoints(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::PointF, count : super::INT, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointsI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destpoints : *const super::Point, count : super::INT, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRect(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::RectF, srcrect : *const super::RectF, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRectI(graphics : *mut super::GpGraphics, metafile : *const super::GpMetafile, destrect : *const super::Rect, srcrect : *const super::Rect, srcunit : super::Unit, callback : super::EnumerateMetafileProc, callbackdata : *mut core::ffi::c_void, imageattributes : *const super::GpImageAttributes) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillClosedCurve(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT, tension : super::REAL, fillmode : super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2I(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT, tension : super::REAL, fillmode : super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillClosedCurveI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillEllipse(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillEllipseI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPath(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPie(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPieI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT, startangle : super::REAL, sweepangle : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPolygon(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT, fillmode : super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPolygon2(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPolygon2I(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillPolygonI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, points : *const super::GpPoint, count : super::INT, fillmode : super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillRectangle(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillRectangleI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, x : super::INT, y : super::INT, width : super::INT, height : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillRectangles(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, rects : *const super::GpRectF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillRectanglesI(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, rects : *const super::GpRect, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFillRegion(graphics : *mut super::GpGraphics, brush : *mut super::GpBrush, region : *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFindFirstImageItem(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFindNextImageItem(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFlattenPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipFlush(graphics : *mut super::GpGraphics, intention : super::GpFlushIntention) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapFillState(cap : *mut super::GpAdjustableArrowCap, fillstate : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapHeight(cap : *mut super::GpAdjustableArrowCap, height : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapMiddleInset(cap : *mut super::GpAdjustableArrowCap, middleinset : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapWidth(cap : *mut super::GpAdjustableArrowCap, width : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetAllPropertyItems(image : *mut super::GpImage, totalbuffersize : super::UINT, numproperties : super::UINT, allitems : *mut super::PropertyItem) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetBrushType(brush : *mut super::GpBrush, r#type : *mut super::GpBrushType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCellAscent(family : *const super::GpFontFamily, style : super::INT, cellascent : *mut super::UINT16) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCellDescent(family : *const super::GpFontFamily, style : super::INT, celldescent : *mut super::UINT16) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetClip(graphics : *mut super::GpGraphics, region : *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetClipBounds(graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetClipBoundsI(graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCompositingMode(graphics : *mut super::GpGraphics, compositingmode : *mut super::CompositingMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCompositingQuality(graphics : *mut super::GpGraphics, compositingquality : *mut super::CompositingQuality) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseCap(customcap : *mut super::GpCustomLineCap, basecap : *mut super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseInset(customcap : *mut super::GpCustomLineCap, inset : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeCaps(customcap : *mut super::GpCustomLineCap, startcap : *mut super::GpLineCap, endcap : *mut super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeJoin(customcap : *mut super::GpCustomLineCap, linejoin : *mut super::GpLineJoin) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapType(customcap : *mut super::GpCustomLineCap, captype : *mut super::CustomLineCapType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapWidthScale(customcap : *mut super::GpCustomLineCap, widthscale : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetDC(graphics : *mut super::GpGraphics, hdc : *mut super::HDC) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetDpiX(graphics : *mut super::GpGraphics, dpi : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetDpiY(graphics : *mut super::GpGraphics, dpi : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetEmHeight(family : *const super::GpFontFamily, style : super::INT, emheight : *mut super::UINT16) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterList(image : *mut super::GpImage, clsidencoder : *const super::CLSID, size : super::UINT, buffer : *mut super::EncoderParameters) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "guiddef", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterListSize(image : *mut super::GpImage, clsidencoder : *const super::CLSID, size : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFamily(font : *mut super::GpFont, family : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFamilyName(family : *const super::GpFontFamily, name : windows_sys::core::PWSTR, language : super::LANGID) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyCount(fontcollection : *mut super::GpFontCollection, numfound : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyList(fontcollection : *const super::GpFontCollection, numsought : super::INT, gpfamilies : *mut *mut super::GpFontFamily, numfound : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontHeight(font : *const super::GpFont, graphics : *const super::GpGraphics, height : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontHeightGivenDPI(font : *const super::GpFont, dpi : super::REAL, height : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontSize(font : *mut super::GpFont, size : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontStyle(font : *mut super::GpFont, style : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetFontUnit(font : *mut super::GpFont, unit : *mut super::Unit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilyMonospace(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySansSerif(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySerif(nativefamily : *mut *mut super::GpFontFamily) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetHatchBackgroundColor(brush : *mut super::GpHatch, backcol : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetHatchForegroundColor(brush : *mut super::GpHatch, forecol : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetHatchStyle(brush : *mut super::GpHatch, hatchstyle : *mut super::GpHatchStyle) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetHemfFromMetafile(metafile : *mut super::GpMetafile, hemf : *mut super::HENHMETAFILE) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageAttributesAdjustedPalette(imageattr : *mut super::GpImageAttributes, colorpalette : *mut super::ColorPalette, coloradjusttype : super::ColorAdjustType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageBounds(image : *mut super::GpImage, srcrect : *mut super::GpRectF, srcunit : *mut super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageDecoders(numdecoders : super::UINT, size : super::UINT, decoders : *mut super::ImageCodecInfo) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageDecodersSize(numdecoders : *mut super::UINT, size : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageDimension(image : *mut super::GpImage, width : *mut super::REAL, height : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageEncoders(numencoders : super::UINT, size : super::UINT, encoders : *mut super::ImageCodecInfo) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageEncodersSize(numencoders : *mut super::UINT, size : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageFlags(image : *mut super::GpImage, flags : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageGraphicsContext(image : *mut super::GpImage, graphics : *mut *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageHeight(image : *mut super::GpImage, height : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageHorizontalResolution(image : *mut super::GpImage, resolution : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageItemData(image : *mut super::GpImage, item : *mut super::ImageItemData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImagePalette(image : *mut super::GpImage, palette : *mut super::ColorPalette, size : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImagePaletteSize(image : *mut super::GpImage, size : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImagePixelFormat(image : *mut super::GpImage, format : *mut super::PixelFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageRawFormat(image : *mut super::GpImage, format : *mut windows_sys::core::GUID) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageThumbnail(image : *mut super::GpImage, thumbwidth : super::UINT, thumbheight : super::UINT, thumbimage : *mut *mut super::GpImage, callback : super::GetThumbnailImageAbort, callbackdata : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageType(image : *mut super::GpImage, r#type : *mut super::ImageType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageVerticalResolution(image : *mut super::GpImage, resolution : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetImageWidth(image : *mut super::GpImage, width : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetInterpolationMode(graphics : *mut super::GpGraphics, interpolationmode : *mut super::InterpolationMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineBlend(brush : *mut super::GpLineGradient, blend : *mut super::REAL, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineBlendCount(brush : *mut super::GpLineGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineColors(brush : *mut super::GpLineGradient, colors : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineGammaCorrection(brush : *mut super::GpLineGradient, usegammacorrection : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlend(brush : *mut super::GpLineGradient, blend : *mut super::ARGB, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlendCount(brush : *mut super::GpLineGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineRect(brush : *mut super::GpLineGradient, rect : *mut super::GpRectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineRectI(brush : *mut super::GpLineGradient, rect : *mut super::GpRect) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineSpacing(family : *const super::GpFontFamily, style : super::INT, linespacing : *mut super::UINT16) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineTransform(brush : *mut super::GpLineGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLineWrapMode(brush : *mut super::GpLineGradient, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLogFontA(font : *mut super::GpFont, graphics : *mut super::GpGraphics, logfonta : *mut super::LOGFONTA) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetLogFontW(font : *mut super::GpFont, graphics : *mut super::GpGraphics, logfontw : *mut super::LOGFONTW) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMatrixElements(matrix : *const super::GpMatrix, matrixout : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMetafileDownLevelRasterizationLimit(metafile : *const super::GpMetafile, metafilerasterizationlimitdpi : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromEmf(hemf : super::HENHMETAFILE, header : *mut super::MetafileHeader) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromFile(filename : *const super::WCHAR, header : *mut super::MetafileHeader) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromMetafile(metafile : *mut super::GpMetafile, header : *mut super::MetafileHeader) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromStream(stream : *mut core::ffi::c_void, header : *mut super::MetafileHeader) -> super::GpStatus);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmetaheader", feature = "gdiplustypes", feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("gdiplus.dll" "C" fn GdipGetMetafileHeaderFromWmf(hwmf : super::HMETAFILE, wmfplaceablefileheader : *const super::WmfPlaceableFileHeader, header : *mut super::MetafileHeader) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetNearestColor(graphics : *mut super::GpGraphics, argb : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPageScale(graphics : *mut super::GpGraphics, scale : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPageUnit(graphics : *mut super::GpGraphics, unit : *mut super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathData(path : *mut super::GpPath, pathdata : *mut super::GpPathData) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathFillMode(path : *mut super::GpPath, fillmode : *mut super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlend(brush : *mut super::GpPathGradient, blend : *mut super::REAL, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlendCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterColor(brush : *mut super::GpPathGradient, colors : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPoint(brush : *mut super::GpPathGradient, points : *mut super::GpPointF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPointI(brush : *mut super::GpPathGradient, points : *mut super::GpPoint) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientFocusScales(brush : *mut super::GpPathGradient, xscale : *mut super::REAL, yscale : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientGammaCorrection(brush : *mut super::GpPathGradient, usegammacorrection : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientPath(brush : *mut super::GpPathGradient, path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientPointCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlend(brush : *mut super::GpPathGradient, blend : *mut super::ARGB, positions : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlendCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientRect(brush : *mut super::GpPathGradient, rect : *mut super::GpRectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientRectI(brush : *mut super::GpPathGradient, rect : *mut super::GpRect) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorCount(brush : *mut super::GpPathGradient, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorsWithCount(brush : *const super::GpPathGradient, color : *mut super::ARGB, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathGradientWrapMode(brush : *mut super::GpPathGradient, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathLastPoint(path : *mut super::GpPath, lastpoint : *mut super::GpPointF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathPoints(param0 : *mut super::GpPath, points : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathPointsI(param0 : *mut super::GpPath, points : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathTypes(path : *const super::GpPath, types : *mut super::BYTE, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathWorldBounds(path : *mut super::GpPath, bounds : *mut super::GpRectF, matrix : *const super::GpMatrix, pen : *const super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPathWorldBoundsI(path : *mut super::GpPath, bounds : *mut super::GpRect, matrix : *const super::GpMatrix, pen : *const super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenBrushFill(pen : *mut super::GpPen, brush : *mut *mut super::GpBrush) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenColor(pen : *mut super::GpPen, argb : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenCompoundArray(pen : *mut super::GpPen, dash : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenCompoundCount(pen : *mut super::GpPen, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenCustomEndCap(pen : *mut super::GpPen, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenCustomStartCap(pen : *mut super::GpPen, customcap : *mut *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenDashArray(pen : *mut super::GpPen, dash : *mut super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenDashCap197819(pen : *mut super::GpPen, dashcap : *mut super::GpDashCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenDashCount(pen : *mut super::GpPen, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenDashOffset(pen : *mut super::GpPen, offset : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenDashStyle(pen : *mut super::GpPen, dashstyle : *mut super::GpDashStyle) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenEndCap(pen : *mut super::GpPen, endcap : *mut super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenFillType(pen : *mut super::GpPen, r#type : *mut super::GpPenType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenLineJoin(pen : *mut super::GpPen, linejoin : *mut super::GpLineJoin) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenMiterLimit(pen : *mut super::GpPen, miterlimit : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenMode(pen : *mut super::GpPen, penmode : *mut super::GpPenAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenStartCap(pen : *mut super::GpPen, startcap : *mut super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenTransform(pen : *mut super::GpPen, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenUnit(pen : *mut super::GpPen, unit : *mut super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPenWidth(pen : *mut super::GpPen, width : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPixelOffsetMode(graphics : *mut super::GpGraphics, pixeloffsetmode : *mut super::PixelOffsetMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPointCount(path : *mut super::GpPath, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPropertyCount(image : *mut super::GpImage, numofproperty : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPropertyIdList(image : *mut super::GpImage, numofproperty : super::UINT, list : *mut super::PROPID) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPropertyItem(image : *mut super::GpImage, propid : super::PROPID, propsize : super::UINT, buffer : *mut super::PropertyItem) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPropertyItemSize(image : *mut super::GpImage, propid : super::PROPID, size : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetPropertySize(image : *mut super::GpImage, totalbuffersize : *mut super::UINT, numproperties : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionBounds(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionBoundsI(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionData(region : *mut super::GpRegion, buffer : *mut super::BYTE, buffersize : super::UINT, sizefilled : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionDataSize(region : *mut super::GpRegion, buffersize : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionHRgn(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, hrgn : *mut super::HRGN) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionScans(region : *mut super::GpRegion, rects : *mut super::GpRectF, count : *mut super::INT, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionScansCount(region : *mut super::GpRegion, count : *mut super::UINT, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRegionScansI(region : *mut super::GpRegion, rects : *mut super::GpRect, count : *mut super::INT, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetRenderingOrigin(graphics : *mut super::GpGraphics, x : *mut super::INT, y : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetSmoothingMode(graphics : *mut super::GpGraphics, smoothingmode : *mut super::SmoothingMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetSolidFillColor(brush : *mut super::GpSolidFill, color : *mut super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatAlign(format : *const super::GpStringFormat, align : *mut super::StringAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatDigitSubstitution(format : *const super::GpStringFormat, language : *mut super::LANGID, substitute : *mut super::StringDigitSubstitute) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatFlags(format : *const super::GpStringFormat, flags : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatHotkeyPrefix(format : *const super::GpStringFormat, hotkeyprefix : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatLineAlign(format : *const super::GpStringFormat, align : *mut super::StringAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatMeasurableCharacterRangeCount(format : *const super::GpStringFormat, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStopCount(format : *const super::GpStringFormat, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStops(format : *const super::GpStringFormat, count : super::INT, firsttaboffset : *mut super::REAL, tabstops : *mut super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetStringFormatTrimming(format : *const super::GpStringFormat, trimming : *mut super::StringTrimming) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetTextContrast(graphics : *mut super::GpGraphics, contrast : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetTextRenderingHint(graphics : *mut super::GpGraphics, mode : *mut super::TextRenderingHint) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetTextureImage(brush : *mut super::GpTexture, image : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetTextureTransform(brush : *mut super::GpTexture, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetTextureWrapMode(brush : *mut super::GpTexture, wrapmode : *mut super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBounds(graphics : *mut super::GpGraphics, rect : *mut super::GpRectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBoundsI(graphics : *mut super::GpGraphics, rect : *mut super::GpRect) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetWorldTransform(graphics : *mut super::GpGraphics, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGraphicsClear(graphics : *mut super::GpGraphics, color : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGraphicsSetAbort(pgraphics : *mut super::GpGraphics, piabort : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageForceValidation(image : *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageGetFrameCount(image : *mut super::GpImage, dimensionid : *const windows_sys::core::GUID, count : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsCount(image : *mut super::GpImage, count : *mut super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsList(image : *mut super::GpImage, dimensionids : *mut windows_sys::core::GUID, count : super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageRotateFlip(image : *mut super::GpImage, rftype : super::RotateFlipType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageSelectActiveFrame(image : *mut super::GpImage, dimensionid : *const windows_sys::core::GUID, frameindex : super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipImageSetAbort(pimage : *mut super::GpImage, piabort : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipInitializePalette(palette : *mut super::ColorPalette, palettetype : super::PaletteType, optimalcolors : super::INT, usetransparentcolor : windows_sys::core::BOOL, bitmap : *mut super::GpBitmap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipInvertMatrix(matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsClipEmpty(graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsEmptyRegion(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsEqualRegion(region : *mut super::GpRegion, region2 : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsInfiniteRegion(region : *mut super::GpRegion, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsMatrixEqual(matrix : *const super::GpMatrix, matrix2 : *const super::GpMatrix, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsMatrixIdentity(matrix : *const super::GpMatrix, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsMatrixInvertible(matrix : *const super::GpMatrix, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPoint(path : *mut super::GpPath, x : super::REAL, y : super::REAL, pen : *mut super::GpPen, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPointI(path : *mut super::GpPath, x : super::INT, y : super::INT, pen : *mut super::GpPen, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsStyleAvailable(family : *const super::GpFontFamily, style : super::INT, isstyleavailable : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleClipEmpty(graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPoint(path : *mut super::GpPath, x : super::REAL, y : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPointI(path : *mut super::GpPath, x : super::INT, y : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisiblePoint(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisiblePointI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRect(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRectI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, width : super::INT, height : super::INT, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPoint(region : *mut super::GpRegion, x : super::REAL, y : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPointI(region : *mut super::GpRegion, x : super::INT, y : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRect(region : *mut super::GpRegion, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRectI(region : *mut super::GpRegion, x : super::INT, y : super::INT, width : super::INT, height : super::INT, graphics : *mut super::GpGraphics, result : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipLoadImageFromFile(filename : *const super::WCHAR, image : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipLoadImageFromFileICM(filename : *const super::WCHAR, image : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
windows_link::link!("gdiplus.dll" "system" fn GdipLoadImageFromStream(stream : *mut core::ffi::c_void, image : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase"))]
windows_link::link!("gdiplus.dll" "system" fn GdipLoadImageFromStreamICM(stream : *mut core::ffi::c_void, image : *mut *mut super::GpImage) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMeasureCharacterRanges(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, regioncount : super::INT, regions : *mut *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMeasureDriverString(graphics : *mut super::GpGraphics, text : *const super::UINT16, length : super::INT, font : *const super::GpFont, positions : *const super::PointF, flags : super::INT, matrix : *const super::GpMatrix, boundingbox : *mut super::RectF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMeasureString(graphics : *mut super::GpGraphics, string : *const super::WCHAR, length : super::INT, font : *const super::GpFont, layoutrect : *const super::RectF, stringformat : *const super::GpStringFormat, boundingbox : *mut super::RectF, codepointsfitted : *mut super::INT, linesfilled : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyLineTransform(brush : *mut super::GpLineGradient, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyMatrix(matrix : *mut super::GpMatrix, matrix2 : *mut super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyPenTransform(pen : *mut super::GpPen, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyTextureTransform(brush : *mut super::GpTexture, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipMultiplyWorldTransform(graphics : *mut super::GpGraphics, matrix : *const super::GpMatrix, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipNewInstalledFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipNewPrivateFontCollection(fontcollection : *mut *mut super::GpFontCollection) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterCopyData(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, points : *mut super::GpPointF, types : *mut super::BYTE, startindex : super::INT, endindex : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterEnumerate(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, points : *mut super::GpPointF, types : *mut super::BYTE, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterGetCount(iterator : *mut super::GpPathIterator, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterGetSubpathCount(iterator : *mut super::GpPathIterator, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterHasCurve(iterator : *mut super::GpPathIterator, hascurve : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterIsValid(iterator : *mut super::GpPathIterator, valid : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterNextMarker(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, startindex : *mut super::INT, endindex : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterNextMarkerPath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterNextPathType(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, pathtype : *mut super::BYTE, startindex : *mut super::INT, endindex : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, startindex : *mut super::INT, endindex : *mut super::INT, isclosed : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpathPath(iterator : *mut super::GpPathIterator, resultcount : *mut super::INT, path : *mut super::GpPath, isclosed : *mut windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPathIterRewind(iterator : *mut super::GpPathIterator) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPlayMetafileRecord(metafile : *const super::GpMetafile, recordtype : super::EmfPlusRecordType, flags : super::UINT, datasize : super::UINT, data : *const super::BYTE) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPrivateAddFontFile(fontcollection : *mut super::GpFontCollection, filename : *const super::WCHAR) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipPrivateAddMemoryFont(fontcollection : *mut super::GpFontCollection, memory : *const core::ffi::c_void, length : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafile(referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileName(filename : *const super::WCHAR, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileNameI(filename : *const super::WCHAR, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafileI(referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafileStream(stream : *mut core::ffi::c_void, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRectF, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "objidlbase", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRecordMetafileStreamI(stream : *mut core::ffi::c_void, referencehdc : super::HDC, r#type : super::EmfType, framerect : *const super::GpRect, frameunit : super::MetafileFrameUnit, description : *const super::WCHAR, metafile : *mut *mut super::GpMetafile) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "windef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipReleaseDC(graphics : *mut super::GpGraphics, hdc : super::HDC) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRemovePropertyItem(image : *mut super::GpImage, propid : super::PROPID) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetClip(graphics : *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetImageAttributes(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetLineTransform(brush : *mut super::GpLineGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetPageTransform(graphics : *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetPath(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetPathGradientTransform(brush : *mut super::GpPathGradient) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetPenTransform(pen : *mut super::GpPen) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetTextureTransform(brush : *mut super::GpTexture) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipResetWorldTransform(graphics : *mut super::GpGraphics) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRestoreGraphics(graphics : *mut super::GpGraphics, state : super::GraphicsState) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipReversePath(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotateLineTransform(brush : *mut super::GpLineGradient, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotateMatrix(matrix : *mut super::GpMatrix, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotatePathGradientTransform(brush : *mut super::GpPathGradient, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotatePenTransform(pen : *mut super::GpPen, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotateTextureTransform(brush : *mut super::GpTexture, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipRotateWorldTransform(graphics : *mut super::GpGraphics, angle : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSaveAdd(image : *mut super::GpImage, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSaveAddImage(image : *mut super::GpImage, newimage : *mut super::GpImage, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSaveGraphics(graphics : *mut super::GpGraphics, state : *mut super::GraphicsState) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSaveImageToFile(image : *mut super::GpImage, filename : *const super::WCHAR, clsidencoder : *const super::CLSID, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "guiddef", feature = "objidlbase", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSaveImageToStream(image : *mut super::GpImage, stream : *mut core::ffi::c_void, clsidencoder : *const super::CLSID, encoderparams : *const super::EncoderParameters) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScaleLineTransform(brush : *mut super::GpLineGradient, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScaleMatrix(matrix : *mut super::GpMatrix, scalex : super::REAL, scaley : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScalePathGradientTransform(brush : *mut super::GpPathGradient, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScalePenTransform(pen : *mut super::GpPen, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScaleTextureTransform(brush : *mut super::GpTexture, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipScaleWorldTransform(graphics : *mut super::GpGraphics, sx : super::REAL, sy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapFillState(cap : *mut super::GpAdjustableArrowCap, fillstate : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapHeight(cap : *mut super::GpAdjustableArrowCap, height : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapMiddleInset(cap : *mut super::GpAdjustableArrowCap, middleinset : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapWidth(cap : *mut super::GpAdjustableArrowCap, width : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipGraphics(graphics : *mut super::GpGraphics, srcgraphics : *mut super::GpGraphics, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "minwindef"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipHrgn(graphics : *mut super::GpGraphics, hrgn : super::HRGN, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipPath(graphics : *mut super::GpGraphics, path : *mut super::GpPath, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipRect(graphics : *mut super::GpGraphics, x : super::REAL, y : super::REAL, width : super::REAL, height : super::REAL, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipRectI(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT, width : super::INT, height : super::INT, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetClipRegion(graphics : *mut super::GpGraphics, region : *mut super::GpRegion, combinemode : super::CombineMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCompositingMode(graphics : *mut super::GpGraphics, compositingmode : super::CompositingMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCompositingQuality(graphics : *mut super::GpGraphics, compositingquality : super::CompositingQuality) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseCap(customcap : *mut super::GpCustomLineCap, basecap : super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseInset(customcap : *mut super::GpCustomLineCap, inset : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeCaps(customcap : *mut super::GpCustomLineCap, startcap : super::GpLineCap, endcap : super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeJoin(customcap : *mut super::GpCustomLineCap, linejoin : super::GpLineJoin) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapWidthScale(customcap : *mut super::GpCustomLineCap, widthscale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetEmpty(region : *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesCachedBackground(imageattr : *mut super::GpImageAttributes, enableflag : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorKeys(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, colorlow : super::ARGB, colorhigh : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorMatrix(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, colormatrix : *const super::ColorMatrix, graymatrix : *const super::ColorMatrix, flags : super::ColorMatrixFlags) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesGamma(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, gamma : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesNoOp(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolor", feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannel(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, channelflags : super::ColorChannelFlags) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "winnt"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannelColorProfile(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, colorprofilefilename : *const super::WCHAR) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesThreshold(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType, enableflag : windows_sys::core::BOOL, threshold : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdipluscolormatrix", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesToIdentity(imageattr : *mut super::GpImageAttributes, r#type : super::ColorAdjustType) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImageAttributesWrapMode(imageattr : *mut super::GpImageAttributes, wrap : super::WrapMode, argb : super::ARGB, clamp : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetImagePalette(image : *mut super::GpImage, palette : *const super::ColorPalette) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetInfinite(region : *mut super::GpRegion) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetInterpolationMode(graphics : *mut super::GpGraphics, interpolationmode : super::InterpolationMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineBlend(brush : *mut super::GpLineGradient, blend : *const super::REAL, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineColors(brush : *mut super::GpLineGradient, color1 : super::ARGB, color2 : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineGammaCorrection(brush : *mut super::GpLineGradient, usegammacorrection : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineLinearBlend(brush : *mut super::GpLineGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLinePresetBlend(brush : *mut super::GpLineGradient, blend : *const super::ARGB, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineSigmaBlend(brush : *mut super::GpLineGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineTransform(brush : *mut super::GpLineGradient, matrix : *const super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetLineWrapMode(brush : *mut super::GpLineGradient, wrapmode : super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetMatrixElements(matrix : *mut super::GpMatrix, m11 : super::REAL, m12 : super::REAL, m21 : super::REAL, m22 : super::REAL, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetMetafileDownLevelRasterizationLimit(metafile : *mut super::GpMetafile, metafilerasterizationlimitdpi : super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPageScale(graphics : *mut super::GpGraphics, scale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPageUnit(graphics : *mut super::GpGraphics, unit : super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathFillMode(path : *mut super::GpPath, fillmode : super::GpFillMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientBlend(brush : *mut super::GpPathGradient, blend : *const super::REAL, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterColor(brush : *mut super::GpPathGradient, colors : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPoint(brush : *mut super::GpPathGradient, points : *const super::GpPointF) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPointI(brush : *mut super::GpPathGradient, points : *const super::GpPoint) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientFocusScales(brush : *mut super::GpPathGradient, xscale : super::REAL, yscale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientGammaCorrection(brush : *mut super::GpPathGradient, usegammacorrection : windows_sys::core::BOOL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientLinearBlend(brush : *mut super::GpPathGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientPath(brush : *mut super::GpPathGradient, path : *const super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientPresetBlend(brush : *mut super::GpPathGradient, blend : *const super::ARGB, positions : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientSigmaBlend(brush : *mut super::GpPathGradient, focus : super::REAL, scale : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientSurroundColorsWithCount(brush : *mut super::GpPathGradient, color : *const super::ARGB, count : *mut super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientTransform(brush : *mut super::GpPathGradient, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathGradientWrapMode(brush : *mut super::GpPathGradient, wrapmode : super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPathMarker(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenBrushFill(pen : *mut super::GpPen, brush : *mut super::GpBrush) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenColor(pen : *mut super::GpPen, argb : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenCompoundArray(pen : *mut super::GpPen, dash : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenCustomEndCap(pen : *mut super::GpPen, customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenCustomStartCap(pen : *mut super::GpPen, customcap : *mut super::GpCustomLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenDashArray(pen : *mut super::GpPen, dash : *const super::REAL, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenDashCap197819(pen : *mut super::GpPen, dashcap : super::GpDashCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenDashOffset(pen : *mut super::GpPen, offset : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenDashStyle(pen : *mut super::GpPen, dashstyle : super::GpDashStyle) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenEndCap(pen : *mut super::GpPen, endcap : super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenLineCap197819(pen : *mut super::GpPen, startcap : super::GpLineCap, endcap : super::GpLineCap, dashcap : super::GpDashCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenLineJoin(pen : *mut super::GpPen, linejoin : super::GpLineJoin) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenMiterLimit(pen : *mut super::GpPen, miterlimit : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenMode(pen : *mut super::GpPen, penmode : super::GpPenAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenStartCap(pen : *mut super::GpPen, startcap : super::GpLineCap) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenTransform(pen : *mut super::GpPen, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenUnit(pen : *mut super::GpPen, unit : super::GpUnit) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPenWidth(pen : *mut super::GpPen, width : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPixelOffsetMode(graphics : *mut super::GpGraphics, pixeloffsetmode : super::PixelOffsetMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusimaging", feature = "gdiplustypes", feature = "shared", feature = "wtypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetPropertyItem(image : *mut super::GpImage, item : *const super::PropertyItem) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetRenderingOrigin(graphics : *mut super::GpGraphics, x : super::INT, y : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetSmoothingMode(graphics : *mut super::GpGraphics, smoothingmode : super::SmoothingMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdipluspixelformats", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetSolidFillColor(brush : *mut super::GpSolidFill, color : super::ARGB) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatAlign(format : *mut super::GpStringFormat, align : super::StringAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatDigitSubstitution(format : *mut super::GpStringFormat, language : super::LANGID, substitute : super::StringDigitSubstitute) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatFlags(format : *mut super::GpStringFormat, flags : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatHotkeyPrefix(format : *mut super::GpStringFormat, hotkeyprefix : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatLineAlign(format : *mut super::GpStringFormat, align : super::StringAlignment) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatMeasurableCharacterRanges(format : *mut super::GpStringFormat, rangecount : super::INT, ranges : *const super::CharacterRange) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatTabStops(format : *mut super::GpStringFormat, firsttaboffset : super::REAL, count : super::INT, tabstops : *const super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetStringFormatTrimming(format : *mut super::GpStringFormat, trimming : super::StringTrimming) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetTextContrast(graphics : *mut super::GpGraphics, contrast : super::UINT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetTextRenderingHint(graphics : *mut super::GpGraphics, mode : super::TextRenderingHint) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetTextureTransform(brush : *mut super::GpTexture, matrix : *const super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetTextureWrapMode(brush : *mut super::GpTexture, wrapmode : super::GpWrapMode) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetWorldTransform(graphics : *mut super::GpGraphics, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipShearMatrix(matrix : *mut super::GpMatrix, shearx : super::REAL, sheary : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipStartPathFigure(path : *mut super::GpPath) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericDefault(format : *mut *mut super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericTypographic(format : *mut *mut super::GpStringFormat) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTestControl(control : super::GpTestControlEnum, param : *mut core::ffi::c_void) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformMatrixPoints(matrix : *mut super::GpMatrix, pts : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformMatrixPointsI(matrix : *mut super::GpMatrix, pts : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformPoints(graphics : *mut super::GpGraphics, destspace : super::GpCoordinateSpace, srcspace : super::GpCoordinateSpace, points : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformPointsI(graphics : *mut super::GpGraphics, destspace : super::GpCoordinateSpace, srcspace : super::GpCoordinateSpace, points : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTransformRegion(region : *mut super::GpRegion, matrix : *mut super::GpMatrix) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateClip(graphics : *mut super::GpGraphics, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateClipI(graphics : *mut super::GpGraphics, dx : super::INT, dy : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateLineTransform(brush : *mut super::GpLineGradient, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateMatrix(matrix : *mut super::GpMatrix, offsetx : super::REAL, offsety : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslatePathGradientTransform(brush : *mut super::GpPathGradient, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslatePenTransform(pen : *mut super::GpPen, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateRegion(region : *mut super::GpRegion, dx : super::REAL, dy : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateRegionI(region : *mut super::GpRegion, dx : super::INT, dy : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateTextureTransform(brush : *mut super::GpTexture, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipTranslateWorldTransform(graphics : *mut super::GpGraphics, dx : super::REAL, dy : super::REAL, order : super::GpMatrixOrder) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPoints(matrix : *mut super::GpMatrix, pts : *mut super::GpPointF, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPointsI(matrix : *mut super::GpMatrix, pts : *mut super::GpPoint, count : super::INT) -> super::GpStatus);
#[cfg(all(feature = "gdiplusenums", feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipWarpPath(path : *mut super::GpPath, matrix : *mut super::GpMatrix, points : *const super::GpPointF, count : super::INT, srcx : super::REAL, srcy : super::REAL, srcwidth : super::REAL, srcheight : super::REAL, warpmode : super::WarpMode, flatness : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipWidenPath(nativepath : *mut super::GpPath, pen : *mut super::GpPen, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplusmatrix", feature = "gdiplustypes"))]
windows_link::link!("gdiplus.dll" "system" fn GdipWindingModeOutline(path : *mut super::GpPath, matrix : *mut super::GpMatrix, flatness : super::REAL) -> super::GpStatus);
#[cfg(all(feature = "gdiplusgpstubs", feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdiplusNotificationHook(token : *mut super::ULONG_PTR) -> super::GpStatus);
#[cfg(feature = "shared")]
windows_link::link!("gdiplus.dll" "system" fn GdiplusNotificationUnhook(token : super::ULONG_PTR));

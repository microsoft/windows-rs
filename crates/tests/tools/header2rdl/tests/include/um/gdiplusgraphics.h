/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusGraphics.h
*
* Abstract:
*
*   GDI+ Graphics Object
*
\**************************************************************************/

#ifndef _GDIPLUSGRAPHICS_H
#define _GDIPLUSGRAPHICS_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#if _MSC_VER >= 1400
#pragma warning(disable:4365) /* signed/unsigned mismatch */
#endif
#pragma warning(disable:4820) /* padding added after data member */
#endif

class Graphics : public GdiplusBase
{
public:
    friend class Region;
    friend class GraphicsPath;
    friend class Image;
    friend class Bitmap;
    friend class Metafile;
    friend class Font;
    friend class FontFamily;
    friend class FontCollection;
    friend class CachedBitmap;

    static Graphics* FromHDC(IN HDC hdc)
    {
        return new Graphics(hdc);
    }

    static Graphics* FromHDC(IN HDC hdc,
                             IN HANDLE hdevice)
    {
        return new Graphics(hdc, hdevice);
    }

    static Graphics* FromHWND(IN HWND hwnd,
                              IN BOOL icm = FALSE)
    {
        return new Graphics(hwnd, icm);
    }

    static Graphics* FromImage(IN Image *image)
    {
        return new Graphics(image);
    }

    Graphics(IN HDC hdc)
    {
        GpGraphics *graphics = NULL;

        lastResult = DllExports::GdipCreateFromHDC(hdc, &graphics);

        SetNativeGraphics(graphics);
    }

    Graphics(IN HDC hdc,
             IN HANDLE hdevice)
    {
        GpGraphics *graphics = NULL;

        lastResult = DllExports::GdipCreateFromHDC2(hdc, hdevice, &graphics);

        SetNativeGraphics(graphics);
    }

    Graphics(IN HWND hwnd,
             IN BOOL icm = FALSE)
    {
        GpGraphics *graphics = NULL;

        if (icm)
        {
            lastResult = DllExports::GdipCreateFromHWNDICM(hwnd, &graphics);
        }
        else
        {
            lastResult = DllExports::GdipCreateFromHWND(hwnd, &graphics);
        }

        SetNativeGraphics(graphics);
    }

    Graphics(IN Image* image)
    {
        GpGraphics *graphics = NULL;

        if (image != NULL)
        {
            lastResult = DllExports::GdipGetImageGraphicsContext(
                                                                image->nativeImage, &graphics);
        }
        SetNativeGraphics(graphics);
    }

    ~Graphics()
    {
        DllExports::GdipDeleteGraphics(nativeGraphics);
    }

    VOID Flush(IN FlushIntention intention = FlushIntentionFlush)
    {
        DllExports::GdipFlush(nativeGraphics, intention);
    }

    //------------------------------------------------------------------------
    // GDI Interop methods
    //------------------------------------------------------------------------

    // Locks the graphics until ReleaseDC is called

    HDC GetHDC()
    {
        HDC     hdc = NULL;

        SetStatus(DllExports::GdipGetDC(nativeGraphics, &hdc));

        return hdc;
    }

    VOID ReleaseHDC(IN HDC hdc)
    {
        SetStatus(DllExports::GdipReleaseDC(nativeGraphics, hdc));
    }

    //------------------------------------------------------------------------
    // Rendering modes
    //------------------------------------------------------------------------

    Status SetRenderingOrigin(IN INT x, IN INT y)
    {
        return SetStatus(
            DllExports::GdipSetRenderingOrigin(
                nativeGraphics, x, y
            )
        );
    }

    Status GetRenderingOrigin(OUT INT *x, OUT INT *y) const
    {
        return SetStatus(
            DllExports::GdipGetRenderingOrigin(
                nativeGraphics, x, y
            )
        );
    }

    Status SetCompositingMode(IN CompositingMode compositingMode)
    {
        return SetStatus(DllExports::GdipSetCompositingMode(nativeGraphics,
                                                            compositingMode));
    }

    CompositingMode GetCompositingMode() const
    {
        CompositingMode mode;

        SetStatus(DllExports::GdipGetCompositingMode(nativeGraphics,
                                                     &mode));

        return mode;
    }

    Status SetCompositingQuality(IN CompositingQuality compositingQuality)
    {
        return SetStatus(DllExports::GdipSetCompositingQuality(
            nativeGraphics,
            compositingQuality));
    }

    CompositingQuality GetCompositingQuality() const
    {
        CompositingQuality quality;

        SetStatus(DllExports::GdipGetCompositingQuality(
            nativeGraphics,
            &quality));

        return quality;
    }

    Status SetTextRenderingHint(IN TextRenderingHint newMode)
    {
        return SetStatus(DllExports::GdipSetTextRenderingHint(nativeGraphics,
                                                          newMode));
    }

    TextRenderingHint GetTextRenderingHint() const
    {
        TextRenderingHint hint;

        SetStatus(DllExports::GdipGetTextRenderingHint(nativeGraphics,
                                                   &hint));

        return hint;
    }

    Status SetTextContrast(IN UINT contrast)
    {
        return SetStatus(DllExports::GdipSetTextContrast(nativeGraphics,
                                                          contrast));
    }

    UINT GetTextContrast() const
    {
        UINT contrast;

        SetStatus(DllExports::GdipGetTextContrast(nativeGraphics,
                                                    &contrast));

        return contrast;
    }

    InterpolationMode GetInterpolationMode() const
    {
        InterpolationMode mode = InterpolationModeInvalid;

        SetStatus(DllExports::GdipGetInterpolationMode(nativeGraphics,
                                                           &mode));

        return mode;
    }

    Status SetInterpolationMode(IN InterpolationMode interpolationMode)
    {
        return SetStatus(DllExports::GdipSetInterpolationMode(nativeGraphics,
                                                           interpolationMode));
    }

#if (GDIPVER >= 0x0110)
    Status SetAbort(GdiplusAbort *pIAbort)
    {
        return SetStatus(DllExports::GdipGraphicsSetAbort(
            nativeGraphics,
            pIAbort
        ));
    }
#endif //(GDIPVER >= 0x0110)

    SmoothingMode GetSmoothingMode() const
    {
        SmoothingMode smoothingMode = SmoothingModeInvalid;

        SetStatus(DllExports::GdipGetSmoothingMode(nativeGraphics,
                                                   &smoothingMode));

        return smoothingMode;
    }

    Status SetSmoothingMode(IN SmoothingMode smoothingMode)
    {
        return SetStatus(DllExports::GdipSetSmoothingMode(nativeGraphics,
                                                          smoothingMode));
    }

    PixelOffsetMode GetPixelOffsetMode() const
    {
        PixelOffsetMode pixelOffsetMode = PixelOffsetModeInvalid;

        SetStatus(DllExports::GdipGetPixelOffsetMode(nativeGraphics,
                                                     &pixelOffsetMode));

        return pixelOffsetMode;
    }

    Status SetPixelOffsetMode(IN PixelOffsetMode pixelOffsetMode)
    {
        return SetStatus(DllExports::GdipSetPixelOffsetMode(nativeGraphics,
                                                            pixelOffsetMode));
    }

    //------------------------------------------------------------------------
    // Manipulate current world transform
    //------------------------------------------------------------------------

    Status SetTransform(IN const Matrix* matrix)
    {
        return SetStatus(DllExports::GdipSetWorldTransform(nativeGraphics,
                                                        matrix->nativeMatrix));
    }
    Status ResetTransform()
    {
        return SetStatus(DllExports::GdipResetWorldTransform(nativeGraphics));
    }

    Status MultiplyTransform(IN const Matrix* matrix,
                             IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyWorldTransform(nativeGraphics,
                                                                matrix->nativeMatrix,
                                                                order));
    }

    Status TranslateTransform(IN REAL dx,
                              IN REAL dy,
                              IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslateWorldTransform(nativeGraphics,
                                                               dx, dy, order));
    }

    Status ScaleTransform(IN REAL sx,
                          IN REAL sy,
                          IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScaleWorldTransform(nativeGraphics,
                                                             sx, sy, order));
    }

    Status RotateTransform(IN REAL angle,
                           IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotateWorldTransform(nativeGraphics,
                                                              angle, order));
    }

    Status GetTransform(OUT Matrix* matrix) const
    {
        return SetStatus(DllExports::GdipGetWorldTransform(nativeGraphics,
                                                           matrix->nativeMatrix));
    }

    Status SetPageUnit(IN Unit unit)
    {
        return SetStatus(DllExports::GdipSetPageUnit(nativeGraphics,
                                                     unit));
    }

    Status SetPageScale(IN REAL scale)
    {
        return SetStatus(DllExports::GdipSetPageScale(nativeGraphics,
                                                      scale));
    }

    Unit GetPageUnit() const
    {
        Unit unit;

        SetStatus(DllExports::GdipGetPageUnit(nativeGraphics, &unit));

        return unit;
    }

    REAL GetPageScale() const
    {
        REAL scale;

        SetStatus(DllExports::GdipGetPageScale(nativeGraphics, &scale));

        return scale;
    }

    REAL GetDpiX() const
    {
        REAL dpi;

        SetStatus(DllExports::GdipGetDpiX(nativeGraphics, &dpi));

        return dpi;
    }

    REAL GetDpiY() const
    {
        REAL dpi;

        SetStatus(DllExports::GdipGetDpiY(nativeGraphics, &dpi));

        return dpi;
    }

    Status TransformPoints(IN CoordinateSpace destSpace,
                           IN CoordinateSpace srcSpace,
                           IN OUT PointF* pts,
                           IN INT count) const
    {
        return SetStatus(DllExports::GdipTransformPoints(nativeGraphics,
                                                         destSpace,
                                                         srcSpace,
                                                         pts,
                                                         count));
    }

    Status TransformPoints(IN CoordinateSpace destSpace,
                           IN CoordinateSpace srcSpace,
                           IN OUT Point* pts,
                           IN INT count) const
    {

        return SetStatus(DllExports::GdipTransformPointsI(nativeGraphics,
                                                          destSpace,
                                                          srcSpace,
                                                          pts,
                                                          count));
    }

    //------------------------------------------------------------------------
    // GetNearestColor (for <= 8bpp surfaces).  Note: Alpha is ignored.
    //------------------------------------------------------------------------
    
    Status GetNearestColor(IN OUT Color* color) const
    {
        if (color == NULL)
        {
            return SetStatus(InvalidParameter);
        }

        ARGB argb = color->GetValue();

        Status status = SetStatus(DllExports::GdipGetNearestColor(nativeGraphics, &argb));

        color->SetValue(argb);

        return status;
    }

    Status DrawLine(IN const Pen* pen,
                    IN REAL x1,
                    IN REAL y1,
                    IN REAL x2,
                    IN REAL y2)
    {
        return SetStatus(DllExports::GdipDrawLine(nativeGraphics,
                                                  pen->nativePen, x1, y1, x2,
                                                  y2));
    }

    Status DrawLine(IN const Pen* pen,
                    IN const PointF& pt1,
                    IN const PointF& pt2)
    {
        return DrawLine(pen, pt1.X, pt1.Y, pt2.X, pt2.Y);
    }

    Status DrawLines(IN const Pen* pen,
                     IN const PointF* points,
                     IN INT count)
    {
        return SetStatus(DllExports::GdipDrawLines(nativeGraphics,
                                                   pen->nativePen,
                                                   points, count));
    }

    Status DrawLine(IN const Pen* pen,
                    IN INT x1,
                    IN INT y1,
                    IN INT x2,
                    IN INT y2)
    {
        return SetStatus(DllExports::GdipDrawLineI(nativeGraphics,
                                                   pen->nativePen,
                                                   x1,
                                                   y1,
                                                   x2,
                                                   y2));
    }

    Status DrawLine(IN const Pen* pen,
                    IN const Point& pt1,
                    IN const Point& pt2)
    {
        return DrawLine(pen,
                        pt1.X,
                        pt1.Y,
                        pt2.X,
                        pt2.Y);
    }

    Status DrawLines(IN const Pen* pen,
                     IN const Point* points,
                     IN INT count)
    {
        return SetStatus(DllExports::GdipDrawLinesI(nativeGraphics,
                                                    pen->nativePen,
                                                    points,
                                                    count));
    }

    Status DrawArc(IN const Pen* pen,
                   IN REAL x,
                   IN REAL y,
                   IN REAL width,
                   IN REAL height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipDrawArc(nativeGraphics,
                                                 pen->nativePen,
                                                 x,
                                                 y,
                                                 width,
                                                 height,
                                                 startAngle,
                                                 sweepAngle));
    }

    Status DrawArc(IN const Pen* pen,
                   IN const RectF& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return DrawArc(pen, rect.X, rect.Y, rect.Width, rect.Height,
                       startAngle, sweepAngle);
    }

    Status DrawArc(IN const Pen* pen,
                   IN INT x,
                   IN INT y,
                   IN INT width,
                   IN INT height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipDrawArcI(nativeGraphics,
                                                  pen->nativePen,
                                                  x,
                                                  y,
                                                  width,
                                                  height,
                                                  startAngle,
                                                  sweepAngle));
    }


    Status DrawArc(IN const Pen* pen,
                   IN const Rect& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return DrawArc(pen,
                       rect.X,
                       rect.Y,
                       rect.Width,
                       rect.Height,
                       startAngle,
                       sweepAngle);
    }

    Status DrawBezier(IN const Pen* pen,
                      IN REAL x1,
                      IN REAL y1,
                      IN REAL x2,
                      IN REAL y2,
                      IN REAL x3,
                      IN REAL y3,
                      IN REAL x4,
                      IN REAL y4)
    {
        return SetStatus(DllExports::GdipDrawBezier(nativeGraphics,
                                                    pen->nativePen, x1, y1,
                                                    x2, y2, x3, y3, x4, y4));
    }

    Status DrawBezier(IN const Pen* pen,
                      IN const PointF& pt1,
                      IN const PointF& pt2,
                      IN const PointF& pt3,
                      IN const PointF& pt4)
    {
        return DrawBezier(pen,
                          pt1.X,
                          pt1.Y,
                          pt2.X,
                          pt2.Y,
                          pt3.X,
                          pt3.Y,
                          pt4.X,
                          pt4.Y);
    }

    Status DrawBeziers(IN const Pen* pen,
                       IN const PointF* points,
                       IN INT count)
    {
        return SetStatus(DllExports::GdipDrawBeziers(nativeGraphics,
                                                     pen->nativePen,
                                                     points,
                                                     count));
    }

    Status DrawBezier(IN const Pen* pen,
                      IN INT x1,
                      IN INT y1,
                      IN INT x2,
                      IN INT y2,
                      IN INT x3,
                      IN INT y3,
                      IN INT x4,
                      IN INT y4)
    {
        return SetStatus(DllExports::GdipDrawBezierI(nativeGraphics,
                                                     pen->nativePen,
                                                     x1,
                                                     y1,
                                                     x2,
                                                     y2,
                                                     x3,
                                                     y3,
                                                     x4,
                                                     y4));
    }

    Status DrawBezier(IN const Pen* pen,
                      IN const Point& pt1,
                      IN const Point& pt2,
                      IN const Point& pt3,
                      IN const Point& pt4)
    {
        return DrawBezier(pen,
                          pt1.X,
                          pt1.Y,
                          pt2.X,
                          pt2.Y,
                          pt3.X,
                          pt3.Y,
                          pt4.X,
                          pt4.Y);
    }

    Status DrawBeziers(IN const Pen* pen,
                       IN const Point* points,
                       IN INT count)
    {
        return SetStatus(DllExports::GdipDrawBeziersI(nativeGraphics,
                                                      pen->nativePen,
                                                      points,
                                                      count));
    }

    Status DrawRectangle(IN const Pen* pen,
                         IN const RectF& rect)
    {
        return DrawRectangle(pen, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status DrawRectangle(IN const Pen* pen,
                         IN REAL x,
                         IN REAL y,
                         IN REAL width,
                         IN REAL height)
    {
        return SetStatus(DllExports::GdipDrawRectangle(nativeGraphics,
                                                       pen->nativePen, x, y,
                                                       width, height));
    }

    Status DrawRectangles(IN const Pen* pen,
                          IN const RectF* rects,
                          IN INT count)
    {
        return SetStatus(DllExports::GdipDrawRectangles(nativeGraphics,
                                                        pen->nativePen,
                                                        rects, count));
    }

    Status DrawRectangle(IN const Pen* pen,
                         IN const Rect& rect)
    {
        return DrawRectangle(pen,
                             rect.X,
                             rect.Y,
                             rect.Width,
                             rect.Height);
    }

    Status DrawRectangle(IN const Pen* pen,
                         IN INT x,
                         IN INT y,
                         IN INT width,
                         IN INT height)
    {
        return SetStatus(DllExports::GdipDrawRectangleI(nativeGraphics,
                                                        pen->nativePen,
                                                        x,
                                                        y,
                                                        width,
                                                        height));
    }

    Status DrawRectangles(IN const Pen* pen,
                          IN const Rect* rects,
                          IN INT count)
    {
        return SetStatus(DllExports::GdipDrawRectanglesI(nativeGraphics,
                                                         pen->nativePen,
                                                         rects,
                                                         count));
    }

    Status DrawEllipse(IN const Pen* pen,
                       IN const RectF& rect)
    {
        return DrawEllipse(pen, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status DrawEllipse(IN const Pen* pen,
                       IN REAL x,
                       IN REAL y,
                       IN REAL width,
                       IN REAL height)
    {
        return SetStatus(DllExports::GdipDrawEllipse(nativeGraphics,
                                                     pen->nativePen,
                                                     x,
                                                     y,
                                                     width,
                                                     height));
    }

    Status DrawEllipse(IN const Pen* pen,
                       IN const Rect& rect)
    {
        return DrawEllipse(pen,
                           rect.X,
                           rect.Y,
                           rect.Width,
                           rect.Height);
    }

    Status DrawEllipse(IN const Pen* pen,
                       IN INT x,
                       IN INT y,
                       IN INT width,
                       IN INT height)
    {
        return SetStatus(DllExports::GdipDrawEllipseI(nativeGraphics,
                                                      pen->nativePen,
                                                      x,
                                                      y,
                                                      width,
                                                      height));
    }

    Status DrawPie(IN const Pen* pen,
                   IN const RectF& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return DrawPie(pen,
                       rect.X,
                       rect.Y,
                       rect.Width,
                       rect.Height,
                       startAngle,
                       sweepAngle);
    }

    Status DrawPie(IN const Pen* pen,
                   IN REAL x,
                   IN REAL y,
                   IN REAL width,
                   IN REAL height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipDrawPie(nativeGraphics,
                                                 pen->nativePen,
                                                 x,
                                                 y,
                                                 width,
                                                 height,
                                                 startAngle,
                                                 sweepAngle));
    }

    Status DrawPie(IN const Pen* pen,
                   IN const Rect& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return DrawPie(pen,
                       rect.X,
                       rect.Y,
                       rect.Width,
                       rect.Height,
                       startAngle,
                       sweepAngle);
    }

    Status DrawPie(IN const Pen* pen,
                   IN INT x,
                   IN INT y,
                   IN INT width,
                   IN INT height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipDrawPieI(nativeGraphics,
                                                  pen->nativePen,
                                                  x,
                                                  y,
                                                  width,
                                                  height,
                                                  startAngle,
                                                  sweepAngle));
    }

    Status DrawPolygon(IN const Pen* pen,
                       IN const PointF* points,
                       IN INT count)
    {
        return SetStatus(DllExports::GdipDrawPolygon(nativeGraphics,
                                                     pen->nativePen,
                                                     points,
                                                     count));
    }

    Status DrawPolygon(IN const Pen* pen,
                       IN const Point* points,
                       IN INT count)
    {
        return SetStatus(DllExports::GdipDrawPolygonI(nativeGraphics,
                                                      pen->nativePen,
                                                      points,
                                                      count));
    }

    Status DrawPath(IN const Pen* pen,
                    IN const GraphicsPath* path)
    {
        return SetStatus(DllExports::GdipDrawPath(nativeGraphics,
                                                  pen ? pen->nativePen : NULL,
                                                  path ? path->nativePath : NULL));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const PointF* points,
                     IN INT count)
    {
        return SetStatus(DllExports::GdipDrawCurve(nativeGraphics,
                                                   pen->nativePen, points,
                                                   count));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const PointF* points,
                     IN INT count,
                     IN REAL tension)
    {
        return SetStatus(DllExports::GdipDrawCurve2(nativeGraphics,
                                                    pen->nativePen, points,
                                                    count, tension));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const PointF* points,
                     IN INT count,
                     IN INT offset,
                     IN INT numberOfSegments,
                     IN REAL tension = 0.5f)
    {
        return SetStatus(DllExports::GdipDrawCurve3(nativeGraphics,
                                                    pen->nativePen, points,
                                                    count, offset,
                                                    numberOfSegments, tension));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const Point* points,
                     IN INT count)
    {
        return SetStatus(DllExports::GdipDrawCurveI(nativeGraphics,
                                                    pen->nativePen,
                                                    points,
                                                    count));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const Point* points,
                     IN INT count,
                     IN REAL tension)
    {
        return SetStatus(DllExports::GdipDrawCurve2I(nativeGraphics,
                                                     pen->nativePen,
                                                     points,
                                                     count,
                                                     tension));
    }

    Status DrawCurve(IN const Pen* pen,
                     IN const Point* points,
                     IN INT count,
                     IN INT offset,
                     IN INT numberOfSegments,
                     IN REAL tension = 0.5f)
    {
        return SetStatus(DllExports::GdipDrawCurve3I(nativeGraphics,
                                                     pen->nativePen,
                                                     points,
                                                     count,
                                                     offset,
                                                     numberOfSegments,
                                                     tension));
    }

    Status DrawClosedCurve(IN const Pen* pen,
                           IN const PointF* points,
                           IN INT count)
    {
        return SetStatus(DllExports::GdipDrawClosedCurve(nativeGraphics,
                                                         pen->nativePen,
                                                         points, count));
    }

    Status DrawClosedCurve(IN const Pen *pen,
                           IN const PointF* points,
                           IN INT count,
                           IN REAL tension)
    {
        return SetStatus(DllExports::GdipDrawClosedCurve2(nativeGraphics,
                                                          pen->nativePen,
                                                          points, count,
                                                          tension));
    }

    Status DrawClosedCurve(IN const Pen* pen,
                           IN const Point* points,
                           IN INT count)
    {
        return SetStatus(DllExports::GdipDrawClosedCurveI(nativeGraphics,
                                                          pen->nativePen,
                                                          points,
                                                          count));
    }

    Status DrawClosedCurve(IN const Pen *pen,
                           IN const Point* points,
                           IN INT count,
                           IN REAL tension)
    {
        return SetStatus(DllExports::GdipDrawClosedCurve2I(nativeGraphics,
                                                           pen->nativePen,
                                                           points,
                                                           count,
                                                           tension));
    }

    Status Clear(IN const Color &color)
    {
        return SetStatus(DllExports::GdipGraphicsClear(
            nativeGraphics,
            color.GetValue()));
    }

    Status FillRectangle(IN const Brush* brush,
                         IN const RectF& rect)
    {
        return FillRectangle(brush, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status FillRectangle(IN const Brush* brush,
                         IN REAL x,
                         IN REAL y,
                         IN REAL width,
                         IN REAL height)
    {
        return SetStatus(DllExports::GdipFillRectangle(nativeGraphics,
                                                       brush->nativeBrush, x, y,
                                                       width, height));
    }

    Status FillRectangles(IN const Brush* brush,
                          IN const RectF* rects,
                          IN INT count)
    {
        return SetStatus(DllExports::GdipFillRectangles(nativeGraphics,
                                                        brush->nativeBrush,
                                                        rects, count));
    }

    Status FillRectangle(IN const Brush* brush,
                         IN const Rect& rect)
    {
        return FillRectangle(brush,
                             rect.X,
                             rect.Y,
                             rect.Width,
                             rect.Height);
    }

    Status FillRectangle(IN const Brush* brush,
                         IN INT x,
                         IN INT y,
                         IN INT width,
                         IN INT height)
    {
        return SetStatus(DllExports::GdipFillRectangleI(nativeGraphics,
                                                        brush->nativeBrush,
                                                        x,
                                                        y,
                                                        width,
                                                        height));
    }

    Status FillRectangles(IN const Brush* brush,
                          IN const Rect* rects,
                          IN INT count)
    {
        return SetStatus(DllExports::GdipFillRectanglesI(nativeGraphics,
                                                         brush->nativeBrush,
                                                         rects,
                                                         count));
    }

    Status FillPolygon(IN const Brush* brush,
                       IN const PointF* points,
                       IN INT count)
    {
        return FillPolygon(brush, points, count, FillModeAlternate);
    }

    Status FillPolygon(IN const Brush* brush,
                       IN const PointF* points,
                       IN INT count,
                       IN FillMode fillMode)
    {
        return SetStatus(DllExports::GdipFillPolygon(nativeGraphics,
                                                     brush->nativeBrush,
                                                     points, count, fillMode));
    }

    Status FillPolygon(IN const Brush* brush,
                       IN const Point* points,
                       IN INT count)
    {
        return FillPolygon(brush, points, count, FillModeAlternate);
    }

    Status FillPolygon(IN const Brush* brush,
                       IN const Point* points,
                       IN INT count,
                       IN FillMode fillMode)
    {
        return SetStatus(DllExports::GdipFillPolygonI(nativeGraphics,
                                                      brush->nativeBrush,
                                                      points, count,
                                                      fillMode));
    }

    Status FillEllipse(IN const Brush* brush,
                       IN const RectF& rect)
    {
        return FillEllipse(brush, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status FillEllipse(IN const Brush* brush,
                       IN REAL x,
                       IN REAL y,
                       IN REAL width,
                       IN REAL height)
    {
        return SetStatus(DllExports::GdipFillEllipse(nativeGraphics,
                                                     brush->nativeBrush, x, y,
                                                     width, height));
    }

    Status FillEllipse(IN const Brush* brush,
                       IN const Rect& rect)
    {
        return FillEllipse(brush, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status FillEllipse(IN const Brush* brush,
                       IN INT x,
                       IN INT y,
                       IN INT width,
                       IN INT height)
    {
        return SetStatus(DllExports::GdipFillEllipseI(nativeGraphics,
                                                      brush->nativeBrush,
                                                      x,
                                                      y,
                                                      width,
                                                      height));
    }

    Status FillPie(IN const Brush* brush,
                   IN const RectF& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return FillPie(brush, rect.X, rect.Y, rect.Width, rect.Height,
                       startAngle, sweepAngle);
    }

    Status FillPie(IN const Brush* brush,
                   IN REAL x,
                   IN REAL y,
                   IN REAL width,
                   IN REAL height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipFillPie(nativeGraphics,
                                                 brush->nativeBrush, x, y,
                                                 width, height, startAngle,
                                                 sweepAngle));
    }

    Status FillPie(IN const Brush* brush,
                   IN const Rect& rect,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return FillPie(brush, rect.X, rect.Y, rect.Width, rect.Height,
                       startAngle, sweepAngle);
    }

    Status FillPie(IN const Brush* brush,
                   IN INT x,
                   IN INT y,
                   IN INT width,
                   IN INT height,
                   IN REAL startAngle,
                   IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipFillPieI(nativeGraphics,
                                                  brush->nativeBrush,
                                                  x,
                                                  y,
                                                  width,
                                                  height,
                                                  startAngle,
                                                  sweepAngle));
    }

    Status FillPath(IN const Brush* brush,
                    IN const GraphicsPath* path)
    {
        return SetStatus(DllExports::GdipFillPath(nativeGraphics,
                                                  brush->nativeBrush,
                                                  path->nativePath));
    }

    Status FillClosedCurve(IN const Brush* brush,
                           IN const PointF* points,
                           IN INT count)
    {
        return SetStatus(DllExports::GdipFillClosedCurve(nativeGraphics,
                                                         brush->nativeBrush,
                                                         points, count));

    }

    Status FillClosedCurve(IN const Brush* brush,
                           IN const PointF* points,
                           IN INT count,
                           IN FillMode fillMode,
                           IN REAL tension = 0.5f)
    {
        return SetStatus(DllExports::GdipFillClosedCurve2(nativeGraphics,
                                                          brush->nativeBrush,
                                                          points, count,
                                                          tension, fillMode));
    }

    Status FillClosedCurve(IN const Brush* brush,
                           IN const Point* points,
                           IN INT count)
    {
        return SetStatus(DllExports::GdipFillClosedCurveI(nativeGraphics,
                                                          brush->nativeBrush,
                                                          points,
                                                          count));
    }

    Status FillClosedCurve(IN const Brush* brush,
                           IN const Point* points,
                           IN INT count,
                           IN FillMode fillMode,
                           IN REAL tension = 0.5f)
    {
        return SetStatus(DllExports::GdipFillClosedCurve2I(nativeGraphics,
                                                           brush->nativeBrush,
                                                           points, count,
                                                           tension, fillMode));
    }

    Status FillRegion(IN const Brush* brush,
                      IN const Region* region)
    {
        return SetStatus(DllExports::GdipFillRegion(nativeGraphics,
                                                    brush->nativeBrush,
                                                    region->nativeRegion));
    }

    Status
    DrawString(
        IN const WCHAR        *string,
        IN INT                 length,
        IN const Font         *font,
        IN const RectF        &layoutRect,
        IN const StringFormat *stringFormat,
        IN const Brush        *brush
    )
    {
        return SetStatus(DllExports::GdipDrawString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &layoutRect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            brush ? brush->nativeBrush : NULL
        ));
    }

    Status
    DrawString(
        const WCHAR        *string,
        INT                 length,
        const Font         *font,
        const PointF       &origin,
        const Brush        *brush
    )
    {
        RectF rect(origin.X, origin.Y, 0.0f, 0.0f);

        return SetStatus(DllExports::GdipDrawString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &rect,
            NULL,
            brush ? brush->nativeBrush : NULL
        ));
    }

    Status
    DrawString(
        const WCHAR        *string,
        INT                 length,
        const Font         *font,
        const PointF       &origin,
        const StringFormat *stringFormat,
        const Brush        *brush
    )
    {
        RectF rect(origin.X, origin.Y, 0.0f, 0.0f);

        return SetStatus(DllExports::GdipDrawString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &rect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            brush ? brush->nativeBrush : NULL
        ));
    }

    Status
    MeasureString(
        IN const WCHAR        *string,
        IN INT                 length,
        IN const Font         *font,
        IN const RectF        &layoutRect,
        IN const StringFormat *stringFormat,
        OUT RectF             *boundingBox,
        OUT INT               *codepointsFitted = 0,
        OUT INT               *linesFilled      = 0
    ) const
    {
        return SetStatus(DllExports::GdipMeasureString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &layoutRect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            boundingBox,
            codepointsFitted,
            linesFilled
        ));
    }

    Status
    MeasureString(
        IN const WCHAR        *string,
        IN INT                 length,
        IN const Font         *font,
        IN const SizeF        &layoutRectSize,
        IN const StringFormat *stringFormat,
        OUT SizeF             *size,
        OUT INT               *codepointsFitted = 0,
        OUT INT               *linesFilled      = 0
    ) const
    {
        RectF   layoutRect(0, 0, layoutRectSize.Width, layoutRectSize.Height);
        RectF   boundingBox;
        Status  status;

        if (size == NULL)
        {
            return SetStatus(InvalidParameter);
        }

        status = SetStatus(DllExports::GdipMeasureString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &layoutRect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            size ? &boundingBox : NULL,
            codepointsFitted,
            linesFilled
        ));

        if (size && status == Ok)
        {
            size->Width  = boundingBox.Width;
            size->Height = boundingBox.Height;
        }

        return status;
    }

    Status
    MeasureString(
        IN const WCHAR        *string,
        IN INT                 length,
        IN const Font         *font,
        IN const PointF       &origin,
        IN const StringFormat *stringFormat,
        OUT RectF             *boundingBox
    ) const
    {
        RectF rect(origin.X, origin.Y, 0.0f, 0.0f);

        return SetStatus(DllExports::GdipMeasureString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &rect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            boundingBox,
            NULL,
            NULL
        ));
    }

    Status
    MeasureString(
        IN const WCHAR  *string,
        IN INT           length,
        IN const Font   *font,
        IN const RectF  &layoutRect,
        OUT RectF       *boundingBox
    ) const
    {
        return SetStatus(DllExports::GdipMeasureString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &layoutRect,
            NULL,
            boundingBox,
            NULL,
            NULL
        ));
    }

    Status
    MeasureString(
        IN const WCHAR  *string,
        IN INT           length,
        IN const Font   *font,
        IN const PointF &origin,
        OUT RectF       *boundingBox
    ) const
    {
        RectF rect(origin.X, origin.Y, 0.0f, 0.0f);

        return SetStatus(DllExports::GdipMeasureString(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            &rect,
            NULL,
            boundingBox,
            NULL,
            NULL
        ));
    }


    Status
    MeasureCharacterRanges(
        IN const WCHAR        *string,
        IN INT                 length,
        IN const Font         *font,
        IN const RectF        &layoutRect,
        IN const StringFormat *stringFormat,
        IN INT                 regionCount,
        OUT Region            *regions
    ) const
    {
        if (!regions || regionCount <= 0)
        {
            return InvalidParameter;
        }

        GpRegion **nativeRegions = new GpRegion* [regionCount];

        if (!nativeRegions)
        {
            return OutOfMemory;
        }

        for (INT i = 0; i < regionCount; i++)
        {
            nativeRegions[i] = regions[i].nativeRegion;
        }

        Status status = SetStatus(DllExports::GdipMeasureCharacterRanges(
            nativeGraphics,
            string,
            length,
            font ? font->nativeFont : NULL,
            layoutRect,
            stringFormat ? stringFormat->nativeFormat : NULL,
            regionCount,
            nativeRegions
        ));

        delete [] nativeRegions;

        return status;
    }

    Status DrawDriverString(
        IN const UINT16  *text,
        IN INT            length,
        IN const Font    *font,
        IN const Brush   *brush,
        IN const PointF  *positions,
        IN INT            flags,
        IN const Matrix        *matrix
    )
    {
        return SetStatus(DllExports::GdipDrawDriverString(
            nativeGraphics,
            text,
            length,
            font ? font->nativeFont : NULL,
            brush ? brush->nativeBrush : NULL,
            positions,
            flags,
            matrix ? matrix->nativeMatrix : NULL
        ));
    }

    Status MeasureDriverString(
        IN const UINT16  *text,
        IN INT            length,
        IN const Font    *font,
        IN const PointF  *positions,
        IN INT            flags,
        IN const Matrix        *matrix,
        OUT RectF        *boundingBox
    ) const
    {
        return SetStatus(DllExports::GdipMeasureDriverString(
            nativeGraphics,
            text,
            length,
            font ? font->nativeFont : NULL,
            positions,
            flags,
            matrix ? matrix->nativeMatrix : NULL,
            boundingBox
        ));
    }

    // Draw a cached bitmap on this graphics destination offset by
    // x, y. Note this will fail with WrongState if the CachedBitmap
    // native format differs from this Graphics.

    Status DrawCachedBitmap(IN CachedBitmap *cb,
                            IN INT x,
                            IN INT y)
    {
        return SetStatus(DllExports::GdipDrawCachedBitmap(
            nativeGraphics,
            cb->nativeCachedBitmap,
            x, y
        ));
    }

    Status DrawImage(IN Image* image,
                     IN const PointF& point)
    {
        return DrawImage(image, point.X, point.Y);
    }

    Status DrawImage(IN Image* image,
                     IN REAL x,
                     IN REAL y)
    {
        return SetStatus(DllExports::GdipDrawImage(nativeGraphics,
                                                   image ? image->nativeImage
                                                         : NULL,
                                                   x,
                                                   y));
    }

    Status DrawImage(IN Image* image, 
                     IN const RectF& rect)
    {
        return DrawImage(image, rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status DrawImage(IN Image* image,
                     IN REAL x,
                     IN REAL y,
                     IN REAL width,
                     IN REAL height)
    {
        return SetStatus(DllExports::GdipDrawImageRect(nativeGraphics,
                                                       image ? image->nativeImage
                                                             : NULL,
                                                       x,
                                                       y,
                                                       width,
                                                       height));
    }

    Status DrawImage(IN Image* image,
                     IN const Point& point)
    {
        return DrawImage(image, point.X, point.Y);
    }

    Status DrawImage(IN Image* image,
                     IN INT x,
                     IN INT y)
    {
        return SetStatus(DllExports::GdipDrawImageI(nativeGraphics,
                                                    image ? image->nativeImage
                                                          : NULL,
                                                    x,
                                                    y));
    }

    Status DrawImage(IN Image* image,
                     IN const Rect& rect)
    {
        return DrawImage(image,
                         rect.X,
                         rect.Y,
                         rect.Width,
                         rect.Height);
    }

    Status DrawImage(IN Image* image,
                     IN INT x,
                     IN INT y,
                     IN INT width,
                     IN INT height) {
        return SetStatus(DllExports::GdipDrawImageRectI(nativeGraphics,
                                                        image ? image->nativeImage
                                                              : NULL,
                                                        x,
                                                        y,
                                                        width,
                                                        height));
    }

    
    Status DrawImage(IN Image* image,
                     IN const PointF* destPoints,
                     IN INT count)
    {
        if (count != 3 && count != 4)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipDrawImagePoints(nativeGraphics,
                                                         image ? image->nativeImage
                                                               : NULL,
                                                         destPoints, count));
    }

    Status DrawImage(IN Image* image,
                     IN const Point* destPoints,
                     IN INT count)
    {
        if (count != 3 && count != 4)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipDrawImagePointsI(nativeGraphics,
                                                          image ? image->nativeImage
                                                                : NULL,
                                                          destPoints,
                                                          count));
    }

    Status DrawImage(IN Image* image,
                     IN REAL x,
                     IN REAL y,
                     IN REAL srcx,
                     IN REAL srcy,
                     IN REAL srcwidth,
                     IN REAL srcheight,
                     IN Unit srcUnit)
    {
        return SetStatus(DllExports::GdipDrawImagePointRect(nativeGraphics,
                                                            image ? image->nativeImage
                                                                  : NULL,
                                                            x, y,
                                                            srcx, srcy,
                                                            srcwidth, srcheight, srcUnit));
    }

    Status DrawImage(IN Image* image,
                     IN const RectF& destRect,
                     IN REAL srcx,
                     IN REAL srcy,
                     IN REAL srcwidth,
                     IN REAL srcheight,
                     IN Unit srcUnit,
                     IN const ImageAttributes* imageAttributes = NULL,
                     IN DrawImageAbort callback = NULL,
                     IN VOID* callbackData = NULL)
    {
        return SetStatus(DllExports::GdipDrawImageRectRect(nativeGraphics,
                                                           image ? image->nativeImage
                                                                 : NULL,
                                                           destRect.X,
                                                           destRect.Y,
                                                           destRect.Width,
                                                           destRect.Height,
                                                           srcx, srcy,
                                                           srcwidth, srcheight,
                                                           srcUnit,
                                                           imageAttributes
                                                            ? imageAttributes->nativeImageAttr
                                                            : NULL,
                                                           callback,
                                                           callbackData));
    }

    Status DrawImage(IN Image* image,
                     IN const PointF* destPoints,
                     IN INT count,
                     IN REAL srcx,
                     IN REAL srcy,
                     IN REAL srcwidth,
                     IN REAL srcheight,
                     IN Unit srcUnit,
                     IN const ImageAttributes* imageAttributes = NULL,
                     IN DrawImageAbort callback = NULL,
                     IN VOID* callbackData = NULL)
    {
        return SetStatus(DllExports::GdipDrawImagePointsRect(nativeGraphics,
                                                             image ? image->nativeImage
                                                                   : NULL,
                                                             destPoints, count,
                                                             srcx, srcy,
                                                             srcwidth,
                                                             srcheight,
                                                             srcUnit,
                                                             imageAttributes
                                                              ? imageAttributes->nativeImageAttr
                                                              : NULL,
                                                             callback,
                                                             callbackData));
    }

    Status DrawImage(IN Image* image,
                     IN INT x,
                     IN INT y,
                     IN INT srcx,
                     IN INT srcy,
                     IN INT srcwidth,
                     IN INT srcheight,
                     IN Unit srcUnit)
    {
        return SetStatus(DllExports::GdipDrawImagePointRectI(nativeGraphics,
                                                             image ? image->nativeImage
                                                                   : NULL,
                                                             x,
                                                             y,
                                                             srcx,
                                                             srcy,
                                                             srcwidth,
                                                             srcheight,
                                                             srcUnit));
    }

    Status DrawImage(IN Image* image,
                     IN const Rect& destRect,
                     IN INT srcx,
                     IN INT srcy,
                     IN INT srcwidth,
                     IN INT srcheight,
                     IN Unit srcUnit,
                     IN const ImageAttributes* imageAttributes = NULL,
                     IN DrawImageAbort callback = NULL,
                     IN VOID* callbackData = NULL)
    {
        return SetStatus(DllExports::GdipDrawImageRectRectI(nativeGraphics,
                                                            image ? image->nativeImage
                                                                  : NULL,
                                                            destRect.X,
                                                            destRect.Y,
                                                            destRect.Width,
                                                            destRect.Height,
                                                            srcx,
                                                            srcy,
                                                            srcwidth,
                                                            srcheight,
                                                            srcUnit,
                                                            imageAttributes
                                                            ? imageAttributes->nativeImageAttr
                                                            : NULL,
                                                            callback,
                                                            callbackData));
    }

    Status DrawImage(IN Image* image,
                     IN const Point* destPoints,
                     IN INT count,
                     IN INT srcx,
                     IN INT srcy,
                     IN INT srcwidth,
                     IN INT srcheight,
                     IN Unit srcUnit,
                     IN const ImageAttributes* imageAttributes = NULL,
                     IN DrawImageAbort callback = NULL,
                     IN VOID* callbackData = NULL)
    {
        return SetStatus(DllExports::GdipDrawImagePointsRectI(nativeGraphics,
                                                              image ? image->nativeImage
                                                                    : NULL,
                                                              destPoints,
                                                              count,
                                                              srcx,
                                                              srcy,
                                                              srcwidth,
                                                              srcheight,
                                                              srcUnit,
                                                              imageAttributes
                                                               ? imageAttributes->nativeImageAttr
                                                               : NULL,
                                                              callback,
                                                              callbackData));
    }
    
#if (GDIPVER >= 0x0110)
    Status DrawImage(
        IN Image *image,
        IN const RectF &destRect,
        IN const RectF &sourceRect,
        IN Unit srcUnit,
        IN const ImageAttributes *imageAttributes = NULL
    )
    {
        return SetStatus(DllExports::GdipDrawImageRectRect(
            nativeGraphics,
            image->nativeImage,
            destRect.X,
            destRect.Y,
            destRect.Width,
            destRect.Height,
            sourceRect.X,
            sourceRect.Y,
            sourceRect.Width,
            sourceRect.Height,
            srcUnit,
            imageAttributes ? imageAttributes->nativeImageAttr : NULL,
            NULL,
            NULL
        ));
    }

    Status DrawImage(
        IN Image *image,
        IN RectF *sourceRect,
        IN Matrix *xForm,
        IN Effect *effect,
        IN ImageAttributes *imageAttributes,
        IN Unit srcUnit
    )
    {
        return SetStatus(DllExports::GdipDrawImageFX(
            nativeGraphics,
            image->nativeImage,
            sourceRect,
            xForm ? xForm->nativeMatrix : NULL,
            effect ? effect->nativeEffect : NULL,
            imageAttributes ? imageAttributes->nativeImageAttr : NULL,
            srcUnit
        ));
    }
#endif //(GDIPVER >= 0x0110)

    // The following methods are for playing an EMF+ to a graphics
    // via the enumeration interface.  Each record of the EMF+ is
    // sent to the callback (along with the callbackData).  Then
    // the callback can invoke the Metafile::PlayRecord method
    // to play the particular record.

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const PointF &          destPoint,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestPoint(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoint,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Point &           destPoint,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestPointI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoint,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const RectF &           destRect,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestRect(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destRect,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Rect &            destRect,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestRectI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destRect,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const PointF *          destPoints,
        IN INT                     count,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestPoints(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoints,
                    count,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Point *           destPoints,
        IN INT                     count,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileDestPointsI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoints,
                    count,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const PointF &          destPoint,
        IN const RectF &           srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestPoint(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoint,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Point &           destPoint,
        IN const Rect &            srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestPointI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoint,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const RectF &           destRect,
        IN const RectF &           srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestRect(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destRect,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Rect &            destRect,
        IN const Rect &            srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestRectI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destRect,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const PointF *          destPoints,
        IN INT                     count,
        IN const RectF &           srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestPoints(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoints,
                    count,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }

    Status
    EnumerateMetafile(
        IN const Metafile *        metafile,
        IN const Point *           destPoints,
        IN INT                     count,
        IN const Rect &            srcRect,
        IN Unit                    srcUnit,
        IN EnumerateMetafileProc   callback,
        IN VOID *                  callbackData    = NULL,
        IN const ImageAttributes *       imageAttributes = NULL
        )
    {
        return SetStatus(DllExports::GdipEnumerateMetafileSrcRectDestPointsI(
                    nativeGraphics,
                    (const GpMetafile *)(metafile ? metafile->nativeImage:NULL),
                    destPoints,
                    count,
                    srcRect,
                    srcUnit,
                    callback,
                    callbackData,
                    imageAttributes ? imageAttributes->nativeImageAttr : NULL));
    }
    
    Status SetClip(IN const Graphics* g,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipGraphics(nativeGraphics,
                                                         g->nativeGraphics,
                                                         combineMode));
    }

    Status SetClip(IN const RectF& rect,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipRect(nativeGraphics,
                                                     rect.X, rect.Y,
                                                     rect.Width, rect.Height,
                                                     combineMode));
    }

    Status SetClip(IN const Rect& rect,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipRectI(nativeGraphics,
                                                      rect.X, rect.Y,
                                                      rect.Width, rect.Height,
                                                      combineMode));
    }

    Status SetClip(IN const GraphicsPath* path,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipPath(nativeGraphics,
                                                     path->nativePath,
                                                     combineMode));
    }

    Status SetClip(IN const Region* region,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipRegion(nativeGraphics,
                                                       region->nativeRegion,
                                                       combineMode));
    }

    // This is different than the other SetClip methods because it assumes
    // that the HRGN is already in device units, so it doesn't transform
    // the coordinates in the HRGN.
    
    Status SetClip(IN HRGN hRgn,
                   IN CombineMode combineMode = CombineModeReplace)
    {
        return SetStatus(DllExports::GdipSetClipHrgn(nativeGraphics, hRgn,
                                                     combineMode));
    }

    Status IntersectClip(IN const RectF& rect)
    {
        return SetStatus(DllExports::GdipSetClipRect(nativeGraphics,
                                                     rect.X, rect.Y,
                                                     rect.Width, rect.Height,
                                                     CombineModeIntersect));
    }

    Status IntersectClip(IN const Rect& rect)
    {
        return SetStatus(DllExports::GdipSetClipRectI(nativeGraphics,
                                                      rect.X, rect.Y,
                                                      rect.Width, rect.Height,
                                                      CombineModeIntersect));
    }

    Status IntersectClip(IN const Region* region)
    {
        return SetStatus(DllExports::GdipSetClipRegion(nativeGraphics,
                                                       region->nativeRegion,
                                                       CombineModeIntersect));
    }

    Status ExcludeClip(IN const RectF& rect)
    {
        return SetStatus(DllExports::GdipSetClipRect(nativeGraphics,
                                                     rect.X, rect.Y,
                                                     rect.Width, rect.Height,
                                                     CombineModeExclude));
    }

    Status ExcludeClip(IN const Rect& rect)
    {
        return SetStatus(DllExports::GdipSetClipRectI(nativeGraphics,
                                                      rect.X, rect.Y,
                                                      rect.Width, rect.Height,
                                                      CombineModeExclude));
    }

    Status ExcludeClip(IN const Region* region)
    {
        return SetStatus(DllExports::GdipSetClipRegion(nativeGraphics,
                                                       region->nativeRegion,
                                                       CombineModeExclude));
    }

    Status ResetClip()
    {
        return SetStatus(DllExports::GdipResetClip(nativeGraphics));
    }

    Status TranslateClip(IN REAL dx,
                         IN REAL dy)
    {
        return SetStatus(DllExports::GdipTranslateClip(nativeGraphics, dx, dy));
    }

    Status TranslateClip(IN INT dx,
                         IN INT dy)
    {
        return SetStatus(DllExports::GdipTranslateClipI(nativeGraphics,
                                                        dx, dy));
    }

    Status GetClip(OUT Region* region) const
    {
        return SetStatus(DllExports::GdipGetClip(nativeGraphics,
                                                 region->nativeRegion));
    }

    Status GetClipBounds(OUT RectF* rect) const
    {
        return SetStatus(DllExports::GdipGetClipBounds(nativeGraphics, rect));
    }

    Status GetClipBounds(OUT Rect* rect) const
    {
        return SetStatus(DllExports::GdipGetClipBoundsI(nativeGraphics, rect));
    }

    BOOL IsClipEmpty() const
    {
        BOOL booln = FALSE;

        SetStatus(DllExports::GdipIsClipEmpty(nativeGraphics, &booln));

        return booln;
    }

    Status GetVisibleClipBounds(OUT RectF *rect) const
    {

        return SetStatus(DllExports::GdipGetVisibleClipBounds(nativeGraphics,
                                                              rect));
    }

    Status GetVisibleClipBounds(OUT Rect *rect) const
    {
       return SetStatus(DllExports::GdipGetVisibleClipBoundsI(nativeGraphics,
                                                              rect));
    }

    BOOL IsVisibleClipEmpty() const
    {
        BOOL booln = FALSE;

        SetStatus(DllExports::GdipIsVisibleClipEmpty(nativeGraphics, &booln));

        return booln;
    }

    BOOL IsVisible(IN INT x,
                   IN INT y) const
    {
        return IsVisible(Point(x,y));
    }

    BOOL IsVisible(IN const Point& point) const
    {
        BOOL booln = FALSE;

        SetStatus(DllExports::GdipIsVisiblePointI(nativeGraphics,
                                                  point.X,
                                                  point.Y,
                                                  &booln));

        return booln;
    }

    BOOL IsVisible(IN INT x,
                   IN INT y,
                   IN INT width,
                   IN INT height) const
    {
        return IsVisible(Rect(x, y, width, height));
    }

    BOOL IsVisible(IN const Rect& rect) const
    {

        BOOL booln = TRUE;

        SetStatus(DllExports::GdipIsVisibleRectI(nativeGraphics,
                                                 rect.X,
                                                 rect.Y,
                                                 rect.Width,
                                                 rect.Height,
                                                 &booln));
        return booln;
    }

    BOOL IsVisible(IN REAL x,
                   IN REAL y) const
    {
        return IsVisible(PointF(x, y));
    }

    BOOL IsVisible(IN const PointF& point) const
    {
        BOOL booln = FALSE;

        SetStatus(DllExports::GdipIsVisiblePoint(nativeGraphics,
                                                 point.X,
                                                 point.Y,
                                                 &booln));

        return booln;
    }

    BOOL IsVisible(IN REAL x,
                   IN REAL y,
                   IN REAL width,
                   IN REAL height) const
    {
        return IsVisible(RectF(x, y, width, height));
    }

    BOOL IsVisible(IN const RectF& rect) const
    {
        BOOL booln = TRUE;

        SetStatus(DllExports::GdipIsVisibleRect(nativeGraphics,
                                                rect.X,
                                                rect.Y,
                                                rect.Width,
                                                rect.Height,
                                                &booln));
        return booln;
    }

    GraphicsState Save() const
    {
        GraphicsState gstate;

        SetStatus(DllExports::GdipSaveGraphics(nativeGraphics, &gstate));

        return gstate;
    }

    Status Restore(IN GraphicsState gstate)
    {
        return SetStatus(DllExports::GdipRestoreGraphics(nativeGraphics,
                                                         gstate));
    }

    GraphicsContainer BeginContainer(IN const RectF &dstrect,
                                     IN const RectF &srcrect,
                                     IN Unit         unit)
    {
        GraphicsContainer state;

        SetStatus(DllExports::GdipBeginContainer(nativeGraphics, &dstrect,
                                                 &srcrect, unit, &state));

        return state;
    }

    GraphicsContainer BeginContainer(IN const Rect    &dstrect,
                                     IN const Rect    &srcrect,
                                     IN Unit           unit)
    {
        GraphicsContainer state;

        SetStatus(DllExports::GdipBeginContainerI(nativeGraphics, &dstrect,
                                                  &srcrect, unit, &state));

        return state;
    }

    GraphicsContainer BeginContainer()
    {
        GraphicsContainer state;

        SetStatus(DllExports::GdipBeginContainer2(nativeGraphics, &state));

        return state;
    }

    Status EndContainer(IN GraphicsContainer state)
    {
        return SetStatus(DllExports::GdipEndContainer(nativeGraphics, state));
    }

    // Only valid when recording metafiles.

    Status AddMetafileComment(IN const BYTE * data,
                              IN UINT sizeData)
    {
        return SetStatus(DllExports::GdipComment(nativeGraphics, sizeData, data));
    }

    static HPALETTE GetHalftonePalette()
    {
        return DllExports::GdipCreateHalftonePalette();
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;

        return lastStatus;
    }

private:
    Graphics(const Graphics &);
    Graphics& operator=(const Graphics &);

protected:
    Graphics(GpGraphics* graphics)
    {
        lastResult = Ok;
        SetNativeGraphics(graphics);
    }

    VOID SetNativeGraphics(GpGraphics *graphics)
    {
        this->nativeGraphics = graphics;
    }

    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

    GpGraphics* GetNativeGraphics() const
    {
        return this->nativeGraphics;
    }

    GpPen* GetNativePen(const Pen* pen)
    {
        return pen->nativePen;
    }

protected:
    GpGraphics* nativeGraphics;
    mutable Status lastResult;

};

//----------------------------------------------------------------------------
// Implementation of GraphicsPath methods that use Graphics
//----------------------------------------------------------------------------

// The GetBounds rectangle may not be the tightest bounds.

inline Status
GraphicsPath::GetBounds(
    OUT RectF* bounds,
    IN const Matrix* matrix,
    IN const Pen* pen) const
{
    GpMatrix* nativeMatrix = NULL;
    GpPen* nativePen = NULL;

    if (matrix)
        nativeMatrix = matrix->nativeMatrix;

    if (pen)
        nativePen = pen->nativePen;

    return SetStatus(DllExports::GdipGetPathWorldBounds(nativePath, bounds,
                                                   nativeMatrix, nativePen));
}

inline Status
GraphicsPath::GetBounds(
    OUT Rect* bounds,
    IN const Matrix* matrix,
    IN const Pen* pen
) const
{
    GpMatrix* nativeMatrix = NULL;
    GpPen* nativePen = NULL;

    if (matrix)
        nativeMatrix = matrix->nativeMatrix;

    if (pen)
        nativePen = pen->nativePen;

    return SetStatus(DllExports::GdipGetPathWorldBoundsI(nativePath, bounds,
                                                    nativeMatrix, nativePen));
}

inline BOOL
GraphicsPath::IsVisible(
    IN REAL x,
    IN REAL y,
    IN const Graphics* g) const
{
   BOOL booln = FALSE;

   GpGraphics* nativeGraphics = NULL;

   if (g)
       nativeGraphics = g->nativeGraphics;

   SetStatus(DllExports::GdipIsVisiblePathPoint(nativePath,
                                                x, y, nativeGraphics,
                                                &booln));
   return booln;
}

inline BOOL
GraphicsPath::IsVisible(
    IN INT x,
    IN INT y,
    IN const Graphics* g) const
{
   BOOL booln = FALSE;

   GpGraphics* nativeGraphics = NULL;

   if (g)
       nativeGraphics = g->nativeGraphics;

   SetStatus(DllExports::GdipIsVisiblePathPointI(nativePath,
                                                 x, y, nativeGraphics,
                                                 &booln));
   return booln;
}

inline BOOL
GraphicsPath::IsOutlineVisible(
    IN REAL x,
    IN REAL y,
    IN const Pen* pen,
    IN const Graphics* g) const
{
    BOOL booln = FALSE;

    GpGraphics* nativeGraphics = NULL;
    GpPen* nativePen = NULL;

    if(g)
        nativeGraphics = g->nativeGraphics;
    if(pen)
        nativePen = pen->nativePen;

    SetStatus(DllExports::GdipIsOutlineVisiblePathPoint(nativePath,
                                                        x, y, nativePen, nativeGraphics,
                                                        &booln));
    return booln;
}

inline BOOL
GraphicsPath::IsOutlineVisible(
    IN INT x,
    IN INT y,
    IN const Pen* pen,
    IN const Graphics* g) const
{
    BOOL booln = FALSE;

    GpGraphics* nativeGraphics = NULL;
    GpPen* nativePen = NULL;

    if(g)
        nativeGraphics = g->nativeGraphics;
    if(pen)
        nativePen = pen->nativePen;

    SetStatus(DllExports::GdipIsOutlineVisiblePathPointI(nativePath,
                                                         x, y, nativePen, nativeGraphics,
                                                         &booln));
    return booln;
}

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

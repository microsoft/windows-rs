/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusPath.h
*
* Abstract:
*
*   GDI+ Graphics Path class
*
\**************************************************************************/

#ifndef _GDIPLUSPATH_H
#define _GDIPLUSPATH_H
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

class GraphicsPath : public GdiplusBase
{
public:
    friend class Graphics;
    friend class Region;
    friend class PathGradientBrush;
    friend class GraphicsPathIterator;
    friend class CustomLineCap;

    GraphicsPath(IN FillMode fillMode = FillModeAlternate)
    {
        nativePath = NULL;
        lastResult = DllExports::GdipCreatePath(fillMode, &nativePath);
    }

    GraphicsPath(IN const PointF* points,
                 IN const BYTE* types,
                 IN INT count,
                 IN FillMode fillMode = FillModeAlternate)
    {
        nativePath = NULL;
        lastResult = DllExports::GdipCreatePath2(points,
                                                 types,
                                                 count,
                                                 fillMode,
                                                 &nativePath);
    }

    GraphicsPath(IN const Point* points,
                 IN const BYTE* types,
                 IN INT count,
                 IN FillMode fillMode = FillModeAlternate)
    {
        nativePath = NULL;
        lastResult = DllExports::GdipCreatePath2I(points,
                                                  types,
                                                  count,
                                                  fillMode,
                                                  &nativePath);
    }

    ~GraphicsPath()
    {
        DllExports::GdipDeletePath(nativePath);
    }

    GraphicsPath* Clone() const
    {
        GpPath *clonepath = NULL;

        SetStatus(DllExports::GdipClonePath(nativePath, &clonepath));

        return new GraphicsPath(clonepath);
    }

    // Reset the path object to empty (and fill mode to FillModeAlternate)

    Status Reset()
    {
        return SetStatus(DllExports::GdipResetPath(nativePath));
    }

    FillMode GetFillMode() const
    {
        FillMode fillmode = FillModeAlternate;

        SetStatus(DllExports::GdipGetPathFillMode(nativePath, &fillmode));

        return fillmode;
    }

    Status SetFillMode(IN FillMode fillmode)
    {
        return SetStatus(DllExports::GdipSetPathFillMode(nativePath, 
                                                         fillmode));
    }

    Status GetPathData(OUT PathData* pathData) const
    {
        if (pathData == NULL) 
        {
            return SetStatus(InvalidParameter);
        }
        
        INT count = GetPointCount();
        
        if ((count <= 0) || (pathData->Count>0 && pathData->Count<count))
        {
            pathData->Count = 0;
            if (pathData->Points)
            {
                delete [] pathData->Points;
                pathData->Points = NULL;
            }

            if (pathData->Types) 
            {
                delete [] pathData->Types;
                pathData->Types = NULL;
            }

            if (count <= 0)
            {
                return Ok;
            }
        }

        if (pathData->Count == 0) 
        {
            pathData->Points = new PointF[count];
            if (pathData->Points == NULL) 
            {
                return SetStatus(OutOfMemory);
            
            }
            pathData->Types = new byte[count];
            if (pathData->Types == NULL) 
            {
                delete [] pathData->Points;
                pathData->Points = NULL;

                return SetStatus(OutOfMemory);
            }
            pathData->Count = count;
        }

        return SetStatus(DllExports::GdipGetPathData(nativePath, pathData));
    }

    Status StartFigure()
    {
        return SetStatus(DllExports::GdipStartPathFigure(nativePath));
    }

    Status CloseFigure()
    {
        return SetStatus(DllExports::GdipClosePathFigure(nativePath));
    }

    Status CloseAllFigures()
    {
        return SetStatus(DllExports::GdipClosePathFigures(nativePath));
    }

    Status SetMarker()
    {
        return SetStatus(DllExports::GdipSetPathMarker(nativePath));
    }

    Status ClearMarkers()
    {
        return SetStatus(DllExports::GdipClearPathMarkers(nativePath));
    }

    Status Reverse()
    {
        return SetStatus(DllExports::GdipReversePath(nativePath));
    }

    Status GetLastPoint(OUT PointF* lastPoint) const
    {
        return SetStatus(DllExports::GdipGetPathLastPoint(nativePath, 
                                                          lastPoint));
    }

    Status AddLine(IN const PointF& pt1, 
                   IN const PointF& pt2)
    {
        return AddLine(pt1.X, pt1.Y, pt2.X, pt2.Y);
    }

    Status AddLine(IN REAL x1,
                   IN REAL y1, 
                   IN REAL x2, 
                   IN REAL y2)
    {
        return SetStatus(DllExports::GdipAddPathLine(nativePath, x1, y1, 
                                                     x2, y2));
    }

    Status AddLines(IN const PointF* points, 
                    IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathLine2(nativePath, points, 
                                                      count));
    }

    Status AddLine(IN const Point& pt1, 
                   IN const Point& pt2)
    {
        return AddLine(pt1.X,
                       pt1.Y,
                       pt2.X,
                       pt2.Y);
    }

    Status AddLine(IN INT x1, 
                   IN INT y1, 
                   IN INT x2, 
                   IN INT y2)
    {
        return SetStatus(DllExports::GdipAddPathLineI(nativePath,
                                                     x1,
                                                     y1,
                                                     x2,
                                                     y2));
    }

    Status AddLines(IN const Point* points, 
                    IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathLine2I(nativePath,
                                                       points,
                                                       count));
    }

    Status AddArc(IN const RectF& rect, 
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return AddArc(rect.X, rect.Y, rect.Width, rect.Height,
                      startAngle, sweepAngle);
    }

    Status AddArc(IN REAL x, 
                  IN REAL y, 
                  IN REAL width, 
                  IN REAL height,
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipAddPathArc(nativePath, x, y, width, 
                                                    height, startAngle, 
                                                    sweepAngle));
    }

    Status AddArc(IN const Rect& rect, 
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return AddArc(rect.X, rect.Y, rect.Width, rect.Height,
                      startAngle, sweepAngle);
    }

    Status AddArc(IN INT x, 
                  IN INT y, 
                  IN INT width, 
                  IN INT height,
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipAddPathArcI(nativePath,
                                                    x,
                                                    y,
                                                    width,
                                                    height,
                                                    startAngle,
                                                    sweepAngle));
    }

    Status AddBezier(IN const PointF& pt1, 
                     IN const PointF& pt2,
                     IN const PointF& pt3, 
                     IN const PointF& pt4)
    {
        return AddBezier(pt1.X, pt1.Y, pt2.X, pt2.Y, pt3.X, pt3.Y, pt4.X,
                         pt4.Y);
    }

    Status AddBezier(IN REAL x1, 
                     IN REAL y1, 
                     IN REAL x2, 
                     IN REAL y2,
                     IN REAL x3, 
                     IN REAL y3, 
                     IN REAL x4, 
                     IN REAL y4)
    {
        return SetStatus(DllExports::GdipAddPathBezier(nativePath, x1, y1, x2, 
                                                       y2, x3, y3, x4, y4));
    }

    Status AddBeziers(IN const PointF* points, 
                      IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathBeziers(nativePath, points, 
                                                        count));
    }

    Status AddBezier(IN const Point& pt1, 
                     IN const Point& pt2,
                     IN const Point& pt3, 
                     IN const Point& pt4)
    {
       return AddBezier(pt1.X, pt1.Y, pt2.X, pt2.Y, pt3.X, pt3.Y, pt4.X,
                        pt4.Y);
    }

    Status AddBezier(IN INT x1, 
                     IN INT y1, 
                     IN INT x2, 
                     IN INT y2,
                     IN INT x3,
                     IN INT y3, 
                     IN INT x4, 
                     IN INT y4)
    {
       return SetStatus(DllExports::GdipAddPathBezierI(nativePath,
                                                      x1,
                                                      y1,
                                                      x2,
                                                      y2,
                                                      x3,
                                                      y3,
                                                      x4,
                                                      y4));
    }

    Status AddBeziers(IN const Point* points,
                      IN INT count)
    {
       return SetStatus(DllExports::GdipAddPathBeziersI(nativePath,
                                                        points,
                                                        count));
    }

    Status AddCurve(IN const PointF* points, 
                    IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathCurve(nativePath,
                                                      points,
                                                      count));
    }

    Status AddCurve(IN const PointF* points, 
                    IN INT count,
                    IN REAL tension)
    {
        return SetStatus(DllExports::GdipAddPathCurve2(nativePath,
                                                       points,
                                                       count,
                                                       tension));
    }

    Status AddCurve(IN const PointF* points, 
                    IN INT count, 
                    IN INT offset,
                    IN INT numberOfSegments, 
                    IN REAL tension)
    {
        return SetStatus(DllExports::GdipAddPathCurve3(nativePath,
                                                       points,
                                                       count,
                                                       offset,
                                                       numberOfSegments,
                                                       tension));
    }

    Status AddCurve(IN const Point* points, 
                    IN INT count)
    {
       return SetStatus(DllExports::GdipAddPathCurveI(nativePath,
                                                     points,
                                                     count));
    }

    Status AddCurve(IN const Point* points, 
                    IN INT count, 
                    IN REAL tension)
    {
       return SetStatus(DllExports::GdipAddPathCurve2I(nativePath,
                                                      points,
                                                      count,
                                                      tension));
    }

    Status AddCurve(IN const Point* points, 
                    IN INT count, 
                    IN INT offset,
                    IN INT numberOfSegments, 
                    IN REAL tension)
    {
       return SetStatus(DllExports::GdipAddPathCurve3I(nativePath,
                                                      points,
                                                      count,
                                                      offset,
                                                      numberOfSegments,
                                                      tension));
    }

    Status AddClosedCurve(IN const PointF* points, 
                          IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathClosedCurve(nativePath,
                                                            points,
                                                            count));
    }

    Status AddClosedCurve(IN const PointF* points, 
                          IN INT count, 
                          IN REAL tension)
    {
        return SetStatus(DllExports::GdipAddPathClosedCurve2(nativePath,
                                                             points,
                                                             count,
                                                             tension));
    }

    Status AddClosedCurve(IN const Point* points, 
                          IN INT count)
    {
       return SetStatus(DllExports::GdipAddPathClosedCurveI(nativePath,
                                                            points,
                                                            count));
    }


    Status AddClosedCurve(IN const Point* points, 
                          IN INT count,
                          IN REAL tension)
    {
       return SetStatus(DllExports::GdipAddPathClosedCurve2I(nativePath,
                                                             points,
                                                             count,
                                                             tension));
    }

    Status AddRectangle(IN const RectF& rect)
    {
        return SetStatus(DllExports::GdipAddPathRectangle(nativePath,
                                                          rect.X,
                                                          rect.Y,
                                                          rect.Width,
                                                          rect.Height));
    }

    Status AddRectangles(IN const RectF* rects, 
                         IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathRectangles(nativePath,
                                                           rects,
                                                           count));
    }

    Status AddRectangle(IN const Rect& rect)
    {
        return SetStatus(DllExports::GdipAddPathRectangleI(nativePath,
                                                          rect.X,
                                                          rect.Y,
                                                          rect.Width,
                                                          rect.Height));
    }

    Status AddRectangles(IN const Rect* rects, INT count)
    {
        return SetStatus(DllExports::GdipAddPathRectanglesI(nativePath,
                                                           rects,
                                                           count));
    }

    Status AddEllipse(IN const RectF& rect)
    {
        return AddEllipse(rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status AddEllipse(IN REAL x, 
                      IN REAL y, 
                      IN REAL width, 
                      IN REAL height)
    {
        return SetStatus(DllExports::GdipAddPathEllipse(nativePath,
                                                        x,
                                                        y,
                                                        width,
                                                        height));
    }

    Status AddEllipse(IN const Rect& rect)
    {
        return AddEllipse(rect.X, rect.Y, rect.Width, rect.Height);
    }

    Status AddEllipse(IN INT x, 
                      IN INT y, 
                      IN INT width, 
                      IN INT height)
    {
        return SetStatus(DllExports::GdipAddPathEllipseI(nativePath,
                                                        x,
                                                        y,
                                                        width,
                                                        height));
    }

    Status AddPie(IN const RectF& rect, 
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return AddPie(rect.X, rect.Y, rect.Width, rect.Height, startAngle,
                      sweepAngle);
    }

    Status AddPie(IN REAL x, 
                  IN REAL y, 
                  IN REAL width, 
                  IN REAL height, 
                  IN REAL startAngle,
                  IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipAddPathPie(nativePath, x, y, width,
                                                    height, startAngle, 
                                                    sweepAngle));
    }

    Status AddPie(IN const Rect& rect, 
                  IN REAL startAngle, 
                  IN REAL sweepAngle)
    {
        return AddPie(rect.X,
                      rect.Y,
                      rect.Width,
                      rect.Height,
                      startAngle,
                      sweepAngle);
    }

    Status AddPie(IN INT x, 
                  IN INT y, 
                  IN INT width, 
                  IN INT height, 
                  IN REAL startAngle,
                  IN REAL sweepAngle)
    {
        return SetStatus(DllExports::GdipAddPathPieI(nativePath,
                                                    x,
                                                    y,
                                                    width,
                                                    height,
                                                    startAngle,
                                                    sweepAngle));
    }

    Status AddPolygon(IN const PointF* points, 
                      IN INT count)
    {
        return SetStatus(DllExports::GdipAddPathPolygon(nativePath, points, 
                                                        count));
    }

    Status AddPolygon(IN const Point* points, 
                      IN INT count)
    {
       return SetStatus(DllExports::GdipAddPathPolygonI(nativePath, points, 
                                                        count));
    }

    Status AddPath(IN const GraphicsPath* addingPath, 
                   IN BOOL connect)
    {
        GpPath* nativePath2 = NULL;
        if(addingPath)
            nativePath2 = addingPath->nativePath;

        return SetStatus(DllExports::GdipAddPathPath(nativePath, nativePath2, 
                                                     connect));
    }

    Status AddString(
        IN const WCHAR         *string,
        IN INT                  length,
        IN const FontFamily    *family,
        IN INT                  style,
        IN REAL                 emSize,  // World units
        IN const PointF        &origin,
        IN const StringFormat  *format
    )
    {
        RectF rect(origin.X, origin.Y, 0.0f, 0.0f);

        return SetStatus(DllExports::GdipAddPathString(
            nativePath,
            string,
            length,
            family ? family->nativeFamily : NULL,
            style,
            emSize,
            &rect,
            format ? format->nativeFormat : NULL
        ));
    }

    Status AddString(
        IN const WCHAR         *string,
        IN INT                  length,
        IN const FontFamily    *family,
        IN INT                  style,
        IN REAL                 emSize,  // World units
        IN const RectF         &layoutRect,
        IN const StringFormat  *format
    )
    {
        return SetStatus(DllExports::GdipAddPathString(
            nativePath,
            string,
            length,
            family ? family->nativeFamily : NULL,
            style,
            emSize,
            &layoutRect,
            format ? format->nativeFormat : NULL
        ));
    }

    Status AddString(
        IN const WCHAR         *string,
        IN INT                  length,
        IN const FontFamily    *family,
        IN INT                  style,
        IN REAL                 emSize,  // World units
        IN const Point         &origin,
        IN const StringFormat  *format
    )
    {
        Rect rect(origin.X, origin.Y, 0, 0);

        return SetStatus(DllExports::GdipAddPathStringI(
            nativePath,
            string,
            length,
            family ? family->nativeFamily : NULL,
            style,
            emSize,
            &rect,
            format ? format->nativeFormat : NULL
        ));
    }

    Status AddString(
        IN const WCHAR         *string,
        IN INT                  length,
        IN const FontFamily    *family,
        IN INT                  style,
        IN REAL                 emSize,  // World units
        IN const Rect          &layoutRect,
        IN const StringFormat  *format
    )
    {
        return SetStatus(DllExports::GdipAddPathStringI(
            nativePath,
            string,
            length,
            family ? family->nativeFamily : NULL,
            style,
            emSize,
            &layoutRect,
            format ? format->nativeFormat : NULL
        ));
    }
    
    Status Transform(IN const Matrix* matrix)
    {
        if(matrix)
            return SetStatus(DllExports::GdipTransformPath(nativePath, 
                                                      matrix->nativeMatrix));
        else
            return Ok;
    }

    // This is not always the tightest bounds.

    Status GetBounds(OUT RectF* bounds, 
                     IN const Matrix* matrix = NULL, 
                     IN const Pen* pen = NULL) const;

    Status GetBounds(OUT Rect* bounds,
                     IN const Matrix* matrix = NULL, 
                     IN const Pen* pen = NULL) const;

    // Once flattened, the resultant path is made of line segments and
    // the original path information is lost.  When matrix is NULL the
    // identity matrix is assumed.
        
    Status Flatten(IN const Matrix* matrix = NULL, 
                   IN REAL flatness = FlatnessDefault)
    {
        GpMatrix* nativeMatrix = NULL;
        if(matrix)
        {
            nativeMatrix = matrix->nativeMatrix;
        }

        return SetStatus(DllExports::GdipFlattenPath(
            nativePath, 
            nativeMatrix, 
            flatness
        ));
    }

    Status Widen(
        IN const Pen* pen, 
        IN const Matrix* matrix = NULL,
        IN REAL flatness = FlatnessDefault
    )
    {
        GpMatrix* nativeMatrix = NULL;
        if(matrix)
            nativeMatrix = matrix->nativeMatrix;

        return SetStatus(DllExports::GdipWidenPath(
            nativePath, 
            pen->nativePen,
            nativeMatrix, 
            flatness
        ));
    }

    Status Outline(
        IN const Matrix *matrix = NULL,
        IN REAL flatness = FlatnessDefault
    )
    {
        GpMatrix* nativeMatrix = NULL;
        if(matrix)
        {
            nativeMatrix = matrix->nativeMatrix;
        }

        return SetStatus(DllExports::GdipWindingModeOutline(
            nativePath, nativeMatrix, flatness
        ));
    }
    
    // Once this is called, the resultant path is made of line segments and
    // the original path information is lost.  When matrix is NULL, the 
    // identity matrix is assumed.
    
    Status Warp(IN const PointF* destPoints, 
                IN INT count,
                IN const RectF& srcRect, 
                IN const Matrix* matrix = NULL,
                IN WarpMode warpMode = WarpModePerspective,
                IN REAL flatness = FlatnessDefault)
    {
        GpMatrix* nativeMatrix = NULL;
        if(matrix)
            nativeMatrix = matrix->nativeMatrix;

        return SetStatus(DllExports::GdipWarpPath(
                                        nativePath,
                                        nativeMatrix,
                                        destPoints,
                                        count,
                                        srcRect.X,
                                        srcRect.Y,
                                        srcRect.Width,
                                        srcRect.Height,
                                        warpMode,
                                        flatness));
    }

    INT GetPointCount() const
    {
        INT count = 0;

        SetStatus(DllExports::GdipGetPointCount(nativePath, &count));

        return count;
    }

    Status GetPathTypes(_Out_writes_bytes_(count) BYTE* types, 
                        _In_ INT count) const
    {
        return SetStatus(DllExports::GdipGetPathTypes(nativePath, types,
                                                      count));
    }

    Status GetPathPoints(OUT PointF* points, 
                         IN INT count) const
    {
        return SetStatus(DllExports::GdipGetPathPoints(nativePath, points, 
                                                       count));
    }

    Status GetPathPoints(OUT Point* points, 
                         IN INT count) const
    {
        return SetStatus(DllExports::GdipGetPathPointsI(nativePath, points, 
                                                        count));
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;

        return lastStatus;
    }

    BOOL IsVisible(IN const PointF& point, 
                   IN const Graphics* g = NULL) const
    {
        return IsVisible(point.X, point.Y, g);
    }
    
    BOOL IsVisible(IN REAL x, 
                   IN REAL y, 
                   IN const Graphics* g = NULL) const;

    BOOL IsVisible(IN const Point& point,
                   IN const Graphics* g = NULL) const
    {
        return IsVisible(point.X, point.Y, g);
    }

    BOOL IsVisible(IN INT x, 
                   IN INT y, 
                   IN const Graphics* g = NULL) const;
    
    BOOL IsOutlineVisible(IN const PointF& point,
                          IN const Pen* pen, 
                          IN const Graphics* g = NULL) const
    {
        return IsOutlineVisible(point.X, point.Y, pen, g);
    }

    BOOL IsOutlineVisible(IN REAL x, 
                          IN REAL y, 
                          IN const Pen* pen, 
                          IN const Graphics* g = NULL) const;

    BOOL IsOutlineVisible(IN const Point& point,
                          IN const Pen* pen, 
                          IN const Graphics* g = NULL) const
    {
        return IsOutlineVisible(point.X, point.Y, pen, g);
    }
    
    BOOL IsOutlineVisible(IN INT x, 
                          IN INT y, 
                          IN const Pen* pen, 
                          IN const Graphics* g = NULL) const;

protected:

    GraphicsPath(const GraphicsPath& path)
    {
        GpPath *clonepath = NULL;
        SetStatus(DllExports::GdipClonePath(path.nativePath, &clonepath));
        SetNativePath(clonepath);
    }

private:
    GraphicsPath& operator=(const GraphicsPath &);

protected:
    GraphicsPath(GpPath* nativePath)
    {
        lastResult = Ok;
        SetNativePath(nativePath);
    }

    VOID SetNativePath(GpPath *nativePathArg)
    {
        this->nativePath = nativePathArg;
    }

    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

protected:
    GpPath* nativePath;
    mutable Status lastResult;
};


//--------------------------------------------------------------------------
// GraphisPathIterator class
//--------------------------------------------------------------------------

#pragma warning(push)
#pragma warning(disable: 6102 6103) // Uses SetStatus()/GetStatus() pattern rather than return codes so PREfast gets confused

class GraphicsPathIterator : public GdiplusBase
{
public:

    GraphicsPathIterator(IN const GraphicsPath* path)
    {
        GpPath* nativePath = NULL;
        if(path)
            nativePath = path->nativePath;

        GpPathIterator *iter = NULL;
        lastResult = DllExports::GdipCreatePathIter(&iter, nativePath);
        SetNativeIterator(iter);
    }

    ~GraphicsPathIterator()
    {
        DllExports::GdipDeletePathIter(nativeIterator);
    }


    INT NextSubpath(OUT INT* startIndex,
                    OUT INT* endIndex,
                    OUT BOOL* isClosed)
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterNextSubpath(nativeIterator,
            &resultCount, startIndex, endIndex, isClosed));

        return resultCount;
    }


    INT NextSubpath(OUT const GraphicsPath* path, 
                    OUT BOOL* isClosed)
    {
        GpPath* nativePath = NULL;

        INT resultCount;

        if(path)
            nativePath= path->nativePath;

        SetStatus(DllExports::GdipPathIterNextSubpathPath(nativeIterator,
            &resultCount, nativePath, isClosed));

        return resultCount;
    }

    INT NextPathType(_Out_ BYTE* pathType, 
                     _Out_ INT* startIndex, 
                     _Out_ INT* endIndex)
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterNextPathType(nativeIterator,
            &resultCount, pathType, startIndex, endIndex));

        return resultCount;
    }

    INT NextMarker(OUT INT* startIndex, 
                   OUT INT* endIndex)
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterNextMarker(nativeIterator,
            &resultCount, startIndex, endIndex));

        return resultCount;
    }


    INT NextMarker(OUT const GraphicsPath* path)
    {
        GpPath* nativePath = NULL;

        INT resultCount;

        if(path)
            nativePath= path->nativePath;

        SetStatus(DllExports::GdipPathIterNextMarkerPath(nativeIterator,
            &resultCount, nativePath));

        return resultCount;
    }

    INT GetCount() const
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterGetCount(nativeIterator, 
                                                   &resultCount));

        return resultCount;
    }

    INT GetSubpathCount() const
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterGetSubpathCount(nativeIterator, 
                                                          &resultCount));

        return resultCount;
    }

    BOOL HasCurve() const
    {
        BOOL hasCurve;

        SetStatus(DllExports::GdipPathIterHasCurve(nativeIterator, &hasCurve));

        return hasCurve;
    }

    VOID Rewind()
    {
        SetStatus(DllExports::GdipPathIterRewind(nativeIterator));
    }

    INT Enumerate(_Out_writes_to_(count, return) PointF *points,
                  _Out_writes_to_(count, return) BYTE *types, 
                  _In_ INT count)
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterEnumerate(nativeIterator,
            &resultCount, points, types, count));

        return resultCount;
    }

    INT CopyData(_Out_writes_to_(endIndex-startIndex+1, return) PointF* points, 
                 _Out_writes_to_(endIndex-startIndex+1, return) BYTE* types,
                 _In_ INT startIndex, 
                 _In_ INT endIndex)
    {
        INT resultCount;

        SetStatus(DllExports::GdipPathIterCopyData(nativeIterator,
            &resultCount, points, types, startIndex, endIndex));

        return resultCount;
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;

        return lastStatus;
    }

private:
    GraphicsPathIterator(const GraphicsPathIterator &);
    GraphicsPathIterator& operator=(const GraphicsPathIterator &);

protected:
    VOID SetNativeIterator(GpPathIterator *nativeIteratorArg)
    {
        this->nativeIterator = nativeIteratorArg;
    }

    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

protected:
    GpPathIterator* nativeIterator;
    mutable Status lastResult;
};

#pragma warning(pop)

//--------------------------------------------------------------------------
// Path Gradient Brush
//--------------------------------------------------------------------------

#pragma warning(push)
#pragma warning(disable: 6102 6103) // Uses SetStatus()/GetStatus() pattern rather than return codes so PREfast gets confused

class PathGradientBrush : public Brush
{
public:
    friend class Pen;

    PathGradientBrush(
        IN const PointF* points,
        IN INT count,
        IN WrapMode wrapMode = WrapModeClamp)
    {
        GpPathGradient *brush = NULL;

        lastResult = DllExports::GdipCreatePathGradient(
                                        points, count,
                                        wrapMode, &brush);
        SetNativeBrush(brush);
    }

    PathGradientBrush(
        IN const Point* points,
        IN INT count,
        IN WrapMode wrapMode = WrapModeClamp)
    {
        GpPathGradient *brush = NULL;

        lastResult = DllExports::GdipCreatePathGradientI(
                                        points, count,
                                        wrapMode, &brush);

        SetNativeBrush(brush);
    }

    PathGradientBrush(
        IN const GraphicsPath* path
        )
    {
        GpPathGradient *brush = NULL;

        lastResult = DllExports::GdipCreatePathGradientFromPath(
                                        path->nativePath, &brush);
        SetNativeBrush(brush);
    }

    Status GetCenterColor(OUT Color* color) const
    {
        ARGB argb;
        
        if (color == NULL) 
        {
            return SetStatus(InvalidParameter);
        }

        SetStatus(DllExports::GdipGetPathGradientCenterColor(
                       (GpPathGradient*) nativeBrush, &argb));

        color->SetValue(argb);

        return lastResult;
    }

    Status SetCenterColor(IN const Color& color)
    {
        SetStatus(DllExports::GdipSetPathGradientCenterColor(
                       (GpPathGradient*) nativeBrush,
                       color.GetValue()));

        return lastResult;
    }

    INT GetPointCount() const
    {
        INT count;

        SetStatus(DllExports::GdipGetPathGradientPointCount(
                       (GpPathGradient*) nativeBrush, &count));

        return count;
    }

    INT GetSurroundColorCount() const
    {
        INT count;

        SetStatus(DllExports::GdipGetPathGradientSurroundColorCount(
                       (GpPathGradient*) nativeBrush, &count));

        return count;
    }

    Status GetSurroundColors(_Out_writes_to_(*count, *count) Color* colors, 
                             _Inout_ INT* count) const
    {
        if(colors == NULL || count == NULL)
        {
            return SetStatus(InvalidParameter);
        }

        INT count1;
        
        SetStatus(DllExports::GdipGetPathGradientSurroundColorCount(
                        (GpPathGradient*) nativeBrush, &count1));

        if(lastResult != Ok)
            return lastResult;

        if((*count < count1) || (count1 <= 0))
            return SetStatus(InsufficientBuffer);

        ARGB* argbs = (ARGB*) new ARGB[count1];
        if(argbs == NULL)
            return SetStatus(OutOfMemory);

        SetStatus(DllExports::GdipGetPathGradientSurroundColorsWithCount(
                    (GpPathGradient*)nativeBrush, argbs, &count1));

        if(lastResult == Ok)
        {
            for(INT i = 0; i < count1; i++)
            {
#pragma warning(suppress: 6385) // Tool bug, doesn't see that *numFound > numSought
                colors[i].SetValue(argbs[i]);
            }        
            *count = count1;
        }

        delete [] argbs;
        return lastResult;
    }

    Status SetSurroundColors(IN const Color* colors, 
                             IN OUT INT* count)
    {
        if(colors == NULL || count == NULL)
        {
            return SetStatus(InvalidParameter);
        }

        INT count1 = GetPointCount();

        if((*count > count1) || (count1 <= 0))
            return SetStatus(InvalidParameter);

        count1 = *count;

        ARGB* argbs = (ARGB*) new ARGB[count1];
        if(argbs == NULL)
            return SetStatus(OutOfMemory);

        for(INT i = 0; i < count1; i++)
        {
            argbs[i] = colors[i].GetValue();
        }

        SetStatus(DllExports::GdipSetPathGradientSurroundColorsWithCount(
                    (GpPathGradient*)nativeBrush, argbs, &count1));

        if(lastResult == Ok)
            *count = count1;

        delete [] argbs;

        return lastResult;
    }

    Status GetGraphicsPath(OUT GraphicsPath* path) const
    {
        if(path == NULL)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipGetPathGradientPath(
                    (GpPathGradient*)nativeBrush, path->nativePath));
    }

    Status SetGraphicsPath(IN const GraphicsPath* path)
    {
        if(path == NULL)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipSetPathGradientPath(
                    (GpPathGradient*)nativeBrush, path->nativePath));
    }

    Status GetCenterPoint(OUT PointF* point) const
    {
        return SetStatus(DllExports::GdipGetPathGradientCenterPoint(
                                (GpPathGradient*)nativeBrush,
                                point));
    }

    Status GetCenterPoint(OUT Point* point) const
    {
        return SetStatus(DllExports::GdipGetPathGradientCenterPointI(
                                (GpPathGradient*)nativeBrush,
                                point));
    }

    Status SetCenterPoint(IN const PointF& point)
    {
        return SetStatus(DllExports::GdipSetPathGradientCenterPoint(
                                (GpPathGradient*)nativeBrush,
                                &point));
    }

    Status SetCenterPoint(IN const Point& point)
    {
        return SetStatus(DllExports::GdipSetPathGradientCenterPointI(
                                (GpPathGradient*)nativeBrush,
                                &point));
    }

    Status GetRectangle(OUT RectF* rect) const
    {
        return SetStatus(DllExports::GdipGetPathGradientRect(
                            (GpPathGradient*)nativeBrush, rect));
    }

    Status GetRectangle(OUT Rect* rect) const
    {
        return SetStatus(DllExports::GdipGetPathGradientRectI(
                            (GpPathGradient*)nativeBrush, rect));
    }

    Status SetGammaCorrection(IN BOOL useGammaCorrection)
    {
        return SetStatus(DllExports::GdipSetPathGradientGammaCorrection(
            (GpPathGradient*)nativeBrush, useGammaCorrection));
    }

    BOOL GetGammaCorrection() const
    {
        BOOL useGammaCorrection;

        SetStatus(DllExports::GdipGetPathGradientGammaCorrection(
            (GpPathGradient*)nativeBrush, &useGammaCorrection));

        return useGammaCorrection;
    }

    INT GetBlendCount() const
    {
       INT count = 0;

       SetStatus(DllExports::GdipGetPathGradientBlendCount(
                           (GpPathGradient*) nativeBrush, &count));

       return count;
    }

    Status GetBlend(OUT REAL* blendFactors,
                    OUT REAL* blendPositions,
                    IN INT count) const
    {
        return SetStatus(DllExports::GdipGetPathGradientBlend(
                            (GpPathGradient*)nativeBrush,
                            blendFactors, blendPositions, count));
    }

    Status SetBlend(IN const REAL* blendFactors, 
                    IN const REAL* blendPositions, 
                    IN INT count)
    {
        return SetStatus(DllExports::GdipSetPathGradientBlend(
                            (GpPathGradient*)nativeBrush,
                            blendFactors, blendPositions, count));
    }

    INT GetInterpolationColorCount() const
    {
       INT count = 0;

       SetStatus(DllExports::GdipGetPathGradientPresetBlendCount(
                        (GpPathGradient*) nativeBrush, &count));

       return count;
    }

    Status SetInterpolationColors(IN const Color* presetColors,
                                  IN const REAL* blendPositions, 
                                  IN INT count)
    {
        if ((count <= 0) || !presetColors) 
        {
            return SetStatus(InvalidParameter);
        }

        ARGB* argbs = (ARGB*) new ARGB[count];
        if(argbs)
        {
            for(INT i = 0; i < count; i++)
            {
                argbs[i] = presetColors[i].GetValue();
            }

            Status status = SetStatus(DllExports::
                               GdipSetPathGradientPresetBlend(
                                    (GpPathGradient*) nativeBrush,
                                    argbs,
                                    blendPositions,
                                    count));
            delete[] argbs;
            return status;
        }
        else
        {
            return SetStatus(OutOfMemory);
        }
    }

    Status GetInterpolationColors(OUT Color* presetColors,
                                  OUT REAL* blendPositions, 
                                  IN INT count) const
    {
        if ((count <= 0) || !presetColors) 
        {
            return SetStatus(InvalidParameter);
        }

        ARGB* argbs = (ARGB*) new ARGB[count];
        
        if (!argbs)
        {
            return SetStatus(OutOfMemory);
        }

        GpStatus status = SetStatus(DllExports::GdipGetPathGradientPresetBlend(
                                (GpPathGradient*)nativeBrush,
                                argbs,
                                blendPositions,
                                count));
        
        for(INT i = 0; i < count; i++)
        {
            presetColors[i] = Color(argbs[i]);
        }
        delete [] argbs;
        
        return status;
    }

    Status SetBlendBellShape(IN REAL focus, 
                             IN REAL scale = 1.0)
    {
        return SetStatus(DllExports::GdipSetPathGradientSigmaBlend(
                            (GpPathGradient*)nativeBrush, focus, scale));
    }

    Status SetBlendTriangularShape(
        IN REAL focus,
        IN REAL scale = 1.0
    )
    {
        return SetStatus(DllExports::GdipSetPathGradientLinearBlend(
                            (GpPathGradient*)nativeBrush, focus, scale));
    }

    Status GetTransform(OUT Matrix *matrix) const
    {
        return SetStatus(DllExports::GdipGetPathGradientTransform(
                            (GpPathGradient*) nativeBrush, 
                            matrix->nativeMatrix));
    }

    Status SetTransform(IN const Matrix* matrix)
    {
        return SetStatus(DllExports::GdipSetPathGradientTransform(
                            (GpPathGradient*) nativeBrush, 
                            matrix->nativeMatrix));
    }

    Status ResetTransform()
    {
        return SetStatus(DllExports::GdipResetPathGradientTransform(
                            (GpPathGradient*)nativeBrush));
    }

    Status MultiplyTransform(IN const Matrix* matrix,
                             IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyPathGradientTransform(
                            (GpPathGradient*)nativeBrush,
                            matrix->nativeMatrix,
                            order));
    }

    Status TranslateTransform(IN REAL dx, 
                              IN REAL dy,
                              IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslatePathGradientTransform(
                            (GpPathGradient*)nativeBrush,
                            dx, dy, order));
    }

    Status ScaleTransform(IN REAL sx, 
                          IN REAL sy,
                          IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScalePathGradientTransform(
                            (GpPathGradient*)nativeBrush,
                            sx, sy, order));
    }

    Status RotateTransform(IN REAL angle, 
                           IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotatePathGradientTransform(
                            (GpPathGradient*)nativeBrush,
                            angle, order));
    }

    Status GetFocusScales(OUT REAL* xScale, 
                          OUT REAL* yScale) const
    {
        return SetStatus(DllExports::GdipGetPathGradientFocusScales(
                            (GpPathGradient*) nativeBrush, xScale, yScale));
    }

    Status SetFocusScales(IN REAL xScale,
                          IN REAL yScale)
    {
        return SetStatus(DllExports::GdipSetPathGradientFocusScales(
                            (GpPathGradient*) nativeBrush, xScale, yScale));
    }

    WrapMode GetWrapMode() const
    {
        WrapMode wrapMode;

        SetStatus(DllExports::GdipGetPathGradientWrapMode(
                     (GpPathGradient*) nativeBrush, &wrapMode));

        return wrapMode;
    }

    Status SetWrapMode(IN WrapMode wrapMode)
    {
        return SetStatus(DllExports::GdipSetPathGradientWrapMode(
                            (GpPathGradient*) nativeBrush, wrapMode));
    }

private:
    PathGradientBrush(const PathGradientBrush &);
    PathGradientBrush& operator=(const PathGradientBrush &);

protected:

    PathGradientBrush()
    {
    }
};

#pragma warning(pop)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GRAPHICSPATH_HPP

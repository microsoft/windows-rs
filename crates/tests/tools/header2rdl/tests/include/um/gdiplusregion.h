/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusRegion.h
*
* Abstract:
*
*   GDI+ Region class implementation
*
\**************************************************************************/

#ifndef _GDIPLUSREGION_H
#define _GDIPLUSREGION_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


inline 
Region::Region()
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegion(&region);

    SetNativeRegion(region);
}

inline 
Region::Region(IN const RectF& rect)
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegionRect(&rect, &region);

    SetNativeRegion(region);
}

inline 
Region::Region(IN const Rect& rect)
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegionRectI(&rect, &region);

    SetNativeRegion(region);
}

inline 
Region::Region(IN const GraphicsPath* path)
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegionPath(path->nativePath, &region);

    SetNativeRegion(region);
}

inline 
Region::Region(IN const BYTE* regionData, IN INT size)
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegionRgnData(regionData, size, 
                                                     &region);

    SetNativeRegion(region);
}

inline 
Region::Region(IN HRGN hRgn)
{
    GpRegion *region = NULL;

    lastResult = DllExports::GdipCreateRegionHrgn(hRgn, &region);

    SetNativeRegion(region);
}

inline 
Region* Region::FromHRGN(IN HRGN hRgn)
{
    GpRegion *region = NULL;

    if (DllExports::GdipCreateRegionHrgn(hRgn, &region) == Ok)
    {
        Region* newRegion = new Region(region);

        if (newRegion == NULL) 
        {
            DllExports::GdipDeleteRegion(region);
        }

        return newRegion;
    }
    else
        return NULL;
}

inline 
Region::~Region()
{
    DllExports::GdipDeleteRegion(nativeRegion);
}

inline Region* 
Region::Clone() const
{
    GpRegion *region = NULL;

    SetStatus(DllExports::GdipCloneRegion(nativeRegion, &region));

    return new Region(region);
}

inline Status 
Region::MakeInfinite()
{
    return SetStatus(DllExports::GdipSetInfinite(nativeRegion));
}

inline Status 
Region::MakeEmpty()
{
    return SetStatus(DllExports::GdipSetEmpty(nativeRegion));
}

inline Status 
Region::Intersect(IN const RectF& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRect(nativeRegion, &rect, 
                                                       CombineModeIntersect));
}

inline Status 
Region::Intersect(IN const Rect& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRectI(nativeRegion, &rect, 
                                                        CombineModeIntersect));
}

inline Status 
Region::Intersect(IN const GraphicsPath* path)
{
    return SetStatus(DllExports::GdipCombineRegionPath(nativeRegion, 
                                                       path->nativePath, 
                                                       CombineModeIntersect));
}

inline Status 
Region::Intersect(IN const Region* region)
{
    return SetStatus(DllExports::GdipCombineRegionRegion(nativeRegion, 
                                                         region->nativeRegion, 
                                                         CombineModeIntersect));
}

inline Status 
Region::Union(IN const RectF& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRect(nativeRegion, &rect, 
                                                       CombineModeUnion));
}

inline Status 
Region::Union(IN const Rect& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRectI(nativeRegion, &rect, 
                                                        CombineModeUnion));
}

inline Status 
Region::Union(IN const GraphicsPath* path)
{
    return SetStatus(DllExports::GdipCombineRegionPath(nativeRegion, 
                                                       path->nativePath, 
                                                       CombineModeUnion));
}

inline Status 
Region::Union(IN const Region* region)
{
    return SetStatus(DllExports::GdipCombineRegionRegion(nativeRegion, 
                                                         region->nativeRegion, 
                                                         CombineModeUnion));
}

inline Status 
Region::Xor(IN const RectF& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRect(nativeRegion, &rect, 
                                                       CombineModeXor));
}

inline Status 
Region::Xor(IN const Rect& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRectI(nativeRegion, &rect, 
                                                        CombineModeXor));
}

inline Status 
Region::Xor(IN const GraphicsPath* path)
{
    return SetStatus(DllExports::GdipCombineRegionPath(nativeRegion, 
                                                       path->nativePath, 
                                                       CombineModeXor));
}

inline Status 
Region::Xor(IN const Region* region)
{
    return SetStatus(DllExports::GdipCombineRegionRegion(nativeRegion, 
                                                         region->nativeRegion, 
                                                         CombineModeXor));
}

inline Status 
Region::Exclude(IN const RectF& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRect(nativeRegion, &rect, 
                                                       CombineModeExclude));
}

inline Status 
Region::Exclude(IN const Rect& rect)
{
     return SetStatus(DllExports::GdipCombineRegionRectI(nativeRegion, &rect, 
                                                         CombineModeExclude));
}

inline Status 
Region::Exclude(IN const GraphicsPath* path)
{
    return SetStatus(DllExports::GdipCombineRegionPath(nativeRegion, 
                                                       path->nativePath, 
                                                       CombineModeExclude));
}

inline Status
Region::Exclude(IN const Region* region)
{
    return SetStatus(DllExports::GdipCombineRegionRegion(nativeRegion,
                                               region->nativeRegion, 
                                                         CombineModeExclude));
}

inline Status 
Region::Complement(IN const RectF& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRect(nativeRegion, &rect, 
                                                       CombineModeComplement));
}

inline Status 
Region::Complement(IN const Rect& rect)
{
    return SetStatus(DllExports::GdipCombineRegionRectI(nativeRegion, &rect, 
                                                        CombineModeComplement));
}

inline Status 
Region::Complement(IN const GraphicsPath* path)
{
    return SetStatus(DllExports::GdipCombineRegionPath(nativeRegion,
                                                path->nativePath, 
                                                CombineModeComplement));
}

inline Status 
Region::Complement(IN const Region* region)
{
    return SetStatus(DllExports::GdipCombineRegionRegion(nativeRegion,
                                                  region->nativeRegion, 
                                                         CombineModeComplement));
}

inline Status 
Region::Translate(IN REAL dx, 
                  IN REAL dy)
{
    return SetStatus(DllExports::GdipTranslateRegion(nativeRegion, dx, dy));
}

inline Status 
Region::Translate(IN INT dx, 
                  IN INT dy)
{
    return SetStatus(DllExports::GdipTranslateRegionI(nativeRegion, dx, dy));
}

inline Status 
Region::Transform(IN const Matrix* matrix)
{
    return SetStatus(DllExports::GdipTransformRegion(nativeRegion, 
                                                     matrix->nativeMatrix));
}

inline Status 
Region::GetBounds(OUT RectF* rect,
                  IN const Graphics* g) const
{
    return SetStatus(DllExports::GdipGetRegionBounds(nativeRegion,
                                                g->nativeGraphics,
                                                rect));
}

inline Status 
Region::GetBounds(OUT Rect* rect,
                  IN const Graphics* g) const
{
    return SetStatus(DllExports::GdipGetRegionBoundsI(nativeRegion,
                                                g->nativeGraphics,
                                                rect));
}

inline HRGN
Region::GetHRGN(IN const Graphics* g) const
{
    HRGN hrgn;

    SetStatus(DllExports::GdipGetRegionHRgn(nativeRegion,
                                            g->nativeGraphics,
                                            &hrgn));

    return hrgn;
}

inline BOOL 
Region::IsEmpty(IN const Graphics *g) const
{
    BOOL booln = FALSE;
   
    SetStatus(DllExports::GdipIsEmptyRegion(nativeRegion,
                                            g->nativeGraphics,
                                            &booln));

    return booln;
}

inline BOOL 
Region::IsInfinite(IN const Graphics *g) const
{
    BOOL booln = FALSE;

    SetStatus(DllExports::GdipIsInfiniteRegion(nativeRegion,
                                                 g->nativeGraphics,
                                                 &booln));

    return booln;
}

inline BOOL 
Region::Equals(IN const Region* region, 
               IN const Graphics* g) const
{
    BOOL booln = FALSE;

    SetStatus(DllExports::GdipIsEqualRegion(nativeRegion,
                                              region->nativeRegion,
                                              g->nativeGraphics,
                                              &booln));
    return booln;
}

// Get the size of the buffer needed for the GetData method
inline UINT 
Region::GetDataSize() const
{
    UINT     bufferSize = 0;
    
    SetStatus(DllExports::GdipGetRegionDataSize(nativeRegion, &bufferSize));
    
    return bufferSize;
}

// buffer     - where to put the data
// bufferSize - how big the buffer is (should be at least as big as GetDataSize())
// sizeFilled - if not NULL, this is an OUT param that says how many bytes
//              of data were written to the buffer.
inline Status 
Region::GetData(_Out_writes_bytes_to_(bufferSize, *sizeFilled) BYTE* buffer, 
                _In_ UINT bufferSize, 
                _Out_opt_ UINT* sizeFilled) const
{
    return SetStatus(DllExports::GdipGetRegionData(nativeRegion, buffer, 
                                                   bufferSize, sizeFilled));
}

/**
 * Hit testing operations
 */
inline BOOL 
Region::IsVisible(IN const PointF& point, 
                  IN const Graphics* g) const
{
    BOOL booln = FALSE;

    SetStatus(DllExports::GdipIsVisibleRegionPoint(nativeRegion,
                                     point.X, point.Y, 
                                     (g == NULL) ? NULL : g->nativeGraphics,
                                     &booln));
    return booln;
}

inline BOOL 
Region::IsVisible(IN const RectF& rect, 
                  IN const Graphics* g) const
{
    BOOL booln = FALSE;

    SetStatus(DllExports::GdipIsVisibleRegionRect(nativeRegion, rect.X,
                                                    rect.Y, rect.Width,
                                                    rect.Height,
                                                    (g == NULL) ?
                                                      NULL : g->nativeGraphics,
                                                    &booln));
    return booln;
}

inline BOOL 
Region::IsVisible(IN const Point& point, 
                  IN const Graphics* g) const
{
    BOOL booln = FALSE;


    SetStatus(DllExports::GdipIsVisibleRegionPointI(nativeRegion,
                                                   point.X,
                                                   point.Y,
                                                   (g == NULL) 
                                                    ? NULL : g->nativeGraphics,
                                                   &booln));
    return booln;
}

inline BOOL 
Region::IsVisible(IN const Rect& rect, 
                  IN const Graphics* g) const
{
    BOOL booln = FALSE;

    SetStatus(DllExports::GdipIsVisibleRegionRectI(nativeRegion,
                                                  rect.X,
                                                  rect.Y,
                                                  rect.Width,
                                                  rect.Height,
                                                  (g == NULL) 
                                                    ? NULL : g->nativeGraphics,
                                                  &booln));
    return booln;
}

inline UINT 
Region::GetRegionScansCount(IN const Matrix* matrix) const
{
    UINT count = 0;

    SetStatus(DllExports::GdipGetRegionScansCount(nativeRegion,
                                                  &count,
                                                  matrix->nativeMatrix));
    return count;
}

// If rects is NULL, return the count of rects in the region.
// Otherwise, assume rects is big enough to hold all the region rects
// and fill them in and return the number of rects filled in.
// The rects are returned in the units specified by the matrix
// (which is typically a world-to-device transform).
// Note that the number of rects returned can vary, depending on the
// matrix that is used.

inline Status 
Region::GetRegionScans(
    IN const Matrix* matrix,
    OUT RectF* rects,
    IN OUT INT* count) const
{
    return SetStatus(DllExports::GdipGetRegionScans(nativeRegion,
                                          rects,
                                          count,
                                          matrix->nativeMatrix));
}

inline Status
Region::GetRegionScans(
    IN const Matrix* matrix,
    OUT Rect* rects,
    IN OUT INT* count) const
{
    return SetStatus(DllExports::GdipGetRegionScansI(nativeRegion,
                                          rects,
                                          count,
                                          matrix->nativeMatrix));
}

inline Region::Region(GpRegion* nativeRegion)
{
    SetNativeRegion(nativeRegion);
}

inline VOID Region::SetNativeRegion(GpRegion* newNativeRegion)
{
    this->nativeRegion = newNativeRegion;
}

inline Status Region::GetLastStatus() const
{
    Status lastStatus = lastResult;
    lastResult = Ok;

    return lastStatus;
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GDIPLUSREGION_H

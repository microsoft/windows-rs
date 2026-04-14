/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusTypes.h
*
* Abstract:
*
*   GDI+ Types
*
\**************************************************************************/

#ifndef _GDIPLUSTYPES_H
#define _GDIPLUSTYPES_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

// Support NOMINMAX by defining local min max macros 
#define GDIPLUS_MIN(a,b) (((a) < (b)) ? (a) : (b))
#define GDIPLUS_MAX(a,b) (((a) > (b)) ? (a) : (b))

//--------------------------------------------------------------------------
// Callback functions
//--------------------------------------------------------------------------

extern "C" {
typedef BOOL (CALLBACK * ImageAbort)(VOID *);
typedef ImageAbort DrawImageAbort;
typedef ImageAbort GetThumbnailImageAbort;
}

// Callback for EnumerateMetafile methods.  The parameters are:

//      recordType      WMF, EMF, or EMF+ record type
//      flags           (always 0 for WMF/EMF records)
//      dataSize        size of the record data (in bytes), or 0 if no data
//      data            pointer to the record data, or NULL if no data
//      callbackData    pointer to callbackData, if any

// This method can then call Metafile::PlayRecord to play the
// record that was just enumerated.  If this method  returns
// FALSE, the enumeration process is aborted.  Otherwise, it continues.

extern "C" {
typedef BOOL (CALLBACK * EnumerateMetafileProc)(EmfPlusRecordType,UINT,UINT,const BYTE*,VOID*);
}

#if (GDIPVER >= 0x0110)
// This is the main GDI+ Abort interface

struct __declspec(novtable) GdiplusAbort
{
    virtual HRESULT __stdcall Abort(void) = 0;
};
#endif //(GDIPVER >= 0x0110)

//--------------------------------------------------------------------------
// Primitive data types
//
// NOTE:
//  Types already defined in standard header files:
//      INT8
//      UINT8
//      INT16
//      UINT16
//      INT32
//      UINT32
//      INT64
//      UINT64
//
//  Avoid using the following types:
//      LONG - use INT
//      ULONG - use UINT
//      DWORD - use UINT32
//--------------------------------------------------------------------------

typedef float REAL;

#define REAL_MAX            FLT_MAX
#define REAL_MIN            FLT_MIN
#define REAL_TOLERANCE     (FLT_MIN * 100)
#define REAL_EPSILON        1.192092896e-07F        /* FLT_EPSILON */

//--------------------------------------------------------------------------
// Forward declarations of common classes
//--------------------------------------------------------------------------

class Size;
class SizeF;
class Point;
class PointF;
class Rect;
class RectF;
class CharacterRange;

//--------------------------------------------------------------------------
// Status return values from GDI+ methods
//--------------------------------------------------------------------------

_Return_type_success_(return == 0) enum Status
{
    Ok = 0,
    GenericError = 1,
    InvalidParameter = 2,
    OutOfMemory = 3,
    ObjectBusy = 4,
    InsufficientBuffer = 5,
    NotImplemented = 6,
    Win32Error = 7,
    WrongState = 8,
    Aborted = 9,
    FileNotFound = 10,
    ValueOverflow = 11,
    AccessDenied = 12,
    UnknownImageFormat = 13,
    FontFamilyNotFound = 14,
    FontStyleNotFound = 15,
    NotTrueTypeFont = 16,
    UnsupportedGdiplusVersion = 17,
    GdiplusNotInitialized = 18,
    PropertyNotFound = 19,
    PropertyNotSupported = 20,
#if (GDIPVER >= 0x0110)
    ProfileNotFound = 21,
#endif //(GDIPVER >= 0x0110)
};

//--------------------------------------------------------------------------
// Represents a dimension in a 2D coordinate system (floating-point coordinates)
//--------------------------------------------------------------------------

class SizeF
{
public:
    SizeF()
    {
        Width = Height = 0.0f;
    }

    SizeF(IN const SizeF& size)
    {
        Width = size.Width;
        Height = size.Height;
    }

    SizeF(IN REAL width,
          IN REAL height)
    {
        Width = width;
        Height = height;
    }

    SizeF operator+(IN const SizeF& sz) const
    {
        return SizeF(Width + sz.Width,
                     Height + sz.Height);
    }

    SizeF operator-(IN const SizeF& sz) const
    {
        return SizeF(Width - sz.Width,
                     Height - sz.Height);
    }

    BOOL Equals(IN const SizeF& sz) const
    {
        return (Width == sz.Width) && (Height == sz.Height);
    }

    BOOL Empty() const
    {
        return (Width == 0.0f && Height == 0.0f);
    }

public:

    REAL Width;
    REAL Height;
};

//--------------------------------------------------------------------------
// Represents a dimension in a 2D coordinate system (integer coordinates)
//--------------------------------------------------------------------------

class Size
{
public:
    Size()
    {
        Width = Height = 0;
    }

    Size(IN const Size& size)
    {
        Width = size.Width;
        Height = size.Height;
    }

    Size(IN INT width,
         IN INT height)
    {
        Width = width;
        Height = height;
    }

    Size operator+(IN const Size& sz) const
    {
        return Size(Width + sz.Width,
                    Height + sz.Height);
    }

    Size operator-(IN const Size& sz) const
    {
        return Size(Width - sz.Width,
                    Height - sz.Height);
    }

    BOOL Equals(IN const Size& sz) const
    {
        return (Width == sz.Width) && (Height == sz.Height);
    }

    BOOL Empty() const
    {
        return (Width == 0 && Height == 0);
    }

public:

    INT Width;
    INT Height;
};

//--------------------------------------------------------------------------
// Represents a location in a 2D coordinate system (floating-point coordinates)
//--------------------------------------------------------------------------

class PointF
{
public:
   PointF()
   {
       X = Y = 0.0f;
   }

   PointF(IN const PointF &point)
   {
       X = point.X;
       Y = point.Y;
   }

   PointF(IN const SizeF &size)
   {
       X = size.Width;
       Y = size.Height;
   }

   PointF(IN REAL x,
          IN REAL y)
   {
       X = x;
       Y = y;
   }

   PointF operator+(IN const PointF& point) const
   {
       return PointF(X + point.X,
                     Y + point.Y);
   }

   PointF operator-(IN const PointF& point) const
   {
       return PointF(X - point.X,
                     Y - point.Y);
   }

   BOOL Equals(IN const PointF& point)
   {
       return (X == point.X) && (Y == point.Y);
   }

public:

    REAL X;
    REAL Y;
};

//--------------------------------------------------------------------------
// Represents a location in a 2D coordinate system (integer coordinates)
//--------------------------------------------------------------------------

class Point
{
public:
   Point()
   {
       X = Y = 0;
   }

   Point(IN const Point &point)
   {
       X = point.X;
       Y = point.Y;
   }

   Point(IN const Size &size)
   {
       X = size.Width;
       Y = size.Height;
   }

   Point(IN INT x,
         IN INT y)
   {
       X = x;
       Y = y;
   }

   Point operator+(IN const Point& point) const
   {
       return Point(X + point.X,
                    Y + point.Y);
   }

   Point operator-(IN const Point& point) const
   {
       return Point(X - point.X,
                    Y - point.Y);
   }

   BOOL Equals(IN const Point& point)
   {
       return (X == point.X) && (Y == point.Y);
   }

public:

    INT X;
    INT Y;
};

//--------------------------------------------------------------------------
// Represents a rectangle in a 2D coordinate system (floating-point coordinates)
//--------------------------------------------------------------------------

class RectF
{
public:

    RectF()
    {
        X = Y = Width = Height = 0.0f;
    }

    RectF(IN REAL x,
          IN REAL y,
          IN REAL width,
          IN REAL height)
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    RectF(IN const PointF& location,
          IN const SizeF& size)
    {
        X = location.X;
        Y = location.Y;
        Width = size.Width;
        Height = size.Height;
    }

    RectF* Clone() const
    {
        return new RectF(X, Y, Width, Height);
    }

    VOID GetLocation(OUT PointF* point) const
    {
        point->X = X;
        point->Y = Y;
    }

    VOID GetSize(OUT SizeF* size) const
    {
        size->Width = Width;
        size->Height = Height;
    }

    VOID GetBounds(OUT RectF* rect) const
    {
        rect->X = X;
        rect->Y = Y;
        rect->Width = Width;
        rect->Height = Height;
    }

    REAL GetLeft() const
    {
        return X;
    }

    REAL GetTop() const
    {
        return Y;
    }

    REAL GetRight() const
    {
        return X+Width;
    }

    REAL GetBottom() const
    {
        return Y+Height;
    }

    BOOL IsEmptyArea() const
    {
        return (Width <= REAL_EPSILON) || (Height <= REAL_EPSILON);
    }

    BOOL Equals(IN const RectF & rect) const
    {
        return X == rect.X &&
               Y == rect.Y &&
               Width == rect.Width &&
               Height == rect.Height;
    }

    BOOL Contains(IN REAL x,
                  IN REAL y) const
    {
        return x >= X && x < X+Width &&
               y >= Y && y < Y+Height;
    }

    BOOL Contains(IN const PointF& pt) const
    {
        return Contains(pt.X, pt.Y);
    }

    BOOL Contains(IN const RectF& rect) const
    {
        return (X <= rect.X) && (rect.GetRight() <= GetRight()) &&
               (Y <= rect.Y) && (rect.GetBottom() <= GetBottom());
    }

    VOID Inflate(IN REAL dx,
                 IN REAL dy)
    {
        X -= dx;
        Y -= dy;
        Width += 2*dx;
        Height += 2*dy;
    }

    VOID Inflate(IN const PointF& point)
    {
        Inflate(point.X, point.Y);
    }

    BOOL Intersect(IN const RectF& rect)
    {
        return Intersect(*this, *this, rect);
    }

    static BOOL Intersect(OUT RectF& c,
                          IN const RectF& a,
                          IN const RectF& b)
    {
        REAL right = GDIPLUS_MIN(a.GetRight(), b.GetRight());
        REAL bottom = GDIPLUS_MIN(a.GetBottom(), b.GetBottom());
        REAL left = GDIPLUS_MAX(a.GetLeft(), b.GetLeft());
        REAL top = GDIPLUS_MAX(a.GetTop(), b.GetTop());

        c.X = left;
        c.Y = top;
        c.Width = right - left;
        c.Height = bottom - top;
        return !c.IsEmptyArea();
    }

    BOOL IntersectsWith(IN const RectF& rect) const
    {
        return (GetLeft() < rect.GetRight() &&
                GetTop() < rect.GetBottom() &&
                GetRight() > rect.GetLeft() &&
                GetBottom() > rect.GetTop());
    }

    static BOOL Union(OUT RectF& c,
                      IN const RectF& a,
                      IN const RectF& b)
    {
        REAL right = GDIPLUS_MAX(a.GetRight(), b.GetRight());
        REAL bottom = GDIPLUS_MAX(a.GetBottom(), b.GetBottom());
        REAL left = GDIPLUS_MIN(a.GetLeft(), b.GetLeft());
        REAL top = GDIPLUS_MIN(a.GetTop(), b.GetTop());

        c.X = left;
        c.Y = top;
        c.Width = right - left;
        c.Height = bottom - top;
        return !c.IsEmptyArea();
    }

    VOID Offset(IN const PointF& point)
    {
        Offset(point.X, point.Y);
    }

    VOID Offset(IN REAL dx,
                IN REAL dy)
    {
        X += dx;
        Y += dy;
    }

public:

    REAL X;
    REAL Y;
    REAL Width;
    REAL Height;
};

//--------------------------------------------------------------------------
// Represents a rectangle in a 2D coordinate system (integer coordinates)
//--------------------------------------------------------------------------

class Rect
{
public:

    Rect()
    {
        X = Y = Width = Height = 0;
    }

    Rect(IN INT x,
         IN INT y,
         IN INT width,
         IN INT height)
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    Rect(IN const Point& location,
         IN const Size& size)
    {
        X = location.X;
        Y = location.Y;
        Width = size.Width;
        Height = size.Height;
    }

    Rect* Clone() const
    {
        return new Rect(X, Y, Width, Height);
    }

    VOID GetLocation(OUT Point* point) const
    {
        point->X = X;
        point->Y = Y;
    }

    VOID GetSize(OUT Size* size) const
    {
        size->Width = Width;
        size->Height = Height;
    }

    VOID GetBounds(OUT Rect* rect) const
    {
        rect->X = X;
        rect->Y = Y;
        rect->Width = Width;
        rect->Height = Height;
    }

    INT GetLeft() const
    {
        return X;
    }

    INT GetTop() const
    {
        return Y;
    }

    INT GetRight() const
    {
        return X+Width;
    }

    INT GetBottom() const
    {
        return Y+Height;
    }

    BOOL IsEmptyArea() const
    {
        return (Width <= 0) || (Height <= 0);
    }

    BOOL Equals(IN const Rect & rect) const
    {
        return X == rect.X &&
               Y == rect.Y &&
               Width == rect.Width &&
               Height == rect.Height;
    }

    BOOL Contains(IN INT x,
                  IN INT y) const
    {
        return x >= X && x < X+Width &&
               y >= Y && y < Y+Height;
    }

    BOOL Contains(IN const Point& pt) const
    {
        return Contains(pt.X, pt.Y);
    }

    BOOL Contains(IN Rect& rect) const
    {
        return (X <= rect.X) && (rect.GetRight() <= GetRight()) &&
               (Y <= rect.Y) && (rect.GetBottom() <= GetBottom());
    }

    VOID Inflate(IN INT dx,
                 IN INT dy)
    {
        X -= dx;
        Y -= dy;
        Width += 2*dx;
        Height += 2*dy;
    }

    VOID Inflate(IN const Point& point)
    {
        Inflate(point.X, point.Y);
    }

    BOOL Intersect(IN const Rect& rect)
    {
        return Intersect(*this, *this, rect);
    }

    static BOOL Intersect(OUT Rect& c,
                          IN const Rect& a,
                          IN const Rect& b)
    {
        INT right = GDIPLUS_MIN(a.GetRight(), b.GetRight());
        INT bottom = GDIPLUS_MIN(a.GetBottom(), b.GetBottom());
        INT left = GDIPLUS_MAX(a.GetLeft(), b.GetLeft());
        INT top = GDIPLUS_MAX(a.GetTop(), b.GetTop());

        c.X = left;
        c.Y = top;
        c.Width = right - left;
        c.Height = bottom - top;
        return !c.IsEmptyArea();
    }

    BOOL IntersectsWith(IN const Rect& rect) const
    {
        return (GetLeft() < rect.GetRight() &&
                GetTop() < rect.GetBottom() &&
                GetRight() > rect.GetLeft() &&
                GetBottom() > rect.GetTop());
    }

    static BOOL Union(OUT Rect& c,
                      IN const Rect& a,
                      IN const Rect& b)
    {
        INT right = GDIPLUS_MAX(a.GetRight(), b.GetRight());
        INT bottom = GDIPLUS_MAX(a.GetBottom(), b.GetBottom());
        INT left = GDIPLUS_MIN(a.GetLeft(), b.GetLeft());
        INT top = GDIPLUS_MIN(a.GetTop(), b.GetTop());

        c.X = left;
        c.Y = top;
        c.Width = right - left;
        c.Height = bottom - top;
        return !c.IsEmptyArea();
    }

    VOID Offset(IN const Point& point)
    {
        Offset(point.X, point.Y);
    }

    VOID Offset(IN INT dx,
                IN INT dy)
    {
        X += dx;
        Y += dy;
    }

public:

    INT X;
    INT Y;
    INT Width;
    INT Height;
};

class PathData
{
public:
    PathData()
    {
        Count = 0;
        Points = NULL;
        Types = NULL;
    }

    ~PathData()
    {
        if (Points != NULL)
        {
            delete [] Points;
        }

        if (Types != NULL)
        {
            delete [] Types;
        }
    }

private:
    PathData(const PathData &);
    PathData& operator=(const PathData &);

public:
    INT Count;
    PointF* Points;
    _Field_size_opt_(Count) BYTE* Types;
};

class CharacterRange
{
public:
    CharacterRange(
        INT first,
        INT length
    ) :
        First   (first),
        Length  (length)
    {}

    CharacterRange() : First(0), Length(0)
    {}

    CharacterRange & operator = (const CharacterRange &rhs)
    {
        First  = rhs.First;
        Length = rhs.Length;
        return *this;
    }

    INT First;
    INT Length;
};

#undef GDIPLUS_MIN
#undef GDIPLUS_MAX

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GDIPLUSTYPES_HPP

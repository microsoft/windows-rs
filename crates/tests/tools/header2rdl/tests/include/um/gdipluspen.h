/**************************************************************************\
* 
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusPen.h
*
* Abstract:
*
*   GDI+ Pen class
*
\**************************************************************************/
#ifndef _GDIPLUSPEN_H
#define _GDIPLUSPEN_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4061) /* enumerator in switch of enum is not explicitly handled by a case label */
#pragma warning(disable:4820) /* padding added after data member */
#endif

//--------------------------------------------------------------------------
// Pen class
//--------------------------------------------------------------------------

class Pen : public GdiplusBase
{
public:
    friend class GraphicsPath;
    friend class Graphics;

    Pen(IN const Color& color, 
        IN REAL width = 1.0f)
    {
        Unit unit = UnitWorld;
        nativePen = NULL;
        lastResult = DllExports::GdipCreatePen1(color.GetValue(),
                                    width, unit, &nativePen);
    }

    Pen(IN const Brush* brush, 
        IN REAL width = 1.0f)
    {
        Unit unit = UnitWorld;
        nativePen = NULL;
        lastResult = DllExports::GdipCreatePen2(brush->nativeBrush,
                                    width, unit, &nativePen);
    }

    ~Pen()
    {
        DllExports::GdipDeletePen(nativePen);
    }

    Pen* Clone() const
    {
        GpPen *clonePen = NULL;

        lastResult = DllExports::GdipClonePen(nativePen, &clonePen);
   
        return new Pen(clonePen, lastResult);
    }

    Status SetWidth(IN REAL width)
    {
        return SetStatus(DllExports::GdipSetPenWidth(nativePen, width));
    }

    REAL GetWidth() const
    {
        REAL width;

        SetStatus(DllExports::GdipGetPenWidth(nativePen, &width));
        
        return width;
    }
    
    // Set/get line caps: start, end, and dash

    // Line cap and join APIs by using LineCap and LineJoin enums.

    Status SetLineCap(IN LineCap startCap, 
                      IN LineCap endCap, 
                      IN DashCap dashCap)
    {
        return SetStatus(DllExports::GdipSetPenLineCap197819(nativePen, 
                                   startCap, endCap, dashCap));
    }

    Status SetStartCap(IN LineCap startCap)
    {
        return SetStatus(DllExports::GdipSetPenStartCap(nativePen, startCap));
    }

    Status SetEndCap(IN LineCap endCap)
    {
        return SetStatus(DllExports::GdipSetPenEndCap(nativePen, endCap));
    }

    Status SetDashCap(IN DashCap dashCap)
    {
        return SetStatus(DllExports::GdipSetPenDashCap197819(nativePen,
                                   dashCap));
    }

    LineCap GetStartCap() const
    {
        LineCap startCap;

        SetStatus(DllExports::GdipGetPenStartCap(nativePen, &startCap));
        
        return startCap;
    }

    LineCap GetEndCap() const
    {
        LineCap endCap;

        SetStatus(DllExports::GdipGetPenEndCap(nativePen, &endCap));

        return endCap;
    }

    DashCap GetDashCap() const
    {
        DashCap dashCap;

        SetStatus(DllExports::GdipGetPenDashCap197819(nativePen,
                            &dashCap));

        return dashCap;
    }

    Status SetLineJoin(IN LineJoin lineJoin)
    {
        return SetStatus(DllExports::GdipSetPenLineJoin(nativePen, lineJoin));
    }

    LineJoin GetLineJoin() const
    {
        LineJoin lineJoin;
        
        SetStatus(DllExports::GdipGetPenLineJoin(nativePen, &lineJoin));
        
        return lineJoin;
    }

    Status SetCustomStartCap(IN const CustomLineCap* customCap)
    {
        GpCustomLineCap* nativeCap = NULL;
        if(customCap)
            nativeCap = customCap->nativeCap;

        return SetStatus(DllExports::GdipSetPenCustomStartCap(nativePen, 
                                                              nativeCap));
    }

    Status GetCustomStartCap(OUT CustomLineCap* customCap) const
    {
        if(!customCap)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipGetPenCustomStartCap(nativePen, 
                                                    &(customCap->nativeCap)));
    }

    Status SetCustomEndCap(IN const CustomLineCap* customCap)
    {
        GpCustomLineCap* nativeCap = NULL;
        if(customCap)
            nativeCap = customCap->nativeCap;

        return SetStatus(DllExports::GdipSetPenCustomEndCap(nativePen, 
                                                            nativeCap));
    }

    Status GetCustomEndCap(OUT CustomLineCap* customCap) const
    {
        if(!customCap)
            return SetStatus(InvalidParameter);

        return SetStatus(DllExports::GdipGetPenCustomEndCap(nativePen, 
                                                    &(customCap->nativeCap)));
    }

    Status SetMiterLimit(IN REAL miterLimit)
    {
        return SetStatus(DllExports::GdipSetPenMiterLimit(nativePen, 
                                                    miterLimit));
    }

    REAL GetMiterLimit() const
    {
        REAL miterLimit;

        SetStatus(DllExports::GdipGetPenMiterLimit(nativePen, &miterLimit));

        return miterLimit;
    }

    Status SetAlignment(IN PenAlignment penAlignment)
    {
        return SetStatus(DllExports::GdipSetPenMode(nativePen, penAlignment));
    }

    PenAlignment GetAlignment() const
    {
        PenAlignment penAlignment;
        
        SetStatus(DllExports::GdipGetPenMode(nativePen, &penAlignment));
        
        return penAlignment;
    }
    
    Status SetTransform(IN const Matrix* matrix)
    {
        return SetStatus(DllExports::GdipSetPenTransform(nativePen, 
                                                       matrix->nativeMatrix));
    }

    Status GetTransform(OUT Matrix* matrix) const
    {
        return SetStatus(DllExports::GdipGetPenTransform(nativePen, 
                                                         matrix->nativeMatrix));
    }

    Status ResetTransform()
    {
        return SetStatus(DllExports::GdipResetPenTransform(nativePen));
    }

    Status MultiplyTransform(IN const Matrix* matrix,
                             IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyPenTransform(nativePen,
                                                         matrix->nativeMatrix,
                                                         order));
    }

    Status TranslateTransform(IN REAL dx, 
                              IN REAL dy,
                              IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslatePenTransform(nativePen,
                                                               dx, 
                                                               dy, 
                                                               order));
    }

    Status ScaleTransform(IN REAL sx, 
                          IN REAL sy,
                          IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScalePenTransform(nativePen,
                                                           sx, 
                                                           sy, 
                                                           order));
    }

    Status RotateTransform(IN REAL angle, 
                           IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotatePenTransform(nativePen,
                                                            angle, 
                                                            order));
    }

    PenType GetPenType() const
    {
       PenType type;
       SetStatus(DllExports::GdipGetPenFillType(nativePen, &type));

       return type;
    }

    Status SetColor(IN const Color& color)
    {
        return SetStatus(DllExports::GdipSetPenColor(nativePen,
                                                     color.GetValue()));
    }

    Status SetBrush(IN const Brush* brush)
    {
        return SetStatus(DllExports::GdipSetPenBrushFill(nativePen, 
                                       brush->nativeBrush));
    }

    Status GetColor(OUT Color* color) const
    {
        if (color == NULL) 
        {
            return SetStatus(InvalidParameter);
        }
        
        PenType type = GetPenType();

        if (type != PenTypeSolidColor) 
        {
            return WrongState;
        }
        
        ARGB argb;
        
        SetStatus(DllExports::GdipGetPenColor(nativePen,
                                              &argb));
        if (lastResult == Ok)
        {
            color->SetValue(argb);
        }
        
        return lastResult;
    }

    Brush* GetBrush() const
    {
       PenType type = GetPenType();

       Brush* brush = NULL;

       switch(type)
       {
       case PenTypeSolidColor:
           brush = new SolidBrush();
           break;

       case PenTypeHatchFill:
           brush = new HatchBrush();
           break;

       case PenTypeTextureFill:
           brush = new TextureBrush();
           break;

       case PenTypePathGradient:
           brush = new Brush();
           break;

       case PenTypeLinearGradient:
           brush = new LinearGradientBrush();
           break;

       default:
           break;
       }

       if(brush)
       {
           GpBrush* nativeBrush;

           SetStatus(DllExports::GdipGetPenBrushFill(nativePen, 
                                                     &nativeBrush));
           brush->SetNativeBrush(nativeBrush);
       }

       return brush;
    }

    DashStyle GetDashStyle() const
    {
        DashStyle dashStyle;

        SetStatus(DllExports::GdipGetPenDashStyle(nativePen, &dashStyle));

        return dashStyle;
    }

    Status SetDashStyle(IN DashStyle dashStyle)
    {
        return SetStatus(DllExports::GdipSetPenDashStyle(nativePen, 
                                                         dashStyle));
    }

    REAL GetDashOffset() const
    {
        REAL dashOffset;

        SetStatus(DllExports::GdipGetPenDashOffset(nativePen, &dashOffset));

        return dashOffset;
    }

    Status SetDashOffset(IN REAL dashOffset)
    {
        return SetStatus(DllExports::GdipSetPenDashOffset(nativePen, 
                                                          dashOffset));
    }
    
    Status SetDashPattern(IN const REAL* dashArray, IN INT count)
    {
        return SetStatus(DllExports::GdipSetPenDashArray(nativePen,
                                                         dashArray, 
                                                         count));
    }
    
    INT GetDashPatternCount() const
    {
        INT count = 0;
        
        SetStatus(DllExports::GdipGetPenDashCount(nativePen, &count));
        
        return count;
    }

    Status GetDashPattern(OUT REAL* dashArray, 
                          IN INT count) const
    {
        if (dashArray == NULL || count <= 0)
            return SetStatus(InvalidParameter); 
        
        return SetStatus(DllExports::GdipGetPenDashArray(nativePen, 
                                                         dashArray, 
                                                         count));
    }

    Status SetCompoundArray(IN const REAL* compoundArray,
                            IN INT count)
    {
        return SetStatus(DllExports::GdipSetPenCompoundArray(nativePen, 
                                                             compoundArray,
                                                             count));
    }

    INT GetCompoundArrayCount() const
    {
        INT count = 0;
        
        SetStatus(DllExports::GdipGetPenCompoundCount(nativePen, &count));
        
        return count;
    }

    Status GetCompoundArray(OUT REAL* compoundArray, 
                            IN INT count) const
    {
        if (compoundArray == NULL || count <= 0)
            return SetStatus(InvalidParameter); 
        
        return SetStatus(DllExports::GdipGetPenCompoundArray(nativePen, 
                                                             compoundArray, 
                                                             count));
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;

        return lastStatus;
    }

private:
    Pen(const Pen &);
    Pen& operator=(const Pen &);

protected:
    Pen(GpPen* nativePenArg, Status status)
    {
        lastResult = status;
        SetNativePen(nativePenArg);
    }

    VOID SetNativePen(GpPen* nativePenArg)
    {
        this->nativePen = nativePenArg;
    }
    
    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else 
            return status;
    }

protected:
    GpPen* nativePen;
    mutable Status lastResult;
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

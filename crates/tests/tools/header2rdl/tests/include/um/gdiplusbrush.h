/**************************************************************************\
* 
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusBrush.h
*
* Abstract:
*
*   GDI+ Brush class
*
\**************************************************************************/

#ifndef _GDIPLUSBRUSH_H
#define _GDIPLUSBRUSH_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

class GraphicsPath;

//--------------------------------------------------------------------------
// Abstract base class for various brush types
//--------------------------------------------------------------------------

class Brush : public GdiplusBase
{
public:
    friend class Pen;
    friend class Graphics;

    virtual ~Brush()
    {
        DllExports::GdipDeleteBrush(nativeBrush);
    }

    virtual Brush* Clone() const
    {
        GpBrush *brush = NULL;

        SetStatus(DllExports::GdipCloneBrush(nativeBrush, &brush));

        Brush *newBrush = new Brush(brush, lastResult);
        
        if (newBrush == NULL) 
        {
            DllExports::GdipDeleteBrush(brush);
        }

        return newBrush;
    }

    BrushType GetType() const
    {
        BrushType type = static_cast<BrushType>(-1);

        SetStatus(DllExports::GdipGetBrushType(nativeBrush, &type));

        return type;
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;

        return lastStatus;
    }

protected:

    Brush()
    {
        SetStatus(NotImplemented);
    }

private:
    Brush(const Brush& brush);
    Brush& operator=(const Brush& brush);
protected:

    Brush(GpBrush* nativeBrushArg, Status status)
    {
        lastResult = status;
        SetNativeBrush(nativeBrushArg);
    }

    VOID SetNativeBrush(GpBrush* nativeBrushArg)
    {
        this->nativeBrush = nativeBrushArg;
    }

    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

    GpBrush* nativeBrush;
    mutable Status lastResult;
};

//--------------------------------------------------------------------------
// Solid Fill Brush Object
//--------------------------------------------------------------------------

class SolidBrush : public Brush
{
public:
    friend class Pen;

    SolidBrush(IN const Color& color)
    {
        GpSolidFill *brush = NULL;

        lastResult = DllExports::GdipCreateSolidFill(color.GetValue(), &brush);

        SetNativeBrush(brush);
    }

    Status GetColor(OUT Color* color) const
    {
        ARGB argb;

        if (color == NULL) 
        {
            return SetStatus(InvalidParameter);
        }

        SetStatus(DllExports::GdipGetSolidFillColor((GpSolidFill*)nativeBrush,
                                                    &argb));

        *color = Color(argb);

        return lastResult;
    }

    Status SetColor(IN const Color& color)
    {
        return SetStatus(DllExports::GdipSetSolidFillColor((GpSolidFill*)nativeBrush, 
                                                           color.GetValue()));
    }

private:
    SolidBrush(const SolidBrush &);
    SolidBrush& operator=(const SolidBrush &);

protected:

    SolidBrush()
    {
    }
};

//--------------------------------------------------------------------------
// Texture Brush Fill Object
//--------------------------------------------------------------------------

class TextureBrush : public Brush
{
public:
    friend class Pen;

    TextureBrush(IN Image* image, 
                 IN WrapMode wrapMode = WrapModeTile)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTexture(
                                                  image->nativeImage,
                                                  wrapMode, &texture);

        SetNativeBrush(texture);
    }

    // When creating a texture brush from a metafile image, the dstRect
    // is used to specify the size that the metafile image should be
    // rendered at in the device units of the destination graphics.
    // It is NOT used to crop the metafile image, so only the width 
    // and height values matter for metafiles.
    
    TextureBrush(IN Image* image, 
                 IN WrapMode wrapMode,
                 IN const RectF &dstRect)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTexture2(
                                                   image->nativeImage,
                                                   wrapMode, 
                                                   dstRect.X, 
                                                   dstRect.Y, 
                                                   dstRect.Width, 
                                                   dstRect.Height,
                                                   &texture);

        SetNativeBrush(texture);
    }
    
    TextureBrush(IN Image *image, 
                 IN const RectF &dstRect,
                 IN const ImageAttributes *imageAttributes = NULL)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTextureIA(
            image->nativeImage,
            (imageAttributes)?imageAttributes->nativeImageAttr:NULL,
            dstRect.X, 
            dstRect.Y, 
            dstRect.Width, 
            dstRect.Height,
            &texture
        );

        SetNativeBrush(texture);
    }
    
    TextureBrush(IN Image *image, 
                 IN const Rect &dstRect,
                 IN const ImageAttributes *imageAttributes = NULL)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTextureIAI(
            image->nativeImage,
            (imageAttributes)?imageAttributes->nativeImageAttr:NULL,
            dstRect.X, 
            dstRect.Y, 
            dstRect.Width, 
            dstRect.Height,
            &texture
        );

        SetNativeBrush(texture);
    }

    TextureBrush(
        IN Image* image,
        IN WrapMode wrapMode,
        
        const IN Rect &dstRect
    )
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTexture2I(
                                                    image->nativeImage,
                                                    wrapMode, 
                                                    dstRect.X, 
                                                    dstRect.Y, 
                                                    dstRect.Width, 
                                                    dstRect.Height,
                                                    &texture);

        SetNativeBrush(texture);
    }
    
    TextureBrush(IN Image* image, 
                 IN WrapMode wrapMode, 
                 IN REAL dstX, 
                 IN REAL dstY, 
                 IN REAL dstWidth, 
                 IN REAL dstHeight)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTexture2(
                                                   image->nativeImage,
                                                   wrapMode, 
                                                   dstX, 
                                                   dstY, 
                                                   dstWidth, 
                                                   dstHeight,
                                                   &texture);

        SetNativeBrush(texture);
    }
    
    TextureBrush(IN Image* image, 
                 IN WrapMode wrapMode, 
                 IN INT dstX, 
                 IN INT dstY, 
                 IN INT dstWidth, 
                 IN INT dstHeight)
    {
        GpTexture *texture = NULL;

        lastResult = DllExports::GdipCreateTexture2I(
                                                    image->nativeImage,
                                                    wrapMode, 
                                                    dstX, 
                                                    dstY, 
                                                    dstWidth, 
                                                    dstHeight,
                                                    &texture);

        SetNativeBrush(texture);
    }

    Status SetTransform(IN const Matrix* matrix)
    {
        return SetStatus(DllExports::GdipSetTextureTransform((GpTexture*)nativeBrush, 
                                                             matrix->nativeMatrix));
    }

    Status GetTransform(OUT Matrix* matrix) const
    {
        return SetStatus(DllExports::GdipGetTextureTransform((GpTexture*)nativeBrush, 
                                                             matrix->nativeMatrix));
    }

    Status ResetTransform()
    {
        return SetStatus(DllExports::GdipResetTextureTransform((GpTexture*)nativeBrush));
    }

    Status MultiplyTransform(IN const Matrix* matrix,
                             IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyTextureTransform((GpTexture*)nativeBrush,
                                                                matrix->nativeMatrix,
                                                                order));
    }

    Status TranslateTransform(IN REAL dx,
                              IN REAL dy,
                              IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslateTextureTransform((GpTexture*)nativeBrush,
                                                               dx, dy, order));
    }

    Status ScaleTransform(IN REAL sx, 
                          IN REAL sy,
                          IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScaleTextureTransform((GpTexture*)nativeBrush,
                                                             sx, sy, order));
    }

    Status RotateTransform(IN REAL angle, 
                           IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotateTextureTransform((GpTexture*)nativeBrush,
                                                              angle, order));
    }

    Status SetWrapMode(IN WrapMode wrapMode)
    {
        return SetStatus(DllExports::GdipSetTextureWrapMode((GpTexture*)nativeBrush, 
                                                            wrapMode));
    }

    WrapMode GetWrapMode() const
    {
        WrapMode wrapMode;

        SetStatus(DllExports::GdipGetTextureWrapMode((GpTexture*)nativeBrush, 
                                                     &wrapMode));
        return wrapMode;
    }

    Image *GetImage() const
    {
        GpImage *image;

        SetStatus(DllExports::GdipGetTextureImage((GpTexture *)nativeBrush,
                                                  &image));

        Image *retimage = new Image(image, lastResult);

        if (retimage == NULL) 
        {
            DllExports::GdipDisposeImage(image);
        }

        return retimage;
    }

private:
    TextureBrush(const TextureBrush &);
    TextureBrush& operator=(const TextureBrush &);

protected:

    TextureBrush()
    {
    }
};

//--------------------------------------------------------------------------
// Linear Gradient Brush Object
//--------------------------------------------------------------------------

class LinearGradientBrush : public Brush
{
public:
    friend class Pen;

    LinearGradientBrush(IN const PointF& point1,
                        IN const PointF& point2,
                        IN const Color& color1,
                        IN const Color& color2)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrush(&point1,
                                                     &point2,
                                                     color1.GetValue(),
                                                     color2.GetValue(),
                                                     WrapModeTile,
                                                     &brush);

        SetNativeBrush(brush);
    }

    LinearGradientBrush(IN const Point& point1,
                        IN const Point& point2,
                        IN const Color& color1,
                        IN const Color& color2)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrushI(&point1,
                                                      &point2,
                                                      color1.GetValue(),
                                                      color2.GetValue(),
                                                      WrapModeTile,
                                                      &brush);

        SetNativeBrush(brush);
    }

    LinearGradientBrush(IN const RectF& rect,
                        IN const Color& color1,
                        IN const Color& color2,
                        IN LinearGradientMode mode)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrushFromRect(&rect,
                                                             color1.GetValue(),
                                                             color2.GetValue(),
                                                             mode,
                                                             WrapModeTile,
                                                             &brush);

        SetNativeBrush(brush);
    }

    LinearGradientBrush(IN const Rect& rect,
                        IN const Color& color1,
                        IN const Color& color2,
                        IN LinearGradientMode mode)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrushFromRectI(&rect,
                                                              color1.GetValue(),
                                                              color2.GetValue(),
                                                              mode,
                                                              WrapModeTile,
                                                              &brush);

        SetNativeBrush(brush);
    }

    LinearGradientBrush(IN const RectF& rect,
                        IN const Color& color1,
                        IN const Color& color2,
                        IN REAL angle,
                        IN BOOL isAngleScalable = FALSE)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrushFromRectWithAngle(&rect,
                                                                      color1.GetValue(),
                                                                      color2.GetValue(),
                                                                      angle,
                                                                      isAngleScalable,
                                                                      WrapModeTile,
                                                                      &brush);

        SetNativeBrush(brush);
    }

    LinearGradientBrush(IN const Rect& rect,
                        IN const Color& color1,
                        IN const Color& color2,
                        IN REAL angle,
                        IN BOOL isAngleScalable = FALSE)
    {
        GpLineGradient *brush = NULL;

        lastResult = DllExports::GdipCreateLineBrushFromRectWithAngleI(&rect,
                                                                       color1.GetValue(),
                                                                       color2.GetValue(),
                                                                       angle,
                                                                       isAngleScalable,
                                                                       WrapModeTile,
                                                                       &brush);

        SetNativeBrush(brush);
    }

    Status SetLinearColors(IN const Color& color1, 
                           IN const Color& color2)
    {
        return SetStatus(DllExports::GdipSetLineColors((GpLineGradient*)nativeBrush,
                                                       color1.GetValue(),
                                                       color2.GetValue()));
    }

    Status GetLinearColors(OUT Color* colors) const
    {
        ARGB argb[2];

        if (colors == NULL) 
        {
            return SetStatus(InvalidParameter);
        }
        
        Status status = SetStatus(DllExports::GdipGetLineColors((GpLineGradient*) nativeBrush, argb));

        if (status == Ok)
        {
            // use bitwise copy operator for Color copy
            colors[0] = Color(argb[0]);
            colors[1] = Color(argb[1]);
        }

        return status;
    }

    Status GetRectangle(OUT RectF* rect) const
    {
        return SetStatus(DllExports::GdipGetLineRect((GpLineGradient*)nativeBrush, rect));
    }

    Status GetRectangle(OUT Rect* rect) const
    {
        return SetStatus(DllExports::GdipGetLineRectI((GpLineGradient*)nativeBrush, rect));
    }

    Status SetGammaCorrection(IN BOOL useGammaCorrection)
    {
        return SetStatus(DllExports::GdipSetLineGammaCorrection((GpLineGradient*)nativeBrush,
                    useGammaCorrection));
    }
    
    BOOL GetGammaCorrection() const
    {
        BOOL useGammaCorrection;

        SetStatus(DllExports::GdipGetLineGammaCorrection((GpLineGradient*)nativeBrush,
                    &useGammaCorrection));

        return useGammaCorrection;
    }

    INT GetBlendCount() const
    {
        INT count = 0;

        SetStatus(DllExports::GdipGetLineBlendCount((GpLineGradient*)
                                                    nativeBrush,
                                                    &count));

        return count;
    }

    Status SetBlend(IN const REAL* blendFactors, 
                    IN const REAL* blendPositions,
                    IN INT count)
    {
        return SetStatus(DllExports::GdipSetLineBlend((GpLineGradient*)
                                                      nativeBrush,
                                                      blendFactors,
                                                      blendPositions,
                                                      count));
    }

    Status GetBlend(OUT REAL* blendFactors, 
                    OUT REAL* blendPositions, 
                    IN INT count) const 
    {
        return SetStatus(DllExports::GdipGetLineBlend((GpLineGradient*)nativeBrush,
                                                      blendFactors,
                                                      blendPositions,
                                                      count));
    }

    INT GetInterpolationColorCount() const
    {
        INT count = 0;

        SetStatus(DllExports::GdipGetLinePresetBlendCount((GpLineGradient*)
                                                          nativeBrush,
                                                          &count));

        return count;
    }

    Status SetInterpolationColors(IN const Color* presetColors,
                                  IN const REAL* blendPositions, 
                                  IN INT count)
    {
        if ((count <= 0) || !presetColors)
            return SetStatus(InvalidParameter);
         
        ARGB *argbs = (ARGB*) new BYTE[count*sizeof(ARGB)];
        
        if (argbs)
        {
            for (INT i = 0; i < count; i++)
            {
                argbs[i] = presetColors[i].GetValue();
            }

            Status status = SetStatus(DllExports::GdipSetLinePresetBlend(
                                                                        (GpLineGradient*) nativeBrush,
                                                                        argbs,
                                                                        blendPositions,
                                                                        count));
            delete [] argbs;
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
            return SetStatus(InvalidParameter);

        ARGB* argbs = (ARGB*) new BYTE[count*sizeof(ARGB)];
        
        if (!argbs)
        {
            return SetStatus(OutOfMemory);
        }

        Status status = SetStatus(DllExports::GdipGetLinePresetBlend((GpLineGradient*)nativeBrush,
                                                                     argbs,
                                                                     blendPositions,
                                                                     count));
        if (status == Ok) 
        {
            for (INT i = 0; i < count; i++)
            { 
                presetColors[i] = Color(argbs[i]);
            }
        }
        
        delete [] argbs;
        
        return status;
    }

    Status SetBlendBellShape(IN REAL focus, 
                             IN REAL scale = 1.0f)
    {
        return SetStatus(DllExports::GdipSetLineSigmaBlend((GpLineGradient*)nativeBrush, focus, scale));
    }

    Status SetBlendTriangularShape(
        IN REAL focus,
        IN REAL scale = 1.0f
    )
    {
        return SetStatus(DllExports::GdipSetLineLinearBlend((GpLineGradient*)nativeBrush, focus, scale));
    }

    Status SetTransform(IN const Matrix* matrix)
    {
        return SetStatus(DllExports::GdipSetLineTransform((GpLineGradient*)nativeBrush, 
                                                          matrix->nativeMatrix));
    }

    Status GetTransform(OUT Matrix *matrix) const
    {
        return SetStatus(DllExports::GdipGetLineTransform((GpLineGradient*)nativeBrush, 
                                                          matrix->nativeMatrix));
    }

    Status ResetTransform()
    {
        return SetStatus(DllExports::GdipResetLineTransform((GpLineGradient*)nativeBrush));
    }

    Status MultiplyTransform(IN const Matrix* matrix,
                             IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipMultiplyLineTransform((GpLineGradient*)nativeBrush,
                                                                matrix->nativeMatrix,
                                                                order));
    }

    Status TranslateTransform(IN REAL dx, 
                              IN REAL dy,
                              IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipTranslateLineTransform((GpLineGradient*)nativeBrush,
                                                               dx, dy, order));
    }

    Status ScaleTransform(IN REAL sx, 
                          IN REAL sy,
                          IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipScaleLineTransform((GpLineGradient*)nativeBrush,
                                                             sx, sy, order));
    }

    Status RotateTransform(IN REAL angle, 
                           IN MatrixOrder order = MatrixOrderPrepend)
    {
        return SetStatus(DllExports::GdipRotateLineTransform((GpLineGradient*)nativeBrush,
                                                              angle, order));
    }

    Status SetWrapMode(IN WrapMode wrapMode)
    {
        return SetStatus(DllExports::GdipSetLineWrapMode((GpLineGradient*)nativeBrush, 
                                                         wrapMode));
    }

    WrapMode GetWrapMode() const
    {
        WrapMode wrapMode;

        SetStatus(DllExports::GdipGetLineWrapMode((GpLineGradient*)
                                                  nativeBrush, 
                                                  &wrapMode));

        return wrapMode;
    }

private:
    LinearGradientBrush(const LinearGradientBrush &);
    LinearGradientBrush& operator=(const LinearGradientBrush &);

protected:

    LinearGradientBrush()
    {
    }
};

//--------------------------------------------------------------------------
// PathGradientBrush object is defined
// in gdipluspath.h.
//--------------------------------------------------------------------------

//--------------------------------------------------------------------------
// Hatch Brush Object
//--------------------------------------------------------------------------

class HatchBrush : public Brush
{
public:
    friend class Pen;

    HatchBrush(IN HatchStyle hatchStyle, 
               IN const Color& foreColor,
               IN const Color& backColor = Color())
    {
        GpHatch *brush = NULL;

        lastResult = DllExports::GdipCreateHatchBrush(hatchStyle, 
                                                      foreColor.GetValue(), 
                                                      backColor.GetValue(),
                                                      &brush);
        SetNativeBrush(brush);
    }

    HatchStyle GetHatchStyle() const
    {
        HatchStyle hatchStyle;

        SetStatus(DllExports::GdipGetHatchStyle((GpHatch*)nativeBrush, 
                                                &hatchStyle));

        return hatchStyle;
    }
    
    Status GetForegroundColor(OUT Color* color) const
    {
        ARGB argb;

        if (color == NULL) 
        {
            return SetStatus(InvalidParameter);
        }
        
        Status status = SetStatus(DllExports::GdipGetHatchForegroundColor(
                                                        (GpHatch*)nativeBrush, 
                                                        &argb));

        color->SetValue(argb);

        return status;
    }

    Status GetBackgroundColor(OUT Color *color) const
    {
        ARGB argb;
        
        if (color == NULL) 
        {
            return SetStatus(InvalidParameter);
        }

        Status status = SetStatus(DllExports::GdipGetHatchBackgroundColor(
                                                        (GpHatch*)nativeBrush,
                                                        &argb));

        color->SetValue(argb);
        
        return status;
    }

private:
    HatchBrush(const HatchBrush &);
    HatchBrush& operator=(const HatchBrush &);

protected:

    HatchBrush()
    {
    }
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

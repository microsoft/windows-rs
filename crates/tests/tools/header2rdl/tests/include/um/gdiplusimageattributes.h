/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   Image Attributes
*
* Abstract:
*
*   GDI+ Image Attributes used with Graphics.DrawImage
*
* There are 5 possible sets of color adjustments:
*          ColorAdjustDefault,
*          ColorAdjustBitmap,
*          ColorAdjustBrush,
*          ColorAdjustPen,
*          ColorAdjustText,
*
* Bitmaps, Brushes, Pens, and Text will all use any color adjustments
* that have been set into the default ImageAttributes until their own
* color adjustments have been set.  So as soon as any "Set" method is
* called for Bitmaps, Brushes, Pens, or Text, then they start from
* scratch with only the color adjustments that have been set for them.
* Calling Reset removes any individual color adjustments for a type
* and makes it revert back to using all the default color adjustments
* (if any).  The SetToIdentity method is a way to force a type to
* have no color adjustments at all, regardless of what previous adjustments
* have been set for the defaults or for that type.
*
\********************************************************************F******/

#ifndef _GDIPLUSIMAGEATTRIBUTES_H
#define _GDIPLUSIMAGEATTRIBUTES_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

class GpImageAttributes;

class ImageAttributes : public GdiplusBase
{
    friend class Graphics;
    friend class TextureBrush;

public:

    ImageAttributes()
    {
        nativeImageAttr = NULL;
        lastResult = DllExports::GdipCreateImageAttributes(&nativeImageAttr);
    }

    ~ImageAttributes()
    {
        DllExports::GdipDisposeImageAttributes(nativeImageAttr);
    }

    ImageAttributes* Clone() const
    {
        GpImageAttributes* clone;

        SetStatus(DllExports::GdipCloneImageAttributes(
                                            nativeImageAttr,
                                            &clone));

        return new ImageAttributes(clone, lastResult);
    }

    Status
    SetToIdentity(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesToIdentity(
                                            nativeImageAttr,
                                            type));
    }

    Status
    Reset(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipResetImageAttributes(
                                            nativeImageAttr,
                                            type));
    }

    Status
    SetColorMatrix(
        IN const ColorMatrix *colorMatrix,
        IN ColorMatrixFlags mode = ColorMatrixFlagsDefault,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorMatrix(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            colorMatrix,
                                            NULL,
                                            mode));
    }

    Status ClearColorMatrix(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorMatrix(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            NULL,
                                            NULL,
                                            ColorMatrixFlagsDefault));
    }

    Status
    SetColorMatrices(
        IN const ColorMatrix *colorMatrix,
        IN const ColorMatrix *grayMatrix,
        IN ColorMatrixFlags mode = ColorMatrixFlagsDefault,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorMatrix(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            colorMatrix,
                                            grayMatrix,
                                            mode));
    }

    Status ClearColorMatrices(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorMatrix(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            NULL,
                                            NULL,
                                            ColorMatrixFlagsDefault));
    }

    Status SetThreshold(
        IN REAL threshold,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesThreshold(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            threshold));
    }

    Status ClearThreshold(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesThreshold(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            0.0));
    }

    Status SetGamma(
        IN REAL gamma,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesGamma(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            gamma));
    }

    Status ClearGamma(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesGamma(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            0.0));
    }

    Status SetNoOp(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesNoOp(
                                            nativeImageAttr,
                                            type,
                                            TRUE));
    }

    Status ClearNoOp(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesNoOp(
                                            nativeImageAttr,
                                            type,
                                            FALSE));
    }

    Status SetColorKey(
        IN const Color& colorLow, 
        IN const Color& colorHigh,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorKeys(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            colorLow.GetValue(),
                                            colorHigh.GetValue()));
    }

    Status ClearColorKey(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesColorKeys(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            NULL,
                                            NULL));
    }

    Status SetOutputChannel(
        IN ColorChannelFlags channelFlags,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesOutputChannel(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            channelFlags));
    }
    
    Status ClearOutputChannel(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesOutputChannel(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            ColorChannelFlagsLast));
    }

    Status SetOutputChannelColorProfile(
        IN const WCHAR *colorProfileFilename,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesOutputChannelColorProfile(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            colorProfileFilename));
    }
    
    Status ClearOutputChannelColorProfile(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesOutputChannelColorProfile(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            NULL));
    }
    
    Status SetRemapTable(
        IN UINT mapSize, 
        IN const ColorMap *map,
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesRemapTable(
                                            nativeImageAttr,
                                            type,
                                            TRUE,
                                            mapSize,
                                            map));
    }

    Status ClearRemapTable(
        IN ColorAdjustType type = ColorAdjustTypeDefault
        )
    {
        return SetStatus(DllExports::GdipSetImageAttributesRemapTable(
                                            nativeImageAttr,
                                            type,
                                            FALSE,
                                            0,
                                            NULL));
    }

    Status SetBrushRemapTable(IN UINT mapSize, 
                              IN const ColorMap *map)
    {
        return this->SetRemapTable(mapSize, map, ColorAdjustTypeBrush);
    }

    Status ClearBrushRemapTable()
    {
        return this->ClearRemapTable(ColorAdjustTypeBrush);
    }

    Status SetWrapMode(IN WrapMode wrap, 
                       IN const Color& color = Color(), 
                       IN BOOL clamp = FALSE) 
    {
        ARGB argb = color.GetValue();

        return SetStatus(DllExports::GdipSetImageAttributesWrapMode(
                           nativeImageAttr, wrap, argb, clamp));
    }

    // The flags of the palette are ignored.

    Status GetAdjustedPalette(IN OUT ColorPalette* colorPalette,
                              IN ColorAdjustType colorAdjustType) const 
    {
        return SetStatus(DllExports::GdipGetImageAttributesAdjustedPalette(
                           nativeImageAttr, colorPalette, colorAdjustType));
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastResult;
        lastResult = Ok;
    
        return lastStatus;
    }
    
private:
    ImageAttributes(const ImageAttributes &);
    ImageAttributes& operator=(const ImageAttributes &);

protected:
    ImageAttributes(GpImageAttributes* imageAttr, Status status)
    {
        SetNativeImageAttr(imageAttr);
        lastResult = status;
    }

    VOID SetNativeImageAttr(GpImageAttributes* nativeImageAttrArg)
    {
        this->nativeImageAttr = nativeImageAttrArg;
    }
    
    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else 
            return status;
    }

protected:
    GpImageAttributes* nativeImageAttr;
    mutable Status lastResult;
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

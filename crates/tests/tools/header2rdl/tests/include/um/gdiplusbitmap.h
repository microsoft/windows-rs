/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusBitmap.h
*
* Abstract:
*
*   GDI+ Bitmap class
*
\**************************************************************************/

#ifndef _GDIPLUSBITMAP_H
#define _GDIPLUSBITMAP_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma warning(push)
#pragma warning(disable: 6102 6103) // Uses SetStatus()/GetStatus() pattern rather than return codes so PREfast gets confused

inline 
Image::Image(
    IN const WCHAR* filename,
    IN BOOL useEmbeddedColorManagement
    )
{
    nativeImage = NULL;
    if(useEmbeddedColorManagement)
    {
        lastResult = DllExports::GdipLoadImageFromFileICM(
            filename, 
            &nativeImage
        );
    }
    else
    {      
        lastResult = DllExports::GdipLoadImageFromFile(
            filename, 
            &nativeImage
        );
    }
}

inline 
Image::Image(
    IN IStream* stream,
    IN BOOL useEmbeddedColorManagement
    )
{
    nativeImage = NULL;
    if(useEmbeddedColorManagement)
    {
        lastResult = DllExports::GdipLoadImageFromStreamICM(
            stream, 
            &nativeImage
        );
    }
    else
    {
        lastResult = DllExports::GdipLoadImageFromStream(
            stream, 
            &nativeImage
        );
    }
}

inline Image* 
Image::FromFile(
    IN const WCHAR* filename,
    IN BOOL useEmbeddedColorManagement
    )
{
    return new Image(
        filename, 
        useEmbeddedColorManagement
    );
}

inline Image*
Image::FromStream(
    IN IStream* stream,
    IN BOOL useEmbeddedColorManagement
    )
{
    return new Image(
        stream,
        useEmbeddedColorManagement
    );
}

inline 
Image::~Image()
{
    DllExports::GdipDisposeImage(nativeImage);
}

inline Image* 
Image::Clone() 
{
    GpImage *cloneimage = NULL;

    SetStatus(DllExports::GdipCloneImage(nativeImage, &cloneimage));

    return new Image(cloneimage, lastResult);
}

inline UINT
Image::GetEncoderParameterListSize(
    IN const CLSID* clsidEncoder
    ) 
{
    UINT size = 0;

    SetStatus(DllExports::GdipGetEncoderParameterListSize(nativeImage,
                                                          clsidEncoder,
                                                          &size));
    return size;
}

inline Status
Image::GetEncoderParameterList(
    IN const CLSID* clsidEncoder,
    IN UINT size,
    OUT EncoderParameters* buffer
    )
{
    return SetStatus(DllExports::GdipGetEncoderParameterList(nativeImage,
                                                             clsidEncoder,
                                                             size,
                                                             buffer));
}

inline Status
Image::Save(
    IN const WCHAR* filename,
    IN const CLSID* clsidEncoder,
    IN const EncoderParameters *encoderParams
    )
{
    return SetStatus(DllExports::GdipSaveImageToFile(nativeImage,
                                                     filename,
                                                     clsidEncoder,
                                                     encoderParams));
}

inline Status
Image::Save(
    IN IStream* stream,
    IN const CLSID* clsidEncoder,
    IN const EncoderParameters *encoderParams
    )
{
    return SetStatus(DllExports::GdipSaveImageToStream(nativeImage,
                                                       stream,
                                                       clsidEncoder,
                                                       encoderParams));
}

inline Status
Image::SaveAdd(
    IN const EncoderParameters *encoderParams
    )
{
    return SetStatus(DllExports::GdipSaveAdd(nativeImage,
                                             encoderParams));
}

inline Status
Image::SaveAdd(
    IN Image* newImage,
    IN const EncoderParameters *encoderParams
    )
{
    if ( newImage == NULL )
    {
        return SetStatus(InvalidParameter);
    }

    return SetStatus(DllExports::GdipSaveAddImage(nativeImage,
                                                  newImage->nativeImage,
                                                  encoderParams));
}

inline ImageType 
Image::GetType() const
{
    ImageType type = ImageTypeUnknown;

    SetStatus(DllExports::GdipGetImageType(nativeImage, &type));

    return type;
}

inline Status 
Image::GetPhysicalDimension(
    OUT SizeF* size
    ) 
{
    if (size == NULL) 
    {
        return SetStatus(InvalidParameter);
    }
    
    REAL width, height;
    Status status;

    status = SetStatus(DllExports::GdipGetImageDimension(nativeImage,
                                                         &width, &height));

    size->Width  = width;
    size->Height = height;

    return status;
}

inline Status 
Image::GetBounds(
    OUT RectF *srcRect, 
    OUT Unit *srcUnit
    )
{
    return SetStatus(DllExports::GdipGetImageBounds(nativeImage,
                                                    srcRect, srcUnit));
}

inline UINT 
Image::GetWidth()
{
    UINT width = 0;

    SetStatus(DllExports::GdipGetImageWidth(nativeImage, &width));

    return width;
}

inline UINT 
Image::GetHeight()
{
    UINT height = 0;

    SetStatus(DllExports::GdipGetImageHeight(nativeImage, &height));

    return height;
}

inline REAL 
Image::GetHorizontalResolution()
{
    REAL resolution = 0.0f;

    SetStatus(DllExports::GdipGetImageHorizontalResolution(nativeImage, &resolution));

    return resolution;
}

inline REAL 
Image::GetVerticalResolution()
{
    REAL resolution = 0.0f;

    SetStatus(DllExports::GdipGetImageVerticalResolution(nativeImage, &resolution));

    return resolution;
}

inline UINT 
Image::GetFlags()
{
    UINT flags = 0;

    SetStatus(DllExports::GdipGetImageFlags(nativeImage, &flags));

    return flags;
}

inline Status 
Image::GetRawFormat(OUT GUID *format)
{
    return SetStatus(DllExports::GdipGetImageRawFormat(nativeImage, format));
}

inline PixelFormat 
Image::GetPixelFormat()
{
    PixelFormat format;

    SetStatus(DllExports::GdipGetImagePixelFormat(nativeImage, &format));

    return format;
}

inline INT 
Image::GetPaletteSize()
{
    INT size = 0;
    
    SetStatus(DllExports::GdipGetImagePaletteSize(nativeImage, &size));
    
    return size;
}

inline Status 
Image::GetPalette(
    OUT ColorPalette *palette,
    IN INT size
)
{
    return SetStatus(DllExports::GdipGetImagePalette(nativeImage, palette, size));
}

inline Status 
Image::SetPalette(
    IN const ColorPalette *palette
    )
{
    return SetStatus(DllExports::GdipSetImagePalette(nativeImage, palette));
}

inline Image* 
Image::GetThumbnailImage(
    IN UINT thumbWidth,
    IN UINT thumbHeight,
    IN GetThumbnailImageAbort callback,
    IN VOID* callbackData
    )
{
    GpImage *thumbimage = NULL;

    SetStatus(DllExports::GdipGetImageThumbnail(nativeImage,
                                                thumbWidth, thumbHeight,
                                                &thumbimage,
                                                callback, callbackData));

    Image *newImage = new Image(thumbimage, lastResult);

    if (newImage == NULL) 
    {
        DllExports::GdipDisposeImage(thumbimage);
    }

    return newImage;
}

inline UINT 
Image::GetFrameDimensionsCount()
{
    UINT count = 0;

    SetStatus(DllExports::GdipImageGetFrameDimensionsCount(nativeImage,
                                                                  &count));

    return count;
}

inline Status 
Image::GetFrameDimensionsList(
    OUT GUID* dimensionIDs, 
    IN UINT count
    )
{
    return SetStatus(DllExports::GdipImageGetFrameDimensionsList(nativeImage,
                                                                 dimensionIDs,
                                                                 count));
}

inline UINT 
Image::GetFrameCount(
    IN const GUID* dimensionID
    )
{
    UINT count = 0;

    SetStatus(DllExports::GdipImageGetFrameCount(nativeImage,
                                                        dimensionID,
                                                        &count));
    return count;
}

inline Status 
Image::SelectActiveFrame(
    IN const GUID *dimensionID, 
    IN UINT frameIndex
    )
{
    return SetStatus(DllExports::GdipImageSelectActiveFrame(nativeImage,
                                                            dimensionID,
                                                            frameIndex));
}

inline Status
Image::RotateFlip(
    IN RotateFlipType rotateFlipType
    )
{
    return SetStatus(DllExports::GdipImageRotateFlip(nativeImage,
                                                     rotateFlipType));
}

inline UINT 
Image::GetPropertyCount()
{
    UINT numProperty = 0;

    SetStatus(DllExports::GdipGetPropertyCount(nativeImage,
                                               &numProperty));

    return numProperty;
}

inline Status 
Image::GetPropertyIdList(
    IN UINT numOfProperty, 
    OUT PROPID* list
    )
{
    return SetStatus(DllExports::GdipGetPropertyIdList(nativeImage,
                                                       numOfProperty, list));
}
    
inline UINT 
Image::GetPropertyItemSize(
    IN PROPID propId
    )
{
    UINT size = 0;

    SetStatus(DllExports::GdipGetPropertyItemSize(nativeImage,
                                                  propId,
                                                  &size));

    return size;
}

inline Status 
Image::GetPropertyItem(
    IN PROPID propId, 
    IN UINT propSize,
    OUT PropertyItem* buffer
    )
{
    return SetStatus(DllExports::GdipGetPropertyItem(nativeImage,
                                                     propId, propSize, buffer));
}

inline Status 
Image::GetPropertySize(
    OUT UINT* totalBufferSize, 
    OUT UINT* numProperties
    )
{
    return SetStatus(DllExports::GdipGetPropertySize(nativeImage,
                                                     totalBufferSize,
                                                     numProperties));
}

inline Status 
Image::GetAllPropertyItems(
    IN UINT totalBufferSize,
    IN UINT numProperties,
    OUT PropertyItem* allItems
    )
{
    if (allItems == NULL) 
    {
        return SetStatus(InvalidParameter);
    }
    return SetStatus(DllExports::GdipGetAllPropertyItems(nativeImage,
                                                         totalBufferSize,
                                                         numProperties,
                                                         allItems));
}

inline Status 
Image::RemovePropertyItem(
    IN PROPID propId
    )
{
    return SetStatus(DllExports::GdipRemovePropertyItem(nativeImage, propId));
}

inline Status 
Image::SetPropertyItem(
    IN const PropertyItem* item
    )
{
    return SetStatus(DllExports::GdipSetPropertyItem(nativeImage, item));
}

#if (GDIPVER >= 0x0110)
inline Status 
Image::FindFirstItem(
    IN ImageItemData* item
    )
{
    return SetStatus(DllExports::GdipFindFirstImageItem(nativeImage, item));
}

inline Status 
Image::FindNextItem(
    IN ImageItemData* item
    )
{
    return SetStatus(DllExports::GdipFindNextImageItem(nativeImage, item));
}

inline Status 
Image::GetItemData(
    IN ImageItemData* item
    )
{
    return SetStatus(DllExports::GdipGetImageItemData(nativeImage, item));
}
#endif //(GDIPVER >= 0x0110)

inline Status
Image::GetLastStatus() const
{
    Status lastStatus = lastResult;
    lastResult = Ok;

    return lastStatus;
}

inline 
Image::Image(GpImage *nativeImage, Status status)
{
    SetNativeImage(nativeImage);
    lastResult = status;
}

inline VOID 
Image::SetNativeImage(GpImage *newNativeImage)
{
    this->nativeImage = newNativeImage;
}

inline 
Bitmap::Bitmap(
    IN const WCHAR *filename, 
    IN BOOL useEmbeddedColorManagement
    )
{
    GpBitmap *bitmap = NULL;

    if(useEmbeddedColorManagement) 
    {
        lastResult = DllExports::GdipCreateBitmapFromFileICM(filename, &bitmap);
    }
    else
    {
        lastResult = DllExports::GdipCreateBitmapFromFile(filename, &bitmap);
    }

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN IStream *stream, 
    IN BOOL useEmbeddedColorManagement
    )
{
    GpBitmap *bitmap = NULL;

    if(useEmbeddedColorManagement)
    {
        lastResult = DllExports::GdipCreateBitmapFromStreamICM(stream, &bitmap);
    }
    else
    {
        lastResult = DllExports::GdipCreateBitmapFromStream(stream, &bitmap);
    }

    SetNativeImage(bitmap);
}

inline
Bitmap::Bitmap(
    _In_ INT width,
    _In_ INT height,
    _In_ INT stride,
    _In_ PixelFormat format,
    _In_reads_opt_(_Inexpressible_("height * stride")) BYTE* scan0
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromScan0(width,
                                                       height,
                                                       stride,
                                                       format,
                                                       scan0,
                                                       &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN INT width,
    IN INT height,
    IN PixelFormat format
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromScan0(width,
                                                       height,
                                                       0,
                                                       format,
                                                       NULL,
                                                       &bitmap);

    SetNativeImage(bitmap);
}

inline
Bitmap::Bitmap(
    IN INT width, 
    IN INT height, 
    IN Graphics* target)
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromGraphics(width,
                                                          height,
                                                          target->nativeGraphics,
                                                          &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN IDirectDrawSurface7 * surface
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromDirectDrawSurface(surface,
                                                       &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN const BITMAPINFO* gdiBitmapInfo, 
    IN VOID* gdiBitmapData
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromGdiDib(gdiBitmapInfo,
                                                        gdiBitmapData,
                                                        &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN HBITMAP hbm, 
    IN HPALETTE hpal
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromHBITMAP(hbm, hpal, &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN HICON hicon
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromHICON(hicon, &bitmap);

    SetNativeImage(bitmap);
}

inline 
Bitmap::Bitmap(
    IN HINSTANCE hInstance, 
    IN const WCHAR *bitmapName
    )
{
    GpBitmap *bitmap = NULL;

    lastResult = DllExports::GdipCreateBitmapFromResource(hInstance,
                                                          bitmapName,
                                                          &bitmap);

    SetNativeImage(bitmap);
}


inline Bitmap* 
Bitmap::FromFile(
    IN const WCHAR *filename,
    IN BOOL useEmbeddedColorManagement
    )
{
    return new Bitmap(
        filename, 
        useEmbeddedColorManagement
    );
}

inline Bitmap* 
Bitmap::FromStream(
    IN IStream *stream,
    IN BOOL useEmbeddedColorManagement
    )
{
    return new Bitmap(
        stream, 
        useEmbeddedColorManagement
    );
}

inline Bitmap* 
Bitmap::FromDirectDrawSurface7(
    IN IDirectDrawSurface7* surface
    )
{
    return new Bitmap(surface);
}

inline Bitmap* 
Bitmap::FromBITMAPINFO(
    IN const BITMAPINFO* gdiBitmapInfo, 
    IN VOID* gdiBitmapData)
{
    return new Bitmap(gdiBitmapInfo, gdiBitmapData);
}

inline Bitmap* 
Bitmap::FromHBITMAP(
    IN HBITMAP hbm, 
    IN HPALETTE hpal
    )
{
    return new Bitmap(hbm, hpal);
}

inline Bitmap* 
Bitmap::FromHICON(
    IN HICON hicon
    )
{
    return new Bitmap(hicon);
}

inline Bitmap* 
Bitmap::FromResource(
    IN HINSTANCE hInstance, 
    IN const WCHAR *bitmapName)
{
    return new Bitmap(hInstance, bitmapName);
}

inline Status 
Bitmap::GetHBITMAP(
    IN const Color& colorBackground,
    OUT HBITMAP* hbmReturn
    )
{
    return SetStatus(DllExports::GdipCreateHBITMAPFromBitmap(
                                        static_cast<GpBitmap*>(nativeImage),
                                        hbmReturn,
                                        colorBackground.GetValue()));
}

inline Status 
Bitmap::GetHICON(
    OUT HICON* hiconReturn
    )
{
    return SetStatus(DllExports::GdipCreateHICONFromBitmap(
                                        static_cast<GpBitmap*>(nativeImage),
                                        hiconReturn));
}

inline Bitmap* 
Bitmap::Clone(
    IN const Rect& rect,
    IN PixelFormat format
    )
{
    return Clone(rect.X, rect.Y, rect.Width, rect.Height, format);
}

inline Bitmap* 
Bitmap::Clone(
    IN INT x,
    IN INT y,
    IN INT width,
    IN INT height,
    IN PixelFormat format
    )
{
   GpBitmap* gpdstBitmap = NULL;
   Bitmap* bitmap;

   if (SetStatus(DllExports::GdipCloneBitmapAreaI(
                               x,
                               y,
                               width,
                               height,
                               format,
                               (GpBitmap *)nativeImage,
                               &gpdstBitmap)) == Ok)
   {
       bitmap = new Bitmap(gpdstBitmap);

       if (bitmap == NULL) 
       {
           DllExports::GdipDisposeImage(gpdstBitmap);
       }

       return bitmap;
   }
   else
       return NULL;
}

inline Bitmap* 
Bitmap::Clone(
    IN const RectF& rect,
    IN PixelFormat format
    )
{
    return Clone(rect.X, rect.Y, rect.Width, rect.Height, format);
}

inline Bitmap*
Bitmap::Clone(
    IN REAL x,
    IN REAL y,
    IN REAL width,
    IN REAL height,
    IN PixelFormat format
    )
{
   GpBitmap* gpdstBitmap = NULL;
   Bitmap* bitmap;

   if (SetStatus(DllExports::GdipCloneBitmapArea(
                               x,
                               y,
                               width,
                               height,
                               format,
                               (GpBitmap *)nativeImage,
                               &gpdstBitmap)) == Ok)
   {
       bitmap = new Bitmap(gpdstBitmap);

       if (bitmap == NULL) 
       {
           DllExports::GdipDisposeImage(gpdstBitmap);
       }

       return bitmap;
   }
   else
       return NULL;
}

inline Bitmap::Bitmap(GpBitmap *nativeBitmap)
{
    lastResult = Ok;

    SetNativeImage(nativeBitmap);
}

inline Status
Bitmap::LockBits(
    IN const Rect* rect,
    IN UINT flags,
    IN PixelFormat format,
    OUT BitmapData* lockedBitmapData
)
{
    return SetStatus(DllExports::GdipBitmapLockBits(
                                    static_cast<GpBitmap*>(nativeImage),
                                    rect,
                                    flags,
                                    format,
                                    lockedBitmapData));
}

inline Status 
Bitmap::UnlockBits(
    IN BitmapData* lockedBitmapData
    )
{
    return SetStatus(DllExports::GdipBitmapUnlockBits(
                                    static_cast<GpBitmap*>(nativeImage),
                                    lockedBitmapData));
}

inline Status 
Bitmap::GetPixel(
    IN INT x, 
    IN INT y, 
    OUT Color *color) 
{
    ARGB argb;

    Status status = SetStatus(DllExports::GdipBitmapGetPixel(
        static_cast<GpBitmap *>(nativeImage),
        x, y,        
        &argb));

    if (status == Ok) 
    {
        color->SetValue(argb);
    }

    return  status;
}

inline Status 
Bitmap::SetPixel(
    IN INT x, 
    IN INT y, 
    IN const Color& color) 
{
    return SetStatus(DllExports::GdipBitmapSetPixel(
        static_cast<GpBitmap *>(nativeImage),
        x, y,
        color.GetValue()));
}

#if (GDIPVER >= 0x0110)
inline Status Image::SetAbort(GdiplusAbort *pIAbort)
{
    return SetStatus(DllExports::GdipImageSetAbort(
        nativeImage,
        pIAbort
        ));
}

inline Status
Bitmap::ConvertFormat(
    PixelFormat format,
    DitherType dithertype,
    PaletteType palettetype,
    ColorPalette *palette,
    REAL alphaThresholdPercent
    )
{
    return SetStatus(DllExports::GdipBitmapConvertFormat(
        static_cast<GpBitmap*>(nativeImage),
        format,
        dithertype,
        palettetype,
        palette,
        alphaThresholdPercent
        ));
}

inline Status 
Bitmap::InitializePalette(
    OUT ColorPalette *palette,   // output palette. must be allocated.
    PaletteType palettetype,     // palette enumeration type.
    INT optimalColors,           // how many optimal colors
    BOOL useTransparentColor,    // add a transparent color to the palette.
    Bitmap *bitmap               // optional bitmap for median cut.
    )
{
    return DllExports::GdipInitializePalette(
        palette,
        palettetype,
        optimalColors,
        useTransparentColor,
        bitmap ? static_cast<GpBitmap*>(bitmap->nativeImage) : NULL
        );
}


inline Status 
Bitmap::ApplyEffect(
    IN  Bitmap **inputs,
    IN  INT numInputs,
    IN  Effect *effect, 
    IN  RECT *ROI,                     // optional parameter.
    OUT RECT *outputRect,              // optional parameter.
    OUT Bitmap **output
)
{
    if (numInputs < 0)
    {
        return InvalidParameter;
    }
    
    GpBitmap *outputNative = NULL;
    GpBitmap **nativeInputs = new GpBitmap * [numInputs];
    
    if (NULL == nativeInputs)
    {
        return OutOfMemory;
    }
    
    for (int i = 0; i < numInputs; i++)
    {
        nativeInputs[i] = static_cast<GpBitmap*>(inputs[i]->nativeImage);
    }
    
    if (effect->auxData)
    {
        DllExports::GdipFree(effect->auxData);
        effect->auxData = NULL;
        effect->auxDataSize = 0;
    }
    
    Status status = DllExports::GdipBitmapCreateApplyEffect(
        nativeInputs,
        numInputs,
        effect->nativeEffect,
        ROI,
        outputRect,
        &outputNative,
        effect->useAuxData,
        &effect->auxData,
        &effect->auxDataSize
        );
    
    if ((Ok == status) && outputNative)
    {
        *output = new Bitmap(outputNative);
        
        if (NULL == *output)
        {
            status = OutOfMemory;
            DllExports::GdipDisposeImage(outputNative);
        }
    }
    else
    {
        *output = NULL;
    }
    
    delete [] nativeInputs;
    
    return status;
}


inline Status
Bitmap::ApplyEffect(
    Effect *effect,
    RECT *ROI
)
{
    if(effect->auxData)
    {
        DllExports::GdipFree(effect->auxData);
        effect->auxData = NULL;
        effect->auxDataSize = 0;
    }
    
    return DllExports::GdipBitmapApplyEffect(
        static_cast<GpBitmap *>(nativeImage),
        effect->nativeEffect,
        ROI,
        effect->useAuxData,
        &effect->auxData,
        &effect->auxDataSize
        );
}

inline Status
Bitmap::GetHistogram(
    IN HistogramFormat format,
    IN UINT NumberOfEntries,
    _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel0,
    _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel1,
    _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel2,
    _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel3
)
{
    return DllExports::GdipBitmapGetHistogram(
        static_cast<GpBitmap *>(nativeImage),
        format,
        NumberOfEntries,
        channel0,
        channel1,
        channel2,
        channel3
        );
}

inline Status 
Bitmap::GetHistogramSize(
    IN HistogramFormat format,
    OUT UINT *NumberOfEntries
)
{
    return DllExports::GdipBitmapGetHistogramSize(
        format,
        NumberOfEntries
        );
}

#endif // (GDIPVER >= 0x0110)


inline Status 
Bitmap::SetResolution(
    IN REAL xdpi, 
    IN REAL ydpi)
{
    return SetStatus(DllExports::GdipBitmapSetResolution(
        static_cast<GpBitmap *>(nativeImage),
        xdpi, ydpi));
}

#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

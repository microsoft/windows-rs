/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusHeaders.h
*
* Abstract:
*
*   GDI+ Region, Font, Image, CustomLineCap class definitions.
*
*
* Class definition and inline class implementation are separated into
* different files to avoid circular dependencies.
*
\**************************************************************************/

#ifndef _GDIPLUSHEADERS_H
#define _GDIPLUSHEADERS_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4263) /* member function does not override any base class virtual member function */
#pragma warning(disable:4264) /* no override available for virtual member function from base 'class'; function is hidden */
#pragma warning(disable:4820) /* padding added after data member */
#endif

class Region : public GdiplusBase
{
public:
    friend class Graphics;

    Region();
    Region(IN const RectF& rect);
    Region(IN const Rect& rect);
    Region(IN const GraphicsPath* path);
    Region(IN const BYTE* regionData, IN INT size);
    Region(IN HRGN hRgn);
    static Region* FromHRGN(IN HRGN hRgn);

    ~Region();
    Region* Clone() const;

    Status MakeInfinite();
    Status MakeEmpty();

    UINT GetDataSize() const;

    // buffer     - where to put the data
    // bufferSize - how big the buffer is (should be at least as big as GetDataSize())
    // sizeFilled - if not NULL, this is an OUT param that says how many bytes
    //              of data were written to the buffer.

    Status GetData(_Out_writes_bytes_to_(bufferSize, *sizeFilled) BYTE* buffer,
                   _In_ UINT bufferSize,
                   _Out_opt_ UINT* sizeFilled = NULL) const;

    Status Intersect(IN const Rect& rect);
    Status Intersect(IN const RectF& rect);
    Status Intersect(IN const GraphicsPath* path);
    Status Intersect(IN const Region* region);
    Status Union(IN const Rect& rect);
    Status Union(IN const RectF& rect);
    Status Union(IN const GraphicsPath* path);
    Status Union(IN const Region* region);
    Status Xor(IN const Rect& rect);
    Status Xor(IN const RectF& rect);
    Status Xor(IN const GraphicsPath* path);
    Status Xor(IN const Region* region);
    Status Exclude(IN const Rect& rect);
    Status Exclude(IN const RectF& rect);
    Status Exclude(IN const GraphicsPath* path);
    Status Exclude(IN const Region* region);
    Status Complement(IN const Rect& rect);
    Status Complement(IN const RectF& rect);
    Status Complement(IN const GraphicsPath* path);
    Status Complement(IN const Region* region);
    Status Translate(IN REAL dx,
                     IN REAL dy);
    Status Translate(IN INT dx,
                     IN INT dy);
    Status Transform(IN const Matrix* matrix);

    Status GetBounds(OUT Rect* rect,
                     IN const Graphics* g) const;

    Status GetBounds(OUT RectF* rect,
                     IN const Graphics* g) const;

    HRGN   GetHRGN  (IN const Graphics * g) const;

    BOOL IsEmpty(IN const Graphics *g) const;
    BOOL IsInfinite(IN const Graphics *g) const;

    BOOL IsVisible(IN INT x,
                   IN INT y,
                   IN const Graphics* g = NULL) const
    {
        return IsVisible(Point(x, y), g);
    }

    BOOL IsVisible(IN const Point& point,
                   IN const Graphics* g = NULL) const;

    BOOL IsVisible(IN REAL x,
                   IN REAL y,
                   IN const Graphics* g = NULL) const
    {
        return IsVisible(PointF(x, y), g);
    }

    BOOL IsVisible(IN const PointF& point,
                   IN const Graphics* g = NULL) const;

    BOOL IsVisible(IN INT x,
                   IN INT y,
                   IN INT width,
                   IN INT height,
                   IN const Graphics* g) const
    {
        return IsVisible(Rect(x, y, width, height), g);
    }

    BOOL IsVisible(IN const Rect& rect,
                   IN const Graphics* g = NULL) const;

    BOOL IsVisible(IN REAL x,
                   IN REAL y,
                   IN REAL width,
                   IN REAL height,
                   IN const Graphics* g = NULL) const
    {
        return IsVisible(RectF(x, y, width, height), g);
    }

    BOOL IsVisible(IN const RectF& rect,
                   IN const Graphics* g = NULL) const;

    BOOL Equals(IN const Region* region,
                IN const Graphics* g) const;

    UINT GetRegionScansCount(IN const Matrix* matrix) const;
    Status GetRegionScans(IN const Matrix* matrix,
                          OUT RectF* rects,
                          OUT INT* count) const;
    Status GetRegionScans(IN const Matrix* matrix,
                          OUT Rect*  rects,
                          OUT INT* count) const;
    Status GetLastStatus() const;

protected:

private:
    Region(const Region &region);
    Region& operator=(const Region &region);
protected:

    _Post_equal_to_(status)
    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

    Region(GpRegion* nativeRegion);

    VOID SetNativeRegion(GpRegion* nativeRegion);

protected:
    GpRegion* nativeRegion;
    mutable Status lastResult;
};


//--------------------------------------------------------------------------
// FontFamily
//--------------------------------------------------------------------------

class FontFamily : public GdiplusBase
{
public:
    friend class Font;
    friend class Graphics;
    friend class GraphicsPath;
    friend class FontCollection;

    FontFamily();

    FontFamily(
        IN const WCHAR          *name,
        IN const FontCollection *fontCollection = NULL
    );

    ~FontFamily();

    static const FontFamily *GenericSansSerif();
    static const FontFamily *GenericSerif();
    static const FontFamily *GenericMonospace();

    Status GetFamilyName(
        _Out_writes_(LF_FACESIZE) LPWSTR    name,
        IN LANGID                           language = 0
    ) const;

    FontFamily * Clone() const;

    BOOL    IsAvailable() const
    {
        return (nativeFamily != NULL);
    };

    BOOL    IsStyleAvailable(IN INT style) const;

    UINT16  GetEmHeight     (IN INT style) const;
    UINT16  GetCellAscent   (IN INT style) const;
    UINT16  GetCellDescent  (IN INT style) const;
    UINT16  GetLineSpacing  (IN INT style) const;
    
    Status GetLastStatus() const;

private:
    FontFamily(const FontFamily &);
    FontFamily& operator=(const FontFamily &);

protected:
    _Post_equal_to_(status) Status SetStatus(Status status) const;

    FontFamily(GpFontFamily * nativeFamily, Status status);

protected:

    GpFontFamily    *nativeFamily;
    mutable Status   lastResult;
};

static FontFamily *GenericSansSerifFontFamily = NULL;
static FontFamily *GenericSerifFontFamily     = NULL;
static FontFamily *GenericMonospaceFontFamily = NULL;

static BYTE GenericSansSerifFontFamilyBuffer[sizeof(FontFamily)] = {0};
static BYTE GenericSerifFontFamilyBuffer    [sizeof(FontFamily)] = {0};
static BYTE GenericMonospaceFontFamilyBuffer[sizeof(FontFamily)] = {0};


//--------------------------------------------------------------------------
// Font
//--------------------------------------------------------------------------

class Font : public GdiplusBase
{
public:
    friend class Graphics;

    Font(IN HDC hdc);
    Font(IN HDC hdc,
         IN const LOGFONTA* logfont);
    Font(IN HDC hdc,
         IN const LOGFONTW* logfont);
    Font(IN HDC hdc,
         IN const HFONT hfont);
    Font(
        IN const FontFamily * family,
        IN REAL         emSize,
        IN INT          style   = FontStyleRegular,
        IN Unit         unit    = UnitPoint
    );

    Font(
        IN const WCHAR *           familyName,
        IN REAL                    emSize,
        IN INT                     style   = FontStyleRegular,
        IN Unit                    unit    = UnitPoint,
        IN const FontCollection *  fontCollection = NULL
    );

    Status GetLogFontA(IN const Graphics* g,
                       OUT  LOGFONTA * logfontA) const;
    Status GetLogFontW(IN const Graphics* g,
                       OUT LOGFONTW * logfontW) const;

    Font* Clone() const;
    ~Font();
    BOOL        IsAvailable()   const;
    INT         GetStyle()      const;
    REAL        GetSize()       const;
    Unit        GetUnit()       const;
    Status      GetLastStatus() const;
    REAL        GetHeight(IN const Graphics *graphics) const;
    REAL        GetHeight(IN REAL dpi) const;

    Status GetFamily(OUT FontFamily *family) const;

private:
    Font(const Font &);
    Font& operator=(const Font &);

protected:
    Font(GpFont* font, Status status);
    VOID SetNativeFont(GpFont *Font);
    _Post_equal_to_(status) Status SetStatus(Status status) const;

protected:

    GpFont* nativeFont;
    mutable Status lastResult;
};

//--------------------------------------------------------------------------
// Font Collection
//--------------------------------------------------------------------------

class FontCollection : public GdiplusBase
{
public:
    friend class FontFamily;

    FontCollection();
    virtual ~FontCollection();

    INT GetFamilyCount() const;

    Status GetFamilies(
        _In_                                  INT          numSought,
        _Out_writes_to_(numSought, *numFound) FontFamily * gpfamilies,
        _Out_                                 INT        * numFound
    ) const;

    Status GetLastStatus() const;

private:
    FontCollection(const FontCollection &);
    FontCollection& operator=(const FontCollection &);

protected:
    _Post_equal_to_(status) Status SetStatus(Status status) const ;

    GpFontCollection *nativeFontCollection;
    mutable Status    lastResult;
};


class InstalledFontCollection : public FontCollection
{
public:
    InstalledFontCollection();
    ~InstalledFontCollection();

private:
    InstalledFontCollection(const InstalledFontCollection &);
    InstalledFontCollection& operator=(const InstalledFontCollection &);

protected:
    _Post_equal_to_(status) Status SetStatus(Status status) const ;
};


class PrivateFontCollection : public FontCollection
{
public:
    PrivateFontCollection();
    ~PrivateFontCollection();

    Status AddFontFile(IN const WCHAR* filename);
    Status AddMemoryFont(IN const VOID* memory,
                         IN INT length);

private:
    PrivateFontCollection(const PrivateFontCollection &);
    PrivateFontCollection& operator=(const PrivateFontCollection &);
};


//--------------------------------------------------------------------------
// Abstract base class for Image and Metafile
//--------------------------------------------------------------------------

class Image : public GdiplusBase
{
public:
    friend class Brush;
    friend class TextureBrush;
    friend class Graphics;

    Image(
        IN const WCHAR* filename,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    Image(
        IN IStream* stream,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    static Image* FromFile(
        IN const WCHAR* filename,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    static Image* FromStream(
        IN IStream* stream,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    virtual ~Image();
    virtual Image* Clone();

    Status Save(IN const WCHAR* filename,
                IN const CLSID* clsidEncoder,
                IN const EncoderParameters *encoderParams = NULL);
    Status Save(IN IStream* stream,
                IN const CLSID* clsidEncoder,
                IN const EncoderParameters *encoderParams = NULL);
    Status SaveAdd(IN const EncoderParameters* encoderParams);
    Status SaveAdd(IN Image* newImage,
                   IN const EncoderParameters* encoderParams);

    ImageType GetType() const;
    Status GetPhysicalDimension(OUT SizeF* size);
    Status GetBounds(OUT RectF* srcRect,
                     OUT Unit* srcUnit);

    UINT GetWidth();
    UINT GetHeight();
    REAL GetHorizontalResolution();
    REAL GetVerticalResolution();
    UINT GetFlags();
    Status GetRawFormat(OUT GUID *format);
    PixelFormat GetPixelFormat();

    INT GetPaletteSize();
    Status GetPalette(OUT ColorPalette* palette,
                      IN INT size);
    Status SetPalette(IN const ColorPalette* palette);

    Image* GetThumbnailImage(IN UINT thumbWidth,
                             IN UINT thumbHeight,
                             IN GetThumbnailImageAbort callback = NULL,
                             IN VOID* callbackData = NULL);
    UINT GetFrameDimensionsCount();
    Status GetFrameDimensionsList(OUT GUID* dimensionIDs,
                                  IN UINT count);
    UINT GetFrameCount(IN const GUID* dimensionID);
    Status SelectActiveFrame(IN const GUID* dimensionID,
                             IN UINT frameIndex);
    Status RotateFlip(IN RotateFlipType rotateFlipType);
    UINT GetPropertyCount();
    Status GetPropertyIdList(IN UINT numOfProperty,
                             OUT PROPID* list);
    UINT GetPropertyItemSize(IN PROPID propId);
    Status GetPropertyItem(IN PROPID propId,
                           IN UINT propSize,
                           OUT PropertyItem* buffer);
    Status GetPropertySize(OUT UINT* totalBufferSize,
                           OUT UINT* numProperties);
    Status GetAllPropertyItems(IN UINT totalBufferSize,
                               IN UINT numProperties,
                               OUT PropertyItem* allItems);
    Status RemovePropertyItem(IN PROPID propId);
    Status SetPropertyItem(IN const PropertyItem* item);

    UINT  GetEncoderParameterListSize(IN const CLSID* clsidEncoder);
    Status GetEncoderParameterList(IN const CLSID* clsidEncoder,
                                   IN UINT size,
                                   OUT EncoderParameters* buffer);
#if (GDIPVER >= 0x0110)
    Status FindFirstItem(IN ImageItemData *item);
    Status FindNextItem(IN ImageItemData *item);
    Status GetItemData(IN ImageItemData *item);
    Status SetAbort(GdiplusAbort *pIAbort);
#endif //(GDIPVER >= 0x0110)

    Status GetLastStatus() const;

protected:

    Image() {}

    Image(GpImage *nativeImage, Status status);

    VOID SetNativeImage(GpImage* nativeImage);

    _Post_equal_to_(status)
    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

    GpImage* nativeImage;
    mutable Status lastResult;
    mutable Status loadStatus;

private:
    Image(IN const Image& C);
    Image& operator=(IN const Image& C);
};

class Bitmap : public Image
{
public:
    friend class Image;
    friend class CachedBitmap;

    Bitmap(
        IN const WCHAR *filename,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    Bitmap(
        IN IStream *stream,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    static Bitmap* FromFile(
        IN const WCHAR *filename,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    static Bitmap* FromStream(
        IN IStream *stream,
        IN BOOL useEmbeddedColorManagement = FALSE
    );

    Bitmap(_In_ INT width,
           _In_ INT height,
           _In_ INT stride,
           _In_ PixelFormat format,
           _In_reads_opt_(_Inexpressible_("height * stride")) BYTE* scan0);
    Bitmap(IN INT width,
           IN INT height,
           IN PixelFormat format = PixelFormat32bppARGB);
    Bitmap(IN INT width,
           IN INT height,
           IN  Graphics* target);

    Bitmap* Clone(IN const Rect& rect,
                  IN PixelFormat format);
    Bitmap* Clone(IN INT x,
                  IN INT y,
                  IN INT width,
                  IN INT height,
                  IN PixelFormat format);
    Bitmap* Clone(IN const RectF& rect,
                  IN PixelFormat format);
    Bitmap* Clone(IN REAL x,
                  IN REAL y,
                  IN REAL width,
                  IN REAL height,
                  IN PixelFormat format);

    Status LockBits(IN const Rect* rect,
                    IN UINT flags,
                    IN PixelFormat format,
                    OUT BitmapData* lockedBitmapData);
    Status UnlockBits(IN BitmapData* lockedBitmapData);
    Status GetPixel(IN INT x,
                    IN INT y,
                    OUT Color *color);
    Status SetPixel(IN INT x,
                    IN INT y,
                    IN const Color &color);
    
#if (GDIPVER >= 0x0110)
    Status ConvertFormat(
        PixelFormat format,
        DitherType dithertype,
        PaletteType palettetype,
        ColorPalette *palette,
        REAL alphaThresholdPercent
        );
    
    // The palette must be allocated and count must be set to the number of
    // entries in the palette. If there are not enough, the API will fail.
    
    static Status InitializePalette(
        IN OUT ColorPalette *palette,  // Palette to initialize.
        PaletteType palettetype,       // palette enumeration type.
        INT optimalColors,             // how many optimal colors
        BOOL useTransparentColor,      // add a transparent color to the palette.
        Bitmap *bitmap                 // optional bitmap for median cut.
        );
        
    Status ApplyEffect(Effect *effect, RECT *ROI);
    
    static Status 
    ApplyEffect(
        IN  Bitmap **inputs,
        IN  INT numInputs,
        IN  Effect *effect, 
        IN  RECT *ROI,           // optional parameter.
        OUT RECT *outputRect,    // optional parameter.
        OUT Bitmap **output
    );
    
    Status GetHistogram(
        IN HistogramFormat format,
        IN UINT NumberOfEntries,
        _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel0,
        _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel1,
        _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel2,
        _Out_writes_bytes_(sizeof(UINT)*256) UINT *channel3
    );
    
    static Status GetHistogramSize(
        IN HistogramFormat format,
        OUT UINT *NumberOfEntries
    );
#endif //(GDIPVER >= 0x0110)
    
    Status SetResolution(IN REAL xdpi,
                         IN REAL ydpi);

    Bitmap(IN IDirectDrawSurface7* surface);
    Bitmap(IN const BITMAPINFO* gdiBitmapInfo,
           IN VOID* gdiBitmapData);
    Bitmap(IN HBITMAP hbm,
           IN HPALETTE hpal);
    Bitmap(IN HICON hicon);
    Bitmap(IN HINSTANCE hInstance,
           IN const WCHAR * bitmapName);
    static Bitmap* FromDirectDrawSurface7(IN IDirectDrawSurface7* surface);
    static Bitmap* FromBITMAPINFO(IN const BITMAPINFO* gdiBitmapInfo,
                                  IN VOID* gdiBitmapData);
    static Bitmap* FromHBITMAP(IN HBITMAP hbm,
                               IN HPALETTE hpal);
    static Bitmap* FromHICON(IN HICON hicon);
    static Bitmap* FromResource(IN HINSTANCE hInstance,
                                IN const WCHAR * bitmapName);

    Status GetHBITMAP(IN const Color& colorBackground,
                      OUT HBITMAP *hbmReturn);
    Status GetHICON(HICON *hicon);

private:
    Bitmap(const Bitmap &);
    Bitmap& operator=(const Bitmap &);

protected:
    Bitmap(GpBitmap *nativeBitmap);
};

class CustomLineCap : public GdiplusBase
{
public:
    friend class Pen;

    CustomLineCap(
        IN const GraphicsPath* fillPath,
        IN const GraphicsPath* strokePath,
        IN LineCap baseCap = LineCapFlat,
        IN REAL baseInset = 0
        );
    virtual ~CustomLineCap();

    CustomLineCap* Clone() const;

    // This changes both the start and end cap.

    Status SetStrokeCap(IN LineCap strokeCap)
    {
        return SetStrokeCaps(strokeCap, strokeCap);
    }

    Status SetStrokeCaps(IN LineCap startCap,
                         IN LineCap endCap);
    Status GetStrokeCaps(OUT LineCap* startCap,
                         OUT LineCap* endCap) const;
    Status SetStrokeJoin(IN LineJoin lineJoin);
    LineJoin GetStrokeJoin() const;
    Status SetBaseCap(IN LineCap baseCap);
    LineCap GetBaseCap() const;
    Status SetBaseInset(IN REAL inset);
    REAL GetBaseInset() const;
    Status SetWidthScale(IN REAL widthScale);
    REAL GetWidthScale() const;
    Status GetLastStatus() const;

protected:
    CustomLineCap();

private:
    CustomLineCap(const CustomLineCap &);
    CustomLineCap& operator=(const CustomLineCap &);

protected:
    CustomLineCap(GpCustomLineCap* nativeCapArg, Status status)
    {
        lastResult = status;
        SetNativeCap(nativeCapArg);
    }

    VOID SetNativeCap(GpCustomLineCap* nativeCapArg)
    {
        this->nativeCap = nativeCapArg;
    }

    _Post_equal_to_(status)
    Status SetStatus(Status status) const
    {
        if (status != Ok)
            return (lastResult = status);
        else
            return status;
    }

protected:
    GpCustomLineCap* nativeCap;
    mutable Status lastResult;
};

class CachedBitmap : public GdiplusBase
{
    friend Graphics;

public:
    CachedBitmap(IN Bitmap *bitmap,
                 IN Graphics *graphics);
    virtual ~CachedBitmap();

    Status GetLastStatus() const;

private:
    CachedBitmap(const CachedBitmap &);
    CachedBitmap& operator=(const CachedBitmap &);

protected:
    GpCachedBitmap *nativeCachedBitmap;
    mutable Status lastResult;
};

class Metafile : public Image
{
public:
    friend class Image;

    // Playback a metafile from a HMETAFILE
    // If deleteWmf is TRUE, then when the metafile is deleted,
    // the hWmf will also be deleted.  Otherwise, it won't be.
    
    Metafile(
        IN HMETAFILE                      hWmf,
        IN const WmfPlaceableFileHeader * wmfPlaceableFileHeader,
        IN BOOL                           deleteWmf = FALSE
        );

    // Playback a metafile from a HENHMETAFILE
    // If deleteEmf is TRUE, then when the metafile is deleted,
    // the hEmf will also be deleted.  Otherwise, it won't be.
    
    Metafile(
        IN HENHMETAFILE hEmf,
        IN BOOL deleteEmf = FALSE
        );
    
    Metafile(IN const WCHAR* filename);

    // Playback a WMF metafile from a file.

    Metafile(
        IN const WCHAR*                   filename,
        IN const WmfPlaceableFileHeader * wmfPlaceableFileHeader
        );

    Metafile(IN IStream* stream);

    // Record a metafile to memory.

    Metafile(
        IN HDC                 referenceHdc,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    // Record a metafile to memory.

    Metafile(
        IN HDC                 referenceHdc,
        IN const RectF &       frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    // Record a metafile to memory.

    Metafile(
        IN HDC                 referenceHdc,
        IN const Rect &        frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN const WCHAR*        fileName,
        IN HDC                 referenceHdc,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN const WCHAR*        fileName,
        IN HDC                 referenceHdc,
        IN const RectF &       frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN const WCHAR*        fileName,
        IN HDC                 referenceHdc,
        IN const Rect &        frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN IStream *           stream,
        IN HDC                 referenceHdc,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN IStream *           stream,
        IN HDC                 referenceHdc,
        IN const RectF &       frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    Metafile(
        IN IStream *           stream,
        IN HDC                 referenceHdc,
        IN const Rect &        frameRect,
        IN MetafileFrameUnit   frameUnit   = MetafileFrameUnitGdi,
        IN EmfType             type        = EmfTypeEmfPlusDual,
        IN const WCHAR *       description = NULL
        );

    static Status GetMetafileHeader(
        IN HMETAFILE                       hWmf,
        IN const WmfPlaceableFileHeader *  wmfPlaceableFileHeader,
        OUT MetafileHeader *               header
        );

    static Status GetMetafileHeader(
        IN HENHMETAFILE        hEmf,
        OUT MetafileHeader *   header
        );

    static Status GetMetafileHeader(
        IN const WCHAR*        filename,
        OUT MetafileHeader *   header
        );

    static Status GetMetafileHeader(
        IN IStream *           stream,
        OUT MetafileHeader *   header
        );

    Status GetMetafileHeader(
        OUT MetafileHeader *    header
        ) const;

    // Once this method is called, the Metafile object is in an invalid state
    // and can no longer be used.  It is the responsiblity of the caller to
    // invoke DeleteEnhMetaFile to delete this hEmf.

    HENHMETAFILE GetHENHMETAFILE();

    // Used in conjuction with Graphics::EnumerateMetafile to play an EMF+
    // The data must be DWORD aligned if it's an EMF or EMF+.  It must be
    // WORD aligned if it's a WMF.
    
    Status PlayRecord(
        IN EmfPlusRecordType   recordType,
        IN UINT                flags,
        IN UINT                dataSize,
        IN const BYTE *        data
        ) const;

    // If you're using a printer HDC for the metafile, but you want the
    // metafile rasterized at screen resolution, then use this API to set
    // the rasterization dpi of the metafile to the screen resolution,
    // e.g. 96 dpi or 120 dpi.
    
    Status SetDownLevelRasterizationLimit(
        IN UINT     metafileRasterizationLimitDpi
        );

    UINT GetDownLevelRasterizationLimit() const;

    static UINT EmfToWmfBits(
        _In_ HENHMETAFILE       hemf,
        _In_ UINT               cbData16,     
        _Out_writes_to_opt_(cbData16, return) LPBYTE            pData16,
        _In_ INT                iMapMode = MM_ANISOTROPIC,
        _In_ INT                eFlags = EmfToWmfBitsFlagsDefault
    );

#if (GDIPVER >= 0x0110)
    Status ConvertToEmfPlus(
        IN const Graphics* refGraphics,
        IN OUT INT* conversionFailureFlag = NULL,
        IN EmfType emfType = EmfTypeEmfPlusOnly,
        IN const WCHAR* description = NULL
    );
    Status ConvertToEmfPlus(
        IN const Graphics* refGraphics,
        IN const WCHAR* filename, 
        IN OUT INT* conversionFailureFlag = NULL,
        IN EmfType emfType = EmfTypeEmfPlusOnly,
        IN const WCHAR* description = NULL
    );
    Status ConvertToEmfPlus(
        IN const Graphics* refGraphics,
        IN IStream* stream, 
        IN OUT INT* conversionFailureFlag = NULL,
        IN EmfType emfType = EmfTypeEmfPlusOnly,
        IN const WCHAR* description = NULL
    );
#endif

protected:
    Metafile()
    {
        SetNativeImage(NULL);
        lastResult = Ok;
    }

private:
    Metafile(const Metafile &);
    Metafile& operator=(const Metafile &);
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // !_GDIPLUSHEADERS.HPP

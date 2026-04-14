/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusFont.h
*
* Abstract:
*
*   GDI+ Font class
*
\**************************************************************************/

#ifndef _GDIPLUSFONT_H
#define _GDIPLUSFONT_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


inline
Font::Font(IN HDC hdc)
{
    GpFont *font = NULL;
    lastResult = DllExports::GdipCreateFontFromDC(hdc, &font);

    SetNativeFont(font);
}

inline
Font::Font(IN HDC hdc,
           IN const HFONT hfont)
{
    GpFont *font = NULL;

    if (hfont)
    {
        LOGFONTA lf;

        if(GetObjectA(hfont, sizeof(LOGFONTA), &lf))
            lastResult = DllExports::GdipCreateFontFromLogfontA(hdc, &lf, &font);
        else
            lastResult = DllExports::GdipCreateFontFromDC(hdc, &font);
    }
    else
    {
        lastResult = DllExports::GdipCreateFontFromDC(hdc, &font);
    }

    SetNativeFont(font);
}

inline
Font::Font(IN HDC hdc,
           IN const LOGFONTW* logfont)
{
    GpFont *font = NULL;
    if (logfont)
    {
        lastResult = DllExports::GdipCreateFontFromLogfontW(hdc, logfont, &font);
    }
    else
    {
        lastResult = DllExports::GdipCreateFontFromDC(hdc, &font);
    }

    SetNativeFont(font);
}

inline
Font::Font(IN HDC hdc,
           IN const LOGFONTA* logfont)
{
    GpFont *font = NULL;

    if (logfont)
    {
        lastResult = DllExports::GdipCreateFontFromLogfontA(hdc, logfont, &font);
    }
    else
    {
        lastResult = DllExports::GdipCreateFontFromDC(hdc, &font);
    }

    SetNativeFont(font);
}

inline
Font::Font(
     IN const FontFamily * family,
     IN REAL         emSize,
     IN INT          style,
     IN Unit         unit
)
{
    GpFont *font = NULL;

    lastResult = DllExports::GdipCreateFont(family ? family->nativeFamily : NULL,
                    emSize,
                    style,
                    unit,
                    &font);

    SetNativeFont(font);
}

inline
Font::Font(
     IN const WCHAR *          familyName,
     IN REAL                   emSize,
     IN INT                    style,
     IN Unit                   unit,
     IN const FontCollection * fontCollection
)
{
    nativeFont = NULL;

    FontFamily family(familyName, fontCollection);
    GpFontFamily *nativeFamily = family.nativeFamily;

    lastResult = family.GetLastStatus();

    if (lastResult != Ok)
    {
        nativeFamily = FontFamily::GenericSansSerif()->nativeFamily;
        lastResult = FontFamily::GenericSansSerif()->lastResult;
        if (lastResult != Ok)
            return;
    }

    lastResult = DllExports::GdipCreateFont(nativeFamily,
                            emSize,
                            style,
                            unit,
                            &nativeFont);

    if (lastResult != Ok)
    {
        nativeFamily = FontFamily::GenericSansSerif()->nativeFamily;
        lastResult = FontFamily::GenericSansSerif()->lastResult;
        if (lastResult != Ok)
            return;

        lastResult = DllExports::GdipCreateFont(
            nativeFamily,
            emSize,
            style,
            unit,
            &nativeFont);
    }
}

inline Status
Font::GetLogFontA(IN const Graphics *g,
                  OUT LOGFONTA *logfontA) const
{
    return SetStatus(DllExports::GdipGetLogFontA(nativeFont, g ? g->nativeGraphics : NULL, logfontA));

}

inline Status
Font::GetLogFontW(IN const Graphics *g,
                  OUT LOGFONTW *logfontW) const
{
    return SetStatus(DllExports::GdipGetLogFontW(nativeFont, g ? g->nativeGraphics : NULL, logfontW));
}


inline Font*
Font::Clone() const
{
    GpFont *cloneFont = NULL;

    SetStatus(DllExports::GdipCloneFont(nativeFont, &cloneFont));

    return new Font(cloneFont, lastResult);
}

inline
Font::~Font()
{
    DllExports::GdipDeleteFont(nativeFont);
}

// Operations

inline BOOL
Font::IsAvailable() const
{
    return (nativeFont ? TRUE : FALSE);
}

inline Status
Font::GetFamily(OUT FontFamily *family) const
{
    if (family == NULL)
    {
        return SetStatus(InvalidParameter);
    }

    Status status = DllExports::GdipGetFamily(nativeFont, &(family->nativeFamily));
    family->SetStatus(status);

    return SetStatus(status);
}

inline INT
Font::GetStyle() const
{
    INT style;

    SetStatus(DllExports::GdipGetFontStyle(nativeFont, &style));

    return style;
}

inline REAL
Font::GetSize() const
{
    REAL size;
    SetStatus(DllExports::GdipGetFontSize(nativeFont, &size));
    return size;
}

inline Unit
Font::GetUnit() const
{
    Unit unit;
    SetStatus(DllExports::GdipGetFontUnit(nativeFont, &unit));
    return unit;
}

inline REAL
Font::GetHeight(IN const Graphics *graphics) const
{
    REAL height;
    SetStatus(DllExports::GdipGetFontHeight(
        nativeFont,
        graphics ? graphics->nativeGraphics : NULL,
        &height
    ));
    return height;
}


inline REAL
Font::GetHeight(IN REAL dpi) const
{
    REAL height;
    SetStatus(DllExports::GdipGetFontHeightGivenDPI(nativeFont, dpi, &height));
    return height;
}

inline
Font::Font(IN GpFont* font,
           IN Status status)
{
    lastResult = status;
    SetNativeFont(font);
}

inline VOID
Font::SetNativeFont(GpFont *Font)
{
    nativeFont = Font;
}

inline Status
Font::GetLastStatus(void) const
{
    return lastResult;
}

inline _Post_equal_to_(status) Status
Font::SetStatus(IN Status status) const
{
    if (status != Ok)
        return (lastResult = status);
    else
        return status;
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

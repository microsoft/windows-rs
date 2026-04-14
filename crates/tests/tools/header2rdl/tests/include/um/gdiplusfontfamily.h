/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusFontFamily.h
*
* Abstract:
*
*   GDI+ Font Family class
*
\**************************************************************************/

#ifndef _GDIPLUS_FONT_FAMILY_H
#define _GDIPLUS_FONT_FAMILY_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

inline 
FontFamily::FontFamily() :
    nativeFamily (NULL),
    lastResult    (Ok)
{
}

inline 
FontFamily::FontFamily(
    IN const WCHAR*          name,
    IN const FontCollection* fontCollection
)
{
    nativeFamily = NULL;
    lastResult = DllExports::GdipCreateFontFamilyFromName(
        name,
        fontCollection ? fontCollection->nativeFontCollection : NULL,
        &nativeFamily
    );
}

inline
FontFamily::FontFamily(
    IN GpFontFamily *nativeOrig,
    IN Status status
)
{
    lastResult    = status;
    nativeFamily = nativeOrig;
}

inline const FontFamily *
FontFamily::GenericSansSerif() 
{
    if (GenericSansSerifFontFamily != NULL)
    {
        return GenericSansSerifFontFamily;
    }

    GenericSansSerifFontFamily =
        (FontFamily*) GenericSansSerifFontFamilyBuffer;

    GenericSansSerifFontFamily->lastResult =
        DllExports::GdipGetGenericFontFamilySansSerif(
            &(GenericSansSerifFontFamily->nativeFamily)
        );

    return GenericSansSerifFontFamily;
}

inline const FontFamily *
FontFamily::GenericSerif() 
{
    if (GenericSerifFontFamily != NULL)
    {
        return GenericSerifFontFamily;
    }

    GenericSerifFontFamily =
        (FontFamily*) GenericSerifFontFamilyBuffer;

    GenericSerifFontFamily->lastResult =
        DllExports::GdipGetGenericFontFamilySerif(
            &(GenericSerifFontFamily->nativeFamily)
        );

    return GenericSerifFontFamily;
}

inline const FontFamily *
FontFamily::GenericMonospace()
{
    if (GenericMonospaceFontFamily != NULL)
    {
        return GenericMonospaceFontFamily;
    }

    GenericMonospaceFontFamily =
        (FontFamily*) GenericMonospaceFontFamilyBuffer;

    GenericMonospaceFontFamily->lastResult =
        DllExports::GdipGetGenericFontFamilyMonospace(
            &(GenericMonospaceFontFamily->nativeFamily)
        );

    return GenericMonospaceFontFamily;
}

inline FontFamily::~FontFamily()
{
    DllExports::GdipDeleteFontFamily (nativeFamily);
}

inline FontFamily *
FontFamily::Clone() const
{
    GpFontFamily * clonedFamily = NULL;

    SetStatus(DllExports::GdipCloneFontFamily (nativeFamily, &clonedFamily));

    return new FontFamily(clonedFamily, lastResult);
}

inline Status 
FontFamily::GetFamilyName(
    _Out_writes_(LF_FACESIZE) LPWSTR    name,
    IN LANGID                           language
) const
{
    return SetStatus(DllExports::GdipGetFamilyName(nativeFamily, 
                                                   name, 
                                                   language));
}

inline BOOL 
FontFamily::IsStyleAvailable(IN INT style) const
{
    BOOL    StyleAvailable;
    Status  status;

    status = SetStatus(DllExports::GdipIsStyleAvailable(nativeFamily, style, &StyleAvailable));

    if (status != Ok)
        StyleAvailable = FALSE;

    return StyleAvailable;
}


inline UINT16 
FontFamily::GetEmHeight(IN INT style) const
{
    UINT16  EmHeight;

    SetStatus(DllExports::GdipGetEmHeight(nativeFamily, style, &EmHeight));

    return EmHeight;
}

inline UINT16 
FontFamily::GetCellAscent(IN INT style) const
{
    UINT16  CellAscent;

    SetStatus(DllExports::GdipGetCellAscent(nativeFamily, style, &CellAscent));

    return CellAscent;
}

inline UINT16 
FontFamily::GetCellDescent(IN INT style) const
{
    UINT16  CellDescent;

    SetStatus(DllExports::GdipGetCellDescent(nativeFamily, style, &CellDescent));

    return CellDescent;
}


inline UINT16 
FontFamily::GetLineSpacing(IN INT style) const
{
    UINT16  LineSpacing;

    SetStatus(DllExports::GdipGetLineSpacing(nativeFamily, style, &LineSpacing));

    return LineSpacing;

}

inline Status 
FontFamily::GetLastStatus() const
{
    Status lastStatus = lastResult;
    lastResult = Ok;

    return lastStatus;
}

inline _Post_equal_to_(status) Status
FontFamily::SetStatus(Status status) const 
{
    if (status != Ok)
        return (lastResult = status);
    else
        return status;
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

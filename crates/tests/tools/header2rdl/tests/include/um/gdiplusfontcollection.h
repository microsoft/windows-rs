/**************************************************************************\
*
* Copyright (c) 2000, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
* 
*   GdiplusFontCollection.h
*
* Abstract:
*
*   Font collections (Installed and Private)
*
\**************************************************************************/

#ifndef _GDIPLUSFONTCOLL_H
#define _GDIPLUSFONTCOLL_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#if _MSC_VER >= 1400
#pragma warning(disable:4365) /* signed/unsigned mismatch */
#endif
#endif

inline
FontCollection::FontCollection()
{
    nativeFontCollection = NULL;
}

inline
FontCollection::~FontCollection()
{
}

inline INT
FontCollection::GetFamilyCount() const
{
    INT numFound = 0;

    lastResult = DllExports::GdipGetFontCollectionFamilyCount(
                             nativeFontCollection, &numFound);



    return numFound;
}

inline Status
FontCollection::GetFamilies(
    _In_                                  INT          numSought,
    _Out_writes_to_(numSought, *numFound) FontFamily * gpfamilies,
    _Out_                                 INT *        numFound
) const
{
    if (numSought <= 0 || gpfamilies == NULL || numFound == NULL)
    {
        return SetStatus(InvalidParameter);
    }
    *numFound = 0;
    GpFontFamily **nativeFamilyList = new GpFontFamily*[numSought];

    if (nativeFamilyList == NULL)
    {
        return SetStatus(OutOfMemory);
    }

    Status status = DllExports::GdipGetFontCollectionFamilyList(
        nativeFontCollection,
        numSought,
        nativeFamilyList,
        numFound
    );
    if (status == Ok)
    {
        for (INT i = 0; i < *numFound; i++)
        {
            DllExports::GdipCloneFontFamily(nativeFamilyList[i],
                                            &gpfamilies[i].nativeFamily);
        }
    }

    delete [] nativeFamilyList;

    return SetStatus(status);
}

inline Status FontCollection::GetLastStatus () const
{
    return lastResult;
}

_Always_(_Post_satisfies_(return == status))
inline Status
FontCollection::SetStatus(IN Status status) const
{
    lastResult = status;
    return lastResult;
}

inline
InstalledFontCollection::InstalledFontCollection()
{
    nativeFontCollection = NULL;
    lastResult = DllExports::GdipNewInstalledFontCollection(&nativeFontCollection);
}

inline
InstalledFontCollection::~InstalledFontCollection()
{
}

inline
PrivateFontCollection::PrivateFontCollection()
{
    nativeFontCollection = NULL;
    lastResult = DllExports::GdipNewPrivateFontCollection(&nativeFontCollection);
}

inline
PrivateFontCollection::~PrivateFontCollection()
{
    DllExports::GdipDeletePrivateFontCollection(&nativeFontCollection);
}

inline Status
PrivateFontCollection::AddFontFile(IN const WCHAR* filename)
{
    return SetStatus(DllExports::GdipPrivateAddFontFile(nativeFontCollection, filename));
}

inline Status
PrivateFontCollection::AddMemoryFont(IN const void* memory,
                                     IN INT length)
{
    return SetStatus(DllExports::GdipPrivateAddMemoryFont(
        nativeFontCollection,
        memory,
        length));
}

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _GDIPLUSFONTCOLL_H

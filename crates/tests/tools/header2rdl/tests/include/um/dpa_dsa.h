
/*****************************************************************************\
*                                                                             *
* dpa_dsa.h - - Interface for the DPA and DSA structures in commtrl           *
*                                                                             *
* Version 1.2                                                                 *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/




#ifndef _INC_DPADSA
#define _INC_DPADSA

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(disable:4820) /* padding added after data member */
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _HRESULT_DEFINED
#define _HRESULT_DEFINED
typedef _Return_type_success_(return >= 0) long HRESULT;
#endif // !_HRESULT_DEFINED

//
// Define API decoration for direct importing of DLL references.
//
#ifndef WINCOMMCTRLAPI
#if !defined(_COMCTL32_) && defined(_WIN32)
#define WINCOMMCTRLAPI DECLSPEC_IMPORT
#else
#define WINCOMMCTRLAPI
#endif
#endif // WINCOMMCTRLAPI

#ifdef __cplusplus
extern "C" {
#endif

#ifndef NO_COMMCTRL_DA
#define __COMMCTRL_DA_DEFINED__
//
//====== Dynamic Array routines ==========================================
//
// Note that the STL and other libraries have similar functionality.
// The routines here are specific to Windows and may not be as convenient
// or fully functional as those in other libraries.
//

#define DA_LAST         (0x7FFFFFFF)
#define DA_ERR          (-1)

typedef int (CALLBACK *PFNDAENUMCALLBACK)(_In_opt_ void *p, _In_opt_ void *pData);
typedef int (CALLBACK *PFNDAENUMCALLBACKCONST)(_In_opt_ const void *p, _In_opt_ void *pData);
typedef int (CALLBACK *PFNDACOMPARE)(_In_opt_ void *p1, _In_opt_ void *p2, _In_ LPARAM lParam);
typedef int (CALLBACK *PFNDACOMPARECONST)(_In_opt_ const void *p1, _In_opt_ const void *p2, _In_ LPARAM lParam);


// Dynamic structure array
struct _DSA;
typedef struct _DSA *HDSA;


WINCOMMCTRLAPI HDSA   WINAPI DSA_Create(int cbItem, int cItemGrow);
WINCOMMCTRLAPI BOOL   WINAPI DSA_Destroy(_Inout_opt_ HDSA hdsa);
WINCOMMCTRLAPI void   WINAPI DSA_DestroyCallback(_Inout_opt_ HDSA hdsa, _In_ PFNDAENUMCALLBACK pfnCB, _In_opt_ void *pData);
#ifdef __cplusplus
extern "C++" __inline void DSA_DestroyCallback(_Inout_opt_ HDSA hdsa, _In_ PFNDAENUMCALLBACKCONST pfnCB, _In_opt_ void *pData)
{
    DSA_DestroyCallback(hdsa, (PFNDAENUMCALLBACK)pfnCB, pData);
}
#endif
WINCOMMCTRLAPI BOOL   WINAPI DSA_DeleteItem(_Inout_ HDSA hdsa, _In_ int i);
WINCOMMCTRLAPI BOOL   WINAPI DSA_DeleteAllItems(_Inout_ HDSA hdsa);
WINCOMMCTRLAPI void   WINAPI DSA_EnumCallback(_In_ HDSA hdsa, _In_ PFNDAENUMCALLBACK pfnCB, _In_opt_ void *pData);
#ifdef __cplusplus
extern "C++" __inline void DSA_EnumCallback(_In_ HDSA hdsa, _In_ PFNDAENUMCALLBACKCONST pfnCB, _In_opt_ void *pData)
{
    DSA_EnumCallback(hdsa, (PFNDAENUMCALLBACK)pfnCB, pData);
}
#endif
WINCOMMCTRLAPI int    WINAPI DSA_InsertItem(_Inout_ HDSA hdsa, _In_ int i, _In_ const void *pitem);
WINCOMMCTRLAPI PVOID  WINAPI DSA_GetItemPtr(_In_ HDSA hdsa, _In_ int i);
_Success_(return) WINCOMMCTRLAPI BOOL   WINAPI DSA_GetItem(_In_ HDSA hdsa, _In_ int i, _Out_writes_(_Inexpressible_(pdsa->cbItem)) void *pitem);
_Success_(return) WINCOMMCTRLAPI BOOL   WINAPI DSA_SetItem(_Inout_ HDSA hdsa, _In_ int i, _In_ const void *pitem);
#define DSA_GetItemCount(hdsa)      (*(int *)(hdsa))
#define DSA_AppendItem(hdsa, pitem) DSA_InsertItem(hdsa, DA_LAST, pitem)

#if (NTDDI_VERSION >= NTDDI_VISTA)
WINCOMMCTRLAPI HDSA   WINAPI DSA_Clone(_In_ HDSA hdsa);
WINCOMMCTRLAPI ULONGLONG WINAPI DSA_GetSize(_In_opt_ HDSA hdsa);
WINCOMMCTRLAPI BOOL   WINAPI DSA_Sort(_Inout_ HDSA pdsa, _In_ PFNDACOMPARE pfnCompare, _In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" __inline BOOL DSA_Sort(_Inout_ HDSA hdsa, _In_ PFNDACOMPARECONST pfnCompare, _In_ LPARAM lParam)
{
    return DSA_Sort(hdsa, (PFNDACOMPARE)(pfnCompare), lParam);
}
#endif
#endif  // NTDDI_VISTA

#define DSA_APPEND      DA_LAST
#define DSA_ERR         DA_ERR

#define PFNDSAENUMCALLBACK          PFNDAENUMCALLBACK
#define PFNDSAENUMCALLBACKCONST     PFNDAENUMCALLBACKCONST
#define PFNDSACOMPARE               PFNDACOMPARE
#define PFNDSACOMPARECONST          PFNDACOMPARECONST

// Dynamic pointer array
struct _DPA;
typedef struct _DPA *HDPA;


WINCOMMCTRLAPI HDPA   WINAPI DPA_Create(int cItemGrow);
WINCOMMCTRLAPI HDPA   WINAPI DPA_CreateEx(_In_ int cpGrow, _In_opt_ HANDLE hheap);
WINCOMMCTRLAPI HDPA   WINAPI DPA_Clone(_In_ const HDPA hdpa, _Inout_opt_ HDPA hdpaNew);
WINCOMMCTRLAPI BOOL   WINAPI DPA_Destroy(_Inout_opt_ HDPA hdpa);
WINCOMMCTRLAPI void   WINAPI DPA_DestroyCallback(_Inout_opt_ HDPA hdpa, _In_ PFNDAENUMCALLBACK pfnCB, _In_opt_ void *pData);
#ifdef __cplusplus
extern "C++" __inline void DPA_DestroyCallback(_In_ HDPA hdpa, _In_ PFNDAENUMCALLBACKCONST pfnCB, _In_opt_ void *pData)
{
    DPA_DestroyCallback(hdpa, (PFNDAENUMCALLBACK)pfnCB, pData);
}
#endif
WINCOMMCTRLAPI PVOID  WINAPI DPA_DeletePtr(_Inout_ HDPA hdpa, _In_ int i);
WINCOMMCTRLAPI BOOL   WINAPI DPA_DeleteAllPtrs(_Inout_ HDPA hdpa);
WINCOMMCTRLAPI void   WINAPI DPA_EnumCallback(_In_opt_ HDPA hdpa, _In_opt_ PFNDAENUMCALLBACK pfnCB, _In_opt_ void *pData);
#ifdef __cplusplus
extern "C++" __inline void DPA_EnumCallback(_In_ HDPA hdpa, _In_ PFNDAENUMCALLBACKCONST pfnCB, _In_opt_ void *pData)
{
    DPA_EnumCallback(hdpa, (PFNDAENUMCALLBACK)pfnCB, pData);
}
#endif
WINCOMMCTRLAPI BOOL   WINAPI DPA_Grow(_Inout_ HDPA pdpa, _In_ int cp);
WINCOMMCTRLAPI int    WINAPI DPA_InsertPtr(_Inout_ HDPA hdpa, _In_ int i, _In_opt_ void *p);
WINCOMMCTRLAPI BOOL   WINAPI DPA_SetPtr(_Inout_ HDPA hdpa, _In_ int i, _In_opt_ void *p);
WINCOMMCTRLAPI PVOID  WINAPI DPA_GetPtr(_In_ HDPA hdpa, _In_ INT_PTR i);
WINCOMMCTRLAPI int    WINAPI DPA_GetPtrIndex(_In_ HDPA hdpa, _In_opt_ const void *p);

#define DPA_GetPtrCount(hdpa)       (*(int *)(hdpa))
#define DPA_SetPtrCount(hdpa, cItems) (*(int *)(hdpa) = (cItems))
#define DPA_FastDeleteLastPtr(hdpa) (--*(int *)(hdpa))
#define DPA_GetPtrPtr(hdpa)         (*((void * **)((BYTE *)(hdpa) + sizeof(void *))))
#define DPA_FastGetPtr(hdpa, i)     (DPA_GetPtrPtr(hdpa)[i])
#define DPA_AppendPtr(hdpa, pitem)  DPA_InsertPtr(hdpa, DA_LAST, pitem)

#if (NTDDI_VERSION >= NTDDI_VISTA)
WINCOMMCTRLAPI ULONGLONG WINAPI DPA_GetSize(_In_opt_ HDPA hdpa);
#endif  // NTDDI_VISTA

WINCOMMCTRLAPI BOOL   WINAPI DPA_Sort(_Inout_ HDPA hdpa, _In_ PFNDACOMPARE pfnCompare, _In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" __inline BOOL DPA_Sort(_Inout_ HDPA hdpa, _In_ PFNDACOMPARECONST pfnCompare, _In_ LPARAM lParam)
{
    return DPA_Sort(hdpa, (PFNDACOMPARE)(pfnCompare), lParam);
}
#endif

//
// Save to and load from a stream.  The stream callback gets a pointer to
// a DPASTREAMINFO structure.
//
// For DPA_SaveStream, the callback is responsible for writing the pvItem
// info to the stream.  (It's not necessary to write the iPos to the
// stream.)  Return S_OK if the element was saved, S_FALSE if it wasn't
// but continue anyway, or some failure.
//
// For DPA_LoadStream, the callback is responsible for allocating an
// item and setting the pvItem field to the new pointer.  Return S_OK
// if the element was loaded, S_FALSE it it wasn't but continue anyway,
// or some failure.
//

typedef struct _DPASTREAMINFO
{
    int    iPos;        // Index of item
    void *pvItem;
} DPASTREAMINFO;

struct IStream;
typedef HRESULT (CALLBACK *PFNDPASTREAM)(_In_ DPASTREAMINFO * pinfo, _In_ struct IStream * pstream, _In_opt_ void *pvInstData);

WINCOMMCTRLAPI HRESULT WINAPI DPA_LoadStream(_Outptr_ HDPA * phdpa, _In_ PFNDPASTREAM pfn, _In_ struct IStream * pstream, _In_opt_ void *pvInstData);
WINCOMMCTRLAPI HRESULT WINAPI DPA_SaveStream(_In_ HDPA hdpa, _In_ PFNDPASTREAM pfn, _In_ struct IStream * pstream, _In_opt_ void *pvInstData);

// Merge two DPAs.  This takes two (optionally) presorted arrays and merges
// the source array into the dest.  DPA_Merge uses the provided callbacks
// to perform comparison and merge operations.  The merge callback is
// called when two elements (one in each list) match according to the
// compare function.  This allows portions of an element in one list to
// be merged with the respective element in the second list.
//
// The first DPA (hdpaDest) is the output array.
//
// Merge options:
//
//    DPAM_SORTED       The arrays are already sorted; don't sort
//    DPAM_UNION        The resulting array is the union of all elements
//                      in both arrays (DPAMM_INSERT may be sent for
//                      this merge option.)
//    DPAM_INTERSECT    Only elements in the source array that intersect
//                      with the dest array are merged.  (DPAMM_DELETE
//                      may be sent for this merge option.)
//    DPAM_NORMAL       Like DPAM_INTERSECT except the dest array
//                      also maintains its original, additional elements.
//
#define DPAM_SORTED             0x00000001
#define DPAM_NORMAL             0x00000002
#define DPAM_UNION              0x00000004
#define DPAM_INTERSECT          0x00000008

// The merge callback should merge contents of the two items and return
// the pointer of the merged item.  It's okay to simply use pvDest
// as the returned pointer.
//
typedef void* (CALLBACK *PFNDPAMERGE)(_In_ UINT uMsg, _In_ void *pvDest, _In_ void *pvSrc, _In_ LPARAM lParam);
typedef const void* (CALLBACK *PFNDPAMERGECONST)(_In_ UINT uMsg, _In_ const void *pvDest, _In_ const void *pvSrc, _In_ LPARAM lParam);

// Messages for merge callback
#define DPAMM_MERGE     1
#define DPAMM_DELETE    2
#define DPAMM_INSERT    3

WINCOMMCTRLAPI BOOL WINAPI DPA_Merge(_Inout_ HDPA hdpaDest, _In_ HDPA hdpaSrc, _In_ DWORD dwFlags, _In_ PFNDACOMPARE pfnCompare, _In_ PFNDPAMERGE pfnMerge, _In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" __inline BOOL DPA_Merge(_Inout_ HDPA hdpaDest, _In_ HDPA hdpaSrc, _In_ DWORD dwFlags, _In_ PFNDACOMPARECONST pfnCompare, _In_ PFNDPAMERGECONST pfnMerge, _In_ LPARAM lParam)
{
    return DPA_Merge(hdpaDest, hdpaSrc, dwFlags, (PFNDACOMPARE)pfnCompare, (PFNDPAMERGE)pfnMerge, lParam);
}
#endif

//
// Search array.  If DPAS_SORTED, then array is assumed to be sorted
// according to pfnCompare, and binary search algorithm is used.
// Otherwise, linear search is used.
//
// Searching starts at iStart (0 to start search at beginning).
//
// DPAS_INSERTBEFORE/AFTER govern what happens if an exact match is not
// found.  If neither are specified, this function returns -1 if no exact
// match is found.  Otherwise, the index of the item before or after the
// closest (including exact) match is returned.
//
// Search option flags
//
#define DPAS_SORTED             0x0001
#define DPAS_INSERTBEFORE       0x0002
#define DPAS_INSERTAFTER        0x0004

WINCOMMCTRLAPI int WINAPI DPA_Search(_In_ HDPA hdpa, _In_opt_ void *pFind, _In_ int iStart, _In_ PFNDACOMPARE pfnCompare, _In_ LPARAM lParam, _In_ UINT options);
#ifdef __cplusplus
extern "C++" __inline int DPA_Search(_In_ HDPA hdpa, _In_opt_ const void *pFind, _In_ int iStart, _In_ PFNDACOMPARECONST pfnCompare, _In_ LPARAM lParam, _In_ UINT options)
{
    return DPA_Search(hdpa, const_cast<void *>(pFind), iStart, (PFNDACOMPARE)pfnCompare, lParam, options);
}
#endif

#define DPA_SortedInsertPtr(hdpa, pFind, iStart, pfnCompare, lParam, options, pitem)  \
            DPA_InsertPtr(hdpa, DPA_Search(hdpa, pFind, iStart, pfnCompare, lParam, (DPAS_SORTED | (options))), (pitem))

#define DPA_APPEND      DA_LAST
#define DPA_ERR         DA_ERR

#define PFNDPAENUMCALLBACK          PFNDAENUMCALLBACK
#define PFNDPAENUMCALLBACKCONST     PFNDAENUMCALLBACKCONST
#define PFNDPACOMPARE               PFNDACOMPARE
#define PFNDPACOMPARECONST          PFNDACOMPARECONST

#endif // NO_COMMCTRL_DA


WINCOMMCTRLAPI BOOL WINAPI Str_SetPtrW(_Inout_ LPWSTR * ppsz, _In_opt_ LPCWSTR psz);




#if !defined(RC_INVOKED) /* RC complains about long symbols in #ifs */
#if defined(ISOLATION_AWARE_ENABLED) && (ISOLATION_AWARE_ENABLED != 0)
#include "dpa_dsa.inl"
#endif /* ISOLATION_AWARE_ENABLED */
#endif /* RC */

#ifdef __cplusplus
}
#endif


#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _INC_DPADSA */


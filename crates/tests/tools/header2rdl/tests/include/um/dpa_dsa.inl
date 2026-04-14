/* Copyright (c) 2001-2026, Microsoft Corp. All rights reserved. */

#if _MSC_VER > 1000
#pragma once
#endif

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(RC_INVOKED) /* RC complains about long symbols in #ifs */
#if defined(ISOLATION_AWARE_ENABLED) && (ISOLATION_AWARE_ENABLED != 0)
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4191) /* cast */
#if _MSC_VER >= 1400
#pragma warning(disable:6101) /* Because there will be success paths where out params are not initialized. */
#endif
#endif


#if !defined(ISOLATION_AWARE_USE_STATIC_LIBRARY)
#define ISOLATION_AWARE_USE_STATIC_LIBRARY 0
#endif

#if !defined(ISOLATION_AWARE_BUILD_STATIC_LIBRARY)
#define ISOLATION_AWARE_BUILD_STATIC_LIBRARY 0
#endif

#if !defined(ISOLATION_AWARE_INLINE)
#if ISOLATION_AWARE_BUILD_STATIC_LIBRARY
#define ISOLATION_AWARE_INLINE /* nothing */
#else
#if defined(__cplusplus)
#define ISOLATION_AWARE_INLINE inline
#else
#define ISOLATION_AWARE_INLINE __inline
#endif
#endif
#endif

#if !ISOLATION_AWARE_USE_STATIC_LIBRARY
FARPROC IsolationAwarePrivatenCv Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY(LPCSTR pszProcName);

#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */
HDSA IsolationAwarePrivatenCv IsolationAwareDSA_Create(int cbItem,int cItemGrow);
BOOL IsolationAwarePrivatenCv IsolationAwareDSA_Destroy(_Inout_opt_ HDSA hdsa);
void IsolationAwarePrivatenCv IsolationAwareDSA_DestroyCallback(_Inout_opt_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
#ifdef __cplusplus
extern "C++" inline  void IsolationAwareDSA_DestroyCallback(_Inout_opt_ HDSA hdsa,_In_ PFNDAENUMCALLBACKCONST pfnCB,_In_opt_ void*pData)
{
    IsolationAwareDSA_DestroyCallback(hdsa,(PFNDAENUMCALLBACK)pfnCB,pData);
}
#endif
BOOL IsolationAwarePrivatenCv IsolationAwareDSA_DeleteItem(_Inout_ HDSA hdsa,_In_ int i);
BOOL IsolationAwarePrivatenCv IsolationAwareDSA_DeleteAllItems(_Inout_ HDSA hdsa);
void IsolationAwarePrivatenCv IsolationAwareDSA_EnumCallback(_In_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
#ifdef __cplusplus
extern "C++" inline  void IsolationAwareDSA_EnumCallback(_In_ HDSA hdsa,_In_ PFNDAENUMCALLBACKCONST pfnCB,_In_opt_ void*pData)
{
    IsolationAwareDSA_EnumCallback(hdsa,(PFNDAENUMCALLBACK)pfnCB,pData);
}
#endif
int IsolationAwarePrivatenCv IsolationAwareDSA_InsertItem(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem);
PVOID IsolationAwarePrivatenCv IsolationAwareDSA_GetItemPtr(_In_ HDSA hdsa,_In_ int i);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareDSA_GetItem(_In_ HDSA hdsa,_In_ int i,_Out_writes_(_Inexpressible_(pdsa->cbItem)) void*pitem);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareDSA_SetItem(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem);
HDSA IsolationAwarePrivatenCv IsolationAwareDSA_Clone(_In_ HDSA hdsa);
ULONGLONG IsolationAwarePrivatenCv IsolationAwareDSA_GetSize(_In_opt_ HDSA hdsa);
BOOL IsolationAwarePrivatenCv IsolationAwareDSA_Sort(_Inout_ HDSA pdsa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" inline  BOOL IsolationAwareDSA_Sort(_Inout_ HDSA pdsa,_In_ PFNDACOMPARECONST pfnCompare,_In_ LPARAM lParam)
{
    return IsolationAwareDSA_Sort(pdsa,(PFNDACOMPARE)pfnCompare,lParam);
}
#endif
HDPA IsolationAwarePrivatenCv IsolationAwareDPA_Create(int cItemGrow);
HDPA IsolationAwarePrivatenCv IsolationAwareDPA_CreateEx(_In_ int cpGrow,_In_opt_ HANDLE hheap);
HDPA IsolationAwarePrivatenCv IsolationAwareDPA_Clone(_In_ const HDPA hdpa,_Inout_opt_ HDPA hdpaNew);
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Destroy(_Inout_opt_ HDPA hdpa);
void IsolationAwarePrivatenCv IsolationAwareDPA_DestroyCallback(_Inout_opt_ HDPA hdpa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
#ifdef __cplusplus
extern "C++" inline  void IsolationAwareDPA_DestroyCallback(_Inout_opt_ HDPA hdpa,_In_ PFNDAENUMCALLBACKCONST pfnCB,_In_opt_ void*pData)
{
    IsolationAwareDPA_DestroyCallback(hdpa,(PFNDAENUMCALLBACK)pfnCB,pData);
}
#endif
PVOID IsolationAwarePrivatenCv IsolationAwareDPA_DeletePtr(_Inout_ HDPA hdpa,_In_ int i);
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_DeleteAllPtrs(_Inout_ HDPA hdpa);
void IsolationAwarePrivatenCv IsolationAwareDPA_EnumCallback(_In_opt_ HDPA hdpa,_In_opt_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
#ifdef __cplusplus
extern "C++" inline  void IsolationAwareDPA_EnumCallback(_In_opt_ HDPA hdpa,_In_opt_ PFNDAENUMCALLBACKCONST pfnCB,_In_opt_ void*pData)
{
    IsolationAwareDPA_EnumCallback(hdpa,(PFNDAENUMCALLBACK)pfnCB,pData);
}
#endif
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Grow(_Inout_ HDPA pdpa,_In_ int cp);
int IsolationAwarePrivatenCv IsolationAwareDPA_InsertPtr(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p);
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_SetPtr(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p);
PVOID IsolationAwarePrivatenCv IsolationAwareDPA_GetPtr(_In_ HDPA hdpa,_In_ INT_PTR i);
int IsolationAwarePrivatenCv IsolationAwareDPA_GetPtrIndex(_In_ HDPA hdpa,_In_opt_ const void*p);
ULONGLONG IsolationAwarePrivatenCv IsolationAwareDPA_GetSize(_In_opt_ HDPA hdpa);
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Sort(_Inout_ HDPA hdpa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" inline  BOOL IsolationAwareDPA_Sort(_Inout_ HDPA hdpa,_In_ PFNDACOMPARECONST pfnCompare,_In_ LPARAM lParam)
{
    return IsolationAwareDPA_Sort(hdpa,(PFNDACOMPARE)pfnCompare,lParam);
}
#endif
HRESULT IsolationAwarePrivatenCv IsolationAwareDPA_LoadStream(_Outptr_ HDPA*phdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData);
HRESULT IsolationAwarePrivatenCv IsolationAwareDPA_SaveStream(_In_ HDPA hdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData);
BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Merge(_Inout_ HDPA hdpaDest,_In_ HDPA hdpaSrc,_In_ DWORD dwFlags,_In_ PFNDACOMPARE pfnCompare,_In_ PFNDPAMERGE pfnMerge,_In_ LPARAM lParam);
#ifdef __cplusplus
extern "C++" inline  BOOL IsolationAwareDPA_Merge(_Inout_ HDPA hdpaDest,_In_ HDPA hdpaSrc,_In_ DWORD dwFlags,_In_ PFNDACOMPARECONST pfnCompare,_In_ PFNDPAMERGE pfnMerge,_In_ LPARAM lParam)
{
    return IsolationAwareDPA_Merge(hdpaDest,hdpaSrc,dwFlags,(PFNDACOMPARE)pfnCompare,pfnMerge,lParam);
}
#endif
int IsolationAwarePrivatenCv IsolationAwareDPA_Search(_In_ HDPA hdpa,_In_opt_ void*pFind,_In_ int iStart,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam,_In_ UINT options);
#ifdef __cplusplus
extern "C++" inline  int IsolationAwareDPA_Search(_In_ HDPA hdpa,_In_opt_ void*pFind,_In_ int iStart,_In_ PFNDACOMPARECONST pfnCompare,_In_ LPARAM lParam,_In_ UINT options)
{
    return IsolationAwareDPA_Search(hdpa,pFind,iStart,(PFNDACOMPARE)pfnCompare,lParam,options);
}
#endif
BOOL IsolationAwarePrivatenCv IsolationAwareStr_SetPtrW(_Inout_ LPWSTR*ppsz,_In_opt_ LPCWSTR psz);

_Post_satisfies_(FAILED(return))
ISOLATION_AWARE_INLINE HRESULT Dpa_dsaIsolationAwarePrivateJVaQPGbueRfhYg(void)
{
    DWORD dwLastError = GetLastError();
    if (dwLastError == NO_ERROR)
        dwLastError = ERROR_INTERNAL_ERROR;
    return HRESULT_FROM_WIN32(dwLastError);
}

#if !ISOLATION_AWARE_USE_STATIC_LIBRARY
ISOLATION_AWARE_INLINE HDSA IsolationAwarePrivatenCv IsolationAwareDSA_Create(int cbItem,int cItemGrow)
{
    HDSA result = NULL;
    typedef HDSA (WINAPI* PFN)(int cbItem,int cItemGrow);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_Create");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(cbItem,cItemGrow);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_Destroy(_Inout_opt_ HDSA hdsa)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_opt_ HDSA hdsa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_Destroy");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdsa);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE void IsolationAwarePrivatenCv IsolationAwareDSA_DestroyCallback(_Inout_opt_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData)
{
    typedef void (WINAPI* PFN)(_Inout_opt_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_DestroyCallback");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    __IsolationAware_pfn(hdsa,pfnCB,pData);
Exit:
    return;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_DeleteItem(_Inout_ HDSA hdsa,_In_ int i)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDSA hdsa,_In_ int i);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_DeleteItem");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdsa,i);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_DeleteAllItems(_Inout_ HDSA hdsa)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDSA hdsa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_DeleteAllItems");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdsa);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE void IsolationAwarePrivatenCv IsolationAwareDSA_EnumCallback(_In_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData)
{
    typedef void (WINAPI* PFN)(_In_ HDSA hdsa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_EnumCallback");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    __IsolationAware_pfn(hdsa,pfnCB,pData);
Exit:
    return;
}

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareDSA_InsertItem(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem)
{
    int nResult = -1;
    typedef int (WINAPI* PFN)(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_InsertItem");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    nResult = __IsolationAware_pfn(hdsa,i,pitem);
Exit:
    return nResult;
}

ISOLATION_AWARE_INLINE PVOID IsolationAwarePrivatenCv IsolationAwareDSA_GetItemPtr(_In_ HDSA hdsa,_In_ int i)
{
    PVOID vResult = NULL;
    typedef PVOID (WINAPI* PFN)(_In_ HDSA hdsa,_In_ int i);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_GetItemPtr");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    vResult = __IsolationAware_pfn(hdsa,i);
Exit:
    return vResult;
}

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_GetItem(_In_ HDSA hdsa,_In_ int i,_Out_writes_(_Inexpressible_(pdsa->cbItem)) void*pitem)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_In_ HDSA hdsa,_In_ int i,_Out_writes_(_Inexpressible_(pdsa->cbItem)) void*pitem);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_GetItem");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdsa,i,pitem);
Exit:
    return fResult;
}

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_SetItem(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDSA hdsa,_In_ int i,_In_ const void*pitem);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_SetItem");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdsa,i,pitem);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE HDSA IsolationAwarePrivatenCv IsolationAwareDSA_Clone(_In_ HDSA hdsa)
{
    HDSA result = NULL;
    typedef HDSA (WINAPI* PFN)(_In_ HDSA hdsa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_Clone");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        result = __IsolationAware_pfn(hdsa);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (result == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return result;
}

ISOLATION_AWARE_INLINE ULONGLONG IsolationAwarePrivatenCv IsolationAwareDSA_GetSize(_In_opt_ HDSA hdsa)
{
    ULONGLONG result = 0 ;
    typedef ULONGLONG (WINAPI* PFN)(_In_opt_ HDSA hdsa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_GetSize");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        result = __IsolationAware_pfn(hdsa);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (result == 0 );
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return result;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDSA_Sort(_Inout_ HDSA pdsa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDSA pdsa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DSA_Sort");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        fResult = __IsolationAware_pfn(pdsa,pfnCompare,lParam);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (fResult == FALSE);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return fResult;
}

ISOLATION_AWARE_INLINE HDPA IsolationAwarePrivatenCv IsolationAwareDPA_Create(int cItemGrow)
{
    HDPA result = NULL;
    typedef HDPA (WINAPI* PFN)(int cItemGrow);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Create");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(cItemGrow);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE HDPA IsolationAwarePrivatenCv IsolationAwareDPA_CreateEx(_In_ int cpGrow,_In_opt_ HANDLE hheap)
{
    HDPA result = NULL;
    typedef HDPA (WINAPI* PFN)(_In_ int cpGrow,_In_opt_ HANDLE hheap);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_CreateEx");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(cpGrow,hheap);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE HDPA IsolationAwarePrivatenCv IsolationAwareDPA_Clone(_In_ const HDPA hdpa,_Inout_opt_ HDPA hdpaNew)
{
    HDPA result = NULL;
    typedef HDPA (WINAPI* PFN)(_In_ const HDPA hdpa,_Inout_opt_ HDPA hdpaNew);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Clone");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(hdpa,hdpaNew);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Destroy(_Inout_opt_ HDPA hdpa)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_opt_ HDPA hdpa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Destroy");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdpa);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE void IsolationAwarePrivatenCv IsolationAwareDPA_DestroyCallback(_Inout_opt_ HDPA hdpa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData)
{
    typedef void (WINAPI* PFN)(_Inout_opt_ HDPA hdpa,_In_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_DestroyCallback");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    __IsolationAware_pfn(hdpa,pfnCB,pData);
Exit:
    return;
}

ISOLATION_AWARE_INLINE PVOID IsolationAwarePrivatenCv IsolationAwareDPA_DeletePtr(_Inout_ HDPA hdpa,_In_ int i)
{
    PVOID vResult = NULL;
    typedef PVOID (WINAPI* PFN)(_Inout_ HDPA hdpa,_In_ int i);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_DeletePtr");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    vResult = __IsolationAware_pfn(hdpa,i);
Exit:
    return vResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_DeleteAllPtrs(_Inout_ HDPA hdpa)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDPA hdpa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_DeleteAllPtrs");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdpa);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE void IsolationAwarePrivatenCv IsolationAwareDPA_EnumCallback(_In_opt_ HDPA hdpa,_In_opt_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData)
{
    typedef void (WINAPI* PFN)(_In_opt_ HDPA hdpa,_In_opt_ PFNDAENUMCALLBACK pfnCB,_In_opt_ void*pData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_EnumCallback");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    __IsolationAware_pfn(hdpa,pfnCB,pData);
Exit:
    return;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Grow(_Inout_ HDPA pdpa,_In_ int cp)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDPA pdpa,_In_ int cp);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Grow");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(pdpa,cp);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareDPA_InsertPtr(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p)
{
    int nResult = -1;
    typedef int (WINAPI* PFN)(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_InsertPtr");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    nResult = __IsolationAware_pfn(hdpa,i,p);
Exit:
    return nResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_SetPtr(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDPA hdpa,_In_ int i,_In_opt_ void*p);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_SetPtr");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdpa,i,p);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE PVOID IsolationAwarePrivatenCv IsolationAwareDPA_GetPtr(_In_ HDPA hdpa,_In_ INT_PTR i)
{
    PVOID vResult = NULL;
    typedef PVOID (WINAPI* PFN)(_In_ HDPA hdpa,_In_ INT_PTR i);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_GetPtr");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    vResult = __IsolationAware_pfn(hdpa,i);
Exit:
    return vResult;
}

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareDPA_GetPtrIndex(_In_ HDPA hdpa,_In_opt_ const void*p)
{
    int nResult = -1;
    typedef int (WINAPI* PFN)(_In_ HDPA hdpa,_In_opt_ const void*p);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_GetPtrIndex");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    nResult = __IsolationAware_pfn(hdpa,p);
Exit:
    return nResult;
}

ISOLATION_AWARE_INLINE ULONGLONG IsolationAwarePrivatenCv IsolationAwareDPA_GetSize(_In_opt_ HDPA hdpa)
{
    ULONGLONG result = 0 ;
    typedef ULONGLONG (WINAPI* PFN)(_In_opt_ HDPA hdpa);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_GetSize");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        result = __IsolationAware_pfn(hdpa);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (result == 0 );
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return result;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Sort(_Inout_ HDPA hdpa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDPA hdpa,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Sort");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdpa,pfnCompare,lParam);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE HRESULT IsolationAwarePrivatenCv IsolationAwareDPA_LoadStream(_Outptr_ HDPA*phdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData)
{
    HRESULT result = S_OK;
    typedef HRESULT (WINAPI* PFN)(_Outptr_ HDPA*phdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_LoadStream");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(phdpa,pfn,pstream,pvInstData);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE HRESULT IsolationAwarePrivatenCv IsolationAwareDPA_SaveStream(_In_ HDPA hdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData)
{
    HRESULT result = S_OK;
    typedef HRESULT (WINAPI* PFN)(_In_ HDPA hdpa,_In_ PFNDPASTREAM pfn,_In_ struct IStream*pstream,_In_opt_ void*pvInstData);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_SaveStream");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    result = __IsolationAware_pfn(hdpa,pfn,pstream,pvInstData);
Exit:
    return result;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDPA_Merge(_Inout_ HDPA hdpaDest,_In_ HDPA hdpaSrc,_In_ DWORD dwFlags,_In_ PFNDACOMPARE pfnCompare,_In_ PFNDPAMERGE pfnMerge,_In_ LPARAM lParam)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ HDPA hdpaDest,_In_ HDPA hdpaSrc,_In_ DWORD dwFlags,_In_ PFNDACOMPARE pfnCompare,_In_ PFNDPAMERGE pfnMerge,_In_ LPARAM lParam);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Merge");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(hdpaDest,hdpaSrc,dwFlags,pfnCompare,pfnMerge,lParam);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareDPA_Search(_In_ HDPA hdpa,_In_opt_ void*pFind,_In_ int iStart,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam,_In_ UINT options)
{
    int nResult = -1;
    typedef int (WINAPI* PFN)(_In_ HDPA hdpa,_In_opt_ void*pFind,_In_ int iStart,_In_ PFNDACOMPARE pfnCompare,_In_ LPARAM lParam,_In_ UINT options);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DPA_Search");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    nResult = __IsolationAware_pfn(hdpa,pFind,iStart,pfnCompare,lParam,options);
Exit:
    return nResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareStr_SetPtrW(_Inout_ LPWSTR*ppsz,_In_opt_ LPCWSTR psz)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ LPWSTR*ppsz,_In_opt_ LPCWSTR psz);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    if (__IsolationAware_pfn == NULL)
    {
        ULONG_PTR ulpCookie = 0;
        const BOOL fActivateActCtxSuccess =
            IsolationAwarePrivateT_SAbnPgpgk ||
            IsolationAwarePrivateT_SqbjaYRiRY ||
            IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
        if (!fActivateActCtxSuccess)
            goto Exit;
        __IsolationAware_pfn = (PFN) Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("Str_SetPtrW");
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            const BOOL fPreserveLastError = (__IsolationAware_pfn == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
        if (__IsolationAware_pfn == NULL)
            goto Exit;
        s_pfn = __IsolationAware_pfn;
    }
    fResult = __IsolationAware_pfn(ppsz,psz);
Exit:
    return fResult;
}

ISOLATION_AWARE_INLINE FARPROC IsolationAwarePrivatenCv Dpa_dsaIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY(LPCSTR pszProcName)
/* This function is shared by the other stubs in this header. */
{
    FARPROC proc = NULL;
    static HMODULE s_module;
    BOOL fActivateActCtxSuccess = FALSE;
    ULONG_PTR ulpCookie = 0;
#ifndef _M_CEE_PURE
    const static IsolationAwarePrivatepBAFGnAG_zBqHyr_vAsB
        c = { IsolationAwarePrivatezlybNQyVOeNelJ, L"Comctl32.dll"
#ifdef _M_IX86
             , IsolationAwarePrivatezlybNQyVOeNeln, "Comctl32.dll"
#endif
    };
#else
    static IsolationAwarePrivatepBAFGnAG_zBqHyr_vAsB c;
    c.WinbaseIsolationAwarePrivateybNQJ = IsolationAwarePrivatezlybNQyVOeNelJ;
    c.WinbaseIsolationAwarePrivateANZRJ = L"Comctl32.dll";
#ifdef _M_IX86
    c.WinbaseIsolationAwarePrivateybNQn = IsolationAwarePrivatezlybNQyVOeNeln;
    c.WinbaseIsolationAwarePrivateANZRn = "Comctl32.dll";
#endif
#endif
    static IsolationAwarePrivatezHGnoyr_zBqHyr_vAsB m;

    __try
    {
        if (!IsolationAwarePrivateT_SqbjaYRiRY)
        {
            fActivateActCtxSuccess = IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
            if (!fActivateActCtxSuccess)
                __leave;
        }
        proc = IsolationAwarePrivatezltRgCebPnQQeRff(&c, &m, pszProcName);
    }
    __finally
    {
        if (
            !IsolationAwarePrivateT_SqbjaYRiRY &&
            fActivateActCtxSuccess)
        {
            const DWORD dwLastError = (proc == NULL) ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (proc == NULL)
                SetLastError(dwLastError);
        }
    }
    return proc;
}

#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */

#define DPA_Clone IsolationAwareDPA_Clone
#define DPA_Create IsolationAwareDPA_Create
#define DPA_CreateEx IsolationAwareDPA_CreateEx
#define DPA_DeleteAllPtrs IsolationAwareDPA_DeleteAllPtrs
#define DPA_DeletePtr IsolationAwareDPA_DeletePtr
#define DPA_Destroy IsolationAwareDPA_Destroy
#define DPA_DestroyCallback IsolationAwareDPA_DestroyCallback
#define DPA_EnumCallback IsolationAwareDPA_EnumCallback
#define DPA_GetPtr IsolationAwareDPA_GetPtr
#define DPA_GetPtrIndex IsolationAwareDPA_GetPtrIndex
#define DPA_GetSize IsolationAwareDPA_GetSize
#define DPA_Grow IsolationAwareDPA_Grow
#define DPA_InsertPtr IsolationAwareDPA_InsertPtr
#define DPA_LoadStream IsolationAwareDPA_LoadStream
#define DPA_Merge IsolationAwareDPA_Merge
#define DPA_SaveStream IsolationAwareDPA_SaveStream
#define DPA_Search IsolationAwareDPA_Search
#define DPA_SetPtr IsolationAwareDPA_SetPtr
#define DPA_Sort IsolationAwareDPA_Sort
#define DSA_Clone IsolationAwareDSA_Clone
#define DSA_Create IsolationAwareDSA_Create
#define DSA_DeleteAllItems IsolationAwareDSA_DeleteAllItems
#define DSA_DeleteItem IsolationAwareDSA_DeleteItem
#define DSA_Destroy IsolationAwareDSA_Destroy
#define DSA_DestroyCallback IsolationAwareDSA_DestroyCallback
#define DSA_EnumCallback IsolationAwareDSA_EnumCallback
#define DSA_GetItem IsolationAwareDSA_GetItem
#define DSA_GetItemPtr IsolationAwareDSA_GetItemPtr
#define DSA_GetSize IsolationAwareDSA_GetSize
#define DSA_InsertItem IsolationAwareDSA_InsertItem
#define DSA_SetItem IsolationAwareDSA_SetItem
#define DSA_Sort IsolationAwareDSA_Sort
#define Str_SetPtrW IsolationAwareStr_SetPtrW
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* ISOLATION_AWARE_ENABLED */
#endif /* RC */


#if defined(__cplusplus)
} /* __cplusplus */
#endif

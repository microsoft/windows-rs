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
FARPROC IsolationAwarePrivatenCv PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY(LPCSTR pszProcName);

#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */
HPROPSHEETPAGE IsolationAwarePrivatenCv IsolationAwareCreatePropertySheetPageA(LPCPROPSHEETPAGEA constPropSheetPagePointer);
HPROPSHEETPAGE IsolationAwarePrivatenCv IsolationAwareCreatePropertySheetPageW(LPCPROPSHEETPAGEW constPropSheetPagePointer);
BOOL IsolationAwarePrivatenCv IsolationAwareDestroyPropertySheetPage(HPROPSHEETPAGE unnamed1);
INT_PTR IsolationAwarePrivatenCv IsolationAwarePropertySheetA(LPCPROPSHEETHEADERA unnamed1);
INT_PTR IsolationAwarePrivatenCv IsolationAwarePropertySheetW(LPCPROPSHEETHEADERW unnamed1);

#if defined(UNICODE)

#define IsolationAwareCreatePropertySheetPage IsolationAwareCreatePropertySheetPageW
#define IsolationAwarePropertySheet IsolationAwarePropertySheetW

#else /* UNICODE */

#define IsolationAwareCreatePropertySheetPage IsolationAwareCreatePropertySheetPageA
#define IsolationAwarePropertySheet IsolationAwarePropertySheetA

#endif /* UNICODE */

#if !ISOLATION_AWARE_USE_STATIC_LIBRARY
ISOLATION_AWARE_INLINE HPROPSHEETPAGE IsolationAwarePrivatenCv IsolationAwareCreatePropertySheetPageA(LPCPROPSHEETPAGEA constPropSheetPagePointer)
{
    HPROPSHEETPAGE result = NULL;
    typedef HPROPSHEETPAGE (WINAPI* PFN)(LPCPROPSHEETPAGEA constPropSheetPagePointer);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;

    const LPPROPSHEETPAGEA_LATEST latestPropSheetPagePointer = (LPPROPSHEETPAGEA_LATEST)constPropSheetPagePointer;
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
            __IsolationAware_pfn = (PFN) PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("CreatePropertySheetPageA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }

#ifdef _M_IX86
        if (IsolationAwarePrivateT_SqbjaYRiRY)
        {
            if ((latestPropSheetPagePointer->dwFlags & PSP_USEFUSIONCONTEXT) != 0)
            {
                latestPropSheetPagePointer->dwFlags &= ~PSP_USEFUSIONCONTEXT;
            }
        }
        else
#endif
        if ((   latestPropSheetPagePointer->dwFlags & PSP_USEFUSIONCONTEXT) == 0
                    && latestPropSheetPagePointer->dwSize >= sizeof(PROPSHEETPAGEA_V3)
                  )
        {
            latestPropSheetPagePointer->dwFlags |= PSP_USEFUSIONCONTEXT;
            latestPropSheetPagePointer->hActCtx = WinbaseIsolationAwarePrivateT_UnPgpgk;
        }
        result = s_pfn(constPropSheetPagePointer);
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

ISOLATION_AWARE_INLINE HPROPSHEETPAGE IsolationAwarePrivatenCv IsolationAwareCreatePropertySheetPageW(LPCPROPSHEETPAGEW constPropSheetPagePointer)
{
    HPROPSHEETPAGE result = NULL;
    typedef HPROPSHEETPAGE (WINAPI* PFN)(LPCPROPSHEETPAGEW constPropSheetPagePointer);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;

    const LPPROPSHEETPAGEW_LATEST latestPropSheetPagePointer = (LPPROPSHEETPAGEW_LATEST)constPropSheetPagePointer;
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
            __IsolationAware_pfn = (PFN) PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("CreatePropertySheetPageW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }

#ifdef _M_IX86
        if (IsolationAwarePrivateT_SqbjaYRiRY)
        {
            if ((latestPropSheetPagePointer->dwFlags & PSP_USEFUSIONCONTEXT) != 0)
            {
                latestPropSheetPagePointer->dwFlags &= ~PSP_USEFUSIONCONTEXT;
            }
        }
        else
#endif
        if ((   latestPropSheetPagePointer->dwFlags & PSP_USEFUSIONCONTEXT) == 0
                    && latestPropSheetPagePointer->dwSize >= sizeof(PROPSHEETPAGEW_V3)
                  )
        {
            latestPropSheetPagePointer->dwFlags |= PSP_USEFUSIONCONTEXT;
            latestPropSheetPagePointer->hActCtx = WinbaseIsolationAwarePrivateT_UnPgpgk;
        }
        result = s_pfn(constPropSheetPagePointer);
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareDestroyPropertySheetPage(HPROPSHEETPAGE unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(HPROPSHEETPAGE unnamed1);
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
            __IsolationAware_pfn = (PFN) PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("DestroyPropertySheetPage");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        fResult = __IsolationAware_pfn(unnamed1);
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

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwarePropertySheetA(LPCPROPSHEETHEADERA unnamed1)
{
    INT_PTR nResult = -1;
    typedef INT_PTR (WINAPI* PFN)(LPCPROPSHEETHEADERA unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("PropertySheetA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        nResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (nResult == -1);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return nResult;
}

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwarePropertySheetW(LPCPROPSHEETHEADERW unnamed1)
{
    INT_PTR nResult = -1;
    typedef INT_PTR (WINAPI* PFN)(LPCPROPSHEETHEADERW unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY("PropertySheetW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        nResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (nResult == -1);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return nResult;
}

ISOLATION_AWARE_INLINE FARPROC IsolationAwarePrivatenCv PrshtIsolationAwarePrivatetRgCebPnQQeRff_pbZPgYQP_QYY(LPCSTR pszProcName)
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

#define CreatePropertySheetPageA IsolationAwareCreatePropertySheetPageA
#define CreatePropertySheetPageW IsolationAwareCreatePropertySheetPageW
#define DestroyPropertySheetPage IsolationAwareDestroyPropertySheetPage
#define PropertySheetA IsolationAwarePropertySheetA
#define PropertySheetW IsolationAwarePropertySheetW
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* ISOLATION_AWARE_ENABLED */
#endif /* RC */


#if defined(__cplusplus)
} /* __cplusplus */
#endif

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
FARPROC IsolationAwarePrivatenCv CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY(LPCSTR pszProcName);

#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */
BOOL IsolationAwarePrivatenCv IsolationAwareGetOpenFileNameA(LPOPENFILENAMEA unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareGetOpenFileNameW(LPOPENFILENAMEW unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareGetSaveFileNameA(LPOPENFILENAMEA unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareGetSaveFileNameW(LPOPENFILENAMEW unnamed1);
short IsolationAwarePrivatenCv IsolationAwareGetFileTitleA(LPCSTR unnamed1,_Out_writes_(cchSize) LPSTR Buf,_In_ WORD cchSize);
short IsolationAwarePrivatenCv IsolationAwareGetFileTitleW(LPCWSTR unnamed1,_Out_writes_(cchSize) LPWSTR Buf,_In_ WORD cchSize);
BOOL IsolationAwarePrivatenCv IsolationAwareChooseColorA(LPCHOOSECOLORA unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareChooseColorW(LPCHOOSECOLORW unnamed1);
HWND IsolationAwarePrivatenCv IsolationAwareFindTextA(LPFINDREPLACEA unnamed1);
HWND IsolationAwarePrivatenCv IsolationAwareFindTextW(LPFINDREPLACEW unnamed1);
HWND IsolationAwarePrivatenCv IsolationAwareReplaceTextA(LPFINDREPLACEA unnamed1);
HWND IsolationAwarePrivatenCv IsolationAwareReplaceTextW(LPFINDREPLACEW unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareChooseFontA(LPCHOOSEFONTA unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwareChooseFontW(LPCHOOSEFONTW unnamed1);
_Success_(return != FALSE) BOOL IsolationAwarePrivatenCv IsolationAwarePrintDlgA(_Inout_ LPPRINTDLGA pPD);
_Success_(return != FALSE) BOOL IsolationAwarePrivatenCv IsolationAwarePrintDlgW(_Inout_ LPPRINTDLGW pPD);
#if defined(STDMETHOD) && (WINVER >= 0x0500)
HRESULT IsolationAwarePrivatenCv IsolationAwarePrintDlgExA(_Inout_ LPPRINTDLGEXA pPD);
HRESULT IsolationAwarePrivatenCv IsolationAwarePrintDlgExW(_Inout_ LPPRINTDLGEXW pPD);
#endif /* defined(STDMETHOD) && (WINVER >= 0x0500) */
DWORD IsolationAwarePrivatenCv IsolationAwareCommDlgExtendedError(void);
BOOL IsolationAwarePrivatenCv IsolationAwarePageSetupDlgA(LPPAGESETUPDLGA unnamed1);
BOOL IsolationAwarePrivatenCv IsolationAwarePageSetupDlgW(LPPAGESETUPDLGW unnamed1);

_Post_satisfies_(FAILED(return))
ISOLATION_AWARE_INLINE HRESULT CommdlgIsolationAwarePrivateJVaQPGbueRfhYg(void)
{
    DWORD dwLastError = GetLastError();
    if (dwLastError == NO_ERROR)
        dwLastError = ERROR_INTERNAL_ERROR;
    return HRESULT_FROM_WIN32(dwLastError);
}

#if defined(UNICODE)

#define IsolationAwareChooseColor IsolationAwareChooseColorW
#define IsolationAwareChooseFont IsolationAwareChooseFontW
#define IsolationAwareFindText IsolationAwareFindTextW
#define IsolationAwareGetFileTitle IsolationAwareGetFileTitleW
#define IsolationAwareGetOpenFileName IsolationAwareGetOpenFileNameW
#define IsolationAwareGetSaveFileName IsolationAwareGetSaveFileNameW
#define IsolationAwarePageSetupDlg IsolationAwarePageSetupDlgW
#define IsolationAwarePrintDlg IsolationAwarePrintDlgW
#define IsolationAwarePrintDlgEx IsolationAwarePrintDlgExW
#define IsolationAwareReplaceText IsolationAwareReplaceTextW

#else /* UNICODE */

#define IsolationAwareChooseColor IsolationAwareChooseColorA
#define IsolationAwareChooseFont IsolationAwareChooseFontA
#define IsolationAwareFindText IsolationAwareFindTextA
#define IsolationAwareGetFileTitle IsolationAwareGetFileTitleA
#define IsolationAwareGetOpenFileName IsolationAwareGetOpenFileNameA
#define IsolationAwareGetSaveFileName IsolationAwareGetSaveFileNameA
#define IsolationAwarePageSetupDlg IsolationAwarePageSetupDlgA
#define IsolationAwarePrintDlg IsolationAwarePrintDlgA
#define IsolationAwarePrintDlgEx IsolationAwarePrintDlgExA
#define IsolationAwareReplaceText IsolationAwareReplaceTextA

#endif /* UNICODE */

#if !ISOLATION_AWARE_USE_STATIC_LIBRARY
ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetOpenFileNameA(LPOPENFILENAMEA unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPOPENFILENAMEA unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetOpenFileNameA");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetOpenFileNameW(LPOPENFILENAMEW unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPOPENFILENAMEW unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetOpenFileNameW");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetSaveFileNameA(LPOPENFILENAMEA unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPOPENFILENAMEA unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetSaveFileNameA");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetSaveFileNameW(LPOPENFILENAMEW unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPOPENFILENAMEW unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetSaveFileNameW");
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

ISOLATION_AWARE_INLINE short IsolationAwarePrivatenCv IsolationAwareGetFileTitleA(LPCSTR unnamed1,_Out_writes_(cchSize) LPSTR Buf,_In_ WORD cchSize)
{
    short nResult = -1;
    typedef short (WINAPI* PFN)(LPCSTR unnamed1,_Out_writes_(cchSize) LPSTR Buf,_In_ WORD cchSize);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetFileTitleA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        nResult = __IsolationAware_pfn(unnamed1,Buf,cchSize);
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

ISOLATION_AWARE_INLINE short IsolationAwarePrivatenCv IsolationAwareGetFileTitleW(LPCWSTR unnamed1,_Out_writes_(cchSize) LPWSTR Buf,_In_ WORD cchSize)
{
    short nResult = -1;
    typedef short (WINAPI* PFN)(LPCWSTR unnamed1,_Out_writes_(cchSize) LPWSTR Buf,_In_ WORD cchSize);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("GetFileTitleW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        nResult = __IsolationAware_pfn(unnamed1,Buf,cchSize);
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareChooseColorA(LPCHOOSECOLORA unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPCHOOSECOLORA unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ChooseColorA");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareChooseColorW(LPCHOOSECOLORW unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPCHOOSECOLORW unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ChooseColorW");
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareFindTextA(LPFINDREPLACEA unnamed1)
{
    HWND windowResult = NULL;
    typedef HWND (WINAPI* PFN)(LPFINDREPLACEA unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("FindTextA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        windowResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (windowResult == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return windowResult;
}

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareFindTextW(LPFINDREPLACEW unnamed1)
{
    HWND windowResult = NULL;
    typedef HWND (WINAPI* PFN)(LPFINDREPLACEW unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("FindTextW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        windowResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (windowResult == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return windowResult;
}

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareReplaceTextA(LPFINDREPLACEA unnamed1)
{
    HWND windowResult = NULL;
    typedef HWND (WINAPI* PFN)(LPFINDREPLACEA unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ReplaceTextA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        windowResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (windowResult == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return windowResult;
}

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareReplaceTextW(LPFINDREPLACEW unnamed1)
{
    HWND windowResult = NULL;
    typedef HWND (WINAPI* PFN)(LPFINDREPLACEW unnamed1);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ReplaceTextW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        windowResult = __IsolationAware_pfn(unnamed1);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (windowResult == NULL);
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return windowResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareChooseFontA(LPCHOOSEFONTA unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPCHOOSEFONTA unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ChooseFontA");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareChooseFontW(LPCHOOSEFONTW unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPCHOOSEFONTW unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("ChooseFontW");
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

_Success_(return != FALSE) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwarePrintDlgA(_Inout_ LPPRINTDLGA pPD)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ LPPRINTDLGA pPD);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PrintDlgA");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        fResult = __IsolationAware_pfn(pPD);
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

_Success_(return != FALSE) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwarePrintDlgW(_Inout_ LPPRINTDLGW pPD)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(_Inout_ LPPRINTDLGW pPD);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PrintDlgW");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        fResult = __IsolationAware_pfn(pPD);
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

#if defined(STDMETHOD) && (WINVER >= 0x0500)

ISOLATION_AWARE_INLINE HRESULT IsolationAwarePrivatenCv IsolationAwarePrintDlgExA(_Inout_ LPPRINTDLGEXA pPD)
{
    HRESULT result = S_OK;
    typedef HRESULT (WINAPI* PFN)(_Inout_ LPPRINTDLGEXA pPD);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return CommdlgIsolationAwarePrivateJVaQPGbueRfhYg();
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PrintDlgExA");
            if (__IsolationAware_pfn == NULL)
            {
                result = CommdlgIsolationAwarePrivateJVaQPGbueRfhYg();
                __leave;
            }
            s_pfn = __IsolationAware_pfn;
        }
        result = __IsolationAware_pfn(pPD);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
    }
    return result;
}

ISOLATION_AWARE_INLINE HRESULT IsolationAwarePrivatenCv IsolationAwarePrintDlgExW(_Inout_ LPPRINTDLGEXW pPD)
{
    HRESULT result = S_OK;
    typedef HRESULT (WINAPI* PFN)(_Inout_ LPPRINTDLGEXW pPD);
    static PFN s_pfn;
    PFN __IsolationAware_pfn = s_pfn;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return CommdlgIsolationAwarePrivateJVaQPGbueRfhYg();
    __try
    {
        if (__IsolationAware_pfn == NULL)
        {
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PrintDlgExW");
            if (__IsolationAware_pfn == NULL)
            {
                result = CommdlgIsolationAwarePrivateJVaQPGbueRfhYg();
                __leave;
            }
            s_pfn = __IsolationAware_pfn;
        }
        result = __IsolationAware_pfn(pPD);
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
        }
    }
    return result;
}

#endif /* defined(STDMETHOD) && (WINVER >= 0x0500) */

ISOLATION_AWARE_INLINE DWORD IsolationAwarePrivatenCv IsolationAwareCommDlgExtendedError(void)
{
    DWORD nResult = 0 ;
    typedef DWORD (WINAPI* PFN)(void);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("CommDlgExtendedError");
            if (__IsolationAware_pfn == NULL)
                __leave;
            s_pfn = __IsolationAware_pfn;
        }
        nResult = __IsolationAware_pfn();
    }
    __finally
    {
        if (!IsolationAwarePrivateT_SAbnPgpgk
         || !IsolationAwarePrivateT_SqbjaYRiRY
        )
        {
            const BOOL fPreserveLastError = (nResult == 0 );
            const DWORD dwLastError = fPreserveLastError ? GetLastError() : NO_ERROR;
            (void)IsolationAwareDeactivateActCtx(0, ulpCookie);
            if (fPreserveLastError)
                SetLastError(dwLastError);
        }
    }
    return nResult;
}

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwarePageSetupDlgA(LPPAGESETUPDLGA unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPPAGESETUPDLGA unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PageSetupDlgA");
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwarePageSetupDlgW(LPPAGESETUPDLGW unnamed1)
{
    BOOL fResult = FALSE;
    typedef BOOL (WINAPI* PFN)(LPPAGESETUPDLGW unnamed1);
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
            __IsolationAware_pfn = (PFN) CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY("PageSetupDlgW");
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

ISOLATION_AWARE_INLINE FARPROC IsolationAwarePrivatenCv CommdlgIsolationAwarePrivatetRgCebPnQQeRff_pbZQYTQP_QYY(LPCSTR pszProcName)
/* This function is shared by the other stubs in this header. */
{
    FARPROC proc = NULL;
    static HMODULE s_module;
    BOOL fActivateActCtxSuccess = FALSE;
    ULONG_PTR ulpCookie = 0;
#ifndef _M_CEE_PURE
    const static IsolationAwarePrivatepBAFGnAG_zBqHyr_vAsB
        c = { IsolationAwarePrivatezlybNQyVOeNelJ, L"Comdlg32.dll"
#ifdef _M_IX86
             , IsolationAwarePrivatezlybNQyVOeNeln, "Comdlg32.dll"
#endif
    };
#else
    static IsolationAwarePrivatepBAFGnAG_zBqHyr_vAsB c;
    c.WinbaseIsolationAwarePrivateybNQJ = IsolationAwarePrivatezlybNQyVOeNelJ;
    c.WinbaseIsolationAwarePrivateANZRJ = L"Comdlg32.dll";
#ifdef _M_IX86
    c.WinbaseIsolationAwarePrivateybNQn = IsolationAwarePrivatezlybNQyVOeNeln;
    c.WinbaseIsolationAwarePrivateANZRn = "Comdlg32.dll";
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

#define ChooseColorA IsolationAwareChooseColorA
#define ChooseColorW IsolationAwareChooseColorW
#define ChooseFontA IsolationAwareChooseFontA
#define ChooseFontW IsolationAwareChooseFontW
#define CommDlgExtendedError IsolationAwareCommDlgExtendedError
#define FindTextA IsolationAwareFindTextA
#define FindTextW IsolationAwareFindTextW
#define GetFileTitleA IsolationAwareGetFileTitleA
#define GetFileTitleW IsolationAwareGetFileTitleW
#define GetOpenFileNameA IsolationAwareGetOpenFileNameA
#define GetOpenFileNameW IsolationAwareGetOpenFileNameW
#define GetSaveFileNameA IsolationAwareGetSaveFileNameA
#define GetSaveFileNameW IsolationAwareGetSaveFileNameW
#define PageSetupDlgA IsolationAwarePageSetupDlgA
#define PageSetupDlgW IsolationAwarePageSetupDlgW
#define PrintDlgA IsolationAwarePrintDlgA
#define PrintDlgExA IsolationAwarePrintDlgExA
#define PrintDlgExW IsolationAwarePrintDlgExW
#define PrintDlgW IsolationAwarePrintDlgW
#define ReplaceTextA IsolationAwareReplaceTextA
#define ReplaceTextW IsolationAwareReplaceTextW
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* ISOLATION_AWARE_ENABLED */
#endif /* RC */


#if defined(__cplusplus)
} /* __cplusplus */
#endif

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
#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */
ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassA(_In_ const WNDCLASSA*lpWndClass);
ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassW(_In_ const WNDCLASSW*lpWndClass);
BOOL IsolationAwarePrivatenCv IsolationAwareUnregisterClassA(_In_ LPCSTR lpClassName,_In_opt_ HINSTANCE hInstance);
BOOL IsolationAwarePrivatenCv IsolationAwareUnregisterClassW(_In_ LPCWSTR lpClassName,_In_opt_ HINSTANCE hInstance);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpClassName,_Out_ LPWNDCLASSA lpWndClass);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpClassName,_Out_ LPWNDCLASSW lpWndClass);
ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassExA(_In_ const WNDCLASSEXA*unnamed1);
ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassExW(_In_ const WNDCLASSEXW*unnamed1);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoExA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpszClass,_Out_ LPWNDCLASSEXA lpwcx);
_Success_(return) BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoExW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpszClass,_Out_ LPWNDCLASSEXW lpwcx);
HWND IsolationAwarePrivatenCv IsolationAwareCreateWindowExA(_In_ DWORD dwExStyle,_In_opt_ LPCSTR lpClassName,_In_opt_ LPCSTR lpWindowName,_In_ DWORD dwStyle,_In_ int X,_In_ int Y,_In_ int nWidth,_In_ int nHeight,_In_opt_ HWND hWndParent,_In_opt_ HMENU hMenu,_In_opt_ HINSTANCE hInstance,_In_opt_ LPVOID lpParam);
HWND IsolationAwarePrivatenCv IsolationAwareCreateWindowExW(_In_ DWORD dwExStyle,_In_opt_ LPCWSTR lpClassName,_In_opt_ LPCWSTR lpWindowName,_In_ DWORD dwStyle,_In_ int X,_In_ int Y,_In_ int nWidth,_In_ int nHeight,_In_opt_ HWND hWndParent,_In_opt_ HMENU hMenu,_In_opt_ HINSTANCE hInstance,_In_opt_ LPVOID lpParam);
HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogIndirectParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEA lpTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogIndirectParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEW lpTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxIndirectParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEA hDialogTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxIndirectParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEW hDialogTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxA(_In_opt_ HWND hWnd,_In_opt_ LPCSTR lpText,_In_opt_ LPCSTR lpCaption,_In_ UINT uType);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxW(_In_opt_ HWND hWnd,_In_opt_ LPCWSTR lpText,_In_opt_ LPCWSTR lpCaption,_In_ UINT uType);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxExA(_In_opt_ HWND hWnd,_In_opt_ LPCSTR lpText,_In_opt_ LPCSTR lpCaption,_In_ UINT uType,_In_ WORD wLanguageId);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxExW(_In_opt_ HWND hWnd,_In_opt_ LPCWSTR lpText,_In_opt_ LPCWSTR lpCaption,_In_ UINT uType,_In_ WORD wLanguageId);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxIndirectA(_In_ const MSGBOXPARAMSA*lpmbp);
int IsolationAwarePrivatenCv IsolationAwareMessageBoxIndirectW(_In_ const MSGBOXPARAMSW*lpmbp);

#if defined(UNICODE)

#define IsolationAwareCreateDialogIndirectParam IsolationAwareCreateDialogIndirectParamW
#define IsolationAwareCreateDialogParam IsolationAwareCreateDialogParamW
#define IsolationAwareCreateWindowEx IsolationAwareCreateWindowExW
#define IsolationAwareDialogBoxIndirectParam IsolationAwareDialogBoxIndirectParamW
#define IsolationAwareDialogBoxParam IsolationAwareDialogBoxParamW
#define IsolationAwareGetClassInfo IsolationAwareGetClassInfoW
#define IsolationAwareGetClassInfoEx IsolationAwareGetClassInfoExW
#define IsolationAwareMessageBox IsolationAwareMessageBoxW
#define IsolationAwareMessageBoxEx IsolationAwareMessageBoxExW
#define IsolationAwareMessageBoxIndirect IsolationAwareMessageBoxIndirectW
#define IsolationAwareRegisterClass IsolationAwareRegisterClassW
#define IsolationAwareRegisterClassEx IsolationAwareRegisterClassExW
#define IsolationAwareUnregisterClass IsolationAwareUnregisterClassW

#else /* UNICODE */

#define IsolationAwareCreateDialogIndirectParam IsolationAwareCreateDialogIndirectParamA
#define IsolationAwareCreateDialogParam IsolationAwareCreateDialogParamA
#define IsolationAwareCreateWindowEx IsolationAwareCreateWindowExA
#define IsolationAwareDialogBoxIndirectParam IsolationAwareDialogBoxIndirectParamA
#define IsolationAwareDialogBoxParam IsolationAwareDialogBoxParamA
#define IsolationAwareGetClassInfo IsolationAwareGetClassInfoA
#define IsolationAwareGetClassInfoEx IsolationAwareGetClassInfoExA
#define IsolationAwareMessageBox IsolationAwareMessageBoxA
#define IsolationAwareMessageBoxEx IsolationAwareMessageBoxExA
#define IsolationAwareMessageBoxIndirect IsolationAwareMessageBoxIndirectA
#define IsolationAwareRegisterClass IsolationAwareRegisterClassA
#define IsolationAwareRegisterClassEx IsolationAwareRegisterClassExA
#define IsolationAwareUnregisterClass IsolationAwareUnregisterClassA

#endif /* UNICODE */

#if !ISOLATION_AWARE_USE_STATIC_LIBRARY
ISOLATION_AWARE_INLINE ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassA(_In_ const WNDCLASSA*lpWndClass)
{
    ATOM result = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        result = RegisterClassA(lpWndClass);
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

ISOLATION_AWARE_INLINE ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassW(_In_ const WNDCLASSW*lpWndClass)
{
    ATOM result = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        result = RegisterClassW(lpWndClass);
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareUnregisterClassA(_In_ LPCSTR lpClassName,_In_opt_ HINSTANCE hInstance)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = UnregisterClassA(lpClassName,hInstance);
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

ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareUnregisterClassW(_In_ LPCWSTR lpClassName,_In_opt_ HINSTANCE hInstance)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = UnregisterClassW(lpClassName,hInstance);
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

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpClassName,_Out_ LPWNDCLASSA lpWndClass)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = GetClassInfoA(hInstance,lpClassName,lpWndClass);
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

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpClassName,_Out_ LPWNDCLASSW lpWndClass)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = GetClassInfoW(hInstance,lpClassName,lpWndClass);
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

ISOLATION_AWARE_INLINE ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassExA(_In_ const WNDCLASSEXA*unnamed1)
{
    ATOM result = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        result = RegisterClassExA(unnamed1);
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

ISOLATION_AWARE_INLINE ATOM IsolationAwarePrivatenCv IsolationAwareRegisterClassExW(_In_ const WNDCLASSEXW*unnamed1)
{
    ATOM result = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return result;
    __try
    {
        result = RegisterClassExW(unnamed1);
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

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoExA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpszClass,_Out_ LPWNDCLASSEXA lpwcx)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = GetClassInfoExA(hInstance,lpszClass,lpwcx);
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

_Success_(return) ISOLATION_AWARE_INLINE BOOL IsolationAwarePrivatenCv IsolationAwareGetClassInfoExW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpszClass,_Out_ LPWNDCLASSEXW lpwcx)
{
    BOOL fResult = FALSE;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return fResult;
    __try
    {
        fResult = GetClassInfoExW(hInstance,lpszClass,lpwcx);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateWindowExA(_In_ DWORD dwExStyle,_In_opt_ LPCSTR lpClassName,_In_opt_ LPCSTR lpWindowName,_In_ DWORD dwStyle,_In_ int X,_In_ int Y,_In_ int nWidth,_In_ int nHeight,_In_opt_ HWND hWndParent,_In_opt_ HMENU hMenu,_In_opt_ HINSTANCE hInstance,_In_opt_ LPVOID lpParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateWindowExA(dwExStyle,lpClassName,lpWindowName,dwStyle,X,Y,nWidth,nHeight,hWndParent,hMenu,hInstance,lpParam);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateWindowExW(_In_ DWORD dwExStyle,_In_opt_ LPCWSTR lpClassName,_In_opt_ LPCWSTR lpWindowName,_In_ DWORD dwStyle,_In_ int X,_In_ int Y,_In_ int nWidth,_In_ int nHeight,_In_opt_ HWND hWndParent,_In_opt_ HMENU hMenu,_In_opt_ HINSTANCE hInstance,_In_opt_ LPVOID lpParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateWindowExW(dwExStyle,lpClassName,lpWindowName,dwStyle,X,Y,nWidth,nHeight,hWndParent,hMenu,hInstance,lpParam);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateDialogParamA(hInstance,lpTemplateName,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateDialogParamW(hInstance,lpTemplateName,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogIndirectParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEA lpTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateDialogIndirectParamA(hInstance,lpTemplate,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE HWND IsolationAwarePrivatenCv IsolationAwareCreateDialogIndirectParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEW lpTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    HWND windowResult = NULL;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return windowResult;
    __try
    {
        windowResult = CreateDialogIndirectParamW(hInstance,lpTemplate,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    INT_PTR nResult = -1;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = DialogBoxParamA(hInstance,lpTemplateName,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCWSTR lpTemplateName,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    INT_PTR nResult = -1;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = DialogBoxParamW(hInstance,lpTemplateName,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxIndirectParamA(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEA hDialogTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    INT_PTR nResult = -1;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = DialogBoxIndirectParamA(hInstance,hDialogTemplate,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE INT_PTR IsolationAwarePrivatenCv IsolationAwareDialogBoxIndirectParamW(_In_opt_ HINSTANCE hInstance,_In_ LPCDLGTEMPLATEW hDialogTemplate,_In_opt_ HWND hWndParent,_In_opt_ DLGPROC lpDialogFunc,_In_ LPARAM dwInitParam)
{
    INT_PTR nResult = -1;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = DialogBoxIndirectParamW(hInstance,hDialogTemplate,hWndParent,lpDialogFunc,dwInitParam);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxA(_In_opt_ HWND hWnd,_In_opt_ LPCSTR lpText,_In_opt_ LPCSTR lpCaption,_In_ UINT uType)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxA(hWnd,lpText,lpCaption,uType);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxW(_In_opt_ HWND hWnd,_In_opt_ LPCWSTR lpText,_In_opt_ LPCWSTR lpCaption,_In_ UINT uType)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxW(hWnd,lpText,lpCaption,uType);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxExA(_In_opt_ HWND hWnd,_In_opt_ LPCSTR lpText,_In_opt_ LPCSTR lpCaption,_In_ UINT uType,_In_ WORD wLanguageId)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxExA(hWnd,lpText,lpCaption,uType,wLanguageId);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxExW(_In_opt_ HWND hWnd,_In_opt_ LPCWSTR lpText,_In_opt_ LPCWSTR lpCaption,_In_ UINT uType,_In_ WORD wLanguageId)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxExW(hWnd,lpText,lpCaption,uType,wLanguageId);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxIndirectA(_In_ const MSGBOXPARAMSA*lpmbp)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxIndirectA(lpmbp);
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

ISOLATION_AWARE_INLINE int IsolationAwarePrivatenCv IsolationAwareMessageBoxIndirectW(_In_ const MSGBOXPARAMSW*lpmbp)
{
    int nResult = 0 ;
    ULONG_PTR ulpCookie = 0;
    const BOOL fActivateActCtxSuccess =
        IsolationAwarePrivateT_SAbnPgpgk ||
        IsolationAwarePrivateT_SqbjaYRiRY ||
        IsolationAwarePrivatenPgViNgRzlnPgpgk(&ulpCookie);
    if (!fActivateActCtxSuccess)
        return nResult;
    __try
    {
        nResult = MessageBoxIndirectW(lpmbp);
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

ISOLATION_AWARE_INLINE FARPROC IsolationAwarePrivatenCv WinuserIsolationAwarePrivatetRgCebPnQQeRff_HfReQP_QYY(LPCSTR pszProcName)
/* This function is shared by the other stubs in this header. */
{
    static HMODULE s_module;
    const static IsolationAwarePrivatepBAFGnAG_zBqHyr_vAsB
        c = { IsolationAwarePrivatezlybNQyVOeNelJ, L"User32.dll"
#ifdef _M_IX86
             , IsolationAwarePrivatezlybNQyVOeNeln, "User32.dll"
#endif
    };
    static IsolationAwarePrivatezHGnoyr_zBqHyr_vAsB m;

    return IsolationAwarePrivatezltRgCebPnQQeRff(&c, &m, pszProcName);
}

#endif /* ISOLATION_AWARE_USE_STATIC_LIBRARY */

#define CreateDialogIndirectParamA IsolationAwareCreateDialogIndirectParamA
#define CreateDialogIndirectParamW IsolationAwareCreateDialogIndirectParamW
#define CreateDialogParamA IsolationAwareCreateDialogParamA
#define CreateDialogParamW IsolationAwareCreateDialogParamW
#define CreateWindowExA IsolationAwareCreateWindowExA
#define CreateWindowExW IsolationAwareCreateWindowExW
#define DialogBoxIndirectParamA IsolationAwareDialogBoxIndirectParamA
#define DialogBoxIndirectParamW IsolationAwareDialogBoxIndirectParamW
#define DialogBoxParamA IsolationAwareDialogBoxParamA
#define DialogBoxParamW IsolationAwareDialogBoxParamW
 /* GetClassInfoA skipped, as it is a popular C++ member function name. */
#define GetClassInfoExA IsolationAwareGetClassInfoExA
#define GetClassInfoExW IsolationAwareGetClassInfoExW
 /* GetClassInfoW skipped, as it is a popular C++ member function name. */
 /* MessageBoxA skipped, as it is a popular C++ member function name. */
#define MessageBoxExA IsolationAwareMessageBoxExA
#define MessageBoxExW IsolationAwareMessageBoxExW
#define MessageBoxIndirectA IsolationAwareMessageBoxIndirectA
#define MessageBoxIndirectW IsolationAwareMessageBoxIndirectW
 /* MessageBoxW skipped, as it is a popular C++ member function name. */
#define RegisterClassA IsolationAwareRegisterClassA
#define RegisterClassExA IsolationAwareRegisterClassExA
#define RegisterClassExW IsolationAwareRegisterClassExW
#define RegisterClassW IsolationAwareRegisterClassW
#define UnregisterClassA IsolationAwareUnregisterClassA
#define UnregisterClassW IsolationAwareUnregisterClassW
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* ISOLATION_AWARE_ENABLED */
#endif /* RC */


#if defined(__cplusplus)
} /* __cplusplus */
#endif

//*********************************************************************
//*                  Microsoft Windows                               **
//*        Copyright (c) Microsoft Corporation. All rights reserved. **
//*********************************************************************
#pragma once

#ifndef _RATINGS_H_
#define _RATINGS_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <winerror.h>
#include <shlwapi.h>

STDAPI RatingEnable(HWND hwndParent, LPCSTR pszUsername, BOOL fEnable);
STDAPI RatingEnableW(HWND hwndParent, LPCWSTR pszUsername, BOOL fEnable);
STDAPI RatingCheckUserAccess(_In_opt_ LPCSTR pszUsername, _In_opt_ LPCSTR pszURL, _In_opt_ LPCSTR pszRatingInfo,
    _In_reads_bytes_opt_(cbData) LPBYTE pData, DWORD cbData, _Out_opt_ void **ppRatingDetails);
STDAPI RatingCheckUserAccessW(_In_opt_ LPCWSTR pszUsername, _In_opt_ LPCWSTR pszURL, _In_opt_ LPCWSTR pszRatingInfo,
    _In_reads_bytes_opt_(cbData) LPBYTE pData, DWORD cbData, _Out_opt_ void **ppRatingDetails);
STDAPI RatingAccessDeniedDialog(HWND hDlg, _In_opt_ LPCSTR pszUsername, LPCSTR pszContentDescription, _Out_ void *pRatingDetails);
STDAPI RatingAccessDeniedDialogW(HWND hDlg, _In_opt_ LPCWSTR pszUsername, LPCWSTR pszContentDescription, _Out_ void *pRatingDetails);
STDAPI RatingAccessDeniedDialog2(HWND hDlg, _In_opt_ LPCSTR pszUsername, _Out_ void *pRatingDetails);
STDAPI RatingAccessDeniedDialog2W(HWND hDlg, _In_opt_ LPCWSTR pszUsername, _Out_ void *pRatingDetails);
STDAPI RatingFreeDetails(_In_opt_ void *pRatingDetails);
STDAPI RatingObtainCancel(HANDLE hRatingObtainQuery);
STDAPI RatingObtainQuery(LPCSTR pszTargetUrl, DWORD dwUserData,
    void (*fCallback)(DWORD dwUserData, HRESULT hr, LPCSTR pszRating, void *lpvRatingDetails),
    _Out_opt_ HANDLE *phRatingObtainQuery);
STDAPI RatingObtainQueryW(LPCWSTR pszTargetUrl, DWORD dwUserData,
    void (*fCallback)(DWORD dwUserData, HRESULT hr, LPCWSTR pszRating, void *lpvRatingDetails),
    _Out_opt_ HANDLE *phRatingObtainQuery);
STDAPI RatingSetupUI(HWND hDlg, LPCSTR pszUsername);
STDAPI RatingSetupUIW(HWND hDlg, LPCWSTR pszUsername);
#ifdef _INC_COMMCTRL
STDAPI RatingAddPropertyPage(PROPSHEETHEADER *ppsh);
#endif

STDAPI RatingAddToApprovedSites(HWND hDlg,
                                DWORD cbPasswordBlob,
                                _Inout_updates_bytes_(cbPasswordBlob) BYTE *pbPasswordBlob,
                                _In_ LPCWSTR lpszUrl,
                                BOOL fAlwaysNever,
                                BOOL fSitePage,
                                BOOL fApprovedSitesEnforced);

STDAPI RatingClickedOnPRFInternal(HWND hWndOwner, HINSTANCE /*p_hInstance*/, _In_ LPSTR lpszFileName, int nShow);
STDAPI RatingClickedOnRATInternal(HWND hWndOwner, HINSTANCE /*p_hInstance*/, _In_ LPSTR lpszFileName, int nShow);

STDAPI RatingEnabledQuery();
STDAPI RatingInit();
STDAPI_(void) RatingTerm();

// A way to check if ratings are installed. We still need to calling
// ratings dll to find out for sure but this allows us to delay load ratings.
__inline BOOL IS_RATINGS_ENABLED()
{
    TCHAR szSup[200];
    DWORD dwType;
    DWORD cbSize = sizeof(szSup);

    return (SHGetValue(HKEY_LOCAL_MACHINE,
                       TEXT("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Ratings"),
                       TEXT("Key"),
                       &dwType, szSup, &cbSize) == ERROR_SUCCESS);
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
// _RATINGS_H_


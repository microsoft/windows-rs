//*@@@+++@@@@******************************************************************
//
// Microsoft Windows Media
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//*@@@---@@@@******************************************************************
//
#pragma once
#ifndef _WMSDK_VALIDATE_H
#define _WMSDK_VALIDATE_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// This routine examines the file extension in the URL or file name that is passed
// in as an argument.  The routine returns S_OK if the file extension is included in a
// set of file extensions that the SDK is typically able to handle. or NS_E_INVALID_NAME
// if the file extension is not included in that set.
//
// This routine cannot be used to determine with absolute certainty if the SDK can
// handle a particular URL, as this cannot be known until the URL is opened.
//

HRESULT STDMETHODCALLTYPE WMCheckURLExtension( /* [in] */ LPCWSTR pwszURL );

//
// This routine examines the URL scheme that is passed in as an argument.  The routine
// returns S_OK if the URL scheme is included in a set of URL schemes that the SDK is
// typically able to handle. or NS_E_INVALID_NAME if the URL scheme is not included in
// that set.
//
// This routine cannot be used to determine with absolute certainty if the SDK can
// handle a particular URL, as this cannot be known until the URL is opened.
//

HRESULT STDMETHODCALLTYPE WMCheckURLScheme( /* [in] */ LPCWSTR pwszURLScheme );

//
// This routine returns S_OK if the data buffer looks like a file type that is supported
// by the SDK.  It returns NS_E_INVALID_DATA if the data buffer cannot be handled by the SDK.
// This routine may return a false positive, but will not return a false negative.
//

HRESULT STDMETHODCALLTYPE WMValidateData(
    /* [in] */ BYTE *pbData,
    /* [in, out] */ DWORD *pdwDataSize
    );

//
// This routine validates that a URL can be played in Offline mode.
// The parameter pwszLanguage can be set to a RFC-1766 language ID, or to NULL if
// any language is acceptable.
// The output parameter pfIsAvailableOffline is set to TRUE if the URL can be
// played in Offline mode.
//

HRESULT STDMETHODCALLTYPE WMIsAvailableOffline(
    /* [in] */ LPCWSTR pwszURL,
    /* [in] */ LPCWSTR pwszLanguage,
    /* [out] */ BOOL *pfIsAvailableOffline
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _WMSDK_VALIDATE_H */

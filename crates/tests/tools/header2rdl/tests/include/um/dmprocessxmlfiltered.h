//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

/*++


Module Name:

    dmprocessxmlfiltered.h

Purpose:

    Header for the DMProcessConfigXMLFiltered API

--*/

#pragma once

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#include <windows.h>

/// <summary>
///     This API wraps DMProcessConfigXML, blocking attempts to access it using WAP XML that 
///     consumes CSPs not allowed in the passed in filter list.
/// </summary>
/// <param name="pszXmlIn">             [in]  Null terminated input WAP XML</param>
/// <param name="rgszAllowedCspNodes">  [in]  Array of WCHAR* that specify which CSP nodes will be allowed to be invoked</param>
/// <param name="dwNumAllowedCspNodes"> [in]  Number of elements passed in rgszAllowedCspNode</param>
/// <param name="pbstrXmlOut">          [out] Resulting null terminated XML from comfiguration. 
///                                           Use SysFreeString to free the memory
/// </param>
/// <returns>
/// <para>Returns HRESULT type</para>
/// </returns>

STDAPI DMProcessConfigXMLFiltered(
    _In_                                PCWSTR  pszXmlIn,
    _In_reads_(dwNumAllowedCspNodes)    PCWSTR* rgszAllowedCspNodes,
    _In_                                DWORD   dwNumAllowedCspNodes,
    _Outptr_result_z_                   BSTR*   pbstrXmlOut);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


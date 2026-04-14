/*++
//
//    Copyright (c) Microsoft Corporation. All rights reserved.
//
//    File Name:
//        WindowsDefender.h
//
//    Abstract:
//        Windows Defender public API header file
//
//    History:
//      08/03/2006      SantanuC        Created 
//
--*/

#pragma once

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*++
//
// Function:
//  WDStatus - Returns the current status of Windows Defender.
//
// Parameters:
//  [out]
//  pfEnabled  - Returns status of Windows Defender as a boolean. 
//               TRUE means Windows Defender is in enabled status.
//               FALSE means Windows Defender is in disabled status.
//
// Returns:
//  S_OK         - Successfully retrieved Windows Defender status.
//  error code   - Any valid windows error code.
//
--*/

HRESULT WINAPI
WDStatus(
    _Out_ BOOL* pfEnabled);

/*++
//
// Function:
//  WDEnable - Turn on or off Windows Defender. Caller must be an administrator
//      (elevated administrator in case of vista) or local system to call this
//      function. Windows Defender will also validate proper signing of calling
//      process (and all the loaded modules) before allowing the caller to 
//      change the status. If the calling process image (or any loaded modules)
//      is not signed or is flagged as a threat by Windows Defender signature
//      then the call will fail with appropriate error code.
//
// Parameters:
//  [in]
//  fEnable  - Windows Defender status caller wants to set. TRUE will enable
//             Windows Defender. FALSE will disable Windows Defender.    
//
// Returns:
//  S_OK                - Successfully change the status of Windows Defender.
//  E_ACCESSDENIED      - Caller does not have sufficient permission or flagged
//                        as a threat by Windows Defender signature database.
//  TRUST_E_NOSIGNATURE - Caller identity is not verifiable through digital
//                        signing.
//  HRESULT_FROM_WIN32(ERROR_ACCESS_DISABLED_BY_POLICY) - Caller request  
//                        contradicts with the Windows Defender status set by 
//                        group policy. 
//  error code          - Any valid windows error code
//
--*/

HRESULT WINAPI
WDEnable(
    _In_ BOOL  fEnable);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif


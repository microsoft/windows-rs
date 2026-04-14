// Copyright (C) Microsoft Corporation. All rights reserved.
 
#ifdef _MSC_VER
#pragma once
#endif //_MSC_VER
 
#include <winapifamily.h>
 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
 
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
 
// Flags about key credential manager status
 
typedef enum KeyCredentialManagerOperationErrorStates {
    KeyCredentialManagerOperationErrorStateNone = 0x0,                  
    KeyCredentialManagerOperationErrorStateDeviceJoinFailure = 0x01,
    KeyCredentialManagerOperationErrorStateTokenFailure = 0x02,         
    KeyCredentialManagerOperationErrorStateCertificateFailure = 0x04,      
    KeyCredentialManagerOperationErrorStateRemoteSessionFailure = 0x08,    
    KeyCredentialManagerOperationErrorStatePolicyFailure = 0x10,           
    KeyCredentialManagerOperationErrorStateHardwareFailure = 0x20,         
    KeyCredentialManagerOperationErrorStatePinExistsFailure = 0x40         
} KeyCredentialManagerOperationErrorStates;
DEFINE_ENUM_FLAG_OPERATORS(KeyCredentialManagerOperationErrorStates);

// Operation types
 
typedef enum KeyCredentialManagerOperationType {
    KeyCredentialManagerProvisioning = 0,
    KeyCredentialManagerPinChange = 1,
    KeyCredentialManagerPinReset = 2,
} KeyCredentialManagerOperationType;
 
 
typedef struct KeyCredentialManagerInfo {
    GUID containerId;
} KeyCredentialManagerInfo;

 
STDAPI
KeyCredentialManagerGetOperationErrorStates(
    _In_ KeyCredentialManagerOperationType keyCredentialManagerOperationType,
    _Out_ BOOL* isReady,
    _Out_ KeyCredentialManagerOperationErrorStates* keyCredentialManagerOperationErrorStates);
 

STDAPI
KeyCredentialManagerShowUIOperation(
    _In_ HWND hWndOwner,
    _In_ KeyCredentialManagerOperationType keyCredentialManagerOperationType);
 

STDAPI
KeyCredentialManagerGetInformation(
    _Outptr_ KeyCredentialManagerInfo** keyCredentialManagerInfo);
 

#ifdef __cplusplus
extern "C" {
#endif //__cplusplus

VOID
KeyCredentialManagerFreeInformation(
    _In_ KeyCredentialManagerInfo* keyCredentialManagerInfo);

#ifdef __cplusplus
}
#endif //__cplusplus
 
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS4 */
 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma once
/*++

Copyright (c) Microsoft Corporation

Module Name:

    MDMRegistration.h

Abstract:

    This file contains structures, function signatures for 3rd Party
    management software that intends to interact with Windows MDE

Environment:

    User Mode - Win32

Notes:

--*/

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#ifndef _MDM_REG_
#define _MDM_REG_

#ifdef _MSC_VER
#pragma once
#endif

#include <winapifamily.h>
#include <winerror.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

////////////////////////////////////////////////////////////////////////////////
//
// Error Codes
//

#define E_DATATYPE_MISMATCH                               HRESULT_FROM_WIN32(ERROR_DATATYPE_MISMATCH)

// We will use this facility code for back end registration errors.
#define MDM_REGISTRATION_FACILITY_CODE                    25 // Errors from desktop 8.0-8.1
#define DEVICE_ENROLLER_FACILITY_CODE                     24 // Errors from 10+ (and phone 8+)

// Invalid Schema , Message Format Error from server.
#define MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR           MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 1)

// Server failed to authenticate the user.
#define MREGISTER_E_DEVICE_AUTHENTICATION_ERROR           MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 2)

// User is not authorized to enroll.
#define MREGISTER_E_DEVICE_AUTHORIZATION_ERROR            MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 3)

// User has no permission on the cert template or CA unreachable.
#define MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR        MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 4)
#define MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR          MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 4)

// Generic Failure from management server, such as DB access error.
#define MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR          MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 5)

// Unhandled exception from server.
#define MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR          MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 6)

// Unhandled exception from server.
#define MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR          MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 7)

// Unknown server error.
#define MREGISTER_E_DEVICE_UNKNOWN_ERROR                  MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 8)

// Another enrollment operation is currently underway.
#define MREGISTER_E_REGISTRATION_IN_PROGRESS              MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 9)

// NO LONGER USED
#define MREGISTER_E_DEVICE_ALREADY_REGISTERED             MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 10)
// Device is already enrolled.
#ifndef MENROLL_E_DEVICE_ALREADY_ENROLLED
#define MENROLL_E_DEVICE_ALREADY_ENROLLED                 MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 10)
#endif

// NO LONGER USED
#define MREGISTER_E_DEVICE_NOT_REGISTERED                 MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 11)
// Device is not enrolled.
#ifndef MENROLL_E_DEVICE_NOT_ENROLLED
#define MENROLL_E_DEVICE_NOT_ENROLLED                     MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 11)
#endif

// NO LONGER USED
#define MREGISTER_E_DISCOVERY_REDIRECTED                  MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 12)

// NO LONGER USED
#define MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR        MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 13)

// NO LONGER USED
#define MREGISTER_E_DISCOVERY_FAILED                      MAKE_HRESULT(SEVERITY_ERROR, MDM_REGISTRATION_FACILITY_CODE, 14)

// User already enrolled too many devices. Delete or unenroll old ones to fix this error (user can fix it without admin)
#ifndef MENROLL_E_DEVICECAPREACHED
#define MENROLL_E_DEVICECAPREACHED                        MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 19)
#endif

// Mobile device management generally not supported (would save an admin call)
#define MENROLL_E_NOTSUPPORTED                            MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 21)

// License of user is in bad state blocking enrollment (user still needs to call admin)
#define MENROLL_E_USERLICENSE                             MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 24)

// The server responded with a custom error string, see DeviceManagement-Enterprise-Diagnostics for details
#ifndef MENROLL_E_CUSTOMSERVERERROR
#define MENROLL_E_CUSTOMSERVERERROR                     MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 50)
#endif

// The server told us to come back later, see DeviceManagement-Enterprise-Diagnostics for details
#ifndef MENROLL_E_SERVER429
#define MENROLL_E_SERVER429                             MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 51)
#endif

// The user canceled the operation
#define MENROLL_E_USER_CANCELED                         MAKE_HRESULT(SEVERITY_ERROR, DEVICE_ENROLLER_FACILITY_CODE, 42)

// Struct returned by the discovery service containing
// the endpoints and information about the management service.
typedef struct _MANAGEMENT_SERVICE_INFO
{
    LPWSTR  pszMDMServiceUri;
    LPWSTR  pszAuthenticationUri;
}MANAGEMENT_SERVICE_INFO,*PMANAGEMENT_SERVICE_INFO;

#define DEVICEREGISTRATIONTYPE_MDM_ONLY 0
#define DEVICEREGISTRATIONTYPE_MAM 5
#define DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD 6
#define DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD 13

// Struct returned by the discovery service containing
// the endpoints and information about the management service.
typedef struct _MANAGEMENT_REGISTRATION_INFO
{
    BOOL fDeviceRegisteredWithManagement;
    DWORD dwDeviceRegistionKind;
    LPWSTR  pszUPN;
    LPWSTR  pszMDMServiceUri;
}MANAGEMENT_REGISTRATION_INFO, *PMANAGEMENT_REGISTRATION_INFO;

typedef enum _REGISTRATION_INFORMATION_CLASS {
    DeviceRegistrationBasicInfo = 1,

    MaxDeviceInfoClass  // MaxDeviceInfoClass should always be the last enum
} REGISTRATION_INFORMATION_CLASS, *PREGISTRATION_INFORMATION_CLASS;

/*++

Routine Description:

This function is used to check if this device is registered with an MDM service.

Arguments:

ppDeviceRegistion - details of the registration, free with HeapFree

Return Value:

HRESULT indicating success or failure.

--*/
HRESULT WINAPI
GetDeviceRegistrationInfo(
    _In_  REGISTRATION_INFORMATION_CLASS DeviceInformationClass,
    _Out_ PVOID* ppDeviceRegistrationInfo
);

/*++

Routine Description:

    This function is used to check if this device is registered with an MDM service.

Arguments:

    pfIsDeviceRegisteredWithManagement - will be set to TRUE if device is registered, FALSE otherwise.
    cchUPN - maximum length of pszUPN
    pszUPN - string parameter to return the UPN

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
IsDeviceRegisteredWithManagement(
    _Out_ BOOL *pfIsDeviceRegisteredWithManagement,
    _In_ DWORD cchUPN,
    _Out_opt_z_cap_(cchUPN) LPWSTR pszUPN
    );


/*++
Routine Description:

    This function is used to check if registration is allowed

Arguments:

    pfIsManagementRegistrationAllowed - will be set to TRUE if device is managed externally, FALSE otherwise.

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
IsManagementRegistrationAllowed(
    _Out_ BOOL *pfIsManagementRegistrationAllowed
    );

/*++
Routine Description:

    This function is used to check if device is not already enrolled and SKU license (SLAPI) allows enrollment

Arguments:

    isEnrollmentAllowed - TRUE if allowed

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
IsMdmUxWithoutAadAllowed(
    _Out_ BOOL* isEnrollmentAllowed
    );

/*++
Routine Description:

    This function is used to set if the device is externally managed

Arguments:

    IsManagedExternally - TRUE if device is managed externally, FALSE otherwise.

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
SetManagedExternally(
    _In_ BOOL IsManagedExternally
    );


/*++

Routine Description:

    This function is used to auto-discover the MDM service.

Arguments:

    pszUPN - email.

Return Value:

    HRESULT indicating success or failure.
    pszUPN - UPN (email address) of the user
    ppMgmtInfo - Struct containing MDM service URIs

--*/
HRESULT WINAPI
DiscoverManagementService(
    _In_z_              LPCWSTR                         pszUPN,
    _Out_               PMANAGEMENT_SERVICE_INFO*       ppMgmtInfo
    );


/*++

Routine Description:

This function is used to register a device with the MDM service synchronously.
It will get the MDM information, including authentication token from AAD

Arguments:

UserToken - The User to impersonate when attempting to get AAD token

Return Value:

HRESULT indicating success or failure.

--*/
HRESULT WINAPI
RegisterDeviceWithManagementUsingAADCredentials(HANDLE UserToken);


/*++

Routine Description:

This function is used to register a device with the MDM service synchronously.
It will get the MDM information, including authentication device token from AAD

Arguments:

None

Return Value:

HRESULT indicating success or failure.

--*/
HRESULT WINAPI
RegisterDeviceWithManagementUsingAADDeviceCredentials();


/*++

Routine Description:

This function is used to register a device with the MDM service synchronously.
It will automatically discover the MDM information, including MDM device enrollment resource URL and authentication device token from AAD

Arguments:

    MDMApplicationID   Unique ID of MDM application that is configured in Azure AD. 
                       Only required when multiple MDM applications are configured on the Azure AD.
                       The maximum length is 255 characters, excluding the terminal null.

Return Value:

HRESULT indicating success or failure.

--*/
HRESULT WINAPI
RegisterDeviceWithManagementUsingAADDeviceCredentials2(_In_opt_ PCWSTR MDMApplicationID);

/*++

Routine Description:

    This function is used to register a device with the MDM service synchronously.

Arguments:

    pszUPN - UPN (email address) of the user
    ppszMDMServiceUri - Management service URI.
    pszAccessToken - Access token obtained from the authentication service

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
RegisterDeviceWithManagement(
    _In_z_                  LPCWSTR     pszUPN,
    _In_z_                  LPCWSTR     ppszMDMServiceUri,
    _In_z_                  LPCWSTR     ppzsAccessToken
    );

/*++

Routine Description:

    This function is used to unregister a device synchronously.

Arguments:

    enrollmentID - enrollmentID of specific management server to unregister from. If null, then unregister from all.

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
UnregisterDeviceWithManagement(
    _In_opt_z_ LPCWSTR enrollmentID
    );

/*++

Routine Description:

    This API is used to get the config info associated with the provider ID.

Arguments:

    providerID - string parameter containing the providerID
    configStringBufferLength - pointer to the buffer length (size of configString in chars)
    configString - a buffer that will contain the ConfigInfo if the function completes successfully
        If the buffer specified by configString parameter is not large enough to hold the data, the function 
        returns ERROR_MORE_DATA and stores the required buffer size in the variable pointed to by configStringBufferLength. 
        In this case, the contents of the configString buffer are undefined. 

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
GetDeviceManagementConfigInfo(
    _In_ PCWSTR providerID,
    _Inout_ DWORD* configStringBufferLength,
    _Out_writes_to_opt_(*configStringBufferLength, *configStringBufferLength) PWSTR configString
    );
    

/*++

Routine Description:

    This API is used to set the config info associated with the provider ID.

Arguments:

    providerID - string parameter containing the providerID
    configString - string parameter containing the ConfigInfo(data to write)

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
SetDeviceManagementConfigInfo(
    _In_ PCWSTR providerID,
    _In_ PCWSTR configString
    );    


/*++

Routine Description:

    This API is used to get the management app hyperlink associated with the MDM service.

Arguments:

    cchHyperlink - maximum length of pszHyperlink
    pszHyperlink - string parameter to return a hyperlink to the MDM service app

Return Value:

    HRESULT indicating success or failure.

--*/
HRESULT WINAPI
GetManagementAppHyperlink(
    _In_ DWORD cchHyperlink,
    _Out_z_cap_(cchHyperlink) LPWSTR pszHyperlink
    );


#if (NTDDI_VERSION > NTDDI_WINBLUE || \
(NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))

/*++
Routine Description:

    This function is used to discover the MDM service given a candidate service URL

Arguments:

    pszUPN - email
    pszDiscoveryServiceCandidate - candidate service URL for for discovery

Return Value:

    HRESULT indicating success or failure.
    ppMgmtInfo - Struct containing MDM service URIs
--*/

HRESULT WINAPI
DiscoverManagementServiceEx(
    _In_z_              LPCWSTR                         pszUPN,
    _In_z_              LPCWSTR                         pszDiscoveryServiceCandidate,
    _Out_               PMANAGEMENT_SERVICE_INFO*       ppMgmtInfo
    );

#endif

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#endif // _MDM_REG_

#endif // NTDDI_VERSION >= NTDDI_WINBLUE

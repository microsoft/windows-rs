/*++

Copyright (c) 2008 Microsoft Corporation

Module Name:

    EhStorExtensions.h

Abstract:

    This module defines the Enhanced Storage WPD interfaces for silo drivers.

Environment:

    User mode only.

--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>
#include <propkey.h>

// ---------------------------------------------------------------------------
// 
// This section defines the device interfaces for Enhanced Storage devices.
// 
// ---------------------------------------------------------------------------
 
//
// {3897F6A4-FD35-4bc8-A0B7-5DBBA36ADAFA}
// 

DEFINE_GUID(
    GUID_DEVINTERFACE_ENHANCED_STORAGE_SILO, 
    0x3897f6a4, 0xfd35, 0x4bc8, 0xa0, 0xb7, 0x5d, 0xbb, 0xa3, 0x6a, 0xda, 0xfa);


// ---------------------------------------------------------------------------
// 
// This section defines all Commands, Parameters and Options essociated with: 
// WPD_CATEGORY_ENHANCED_STORAGE 
// 
// This category is for commands and parameters for storage functional objects.
// 
// ---------------------------------------------------------------------------

DEFINE_GUID(
    WPD_CATEGORY_ENHANCED_STORAGE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c);


// ---------------------------------------------------------------------------
//
// Authentication specific commands
//
// ---------------------------------------------------------------------------

// 
// ENHANCED_STORAGE_COMMAND_SILO_IS_AUTHENTICATION_SILO
//     This command will return whether or not the silo is an authentication silo.
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - The last status code for Authentication or UnAuthentication
//      ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO [VT_BOOLEAN] - TRUE if an Auth-C silo, FALSE otherwise
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_SILO_IS_AUTHENTICATION_SILO,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    6);


// 
// ENHANCED_STORAGE_COMMAND_SILO_GET_AUTHENTICATION_STATE
//     This command will return the authentication state for the silo.
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - The last status code for Authentication or UnAuthentication
//      ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE [VT_UI4]
//

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_SILO_GET_AUTHENTICATION_STATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    7);

//
// ENHANCED_STORAGE_COMMAND_SILO_ENUMERATE_SILOS
//     This command will enumerate the silo information for the specified silo type
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      [ Required ] ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE
//  Results: 
//     WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_SILO_ENUMERATE_SILOS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    11);

// ---------------------------------------------------------------------------
//
// Certificate specific commands
//
// ---------------------------------------------------------------------------


//
// ENHANCED_STORAGE_COMMAND_CERT_HOST_CERTIFICATE_AUTHENTICATION
//      This command will attempt to do a host authentication based on an HCh
//      (or XCh) from the device. If an index or certificate is specified, it
//      will use that certificate.
//      The default behavior is to authenticate any of the HCh certs present on
//      the device if possible (or XCh.)
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      [ Optional ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX [VT_UINT]
//      [ Optional ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE [VT_VECTOR | VT_UI1]
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_HOST_CERTIFICATE_AUTHENTICATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    101);

//
// DEVICE_AUTHENTICATION
// ENHANCED_STORAGE_COMMAND_CERT_DEVICE_CERTIFICATE_AUTHENTICATION
//      This command will attempt to do a device authentication operation. If
//      an index or certificate is specified, it will use that certificate. It
//      must be ASCm or ASCh.
//      The default behavior is to authenticate ASCm.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      [ Optional ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX [VT_UINT]
//      [ Optional ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE [VT_VECTOR | VT_UI1]
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_DEVICE_CERTIFICATE_AUTHENTICATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    102);

//
// ENHANCED_STORAGE_COMMAND_CERT_ADMIN_CERTIFICATE_AUTHENTICATION
//      This command will attempt to do an admin authentication based on the PCp
//      (or XCp) from the device.
//      This is an admin command - it requires both read and write access.
//  Access: 
//      (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      None
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_ADMIN_CERTIFICATE_AUTHENTICATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    103);

//
// ENHANCED_STORAGE_COMMAND_CERT_INITIALIZE_TO_MANUFACTURER_STATE
//      This command will attempt to initialized to the manufacturer state.
//      Requires PCp authentication.
//  Access: 
//      (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_INITIALIZE_TO_MANUFACTURER_STATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    104);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE_COUNT
//      This command will get the number of certificate slots on the device.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      none.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT [VT_UINT]
//

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE_COUNT,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    105);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE
//      This command will return the certificate at the certificate index location.
//      Index 0 is a special location that returns the ASCm chain in PKCS7 format.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      [ Required ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX [VT_UINT]
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH [VT_UINT]
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE [VT_VECTOR | VT_UI1]

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    106);

//
// ENHANCED_STORAGE_COMMAND_CERT_SET_CERTIFICATE
//      This command will set a certificate to the certificate index location.
//      Requires admin authentication.
//  Access: 
//      (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      [ Required ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX [VT_UINT]
//      [ Required ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE [VT_UINT]
//      [ Required ] ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY [VT_UINT]
//      [ Required ] ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX [VT_UINT]
//      [ Required ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE [VT_VECTOR | VT_UI1]
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_SET_CERTIFICATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    107);

//
// ENHANCED_STORAGE_COMMAND_CERT_CREATE_CERTIFICATE_REQUEST
//      This command will esk the device to create a certificate request. 
//      This will then be signed by the application's chosen CA.
//  Access: 
//      (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      None.
//  Results:
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST [VT_VECTOR | VT_UI1]
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_CREATE_CERTIFICATE_REQUEST,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    108);

//
// ENHANCED_STORAGE_COMMAND_CERT_UNAUTHENTICATION
//      This command will issue a command to set the cert silo to the
//      initialized state.
//  Access: 
//      (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION 
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_UNAUTHENTICATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    110);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITY
//      This command will issue a command to get a silo capability from the
//      silo. Data returned is in the format returned from the silo.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      [ Required ] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE [VT_UINT]
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY [VT_VECTOR | VT_UI1]
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITY,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    111);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITIES
//      This command will return the silo capabilities as a collection of
//      capabilities.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES [VT_UNKNOWN]
//      - ENHANCED_STORAGE_CAPABILITY_HASH_ALGS [VT_LPWSTR - semi-colon delimited] 
//      - ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY [VT_LPWSTR - semi-colon delimited]
//      - ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS [VT_LPWSTR - semi-colon delimited]
//      - ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE [ VT_BOOL ]
//      - ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING [ VT_BOOL ]
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITIES,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    112);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_ACT_FRIENDLY_NAME
//      This command will return the friendly name of the ACT containing the silo.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      [optional] ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME [VT_LPWSTR]
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_ACT_FRIENDLY_NAME,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    113);

//
// ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_GUID
//      This command will return the silo's GUID.
//  Access: 
//      (FILE_READ_ACCESS) 
//  Parameters: 
//      None.
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT
//      ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID [VT_LPWSTR]
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_GUID,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    114);


// ---------------------------------------------------------------------------
//
// Password specific commands
//
// ---------------------------------------------------------------------------


//
// ENHANCED_STORAGE_COMMAND_PASSWORD_AUTHORIZE_ACT_ACCESS
//     This command attempts to authenticate to the silo for ACT's data access
//  Access:  
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_AUTHORIZE_ACT_ACCESS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    203);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_UNAUTHORIZE_ACT_ACCESS
//     This command attempts to un-authenticate to the silo for ACT's data
//     access.
//  Access:  
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_PASSWORD
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION 
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_UNAUTHORIZE_ACT_ACCESS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    204);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_QUERY_INFORMATION
//     This command queries the current password silo information
//  Access: 
//     (FILE_READ_ACCESS) 
//  Parameters: 
//      none
//  Results:
//      ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE
//      ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO
//      ENHANCED_STORAGE_PROPERTY_ADMIN_HINT
//      ENHANCED_STORAGE_PROPERTY_USER_HINT
//      ENHANCED_STORAGE_PROPERTY_USER_NAME
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_QUERY_INFORMATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    205);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_CONFIG_ADMINISTRATOR
//     This command configures the administrator account
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_AUTH_REQUIRED_FOR_INITIALIZE
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_CONFIG_ADMINISTRATOR,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    206);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_CREATE_USER
//     This command creates a user account
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_USER_HINT
//     [ Required ] ENHANCED_STORAGE_PROPERTY_USER_NAME 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_CREATE_USER,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    207);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_DELETE_USER
//     This command deletes the existing user account
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//      none
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_DELETE_USER,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    208);


//
// ENHANCED_STORAGE_COMMAND_PASSWORD_CHANGE_PASSWORD
//     This command changes the password for adminstritor or user account
//  Access:
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_HINT
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER 
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_CHANGE_PASSWORD,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    209);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_INITIALIZE_USER_PASSWORD
//     This command initializes the existing user password 
//  Access:
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Required ] ENHANCED_STORAGE_PROPERTY_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD
//     [ Required ] ENHANCED_STORAGE_PROPERTY_NEW_HINT
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_INITIALIZE_USER_PASSWORD,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    210);

//
// ENHANCED_STORAGE_COMMAND_PASSWORD_START_INITIALIZE_TO_MANUFACTURER_STATE
//     This command starts the initialization process
//  Access: 
//     (FILE_READ_ACCESS | FILE_WRITE_ACCESS) 
//  Parameters: 
//     [ Optional ] ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER 
//  Results: 
//      WPD_PROPERTY_COMMON_HRESULT - status code for the operation
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_COMMAND_PASSWORD_START_INITIALIZE_TO_MANUFACTURER_STATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    211);


// ---------------------------------------------------------------------------
//
// This section defines all WPD Enhanced Storage Properties
//
// ---------------------------------------------------------------------------

// 
// ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE
//   [ VT_UI4 ] Authentication status of the Enhanced Storage Silo
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    1006);

// State definitions for ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE
#define ENHANCED_STORAGE_AUTHN_STATE_UNKNOWN                      0x00000000  // Initial setting before PnP entry and the silo state is unknow.
#define ENHANCED_STORAGE_AUTHN_STATE_NO_AUTHENTICATION_REQUIRED   0x00000001  // The silo has not been provisioned
#define ENHANCED_STORAGE_AUTHN_STATE_NOT_AUTHENTICATED            0x00000002  // The silo is not authenticated
#define ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATED                0x00000003  // The silo is authenticated
#define ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATION_DENIED        0x80000001  // Authentication was denied.
#define ENHANCED_STORAGE_AUTHN_STATE_DEVICE_ERROR                 0x80000002  // The silo timed out or another device error happened

// 
// ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO
//   [ VT_BOOL ] Is this silo an authentication silo? 
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    1009);

//
// ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION
//   [ VT_BOOL ] TRUE: temporary, FALSE: persistent
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    1010);

// ---------------------------------------------------------------------------
//
// Password silo specific properties
//
// ---------------------------------------------------------------------------

// 
// ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES
//   [ VT_UI4 ] Maximum number of password authentication failures
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2001);

// 
// ENHANCED_STORAGE_PROPERTY_PASSWORD
//   [ VT_BLOB ] The password to send or set
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_PASSWORD,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2004);

// 
// ENHANCED_STORAGE_PROPERTY_OLD_PASSWORD
//   [ VT_BLOB ] The password used for changing password.
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_OLD_PASSWORD,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2005);

//
// ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR
//   [ VT_BOOL ] TRUE: user, FALSE: admin
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2006);

//
// ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR
//   [ VT_BOOL ] TRUE: user, FALSE: admin
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2007);

// 
// ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD
//   [ VT_BLOB ] The new password.  Used to re-set the password
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2008);

// 
// ENHANCED_STORAGE_PROPERTY_USER_HINT
//   [ VT_LPCSTR ] The user hint
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_USER_HINT,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2009);

// 
// ENHANCED_STORAGE_PROPERTY_USER_NAME
//   [ VT_LPCSTR ] The friendly user name
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_USER_NAME,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2010);

// 
// ENHANCED_STORAGE_PROPERTY_ADMIN_HINT
//   [ VT_LPCSTR ] The admin hint
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_ADMIN_HINT,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2011);

// 
// ENHANCED_STORAGE_PROPERTY_SILO_NAME
//   [ VT_LPCSTR ] The friendly name for the silo
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_SILO_NAME,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c, 
    2012);

// 
// ENHANCED_STORAGE_PROPERTY_SILO_FRIENDLYNAME_SPECIFIED
//   [ VT_BOOL ] Flag to indicate if silo friendly name is given
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_SILO_FRIENDLYNAME_SPECIFIED,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2013);

// 
// ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO
//   [ VT_BLOB ] The password silo information
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2014);

// 
// ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER
//   [ VT_BLOB ] Security Identifier for the password silo device
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2015);

// 
// ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE
//   [ VT_UINT ] Query Silo Type
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2016);

// 
// ENHANCED_STORAGE_PROPERTY_QUERY_SILO_RESULTS
//   [ VT_BLOB ] Query Silo Properties result
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_QUERY_SILO_RESULTS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    2017);

// 
// Data format used for the password silo information property
// 

typedef struct _ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {

    BYTE CurrentAdminFailures;
    BYTE CurrentUserFailures;
    DWORD TotalUserAuthenticationCount;
    DWORD TotalAdminAuthenticationCount;

    BOOL FipsCompliant;
    BOOL SecurityIDAvailable;
    BOOL InitializeInProgress;
    BOOL ITMSArmed;
    BOOL ITMSArmable;
    BOOL UserCreated;
    BOOL ResetOnPORDefault;
    BOOL ResetOnPORCurrent;
    
    BYTE MaxAdminFailures;
    BYTE MaxUserFailures;

    DWORD TimeToCompleteInitialization;
    DWORD TimeRemainingToCompleteInitialization;
    DWORD MinTimeToAuthenticate;

    //
    // Capabilities (never changed)
    // 

    BYTE MaxAdminPasswordSize;
    BYTE MinAdminPasswordSize;
    BYTE MaxAdminHintSize;
    BYTE MaxUserPasswordSize;
    BYTE MinUserPasswordSize;
    BYTE MaxUserHintSize;
    BYTE MaxUserNameSize;
    BYTE MaxSiloNameSize;
    WORD MaxChallengeSize;

} ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION, *PENHANCED_STORAGE_PASSWORD_SILO_INFORMATION;

// ---------------------------------------------------------------------------
//
// Certificate silo specific properties.
//
// ---------------------------------------------------------------------------

//
// ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT
//   [ VT_UINT ] The number of certificate slots available on the device
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3001);

//
// ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT
//   [ VT_UINT ] The number of certificate slots in use on the device
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3002);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX
//   [ VT_UINT ] The index for the certificate slot on the device
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3003);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE
//  [ VT_UINT ] The type of certificate
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3004);

//
// Certificate Types defines
//
#define CERT_TYPE_EMPTY     0x00    // No certificate
#define CERT_TYPE_ASCm      0x01    // Manufacturer's certificate (ASCm)
#define CERT_TYPE_PCp       0x02    // Provisioning Certificate (PCp)
#define CERT_TYPE_ASCh      0x03    // Authentication Silo Certificate (ASCh)
#define CERT_TYPE_HCh       0x04    // Host certificate (HCh)
#define CERT_TYPE_SIGNER    0x06    // Signer certificate (SCh)

//
// ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY
//  [ VT_UINT ] The validation policy for the certificate
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3005);

//
// Validation Policy values
// - None: the corresponding private key of the stored certificate shall be used
//         for authentication.
// - Basic: the certificate and certificate chain conforms to the basic validation
//         policy.
// - Extended: the certificate chain conforms to the extended validation policy.
//         The use of this validation policy must result in an error condition
//         of the Authentication Silo if it does not support parsing of certificate
//         extensions.
//
#define CERT_VALIDATION_POLICY_RESERVED             0x00
#define CERT_VALIDATION_POLICY_NONE                 0x01
#define CERT_VALIDATION_POLICY_BASIC                0x02
#define CERT_VALIDATION_POLICY_EXTENDED             0x03

//
// ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX
//  [ VT_UINT ] The index of the next valid cert
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3006);

//
// ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX
//  [ VT_UINT ] The index of the next valid cert of same type
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3007);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH
//  [ VT_UINT ] Length of the certificate in bytes
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3008);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE
//   [ VT_VECTOR | VT_UI1 ] The certificate buffer in X.509 format
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3009);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST
//   [ VT_VECTOR | VT_UI1 ] The certificate request buffer
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3010);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE
//  [ VT_UINT ] Silo capability type
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3011);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY
//  [ VT_VECTOR | VT_UINT ] The "raw" capability data return from the silo
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3012);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES
//  [ VT_UNKNOWN ] The certificate silo capabilities returned in a collection
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3013);

//
// Cert Silo Capability Types
// 
#define CERT_CAPABILITY_HASH_ALG                    0x1
#define CERT_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY 0x2
#define CERT_CAPABILITY_SIGNATURE_ALG               0x3
#define CERT_CAPABILITY_CERTIFICATE_SUPPORT         0x4
#define CERT_CAPABILITY_OPTIONAL_FEATURES           0x5
#define CERT_MAX_CAPABILITY                         0xFF    // Maximum capability value

//
// Supported identifiers defined in 1667 spec
//
#define CERT_RSA_1024_OID "1.2.840.113549.1.1.1,1024"
#define CERT_RSA_2048_OID "1.2.840.113549.1.1.1,2048"
#define CERT_RSA_3072_OID "1.2.840.113549.1.1.1,3072"
#define CERT_RSASSA_PSS_SHA1_OID "1.2.840.113549.1.1.10,1.3.14.3.2.26"
#define CERT_RSASSA_PSS_SHA256_OID "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.1"
#define CERT_RSASSA_PSS_SHA384_OID "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.2"
#define CERT_RSASSA_PSS_SHA512_OID "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.3"

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME
//  [ VT_LPWSTR ] The certificate silo's ACT friendly name
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3014);

//
// ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID
//  [ VT_LPWSTR ] The certificate silo GUID
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3015);

//
// ENHANCED_STORAGE_PROPERTY_SIGNER CERTIFICATE_INDEX
//   [ VT_UINT ] The index for the signer certificate slot on the device
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    3016);


// ---------------------------------------------------------------------------
//
// Silo capability specific properties.
//
// ---------------------------------------------------------------------------


//
// ENHANCED_STORAGE_CAPABILITY_HASH_ALGS
//  [VT_LPWSTR] Semi-colon delimited string of hash algorithm identifiers
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_CAPABILITY_HASH_ALGS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    4001);

//
// ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY
//  [VT_LPWSTR] Semi-colon delimited string of asymmetric key cryptography supported
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    4002);

//
// ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS
//  [VT_LPWSTR] Semi-colon delimited string of signing algorithm identifiers
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    4003);

//
// ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE
//  [ VT_BOOL ] Boolean indicating whether silo can render user data unusable
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    4004);

//
// ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING
//  [ VT_BOOL ] Boolean indicating whether certificate extension fields are supported
// 

DEFINE_PROPERTYKEY(
    ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING,
    0x91248166, 0xb832, 0x4ad4, 0xba, 0xa4, 0x7c, 0xa0, 0xb6, 0xb2, 0x79, 0x8c,
    4005);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


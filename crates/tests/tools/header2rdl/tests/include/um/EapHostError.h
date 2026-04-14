//////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Scenario-specific error codes, reported by EapHost and Eap Method DLLs.
//
///////////////////////////////////////////////////////////////////////////////

#ifndef EAPHOSTERROR_H
#define EAPHOSTERROR_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#define FACILITY_EAP_MESSAGE           0x0842  // 66L + MessageId bit


// Errors common across different groups.
#define _EAP_CERT_FIRST     (+ 0x0)
#define _EAP_CERT_LAST      (+ 0xF)

#define _EAP_CERT_NOT_FOUND           (+ 0x1)
#define _EAP_CERT_INVALID             (+ 0x2)
#define _EAP_CERT_EXPIRED             (+ 0x3)
#define _EAP_CERT_REVOKED             (+ 0x4)
#define _EAP_CERT_OTHER_ERROR         (+ 0x5)
#define _EAP_CERT_REJECTED            (+ 0x6)
#define _EAP_CERT_NAME_REQUIRED       (+ 0x7)

#define _EAP_GENERAL_FIRST  (+ 0x10)   // (+16)
#define _EAP_GENERAL_LAST   (+ 0x3F)   // (+31)

//
// Individual groups of Eap-related Errors.
//

#define EAP_GROUP_MASK       0x0000FF00L


#define EAP_E_EAPHOST_FIRST  0x80420000L
#define EAP_E_EAPHOST_LAST   0x804200FFL
#define EAP_I_EAPHOST_FIRST  0x80420000L
#define EAP_I_EAPHOST_LAST   0x804200FFL

// Cert-Store can't be accessed on either authenticator or peer
#define EAP_E_CERT_STORE_INACCESSIBLE               0x80420010  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 0)

// Requested EAP-method is not installed
#define EAP_E_EAPHOST_METHOD_NOT_INSTALLED          0x80420011  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 1)

// 
#define EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET  0x80420012  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 2)

// EAPHost not able to communicate with EAPQEC on a NAP enabled client
#define EAP_E_EAPHOST_EAPQEC_INACCESSIBLE           0x80420013  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 3)

// EAPHost returns this error, if the authenticator fails the authentication after peer sent its identity
#define EAP_E_EAPHOST_IDENTITY_UNKNOWN              0x80420014  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 4)

// EAPHost returns this error on authentication failure.
#define EAP_E_AUTHENTICATION_FAILED                 0x80420015  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 5)

// EAPHost returns this error, when the client & the server aren't configured with compatible EAP-types.
#define EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED        0x40420016  // (EAP_I_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 6)

// EAPMethod received an EAP packet that can not be processed
#define EAP_E_EAPHOST_METHOD_INVALID_PACKET         0x80420017  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 7)

// EAPHost received a packet that can not be processed. 
#define EAP_E_EAPHOST_REMOTE_INVALID_PACKET         0x80420018  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 8)

// EAPHost ConfigSchema validation failed
#define EAP_E_EAPHOST_XML_MALFORMED                 0x80420019  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 9)

//Method doesn't support SSO for the provided config
#define EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO                  0x8042001A  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + A)

// EAPHost returns this error, when a configured EAP-method does not support a requested operation (procedure call)
#define EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED                 0x80420020  // (EAP_E_EAPHOST_FIRST + _EAP_GENERAL_FIRST + 10)


#define EAP_E_USER_FIRST  0x80420100L
#define EAP_E_USER_LAST   0x804201FFL
#define EAP_I_USER_FIRST  0x40420100L
#define EAP_I_USER_LAST   0x404201FFL

// EAPHost could not find user-certificate for authentication    
#define EAP_E_USER_CERT_NOT_FOUND           0x80420100  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_NOT_FOUND)

// user-cert being user for authentication does not have proper usage (EKU) set 
#define EAP_E_USER_CERT_INVALID             0x80420101  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_INVALID)

// EAPhost found user-cert which has expired already
#define EAP_E_USER_CERT_EXPIRED             0x80420102  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_EXPIRED)

// user-cert being used for authentication has been revoked
#define EAP_E_USER_CERT_REVOKED             0x80420103  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_REVOKED)

// unknown error occured with user-cert being used for authentication
#define EAP_E_USER_CERT_OTHER_ERROR         0x80420104  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_OTHER_ERROR)

// Authenticator rejected user-cert for authentication
#define EAP_E_USER_CERT_REJECTED            0x80420105  // (EAP_E_USER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_REJECTED)

// Received EAP-Failure after Identity exchange:  There is likely a problem with the authenticating user's account.
#define EAP_I_USER_ACCOUNT_OTHER_ERROR      0x40420110  // (EAP_I_USER_FIRST + _EAP_GENERAL_FIRST + 0

// Authenticator rejected user credentials for authentication 
#define EAP_E_USER_CREDENTIALS_REJECTED     0x80420111  // (EAP_E_USER_FIRST + _EAP_GENERAL_FIRST + 1)

// Authenticator rejected user credentials for authentication 
#define EAP_E_USER_NAME_PASSWORD_REJECTED   0x80420112  // (EAP_E_USER_FIRST + _EAP_GENERAL_FIRST + 2)

//No Smart Card Reader Present
#define EAP_E_NO_SMART_CARD_READER       0x80420113  // (EAP_E_USER_FIRST + _EAP_GENERAL_FIRST + 3)


#define EAP_E_SERVER_FIRST  0x80420200L
#define EAP_E_SERVER_LAST   0x804202FFL

// EAPHost could not find server-certificate for authentication    
#define EAP_E_SERVER_CERT_NOT_FOUND         0x80420200  // (EAP_E_SERVER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_NOT_FOUND)

// server-cert being user for authentication does not have proper usage (EKU) set  
#define EAP_E_SERVER_CERT_INVALID           0x80420201  // (EAP_E_SERVER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_INVALID)

// EAPhost found server-cert which has expired already
#define EAP_E_SERVER_CERT_EXPIRED           0x80420202  // (EAP_E_SERVER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_EXPIRED)

// server-cert being used for authentication has been revoked
#define EAP_E_SERVER_CERT_REVOKED           0x80420203  // (EAP_E_SERVER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_REVOKED)

// unknown error occured with server-cert being used for authentication
#define EAP_E_SERVER_CERT_OTHER_ERROR       0x80420204  // (EAP_E_SERVER_FIRST + _EAP_CERT_FIRST + _EAP_CERT_OTHER_ERROR)


#define EAP_E_USER_ROOT_CERT_FIRST  0x80420300L
#define EAP_E_USER_ROOT_CERT_LAST   0x804203FFL


// EAPHost could not find a certificate in trusted-root cert-store for user cert velidation
#define EAP_E_USER_ROOT_CERT_NOT_FOUND      0x80420300  // (EAP_E_USER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_NOT_FOUND)

// The authentication failed because the root certificate used for this network is invalid
#define EAP_E_USER_ROOT_CERT_INVALID        0x80420301  // (EAP_E_USER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_INVALID)

// Trusted root certificate needed for user-cert validation has been expired.
#define EAP_E_USER_ROOT_CERT_EXPIRED        0x80420302  // (EAP_E_USER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_EXPIRED)

#define EAP_E_SERVER_ROOT_CERT_FIRST  0x80420400L
#define EAP_E_SERVER_ROOT_CERT_LAST   0x804204FFL

// EAPHost could not find a Root certificate in trusted-root cert-store for server cert velidation
#define EAP_E_SERVER_ROOT_CERT_NOT_FOUND        0x80420400  // (EAP_E_SERVER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_NOT_FOUND)

// The authentication failed because the server certificate required for this network on the server computer is invalid
#define EAP_E_SERVER_ROOT_CERT_INVALID          0x80420401  // (EAP_E_SERVER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_INVALID)

// The authentication failed because the certificate on the server computer does not have a server name specified
#define EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED    0x80420406  // (EAP_E_SERVER_ROOT_CERT_FIRST + _EAP_CERT_FIRST + _EAP_CERT_NAME_REQUIRED)

// The authentication failed because there is no valid SIM for authentication
#define EAP_E_SIM_NOT_VALID                     0x80420500

// Alternate names for certain errors.
#define EAP_METHOD_INVALID_PACKET  EAP_E_EAPHOST_METHOD_INVALID_PACKET
#define EAP_INVALID_PACKET         EAP_E_EAPHOST_REMOTE_INVALID_PACKET

// This is not a fixed GUID when it reaches supplicant, but 1st portion will be 
// filled by Generic Win32/Ras error. This helps create unique GUID for every 
// unique error that we don't understand. This helps collect SQM data.
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Default \
        = { 0x00000000, 0x0000, 0x0000, 0, 0, 0, 0, 0, 0, 0, 0 };


EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_MethodDLLNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 1 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactSysadmin \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 2 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_CertStoreInaccessible \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 4 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Generic_AuthFailure \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 1, 4 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_IdentityUnknown \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 2, 4 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_SimNotValid \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 3, 4 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_CertExpired \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 5 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_CertInvalid \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 6 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_CertNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 7 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_CertRevoked \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 8 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_CertOtherError \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 1, 8 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertExpired \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 9 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertInvalid \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xA } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xB } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertOtherError \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xC } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertRejected \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xD } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CertRevoked \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xE } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_Account_OtherProblem \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 1, 0xE } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_CredsRejected \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 2, 0xE } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_Root_CertExpired \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0xF } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_Root_CertInvalid \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x10 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_User_Root_CertNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x11 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_Root_CertNameRequired \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x12 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Server_Root_CertNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 1, 0x12 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 2, 0x12 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_EapQecInaccessible \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 3, 0x12 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_Server_ClientSelectServerCert \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x18 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_User_AuthFailure \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x19 } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_User_GetNewCert \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1A } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_User_SelectValidCert \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1B } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_Retry_Authentication \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 1, 0x1B } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_EapNegotiationFailed \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1C } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_XmlMalformed \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1D } };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_MethodDoesNotSupportOperation \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1E } };
      
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_AuthFailure \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x1F } };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_IdentityUnknown \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x20 }};
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_NegotiationFailed \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x21}};        
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_MethodNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x22} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_RestartNap \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x23} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x24} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x25} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_RootCertInvalid \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x26} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_RootCertNotFound \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x27} };        

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_RootExpired \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x28} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_CertNameAbsent \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x29} };
        
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x2A} };

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_No_SmartCardReader_Found \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x2B }};

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_ContactAdmin_InvalidUserCert \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x2C }};

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_Method_Not_Support_Sso \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x2D }};

EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Repair_No_ValidSim_Found \
        = { 0x9612fc67, 0x6150, 0x4209, { 0xa8, 0x5e, 0xa8, 0xd8, 0, 0, 0, 0x2E }};

 EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Help_ObtainingCerts \
        = { 0xf535eea3, 0x1bdd, 0x46ca, { 0xa2, 0xfc, 0xa6, 0x65, 0x59, 0x39, 0xb7, 0xe8 } };
 
// This currently is a generic networking troubleshooting help page, not EAP specific.
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Help_Troubleshooting \
        = { 0x33307acf, 0x0698, 0x41ba, { 0xb0, 0x14, 0xea, 0x0a, 0x2e, 0xb8, 0xd0, 0xa8 } };

// This is used when the method does not supportSSO for provided config.
EXTERN_C const CLSID DECLSPEC_SELECTANY GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso \
        = { 0xda18bd32, 0x004F, 0x41fa, { 0xae, 0x08, 0x0b, 0xc8, 0x5e, 0x58, 0x45, 0xac } };

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAPHOSTERROR_H


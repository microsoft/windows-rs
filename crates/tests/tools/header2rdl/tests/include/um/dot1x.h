//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef __DOT1X_H_
#define __DOT1X_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#include <l2cmn.h>
#include <eaptypes.h>

#if __midl
#define V1_ENUM [v1_enum]
#else
#define V1_ENUM
#endif

#pragma warning(disable:4214) // bit field types other than int


/*
    The identity that is being used by the 1X module. This is a function of the
    onex auth mode and system triggers (e.g. user logon/logoff)
    */  
typedef V1_ENUM enum _ONEX_AUTH_IDENTITY 
{
    OneXAuthIdentityNone,
    OneXAuthIdentityMachine,
    OneXAuthIdentityUser,
    OneXAuthIdentityExplicitUser,
    OneXAuthIdentityGuest,
    OneXAuthIdentityInvalid
} ONEX_AUTH_IDENTITY, *PONEX_AUTH_IDENTITY;

/*
    The 1X authentication status. Each of the auth status corresponds to one or
    more of the internal 1X states. 
    */
typedef V1_ENUM enum _ONEX_AUTH_STATUS
{
    OneXAuthNotStarted,
    OneXAuthInProgress,
    OneXAuthNoAuthenticatorFound,
    OneXAuthSuccess,
    OneXAuthFailure,
    OneXAuthInvalid
} ONEX_AUTH_STATUS, *PONEX_AUTH_STATUS;

/*
    1X specific reason codes
    */
typedef V1_ENUM enum _ONEX_REASON_CODE
{
    ONEX_REASON_CODE_SUCCESS = 0,
    ONEX_REASON_START = L2_REASON_CODE_ONEX_BASE,
    ONEX_UNABLE_TO_IDENTIFY_USER,
    ONEX_IDENTITY_NOT_FOUND,
    ONEX_UI_DISABLED,
    ONEX_UI_FAILURE,
    ONEX_EAP_FAILURE_RECEIVED,
    ONEX_AUTHENTICATOR_NO_LONGER_PRESENT,
    ONEX_NO_RESPONSE_TO_IDENTITY,
    ONEX_PROFILE_VERSION_NOT_SUPPORTED,
    ONEX_PROFILE_INVALID_LENGTH,
    ONEX_PROFILE_DISALLOWED_EAP_TYPE,
    ONEX_PROFILE_INVALID_EAP_TYPE_OR_FLAG,
    ONEX_PROFILE_INVALID_ONEX_FLAGS,
    ONEX_PROFILE_INVALID_TIMER_VALUE,
    ONEX_PROFILE_INVALID_SUPPLICANT_MODE,
    ONEX_PROFILE_INVALID_AUTH_MODE,
    ONEX_PROFILE_INVALID_EAP_CONNECTION_PROPERTIES,
    ONEX_UI_CANCELLED,
    ONEX_PROFILE_INVALID_EXPLICIT_CREDENTIALS,
    ONEX_PROFILE_EXPIRED_EXPLICIT_CREDENTIALS,
    ONEX_UI_NOT_PERMITTED    
} ONEX_REASON_CODE, *PONEX_REASON_CODE;

/*
    Events for which 1X sends notifications to MSM

    On getting an event notification the callee should switch on the dwEvent.
    
    If dwEvent =  OneXNotificationTypeResultUpdate, pvEventData points to an
    ONEX_RESULT_UPDATE_DATA structure
    
    If dwEvent = OneXNotificationTypeOneXUserIdentified, pvEventData points to
    a ONEX_USER_INFO structure
    
    If dwEvent = OneXNotificationTypeGotOneXIdentityString, pvEventData points to
    the identity (LPWSTR) being used for the 1X authentication
    
    If dwEvent = OneXNotificationTypeFallenBackOnGuest pvEventData is NULL
    
    if dwEvent = OneXNotificationTypeAuthRestarted, pvEventData points
    to the restart reason    
    */
typedef V1_ENUM enum _ONEX_NOTIFICATION_TYPE
{
    OneXPublicNotificationBase = 0,
    OneXNotificationTypeResultUpdate,
    OneXNotificationTypeAuthRestarted,
    OneXNotificationTypeEventInvalid,
    OneXNumNotifications = OneXNotificationTypeEventInvalid
} ONEX_NOTIFICATION_TYPE, *PONEX_NOTIFICATION_TYPE;

/*
    The following list enumerates the reason for the 1X authentication process 
    getting restarted
    */
typedef V1_ENUM enum _ONEX_AUTH_RESTART_REASON
{
    OneXRestartReasonPeerInitiated,
    OneXRestartReasonMsmInitiated,
    OneXRestartReasonOneXHeldStateTimeout,
    OneXRestartReasonOneXAuthTimeout,
    OneXRestartReasonOneXConfigurationChanged,
    OneXRestartReasonOneXUserChanged,
    OneXRestartReasonQuarantineStateChanged,
    OneXRestartReasonAltCredsTrial,
    OneXRestartReasonInvalid
} ONEX_AUTH_RESTART_REASON, *PONEX_AUTH_RESTART_REASON;

typedef struct _ONEX_VARIABLE_BLOB 
{
    DWORD dwSize; 
    DWORD dwOffset;
} ONEX_VARIABLE_BLOB, *PONEX_VARIABLE_BLOB;

/*
    The set of parameters that define the authentication context for 1X
    */
typedef struct _ONEX_AUTH_PARAMS
{
    BOOL fUpdatePending;
    ONEX_VARIABLE_BLOB oneXConnProfile;    
    ONEX_AUTH_IDENTITY authIdentity;
    DWORD dwQuarantineState;
    
    DWORD fSessionId:1;
    DWORD fhUserToken:1;
    DWORD fOnexUserProfile:1;
    DWORD fIdentity:1;
    DWORD fUserName:1;
    DWORD fDomain:1;

    DWORD dwSessionId;

    HANDLE hUserToken;
    
    ONEX_VARIABLE_BLOB OneXUserProfile;
    ONEX_VARIABLE_BLOB Identity;
    ONEX_VARIABLE_BLOB UserName;
    ONEX_VARIABLE_BLOB Domain;
} ONEX_AUTH_PARAMS, *PONEX_AUTH_PARAMS;

typedef struct _ONEX_EAP_ERROR
{
	DWORD dwWinError;
	EAP_METHOD_TYPE type;
	DWORD dwReasonCode;

	GUID   rootCauseGuid;
	GUID   repairGuid;
	GUID   helpLinkGuid;

	DWORD fRootCauseString:1;
	DWORD fRepairString:1;
	
	ONEX_VARIABLE_BLOB RootCauseString;
	ONEX_VARIABLE_BLOB RepairString;
} ONEX_EAP_ERROR, *PONEX_EAP_ERROR;

typedef struct _ONEX_STATUS
{
    ONEX_AUTH_STATUS authStatus;
    /*
        Any errors that happened during the authentication are indicated as 
        error and reason codes below
        */
    DWORD dwReason;
    DWORD dwError;    
} ONEX_STATUS, *PONEX_STATUS;

typedef V1_ENUM enum _ONEX_EAP_METHOD_BACKEND_SUPPORT
{
    OneXEapMethodBackendSupportUnknown,
    OneXEapMethodBackendSupported,
    OneXEapMethodBackendUnsupported
}ONEX_EAP_METHOD_BACKEND_SUPPORT;

typedef struct _ONEX_RESULT_UPDATE_DATA
{
    ONEX_STATUS oneXStatus;
    ONEX_EAP_METHOD_BACKEND_SUPPORT BackendSupport;
    BOOL fBackendEngaged;

    DWORD fOneXAuthParams:1;
    DWORD fEapError:1;
    
    ONEX_VARIABLE_BLOB authParams;
    ONEX_VARIABLE_BLOB eapError;
} ONEX_RESULT_UPDATE_DATA, *PONEX_RESULT_UPDATE_DATA;

typedef struct _ONEX_USER_INFO
{
    ONEX_AUTH_IDENTITY authIdentity;

    DWORD fUserName:1;
    DWORD fDomainName:1;

    ONEX_VARIABLE_BLOB UserName;
    ONEX_VARIABLE_BLOB DomainName;
} ONEX_USER_INFO, *PONEX_USER_INFO;

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif 

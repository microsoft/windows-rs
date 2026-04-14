////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation.
//
// SYNOPSIS
//
//    IDL source for interaction with EAPHost supplicants.
//
////////////////////////////////////////////////////////////

#ifndef EAPHOSTPEERTYPES_H
#define EAPHOSTPEERTYPES_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Request to provide guest access.
#define EAP_PEER_FLAG_GUEST_ACCESS           0x00000040 

typedef
#ifdef __midl
   [v1_enum]
#endif


// Possible reasons for which EAP-method can call getResult from EAPhost 
enum tagEapHostPeerMethodResultReason
{
   // if method has obtained success from some
   EapHostPeerMethodResultAltSuccessReceived = 1,

   // method timedout waiting for response
   EapHostPeerMethodResultTimeout,

   // normal completion of authentication process
   EapHostPeerMethodResultFromMethod
} EapHostPeerMethodResultReason;

   
typedef
#ifdef __midl
   [v1_enum]
#endif
enum tagEapHostPeerResponseAction
{
        // Discard the request as EAP cannot use it.
        EapHostPeerResponseDiscard = 0,
        // Send the packet to the authenticator
        EapHostPeerResponseSend,
        // Respond to EAP by acting on the returned attributes
        EapHostPeerResponseResult,
        // Invoke appropriate UI based on the context
        EapHostPeerResponseInvokeUi,
        // Display a string to the user received as part of a notification 
        // request
        // EapHostPeerResponseNotification,
        // The supplicant needs to take an action based on the context.
        EapHostPeerResponseRespond,
        // The supplicant needs to restart authentication with the same session. 
   //EapHostPeerResponseResetAuthentication,
        // The session could not be found. So the supplicant either needs to 
        // start session again with the same packet or discard the packet. 
   EapHostPeerResponseStartAuthentication,
        // The supplicant needs to take no action at all.
        EapHostPeerResponseNone
} EapHostPeerResponseAction;


typedef
#ifdef __midl
   [v1_enum]
#endif
enum tagEapHostPeerAuthParams {
   EapHostPeerAuthStatus = 1,    // current auth status
   EapHostPeerIdentity,           // obtain the inner method id
   EapHostPeerIdentityExtendedInfo, // obtained from Identity packet
                                                                  // This includes NLA information for wireless
   EapHostNapInfo //obtain the Nap details
} EapHostPeerAuthParams;


typedef
#ifdef __midl
   [v1_enum]
#endif

// possible values for EAP status during authentication process
enum _EAPHOST_AUTH_STATUS
{
   EapHostInvalidSession = 0,
   EapHostAuthNotStarted,
   EapHostAuthIdentityExchange,
   EapHostAuthNegotiatingType,
   EapHostAuthInProgress,
   EapHostAuthSucceeded,
   EapHostAuthFailed
} EAPHOST_AUTH_STATUS;


// describes the current authentication info through different stages
// of EAP authentication process.
typedef struct _EAPHOST_AUTH_INFO
{
   EAPHOST_AUTH_STATUS status;
   DWORD dwErrorCode;
   DWORD dwReasonCode;
} EAPHOST_AUTH_INFO;


// Describes the isolation state of a machine, i.e. 
// whether connectivity is affected.
typedef
#ifdef __midl
   [v1_enum]
#endif
enum _ISOLATION_STATE
{
   ISOLATION_STATE_UNKNOWN = 0,
   ISOLATION_STATE_NOT_RESTRICTED = 1,
   ISOLATION_STATE_IN_PROBATION = 2,
   ISOLATION_STATE_RESTRICTED_ACCESS = 3
} ISOLATION_STATE;

// This carries the result information passed from EAPHost to EAP-method,
// when EAP-method calls GetResult
typedef struct tagEapHostPeerMethodResult
{
   BOOL fIsSuccess;
   UINT32 dwFailureReasonCode;
   BOOL fSaveConnectionData;
   UINT32 dwSizeofConnectionData;
#ifdef __midl   
   [size_is(dwSizeofConnectionData)] BYTE* pConnectionData;
#else
   BYTE* pConnectionData;
#endif
   BOOL fSaveUserData;
   UINT32 dwSizeofUserData;
#ifdef __midl   
   [size_is(dwSizeofUserData)] BYTE* pUserData;
#else
   BYTE* pUserData;
#endif
   EAP_ATTRIBUTES* pAttribArray;
   ISOLATION_STATE isolationState;      
   EAP_METHOD_INFO* pEapMethodInfo;
   EAP_ERROR* pEapError;
} EapHostPeerMethodResult;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //EAPHOSTPEERTYPES_IDL


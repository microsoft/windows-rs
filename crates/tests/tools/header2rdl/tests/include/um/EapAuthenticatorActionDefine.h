////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation.
//
// SYNOPSIS
//
//    IDL source for interaction with EAPHost supplicants.
//
////////////////////////////////////////////////////////////

#ifndef EAPAUTHENTICATORACTIONDEFINE_H
#define EAPAUTHENTICATORACTIONDEFINE_H

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

typedef
#ifdef __midl
   [v1_enum]
#endif
enum _EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION
{
   // Discard the request as EAP cannot use it.
   EAP_METHOD_AUTHENTICATOR_RESPONSE_DISCARD = 0,
   // Send the packet to the authenticator
   EAP_METHOD_AUTHENTICATOR_RESPONSE_SEND,
   // Respond to EAP by acting on the returned attributes
   EAP_METHOD_AUTHENTICATOR_RESPONSE_RESULT,
   // The supplicant needs to take an action based on the context.
   EAP_METHOD_AUTHENTICATOR_RESPONSE_RESPOND,
   EAP_METHOD_AUTHENTICATOR_RESPONSE_AUTHENTICATE,
   EAP_METHOD_AUTHENTICATOR_RESPONSE_HANDLE_IDENTITY
   // The supplicant needs to take no action at all.
   //EAP_METHOD_AUTHENTICATOR_RESPONSE_NONE
} EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION;

typedef struct _EAP_METHOD_AUTHENTICATOR_RESULT
{
   BOOL fIsSuccess;
   DWORD dwFailureReason;
   EAP_ATTRIBUTES* pAuthAttribs;
} EAP_METHOD_AUTHENTICATOR_RESULT;

typedef
#ifdef __midl
   [v1_enum]
#endif
enum tagEapPeerMethodResponseAction
{
        // Discard the request as EAP cannot use it.
        EapPeerMethodResponseActionDiscard = 0,
        // Send the packet to the authenticator
        EapPeerMethodResponseActionSend,
        // Respond to EAP by acting on the returned attributes
        EapPeerMethodResponseActionResult,
        // Invoke appropriate UI based on the context
        EapPeerMethodResponseActionInvokeUI,
        // The supplicant needs to take an action based on the context.
        EapPeerMethodResponseActionRespond,
        // The supplicant needs to take no action at all.
        EapPeerMethodResponseActionNone
} EapPeerMethodResponseAction;

// Structures used by EAP methods on the peer to communicate with the host.

typedef struct tagEapPeerMethodOuput 
{
        //EapPeerMethodDecision decision;
        //EapPeerMethodState state;
        EapPeerMethodResponseAction action;
        BOOL fAllowNotifications;
} EapPeerMethodOutput;

// ENUMERATIONS USED by EAP methods on the peer to communicate their status to
// EapHost.

typedef
#ifdef __midl
   [v1_enum]
#endif
enum tagEapPeerMethodResultReason
{
   EapPeerMethodResultUnknown = 1,
   EapPeerMethodResultSuccess,
   EapPeerMethodResultFailure
} EapPeerMethodResultReason, EapPeerMethodResultReasonOle;

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAPAUTHENTICATORACTIONDEFINE_H

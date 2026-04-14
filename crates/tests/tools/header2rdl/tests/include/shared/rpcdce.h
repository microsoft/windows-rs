#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rpcdce.h

Abstract:

    This module contains the DCE RPC runtime APIs.

--*/

#ifndef __RPCDCE_H__
#define __RPCDCE_H__

#if _MSC_VER > 1000
#pragma once
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef IN
#define IN
#endif

#ifndef OUT
#define OUT
#endif

#ifndef OPTIONAL
#define OPTIONAL
#endif

#ifndef DECLSPEC_NORETURN
#if (_MSC_VER >= 1200) && !defined(MIDL_PASS)
#define DECLSPEC_NORETURN   __declspec(noreturn)
#else
#define DECLSPEC_NORETURN
#endif
#endif

/*typedef char small;*/
/*typedef unsigned char byte;*/
/*typedef unsigned char boolean;*/

#include <specstrings.h>

typedef _Null_terminated_ unsigned char __RPC_FAR * RPC_CSTR;

#if defined(RPC_USE_NATIVE_WCHAR) && defined(_NATIVE_WCHAR_T_DEFINED)
typedef _Null_terminated_ wchar_t __RPC_FAR * RPC_WSTR;
typedef _Null_terminated_ const wchar_t * RPC_CWSTR;
#else
typedef _Null_terminated_ unsigned short __RPC_FAR * RPC_WSTR;
typedef _Null_terminated_ const unsigned short * RPC_CWSTR;
#endif

typedef I_RPC_HANDLE RPC_BINDING_HANDLE;
typedef RPC_BINDING_HANDLE handle_t;
#define rpc_binding_handle_t RPC_BINDING_HANDLE

#ifndef GUID_DEFINED
#include <guiddef.h>
#endif /* GUID_DEFINED */

#ifndef UUID_DEFINED
#define UUID_DEFINED
typedef GUID UUID;
#ifndef uuid_t
#define uuid_t UUID
#endif
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef struct _RPC_BINDING_VECTOR
{
    unsigned long Count;
    _Field_size_(Count) RPC_BINDING_HANDLE BindingH[1];
} RPC_BINDING_VECTOR;
#ifndef rpc_binding_vector_t
#define rpc_binding_vector_t RPC_BINDING_VECTOR
#endif

typedef struct _UUID_VECTOR
{
  unsigned long Count;
  _Field_size_(Count) UUID *Uuid[1];
} UUID_VECTOR;
#ifndef uuid_vector_t
#define uuid_vector_t UUID_VECTOR
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef void __RPC_FAR * RPC_IF_HANDLE;

#ifndef IFID_DEFINED
#define IFID_DEFINED
typedef struct _RPC_IF_ID
{
    UUID Uuid;
    unsigned short VersMajor;
    unsigned short VersMinor;
} RPC_IF_ID;
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define RPC_C_BINDING_INFINITE_TIMEOUT 10
#define RPC_C_BINDING_MIN_TIMEOUT 0
#define RPC_C_BINDING_DEFAULT_TIMEOUT 5
#define RPC_C_BINDING_MAX_TIMEOUT 9

#define RPC_C_CANCEL_INFINITE_TIMEOUT -1

#define RPC_C_LISTEN_MAX_CALLS_DEFAULT 1234
#define RPC_C_PROTSEQ_MAX_REQS_DEFAULT 10

// RPC_POLICY EndpointFlags.
#define RPC_C_BIND_TO_ALL_NICS          1
#define RPC_C_USE_INTERNET_PORT         0x1
#define RPC_C_USE_INTRANET_PORT         0x2
#define RPC_C_DONT_FAIL                 0x4
#define RPC_C_RPCHTTP_USE_LOAD_BALANCE  0x8
#define RPC_C_TRY_ENFORCE_MAX_CALLS     0x10

#if (NTDDI_VERSION < NTDDI_VISTA)
// RPC_POLICY EndpointFlags specific to the Falcon/RPC transport (deprecated for Vista)
#define RPC_C_MQ_TEMPORARY                  0x0000
#define RPC_C_MQ_PERMANENT                  0x0001
#define RPC_C_MQ_CLEAR_ON_OPEN              0x0002
#define RPC_C_MQ_USE_EXISTING_SECURITY      0x0004
#define RPC_C_MQ_AUTHN_LEVEL_NONE           0x0000
#define RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY  0x0008
#define RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY    0x0010

// Falcon/Rpc options are deprecated from Vista
#define RPC_C_MQ_EXPRESS                0  // Client: RPC_C_MQ_DELIVERY.
#define RPC_C_MQ_RECOVERABLE            1

#define RPC_C_MQ_JOURNAL_NONE           0  // Client: RPC_C_MQ_JOURNAL.
#define RPC_C_MQ_JOURNAL_DEADLETTER     1
#define RPC_C_MQ_JOURNAL_ALWAYS         2

// Client: RpcBindingSetOption() values for the Falcon/RPC transport (some are deprecated from Vista)

#define RPC_C_OPT_MQ_DELIVERY            1
#define RPC_C_OPT_MQ_PRIORITY            2
#define RPC_C_OPT_MQ_JOURNAL             3
#define RPC_C_OPT_MQ_ACKNOWLEDGE         4
#define RPC_C_OPT_MQ_AUTHN_SERVICE       5
#define RPC_C_OPT_MQ_AUTHN_LEVEL         6
#define RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE 7
#define RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED 8
#endif // (NTDDI_VERSION < NTDDI_VISTA)

#define RPC_C_OPT_BINDING_NONCAUSAL      9
#define RPC_C_OPT_SECURITY_CALLBACK      10
#define RPC_C_OPT_UNIQUE_BINDING         11

#if (NTDDI_VERSION <= NTDDI_WIN2K)
#define RPC_C_OPT_MAX_OPTIONS            12

#elif (NTDDI_VERSION <= NTDDI_WS03)
#define RPC_C_OPT_CALL_TIMEOUT           12
#define RPC_C_OPT_DONT_LINGER            13
#define RPC_C_OPT_MAX_OPTIONS            14

#else
#define RPC_C_OPT_TRANS_SEND_BUFFER_SIZE 5
#define RPC_C_OPT_CALL_TIMEOUT           12
#define RPC_C_OPT_DONT_LINGER            13
#define RPC_C_OPT_TRUST_PEER             14
#define RPC_C_OPT_ASYNC_BLOCK            15
#define RPC_C_OPT_OPTIMIZE_TIME          16
#define RPC_C_OPT_MAX_OPTIONS            17

#endif // (NTDDI_VERSION <= NTDDI_WIN2K)

// flags for RpcServerInqAuthClientEx
//
#define RPC_C_FULL_CERT_CHAIN 0x0001



#ifdef RPC_UNICODE_SUPPORTED
typedef struct _RPC_PROTSEQ_VECTORA
{
    unsigned int Count;
    unsigned char __RPC_FAR * Protseq[1];
} RPC_PROTSEQ_VECTORA;

typedef struct _RPC_PROTSEQ_VECTORW
{
    unsigned int Count;
    unsigned short __RPC_FAR * Protseq[1];
} RPC_PROTSEQ_VECTORW;

#ifdef UNICODE
#define RPC_PROTSEQ_VECTOR RPC_PROTSEQ_VECTORW
#else /* UNICODE */
#define RPC_PROTSEQ_VECTOR RPC_PROTSEQ_VECTORA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

typedef struct _RPC_PROTSEQ_VECTOR
{
    unsigned int Count;
    unsigned char __RPC_FAR * Protseq[1];
} RPC_PROTSEQ_VECTOR;

#endif /* RPC_UNICODE_SUPPORTED */
typedef struct _RPC_POLICY {
    unsigned int Length ;
    unsigned long EndpointFlags ;
    unsigned long NICFlags ;
    } RPC_POLICY,  __RPC_FAR *PRPC_POLICY ;

typedef void __RPC_USER
RPC_OBJECT_INQ_FN (
    _In_ UUID __RPC_FAR * ObjectUuid,
    _Out_ UUID __RPC_FAR * TypeUuid,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

_Success_(return == 0) /*RPC_S_OK*/
typedef RPC_STATUS RPC_ENTRY
RPC_IF_CALLBACK_FN (
    _In_ RPC_IF_HANDLE  InterfaceUuid,
    _In_ void *Context
    ) ;

typedef void RPC_ENTRY
RPC_SECURITY_CALLBACK_FN (
    _In_ void *Context
    ) ;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#define RPC_MGR_EPV void

typedef struct
{
    unsigned int Count;
    unsigned long Stats[1];
} RPC_STATS_VECTOR;

#define RPC_C_STATS_CALLS_IN 0
#define RPC_C_STATS_CALLS_OUT 1
#define RPC_C_STATS_PKTS_IN 2
#define RPC_C_STATS_PKTS_OUT 3

typedef struct
{
  unsigned long Count;
  RPC_IF_ID __RPC_FAR * IfId[1];
} RPC_IF_ID_VECTOR;

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingCopy (
    _In_ RPC_BINDING_HANDLE SourceBinding,
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * DestinationBinding
    );

/* client */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingFree (
    _Inout_ RPC_BINDING_HANDLE __RPC_FAR * Binding
    );

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetOption ( 
    _In_ RPC_BINDING_HANDLE hBinding,
    _In_ unsigned long      option,
    _In_ ULONG_PTR          optionValue 
    );

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqOption (
    _In_  RPC_BINDING_HANDLE hBinding,
    _In_  unsigned long      option,
    _Out_ ULONG_PTR         *pOptionValue 
    );

/* client */

#if !defined(_KRPCENV_)

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingFromStringBindingA (    
    _In_ RPC_CSTR StringBinding,
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * Binding
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingFromStringBindingW (
    _In_ RPC_WSTR StringBinding,        
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * Binding
    );

#ifdef UNICODE
#define RpcBindingFromStringBinding RpcBindingFromStringBindingW
#else /* UNICODE */
#define RpcBindingFromStringBinding RpcBindingFromStringBindingA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingFromStringBinding (
    _In_ RPC_CSTR StringBinding, 
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * Binding
    );

#endif /* RPC_UNICODE_SUPPORTED */
#endif /* _KRPCENV_ */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* client */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcSsGetContextBinding (
    _In_ void *ContextHandle,
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * Binding
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqMaxCalls (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned int * MaxCalls
    );

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqObject (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ UUID __RPC_FAR * ObjectUuid
    );

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingReset (
    _In_ RPC_BINDING_HANDLE Binding
    );

/* RpcBindingServerFromClient : UNSUPPORTED */
/* RpcBindingSetAuthInfo */

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetObject (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ UUID __RPC_FAR * ObjectUuid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqDefaultProtectLevel (
    _In_  unsigned long AuthnSvc,
    _Out_ unsigned long __RPC_FAR *AuthnLevel
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


/* client/server */

#ifdef RPC_UNICODE_SUPPORTED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingToStringBindingA (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_ RPC_CSTR __RPC_FAR * StringBinding
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingToStringBindingW (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_ RPC_WSTR __RPC_FAR * StringBinding
    );

#ifdef UNICODE
#define RpcBindingToStringBinding RpcBindingToStringBindingW
#else /* UNICODE */
#define RpcBindingToStringBinding RpcBindingToStringBindingA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingToStringBinding (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_ RPC_CSTR __RPC_FAR * StringBinding
    );

#endif /* RPC_UNICODE_SUPPORTED */


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* client/server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingVectorFree (
    _Inout_ RPC_BINDING_VECTOR __RPC_FAR * __RPC_FAR * BindingVector
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/* client/server */

#if !defined(_KRPCENV_)

#ifdef RPC_UNICODE_SUPPORTED


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcStringBindingComposeA (
    _In_opt_ RPC_CSTR ObjUuid,
    _In_opt_ RPC_CSTR ProtSeq,    
    _In_opt_ RPC_CSTR NetworkAddr,
    _In_opt_ RPC_CSTR Endpoint,
    _In_opt_ RPC_CSTR Options,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * StringBinding
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcStringBindingComposeW (
    _In_opt_ RPC_WSTR ObjUuid,
    _In_opt_ RPC_WSTR ProtSeq,    
    _In_opt_ RPC_WSTR NetworkAddr,
    _In_opt_ RPC_WSTR Endpoint,
    _In_opt_ RPC_WSTR Options,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * StringBinding
    );

#ifdef UNICODE
#define RpcStringBindingCompose RpcStringBindingComposeW
#else /* UNICODE */
#define RpcStringBindingCompose RpcStringBindingComposeA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcStringBindingCompose (
    _In_opt_ RPC_CSTR ObjUuid,
    _In_opt_ RPC_CSTR ProtSeq,    
    _In_opt_ RPC_CSTR NetworkAddr,
    _In_opt_ RPC_CSTR Endpoint,
    _In_opt_ RPC_CSTR Options,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * StringBinding
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

/* client/server */

#ifdef RPC_UNICODE_SUPPORTED


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcStringBindingParseA (
    _In_ RPC_CSTR StringBinding,    
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ObjUuid,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Protseq,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * NetworkAddr,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Endpoint,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * NetworkOptions
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcStringBindingParseW (
    _In_ RPC_WSTR StringBinding,    
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ObjUuid,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * Protseq,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * NetworkAddr,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * Endpoint,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * NetworkOptions
    );

#ifdef UNICODE
#define RpcStringBindingParse RpcStringBindingParseW
#else /* UNICODE */
#define RpcStringBindingParse RpcStringBindingParseA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcStringBindingParse (
    _In_ RPC_CSTR StringBinding,    
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ObjUuid,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Protseq,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * NetworkAddr,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Endpoint,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * NetworkOptions
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

#endif /* _KRPCENV_ */

/* client/server */

#ifdef RPC_UNICODE_SUPPORTED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcStringFreeA (
    _Inout_ RPC_CSTR __RPC_FAR * String    
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcStringFreeW (
    _Inout_ RPC_WSTR __RPC_FAR * String
    );

#ifdef UNICODE
#define RpcStringFree RpcStringFreeW
#else /* UNICODE */
#define RpcStringFree RpcStringFreeA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcStringFree (
    _At_(*_Curr_, _Frees_ptr_opt_)
    _Inout_ RPC_CSTR __RPC_FAR * String
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcIfInqId (
    _In_ RPC_IF_HANDLE RpcIfHandle,
    _Out_ RPC_IF_ID __RPC_FAR * RpcIfId
    );

/* client/server */

#ifdef RPC_UNICODE_SUPPORTED


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNetworkIsProtseqValidA (
    _In_ RPC_CSTR Protseq     
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNetworkIsProtseqValidW (
    _In_ RPC_WSTR Protseq
    );

#ifdef UNICODE
#define RpcNetworkIsProtseqValid RpcNetworkIsProtseqValidW
#else /* UNICODE */
#define RpcNetworkIsProtseqValid RpcNetworkIsProtseqValidA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcNetworkIsProtseqValid (
    _In_ RPC_CSTR Protseq
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqComTimeout (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned int __RPC_FAR * Timeout
    );


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtSetComTimeout (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned int Timeout
    );

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtSetCancelTimeout(
    _In_ long Timeout
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* server */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNetworkInqProtseqsA (
    _Outptr_ RPC_PROTSEQ_VECTORA __RPC_FAR * __RPC_FAR * ProtseqVector    
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNetworkInqProtseqsW (
    _Outptr_ RPC_PROTSEQ_VECTORW __RPC_FAR * __RPC_FAR * ProtseqVector    
    );

#ifdef UNICODE
#define RpcNetworkInqProtseqs RpcNetworkInqProtseqsW
#else /* UNICODE */
#define RpcNetworkInqProtseqs RpcNetworkInqProtseqsA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcNetworkInqProtseqs (
    _Outptr_ RPC_PROTSEQ_VECTOR __RPC_FAR * __RPC_FAR * ProtseqVector
    );

#endif /* RPC_UNICODE_SUPPORTED */

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcObjectInqType (
    _In_ UUID __RPC_FAR * ObjUuid,
    _Out_opt_ OPTIONAL UUID __RPC_FAR * TypeUuid
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcObjectSetInqFn (
    _In_ RPC_OBJECT_INQ_FN __RPC_FAR * InquiryFn
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcObjectSetType (
    _In_ UUID __RPC_FAR * ObjUuid,
    _In_opt_ OPTIONAL UUID __RPC_FAR * TypeUuid
    );

/* server */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcProtseqVectorFreeA (
    _Inout_ RPC_PROTSEQ_VECTORA __RPC_FAR * __RPC_FAR * ProtseqVector
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcProtseqVectorFreeW (
    _Inout_ RPC_PROTSEQ_VECTORW __RPC_FAR * __RPC_FAR * ProtseqVector
    );

#ifdef UNICODE
#define RpcProtseqVectorFree RpcProtseqVectorFreeW
#else /* UNICODE */
#define RpcProtseqVectorFree RpcProtseqVectorFreeA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcProtseqVectorFree (
    _Inout_ RPC_PROTSEQ_VECTOR __RPC_FAR * __RPC_FAR * ProtseqVector
    );

#endif /* RPC_UNICODE_SUPPORTED */

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerInqBindings (
    _Outptr_ RPC_BINDING_VECTOR __RPC_FAR * __RPC_FAR * BindingVector
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerInqBindingsEx (
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _Outptr_ RPC_BINDING_VECTOR __RPC_FAR * __RPC_FAR * BindingVector
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerInqIf (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _Outptr_ RPC_MGR_EPV __RPC_FAR * __RPC_FAR * MgrEpv
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerListen (
    _In_ unsigned int MinimumCallThreads,
    _In_ unsigned int MaxCalls,
    _In_ unsigned int DontWait
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerRegisterIf (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _In_opt_ RPC_MGR_EPV __RPC_FAR * MgrEpv
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerRegisterIfEx (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _In_opt_ RPC_MGR_EPV __RPC_FAR * MgrEpv,
    _In_ unsigned int Flags,
    _In_ unsigned int MaxCalls,
    _In_opt_ RPC_IF_CALLBACK_FN __RPC_FAR *IfCallback
    );

/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerRegisterIf2 (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID * MgrTypeUuid,
    _In_opt_ RPC_MGR_EPV * MgrEpv,
    _In_ unsigned int Flags,
    _In_ unsigned int MaxCalls,
    _In_ unsigned int MaxRpcSize,
    _In_opt_ RPC_IF_CALLBACK_FN *IfCallbackFn
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerRegisterIf3 (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _In_opt_ RPC_MGR_EPV __RPC_FAR * MgrEpv,
    _In_ unsigned int Flags,
    _In_ unsigned int MaxCalls,
    _In_ unsigned int MaxRpcSize,
    _In_opt_ RPC_IF_CALLBACK_FN __RPC_FAR *IfCallback,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUnregisterIf (
    _In_opt_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _In_ unsigned int WaitForCallsToComplete
    );

#if (NTDDI_VERSION >= NTDDI_WINXP)
/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUnregisterIfEx (
    _In_opt_ RPC_IF_HANDLE IfSpec,
    _In_opt_ UUID __RPC_FAR * MgrTypeUuid,
    _In_ int RundownContextHandles
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseAllProtseqs (
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseAllProtseqsEx (
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseAllProtseqsIf (
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseAllProtseqsIfEx (
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );


/* server */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqExA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqExW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#ifdef UNICODE
#define RpcServerUseProtseq RpcServerUseProtseqW
#define RpcServerUseProtseqEx RpcServerUseProtseqExW
#else /* UNICODE */
#define RpcServerUseProtseq RpcServerUseProtseqA
#define RpcServerUseProtseqEx RpcServerUseProtseqExA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseq (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEx (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#endif /* RPC_UNICODE_SUPPORTED */

/* server */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEpA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_CSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEpExA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_CSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEpW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_WSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEpExW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_WSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#ifdef UNICODE
#define RpcServerUseProtseqEp RpcServerUseProtseqEpW
#define RpcServerUseProtseqEpEx RpcServerUseProtseqEpExW
#else /* UNICODE */
#define RpcServerUseProtseqEp RpcServerUseProtseqEpA
#define RpcServerUseProtseqEpEx RpcServerUseProtseqEpExA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEp (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_CSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqEpEx (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_WSTR Endpoint,
    _In_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#endif /* RPC_UNICODE_SUPPORTED */

/* server */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIfA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIfExA (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIfW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIfExW (
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#ifdef UNICODE
#define RpcServerUseProtseqIf RpcServerUseProtseqIfW
#define RpcServerUseProtseqIfEx RpcServerUseProtseqIfExW
#else /* UNICODE */
#define RpcServerUseProtseqIf RpcServerUseProtseqIfA
#define RpcServerUseProtseqIfEx RpcServerUseProtseqIfExA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIf (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_opt_ void __RPC_FAR * SecurityDescriptor
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerUseProtseqIfEx (
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ void __RPC_FAR * SecurityDescriptor,
    _In_ PRPC_POLICY Policy
    );

#endif /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
void
RPC_ENTRY
RpcServerYield (
    void
    );

/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcMgmtStatsVectorFree (
    _Inout_ RPC_STATS_VECTOR ** StatsVector
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqStats (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _Outptr_ RPC_STATS_VECTOR ** Statistics
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtIsServerListening (
    _In_opt_ RPC_BINDING_HANDLE Binding
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtStopServerListening (
    _In_opt_ RPC_BINDING_HANDLE Binding
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtWaitServerListen (
    void
    );

/* server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtSetServerStackSize (
    _In_ unsigned long ThreadStackSize
    );

/* server */
RPCRTAPI
void
RPC_ENTRY
RpcSsDontSerializeContext (
    void
    );

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEnableIdleCleanup (
    void
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqIfIds (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _Outptr_ RPC_IF_ID_VECTOR __RPC_FAR * __RPC_FAR * IfIdVector
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcIfIdVectorFree (
    _Inout_ RPC_IF_ID_VECTOR __RPC_FAR * __RPC_FAR * IfIdVector
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef RPC_UNICODE_SUPPORTED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqServerPrincNameA (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_CSTR __RPC_FAR * ServerPrincName
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtInqServerPrincNameW (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_WSTR __RPC_FAR * ServerPrincName
    );

#ifdef UNICODE
#define RpcMgmtInqServerPrincName RpcMgmtInqServerPrincNameW
#else /* UNICODE */
#define RpcMgmtInqServerPrincName RpcMgmtInqServerPrincNameA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcMgmtInqServerPrincName (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_WSTR __RPC_FAR * ServerPrincName
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerInqDefaultPrincNameA (
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_CSTR __RPC_FAR * PrincName
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerInqDefaultPrincNameW (
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_WSTR __RPC_FAR * PrincName
    );

#ifdef UNICODE
#define RpcServerInqDefaultPrincName RpcServerInqDefaultPrincNameW
#else /* UNICODE */
#define RpcServerInqDefaultPrincName RpcServerInqDefaultPrincNameA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInqDefaultPrincName (
    _In_ unsigned long AuthnSvc,
    _Outptr_ RPC_WSTR __RPC_FAR * PrincName
    );

#endif /* RPC_UNICODE_SUPPORTED */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* client */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpResolveBinding (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ RPC_IF_HANDLE IfSpec
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* client */

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNsBindingInqEntryNameA (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _Outptr_ RPC_CSTR __RPC_FAR * EntryName
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcNsBindingInqEntryNameW (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _Outptr_ RPC_WSTR __RPC_FAR * EntryName
    );

#ifdef UNICODE
#define RpcNsBindingInqEntryName RpcNsBindingInqEntryNameW
#else /* UNICODE */
#define RpcNsBindingInqEntryName RpcNsBindingInqEntryNameA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcNsBindingInqEntryName (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _Outptr_ RPC_CSTR __RPC_FAR * EntryName
    );

#endif /* RPC_UNICODE_SUPPORTED */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef void __RPC_FAR * RPC_AUTH_IDENTITY_HANDLE;
typedef void __RPC_FAR * RPC_AUTHZ_HANDLE;

#define RPC_C_AUTHN_LEVEL_DEFAULT       0
#define RPC_C_AUTHN_LEVEL_NONE          1
#define RPC_C_AUTHN_LEVEL_CONNECT       2
#define RPC_C_AUTHN_LEVEL_CALL          3
#define RPC_C_AUTHN_LEVEL_PKT           4
#define RPC_C_AUTHN_LEVEL_PKT_INTEGRITY 5
#define RPC_C_AUTHN_LEVEL_PKT_PRIVACY   6

#define RPC_C_IMP_LEVEL_DEFAULT      0
#define RPC_C_IMP_LEVEL_ANONYMOUS    1
#define RPC_C_IMP_LEVEL_IDENTIFY     2
#define RPC_C_IMP_LEVEL_IMPERSONATE  3
#define RPC_C_IMP_LEVEL_DELEGATE     4

#define RPC_C_QOS_IDENTITY_STATIC    0
#define RPC_C_QOS_IDENTITY_DYNAMIC   1

#define RPC_C_QOS_CAPABILITIES_DEFAULT                        0x0
#define RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH                    0x1
#define RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC                   0x2
#define RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY                  0x4

#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE        0x8
#define RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT                 0x10
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY   0x20
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define RPC_C_PROTECT_LEVEL_DEFAULT       (RPC_C_AUTHN_LEVEL_DEFAULT)
#define RPC_C_PROTECT_LEVEL_NONE          (RPC_C_AUTHN_LEVEL_NONE)
#define RPC_C_PROTECT_LEVEL_CONNECT       (RPC_C_AUTHN_LEVEL_CONNECT)
#define RPC_C_PROTECT_LEVEL_CALL          (RPC_C_AUTHN_LEVEL_CALL)
#define RPC_C_PROTECT_LEVEL_PKT           (RPC_C_AUTHN_LEVEL_PKT)
#define RPC_C_PROTECT_LEVEL_PKT_INTEGRITY (RPC_C_AUTHN_LEVEL_PKT_INTEGRITY)
#define RPC_C_PROTECT_LEVEL_PKT_PRIVACY   (RPC_C_AUTHN_LEVEL_PKT_PRIVACY)

#define RPC_C_AUTHN_NONE          0
#define RPC_C_AUTHN_DCE_PRIVATE   1
#define RPC_C_AUTHN_DCE_PUBLIC    2
#define RPC_C_AUTHN_DEC_PUBLIC    4
#define RPC_C_AUTHN_GSS_NEGOTIATE 9
#define RPC_C_AUTHN_WINNT        10
#define RPC_C_AUTHN_GSS_SCHANNEL 14
#define RPC_C_AUTHN_GSS_KERBEROS 16
#define RPC_C_AUTHN_DPA          17
#define RPC_C_AUTHN_MSN          18
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define RPC_C_AUTHN_DIGEST       21
#endif // (NTDDI_VERSION >= WINXP)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define RPC_C_AUTHN_KERNEL         20
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#define RPC_C_AUTHN_NEGO_EXTENDER 30
#define RPC_C_AUTHN_PKU2U         31

#define RPC_C_AUTHN_LIVE_SSP      32
#define RPC_C_AUTHN_LIVEXP_SSP    35
#define RPC_C_AUTHN_CLOUD_AP      36
#define RPC_C_AUTHN_MSONLINE      82 

#define RPC_C_AUTHN_MQ          100
#define RPC_C_AUTHN_DEFAULT     0xFFFFFFFFL

#define RPC_C_NO_CREDENTIALS ((RPC_AUTH_IDENTITY_HANDLE) MAXUINT_PTR)

#define RPC_C_SECURITY_QOS_VERSION      1L
#define RPC_C_SECURITY_QOS_VERSION_1    1L

typedef struct _RPC_SECURITY_QOS {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
} RPC_SECURITY_QOS, *PRPC_SECURITY_QOS;

#ifndef _AUTH_IDENTITY_DEFINED
#define _AUTH_IDENTITY_DEFINED

#define SEC_WINNT_AUTH_IDENTITY_ANSI    0x1
#define SEC_WINNT_AUTH_IDENTITY_UNICODE 0x2

typedef struct _SEC_WINNT_AUTH_IDENTITY_W {
  _Field_size_bytes_((UserLength+1)*sizeof(WCHAR)) unsigned short __RPC_FAR *User;
  unsigned long UserLength;
  _Field_size_bytes_((DomainLength+1)*sizeof(WCHAR)) unsigned short __RPC_FAR *Domain;
  unsigned long DomainLength;
  _Field_size_bytes_((PasswordLength+1)*sizeof(WCHAR)) unsigned short __RPC_FAR *Password;
  unsigned long PasswordLength;
  unsigned long Flags;
} SEC_WINNT_AUTH_IDENTITY_W, *PSEC_WINNT_AUTH_IDENTITY_W;

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define _AUTH_IDENTITY_A_DEFINED
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

typedef struct _SEC_WINNT_AUTH_IDENTITY_A {
  _Field_size_(UserLength+1) unsigned char __RPC_FAR *User;
  unsigned long UserLength;
  _Field_size_(DomainLength+1) unsigned char __RPC_FAR *Domain;
  unsigned long DomainLength;
  _Field_size_(PasswordLength+1) unsigned char __RPC_FAR *Password;
  unsigned long PasswordLength;
  unsigned long Flags;
} SEC_WINNT_AUTH_IDENTITY_A, *PSEC_WINNT_AUTH_IDENTITY_A;

#ifdef UNICODE
#define SEC_WINNT_AUTH_IDENTITY SEC_WINNT_AUTH_IDENTITY_W
#define PSEC_WINNT_AUTH_IDENTITY PSEC_WINNT_AUTH_IDENTITY_W
#define _SEC_WINNT_AUTH_IDENTITY _SEC_WINNT_AUTH_IDENTITY_W
#else // UNICODE
#define SEC_WINNT_AUTH_IDENTITY SEC_WINNT_AUTH_IDENTITY_A
#define PSEC_WINNT_AUTH_IDENTITY PSEC_WINNT_AUTH_IDENTITY_A
#define _SEC_WINNT_AUTH_IDENTITY _SEC_WINNT_AUTH_IDENTITY_A
#endif // UNICODE

#if (NTDDI_VERSION >= NTDDI_WINXP)

#define RPC_C_SECURITY_QOS_VERSION_2 2L

#define RPC_C_AUTHN_INFO_TYPE_HTTP                  1

#define RPC_C_HTTP_AUTHN_TARGET_SERVER              1
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_C_HTTP_AUTHN_TARGET_PROXY               2
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define RPC_C_HTTP_AUTHN_SCHEME_BASIC      0x00000001
#define RPC_C_HTTP_AUTHN_SCHEME_NTLM       0x00000002
#define RPC_C_HTTP_AUTHN_SCHEME_PASSPORT   0x00000004
#define RPC_C_HTTP_AUTHN_SCHEME_DIGEST     0x00000008
#define RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE  0x00000010
#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_C_HTTP_AUTHN_SCHEME_CERT       0x00010000
// 0x00020000 & 0x00040000 are reserved
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#define RPC_C_HTTP_FLAG_USE_SSL                     1
#define RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME       2
#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID      8
#endif // (NTDDI_VERSION >= NTDDI_WS03)
#if (NTDDI_VERSION >= NTDDI_VISTASP1)
#define RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK 16
#endif // (NTDDI_VERSION >= NTDDI_VISTASP1)


typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_W
{
    SEC_WINNT_AUTH_IDENTITY_W *TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    unsigned long *AuthnSchemes;
    unsigned short __RPC_FAR *ServerCertificateSubject;
} RPC_HTTP_TRANSPORT_CREDENTIALS_W, *PRPC_HTTP_TRANSPORT_CREDENTIALS_W;

typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_A
{
    SEC_WINNT_AUTH_IDENTITY_A *TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    unsigned long *AuthnSchemes;
    unsigned char __RPC_FAR *ServerCertificateSubject;
} RPC_HTTP_TRANSPORT_CREDENTIALS_A, *PRPC_HTTP_TRANSPORT_CREDENTIALS_A;

#if (NTDDI_VERSION >= NTDDI_VISTA)

typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W
{
    SEC_WINNT_AUTH_IDENTITY_W *TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    _Field_size_(NumberOfAuthnSchemes) unsigned long *AuthnSchemes;
    unsigned short __RPC_FAR *ServerCertificateSubject;
    SEC_WINNT_AUTH_IDENTITY_W *ProxyCredentials;
    unsigned long NumberOfProxyAuthnSchemes;
    _Field_size_(NumberOfProxyAuthnSchemes) unsigned long *ProxyAuthnSchemes;
} RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W, *PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W;

typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A
{
    SEC_WINNT_AUTH_IDENTITY_A *TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    unsigned long *AuthnSchemes;
    unsigned char __RPC_FAR *ServerCertificateSubject;
    SEC_WINNT_AUTH_IDENTITY_A *ProxyCredentials;
    unsigned long NumberOfProxyAuthnSchemes;
    unsigned long *ProxyAuthnSchemes;
} RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A, *PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A;

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)

typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W
{
    RPC_AUTH_IDENTITY_HANDLE TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    _Field_size_(NumberOfAuthnSchemes) unsigned long *AuthnSchemes;
    unsigned short __RPC_FAR *ServerCertificateSubject;
    RPC_AUTH_IDENTITY_HANDLE ProxyCredentials;
    unsigned long NumberOfProxyAuthnSchemes;
    _Field_size_(NumberOfProxyAuthnSchemes) unsigned long *ProxyAuthnSchemes;
} RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W, *PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W;

typedef struct _RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A
{
    RPC_AUTH_IDENTITY_HANDLE TransportCredentials;
    unsigned long Flags;
    unsigned long AuthenticationTarget;
    unsigned long NumberOfAuthnSchemes;
    _Field_size_(NumberOfAuthnSchemes) unsigned long *AuthnSchemes;
    unsigned char __RPC_FAR *ServerCertificateSubject;
    RPC_AUTH_IDENTITY_HANDLE ProxyCredentials;
    unsigned long NumberOfProxyAuthnSchemes;
    _Field_size_(NumberOfProxyAuthnSchemes) unsigned long *ProxyAuthnSchemes;
} RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A, *PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

typedef struct _RPC_SECURITY_QOS_V2_W {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_W *HttpCredentials;
      } u;
} RPC_SECURITY_QOS_V2_W, *PRPC_SECURITY_QOS_V2_W;

typedef struct _RPC_SECURITY_QOS_V2_A {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_A *HttpCredentials;
      } u;
} RPC_SECURITY_QOS_V2_A, *PRPC_SECURITY_QOS_V2_A;

#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_C_SECURITY_QOS_VERSION_3 3L

typedef struct _RPC_SECURITY_QOS_V3_W {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_W *HttpCredentials;
      } u;
  void *Sid;
} RPC_SECURITY_QOS_V3_W, *PRPC_SECURITY_QOS_V3_W;

typedef struct _RPC_SECURITY_QOS_V3_A {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_A *HttpCredentials;
      } u;
  void *Sid;
} RPC_SECURITY_QOS_V3_A, *PRPC_SECURITY_QOS_V3_A;
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)

#define RPC_C_SECURITY_QOS_VERSION_4 4L

typedef struct _RPC_SECURITY_QOS_V4_W {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_W *HttpCredentials;
      } u;
  void *Sid;
  unsigned int EffectiveOnly;
} RPC_SECURITY_QOS_V4_W, *PRPC_SECURITY_QOS_V4_W;

typedef struct _RPC_SECURITY_QOS_V4_A {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_A *HttpCredentials;
      } u;
  void *Sid;
  unsigned int EffectiveOnly;
} RPC_SECURITY_QOS_V4_A, *PRPC_SECURITY_QOS_V4_A;

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define RPC_C_SECURITY_QOS_VERSION_5 5L

typedef struct _RPC_SECURITY_QOS_V5_W {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_W *HttpCredentials;
      } u;
  void *Sid;
  unsigned int EffectiveOnly;
  void *ServerSecurityDescriptor;
} RPC_SECURITY_QOS_V5_W, *PRPC_SECURITY_QOS_V5_W;

typedef struct _RPC_SECURITY_QOS_V5_A {
  unsigned long Version;
  unsigned long Capabilities;
  unsigned long IdentityTracking;
  unsigned long ImpersonationType;
  unsigned long AdditionalSecurityInfoType;
  union 
      {
      RPC_HTTP_TRANSPORT_CREDENTIALS_A *HttpCredentials;
      } u;
  void *Sid;
  unsigned int EffectiveOnly;
  void *ServerSecurityDescriptor;
} RPC_SECURITY_QOS_V5_A, *PRPC_SECURITY_QOS_V5_A;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif // NTDDI_WINXP
#endif // _AUTH_IDENTITY_DEFINED 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (NTDDI_VERSION >= NTDDI_WINXP)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef UNICODE

#define RPC_SECURITY_QOS_V2 RPC_SECURITY_QOS_V2_W
#define PRPC_SECURITY_QOS_V2 PRPC_SECURITY_QOS_V2_W
#define _RPC_SECURITY_QOS_V2 _RPC_SECURITY_QOS_V2_W

#define RPC_HTTP_TRANSPORT_CREDENTIALS RPC_HTTP_TRANSPORT_CREDENTIALS_W
#define PRPC_HTTP_TRANSPORT_CREDENTIALS PRPC_HTTP_TRANSPORT_CREDENTIALS_W
#define _RPC_HTTP_TRANSPORT_CREDENTIALS _RPC_HTTP_TRANSPORT_CREDENTIALS_W

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_HTTP_TRANSPORT_CREDENTIALS_V2 RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W
#define PRPC_HTTP_TRANSPORT_CREDENTIALS_V2 PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W
#define _RPC_HTTP_TRANSPORT_CREDENTIALS_V2 _RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define RPC_HTTP_TRANSPORT_CREDENTIALS_V3 RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W
#define PRPC_HTTP_TRANSPORT_CREDENTIALS_V3 PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W
#define _RPC_HTTP_TRANSPORT_CREDENTIALS_V3 _RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_SECURITY_QOS_V3 RPC_SECURITY_QOS_V3_W
#define PRPC_SECURITY_QOS_V3 PRPC_SECURITY_QOS_V3_W
#define _RPC_SECURITY_QOS_V3 _RPC_SECURITY_QOS_V3_W
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_SECURITY_QOS_V4 RPC_SECURITY_QOS_V4_W
#define PRPC_SECURITY_QOS_V4 PRPC_SECURITY_QOS_V4_W
#define _RPC_SECURITY_QOS_V4 _RPC_SECURITY_QOS_V4_W
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define RPC_SECURITY_QOS_V5 RPC_SECURITY_QOS_V5_W
#define PRPC_SECURITY_QOS_V5 PRPC_SECURITY_QOS_V5_W
#define _RPC_SECURITY_QOS_V5 _RPC_SECURITY_QOS_V5_W
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#else // UNICODE

#define RPC_SECURITY_QOS_V2 RPC_SECURITY_QOS_V2_A
#define PRPC_SECURITY_QOS_V2 PRPC_SECURITY_QOS_V2_A
#define _RPC_SECURITY_QOS_V2 _RPC_SECURITY_QOS_V2_A

#define RPC_HTTP_TRANSPORT_CREDENTIALS RPC_HTTP_TRANSPORT_CREDENTIALS_A
#define PRPC_HTTP_TRANSPORT_CREDENTIALS PRPC_HTTP_TRANSPORT_CREDENTIALS_A
#define _RPC_HTTP_TRANSPORT_CREDENTIALS _RPC_HTTP_TRANSPORT_CREDENTIALS_A

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_HTTP_TRANSPORT_CREDENTIALS_V2 RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A
#define PRPC_HTTP_TRANSPORT_CREDENTIALS_V2 PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A
#define _RPC_HTTP_TRANSPORT_CREDENTIALS_V2 _RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define RPC_HTTP_TRANSPORT_CREDENTIALS_V3 RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A
#define PRPC_HTTP_TRANSPORT_CREDENTIALS_V3 PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A
#define _RPC_HTTP_TRANSPORT_CREDENTIALS_V3 _RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WS03)
#define RPC_SECURITY_QOS_V3 RPC_SECURITY_QOS_V3_A
#define PRPC_SECURITY_QOS_V3 PRPC_SECURITY_QOS_V3_A
#define _RPC_SECURITY_QOS_V3 _RPC_SECURITY_QOS_V3_A
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_SECURITY_QOS_V4 RPC_SECURITY_QOS_V4_A
#define PRPC_SECURITY_QOS_V4 PRPC_SECURITY_QOS_V4_A
#define _RPC_SECURITY_QOS_V4 _RPC_SECURITY_QOS_V4_A
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define RPC_SECURITY_QOS_V5 RPC_SECURITY_QOS_V5_A
#define PRPC_SECURITY_QOS_V5 PRPC_SECURITY_QOS_V5_A
#define _RPC_SECURITY_QOS_V5 _RPC_SECURITY_QOS_V5_A
#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (NTDDI_VERSION >= NTDDI_VISTA)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#define RPC_PROTSEQ_TCP                             (0x1)
#define RPC_PROTSEQ_NMP                             (0x2)
#define RPC_PROTSEQ_LRPC                            (0x3)
#define RPC_PROTSEQ_HTTP                            (0x4)

#define RPC_BHT_OBJECT_UUID_VALID                   (0x1)

#define RPC_BHO_NONCAUSAL                           (0x1)
#define RPC_BHO_DONTLINGER                          (0x2)
#define RPC_BHO_EXCLUSIVE_AND_GUARANTEED            (0x4)

typedef struct _RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    unsigned long Version;
    unsigned long Flags;
    unsigned long ProtocolSequence;
    unsigned short *NetworkAddress;
    unsigned short *StringEndpoint;
    union
    {
        unsigned short *Reserved;
    } u1;
    UUID ObjectUuid;
} RPC_BINDING_HANDLE_TEMPLATE_V1_W, *PRPC_BINDING_HANDLE_TEMPLATE_V1_W;

typedef struct _RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    unsigned long Version;
    unsigned long Flags;
    unsigned long ProtocolSequence;
    unsigned char *NetworkAddress;
    unsigned char *StringEndpoint;
    union
    {
        unsigned char *Reserved;
    } u1;
    UUID ObjectUuid;
} RPC_BINDING_HANDLE_TEMPLATE_V1_A, *PRPC_BINDING_HANDLE_TEMPLATE_V1_A;

typedef struct _RPC_BINDING_HANDLE_SECURITY_V1_W {
    unsigned long Version;
    unsigned short *ServerPrincName;
    unsigned long AuthnLevel;
    unsigned long AuthnSvc;
    SEC_WINNT_AUTH_IDENTITY_W *AuthIdentity;
    RPC_SECURITY_QOS *SecurityQos;
} RPC_BINDING_HANDLE_SECURITY_V1_W, *PRPC_BINDING_HANDLE_SECURITY_V1_W;

#ifdef _AUTH_IDENTITY_A_DEFINED

typedef struct _RPC_BINDING_HANDLE_SECURITY_V1_A {
    unsigned long Version;
    unsigned char *ServerPrincName;
    unsigned long AuthnLevel;
    unsigned long AuthnSvc;
    SEC_WINNT_AUTH_IDENTITY_A *AuthIdentity;
    RPC_SECURITY_QOS *SecurityQos;
} RPC_BINDING_HANDLE_SECURITY_V1_A, *PRPC_BINDING_HANDLE_SECURITY_V1_A;

#endif // _AUTH_IDENTITY_A_DEFINED

typedef struct _RPC_BINDING_HANDLE_OPTIONS_V1 {
    unsigned long Version;
    unsigned long Flags;
    unsigned long ComTimeout;
    unsigned long CallTimeout;
} RPC_BINDING_HANDLE_OPTIONS_V1, *PRPC_BINDING_HANDLE_OPTIONS_V1;

#ifdef UNICODE

#define RPC_BINDING_HANDLE_TEMPLATE_V1 RPC_BINDING_HANDLE_TEMPLATE_V1_W
#define PRPC_BINDING_HANDLE_TEMPLATE_V1 PRPC_BINDING_HANDLE_TEMPLATE_V1_W
#define _RPC_BINDING_HANDLE_TEMPLATE_V1 _RPC_BINDING_HANDLE_TEMPLATE_V1_W

#define RPC_BINDING_HANDLE_SECURITY_V1 RPC_BINDING_HANDLE_SECURITY_V1_W
#define PRPC_BINDING_HANDLE_SECURITY_V1 PRPC_BINDING_HANDLE_SECURITY_V1_W
#define _RPC_BINDING_HANDLE_SECURITY_V1 _RPC_BINDING_HANDLE_SECURITY_V1_W

#else

#define RPC_BINDING_HANDLE_TEMPLATE_V1 RPC_BINDING_HANDLE_TEMPLATE_V1_A
#define PRPC_BINDING_HANDLE_TEMPLATE_V1 PRPC_BINDING_HANDLE_TEMPLATE_V1_A
#define _RPC_BINDING_HANDLE_TEMPLATE_V1 _RPC_BINDING_HANDLE_TEMPLATE_V1_A

#define RPC_BINDING_HANDLE_SECURITY_V1 RPC_BINDING_HANDLE_SECURITY_V1_A
#define PRPC_BINDING_HANDLE_SECURITY_V1 PRPC_BINDING_HANDLE_SECURITY_V1_A
#define _RPC_BINDING_HANDLE_SECURITY_V1 _RPC_BINDING_HANDLE_SECURITY_V1_A

#endif // !UNICODE

#ifdef _AUTH_IDENTITY_A_DEFINED

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingCreateA (
    _In_ RPC_BINDING_HANDLE_TEMPLATE_V1_A * Template,
    _In_opt_ RPC_BINDING_HANDLE_SECURITY_V1_A * Security,
    _In_opt_ RPC_BINDING_HANDLE_OPTIONS_V1 * Options,
    _Out_ RPC_BINDING_HANDLE * Binding
    );

#endif // _AUTH_IDENTITY_A_DEFINED

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingCreateW (
    _In_ RPC_BINDING_HANDLE_TEMPLATE_V1_W * Template,
    _In_opt_ RPC_BINDING_HANDLE_SECURITY_V1_W * Security,
    _In_opt_ RPC_BINDING_HANDLE_OPTIONS_V1 * Options,
    _Out_ RPC_BINDING_HANDLE * Binding
    );

#ifdef UNICODE
#define RpcBindingCreate RpcBindingCreateW
#else /* UNICODE */
#define RpcBindingCreate RpcBindingCreateA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcBindingGetTrainingContextHandle (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_ void ** ContextHandle
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcServerInqBindingHandle (
    _Out_ RPC_BINDING_HANDLE * Binding
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#if (NTDDI_VERSION >= NTDDI_WS03)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef enum _RPC_HTTP_REDIRECTOR_STAGE
{
    RPCHTTP_RS_REDIRECT = 1,
    RPCHTTP_RS_ACCESS_1,
    RPCHTTP_RS_SESSION,
    RPCHTTP_RS_ACCESS_2,
    RPCHTTP_RS_INTERFACE
} RPC_HTTP_REDIRECTOR_STAGE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WS03)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WS03)
typedef RPC_STATUS
(__RPC_USER * RPC_NEW_HTTP_PROXY_CHANNEL) (
    _In_ RPC_HTTP_REDIRECTOR_STAGE RedirectorStage,
    _In_ RPC_WSTR ServerName,
    _In_ RPC_WSTR ServerPort,
    _In_opt_ RPC_WSTR RemoteUser,
    _In_opt_ RPC_WSTR AuthType,
    _Inout_ void __RPC_FAR * ResourceUuid,
    _Inout_ void __RPC_FAR * SessionId,
    _In_opt_ void __RPC_FAR * Interface,
    _In_opt_ void __RPC_FAR * Reserved,
    _In_ unsigned long Flags,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * NewServerName,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * NewServerPort
    );
    
#else

typedef RPC_STATUS
(__RPC_USER * RPC_NEW_HTTP_PROXY_CHANNEL) (
    _In_ unsigned short __RPC_FAR *ServerName,
    _In_ unsigned short __RPC_FAR *ServerPort,
    _In_ unsigned char __RPC_FAR *RemoteUser,
    _Out_ unsigned short __RPC_FAR **NewServerName
    );
#endif // (NTDDI_VERSION >= NTDDI_WS03)

typedef void
(__RPC_USER * RPC_HTTP_PROXY_FREE_STRING) (
    _In_ RPC_WSTR String
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WINXP) 

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#define RPC_C_AUTHZ_NONE    0
#define RPC_C_AUTHZ_NAME    1
#define RPC_C_AUTHZ_DCE     2
#define RPC_C_AUTHZ_DEFAULT 0xffffffff

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcImpersonateClient (
    _In_opt_ RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcImpersonateClient2 (
    _In_opt_ RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcRevertToSelfEx (
    _In_opt_ RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcRevertToSelf (
    void
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcImpersonateClientContainer (
    _In_opt_ RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcRevertContainerImpersonation (
    void
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

// <<<TODO RIGHT HERE>>>

#ifdef RPC_UNICODE_SUPPORTED

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClientA (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClientW (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClientExA (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc,
    _In_  unsigned long             Flags
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClientExW (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc,
    _In_  unsigned long             Flags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthInfoA (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE __RPC_FAR * AuthIdentity,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthInfoW (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE __RPC_FAR * AuthIdentity,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetAuthInfoA (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_opt_ RPC_CSTR ServerPrincName,
    _In_ unsigned long AuthnLevel,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,
    _In_ unsigned long AuthzSvc
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetAuthInfoExA (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_opt_ RPC_CSTR ServerPrincName,
    _In_ unsigned long AuthnLevel,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,
    _In_ unsigned long AuthzSvc,
    _In_opt_ RPC_SECURITY_QOS * SecurityQos
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetAuthInfoW (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_opt_ RPC_WSTR ServerPrincName,
    _In_ unsigned long AuthnLevel,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,
    _In_ unsigned long AuthzSvc
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingSetAuthInfoExW (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_opt_ RPC_WSTR ServerPrincName,
    _In_ unsigned long AuthnLevel,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,
    _In_ unsigned long AuthzSvc,
    _In_opt_ RPC_SECURITY_QOS * SecurityQOS
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthInfoExA (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE __RPC_FAR * AuthIdentity,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc,
    _In_  unsigned long RpcQosVersion,
    _Out_opt_ RPC_SECURITY_QOS *SecurityQOS
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthInfoExW (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE __RPC_FAR * AuthIdentity,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc,
    _In_ unsigned long RpcQosVersion,
    _Out_opt_ RPC_SECURITY_QOS * SecurityQOS
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef void
(__RPC_USER * RPC_AUTH_KEY_RETRIEVAL_FN) (
    _In_ void __RPC_FAR * Arg,
    _In_ RPC_WSTR ServerPrincName,
    _In_ unsigned long KeyVer,
    _Outptr_ void __RPC_FAR * __RPC_FAR * Key,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY 
RpcServerCompleteSecurityCallback(
    _In_ RPC_BINDING_HANDLE BindingHandle,
    _In_ RPC_STATUS Status
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerRegisterAuthInfoA (
    _In_opt_ RPC_CSTR ServerPrincName,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_KEY_RETRIEVAL_FN GetKeyFn,
    _In_opt_ void __RPC_FAR * Arg
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerRegisterAuthInfoW (
    _In_opt_ RPC_WSTR ServerPrincName,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_KEY_RETRIEVAL_FN GetKeyFn,
    _In_opt_ void __RPC_FAR * Arg
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifdef UNICODE
#define RpcBindingInqAuthClient RpcBindingInqAuthClientW
#define RpcBindingInqAuthClientEx RpcBindingInqAuthClientExW
#define RpcBindingInqAuthInfo RpcBindingInqAuthInfoW
#define RpcBindingSetAuthInfo RpcBindingSetAuthInfoW
#define RpcServerRegisterAuthInfo RpcServerRegisterAuthInfoW
#define RpcBindingInqAuthInfoEx RpcBindingInqAuthInfoExW
#define RpcBindingSetAuthInfoEx RpcBindingSetAuthInfoExW
#else /* UNICODE */
#define RpcBindingInqAuthClient RpcBindingInqAuthClientA
#define RpcBindingInqAuthClientEx RpcBindingInqAuthClientExA
#define RpcBindingInqAuthInfo RpcBindingInqAuthInfoA
#define RpcBindingSetAuthInfo RpcBindingSetAuthInfoA
#define RpcServerRegisterAuthInfo RpcServerRegisterAuthInfoA
#define RpcBindingInqAuthInfoEx RpcBindingInqAuthInfoExA
#define RpcBindingSetAuthInfoEx RpcBindingSetAuthInfoExA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClient (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthClientEx (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_AUTHZ_HANDLE __RPC_FAR * Privs,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc,
    _In_  unsigned long             Flags
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingInqAuthInfo (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * ServerPrincName,
    _Out_opt_ unsigned long __RPC_FAR * AuthnLevel,
    _Out_opt_ unsigned long __RPC_FAR * AuthnSvc,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE __RPC_FAR * AuthIdentity,
    _Out_opt_ unsigned long __RPC_FAR * AuthzSvc
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcBindingSetAuthInfo (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ RPC_CSTR ServerPrincName,
    _In_ unsigned long AuthnLevel,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,
    _In_ unsigned long AuthzSvc
    );

typedef void
(__RPC_USER * RPC_AUTH_KEY_RETRIEVAL_FN) (
    _In_ void __RPC_FAR * Arg,
    _In_ unsigned char __RPC_FAR * ServerPrincName,
    _In_ unsigned long KeyVer,
    _Out_ void __RPC_FAR * __RPC_FAR * Key,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerRegisterAuthInfo (
    _In_ RPC_CSTR ServerPrincName,
    _In_ unsigned long AuthnSvc,
    _In_opt_ RPC_AUTH_KEY_RETRIEVAL_FN GetKeyFn,
    _In_opt_ OPTIONAL void __RPC_FAR * Arg
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WINXP)
#if !defined(_M_IA64)
typedef struct {
    unsigned char __RPC_FAR * UserName;
    unsigned char __RPC_FAR * ComputerName;
    unsigned short Privilege;
    unsigned long AuthFlags;
} RPC_CLIENT_INFORMATION1, __RPC_FAR * PRPC_CLIENT_INFORMATION1;
#endif 
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcBindingServerFromClient (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Out_ RPC_BINDING_HANDLE __RPC_FAR * ServerBinding
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
DECLSPEC_NORETURN
void
RPC_ENTRY
RpcRaiseException (
    _In_ RPC_STATUS exception
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcTestCancel(
    void
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcServerTestCancel (
    _In_opt_ RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcCancelThread(
    _In_ void * Thread
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcCancelThreadEx(
    _In_ void * Thread,
    _In_ long Timeout
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidCreate (
    _Out_ UUID __RPC_FAR * Uuid
    );

/* client/server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
UuidCreateSequential (
    _Out_ UUID __RPC_FAR * Uuid
    );

#ifdef RPC_UNICODE_SUPPORTED

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidToStringA (
    _In_ const UUID __RPC_FAR * Uuid,
    _Outptr_ RPC_CSTR __RPC_FAR * StringUuid
    );

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidFromStringA (
    _In_opt_ RPC_CSTR StringUuid,
    _Out_ UUID __RPC_FAR * Uuid
    );

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidToStringW (
    _In_ const UUID __RPC_FAR * Uuid,
    _Outptr_ RPC_WSTR __RPC_FAR * StringUuid
    );

/* client/server */
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidFromStringW (
    _In_opt_ RPC_WSTR StringUuid,
    _Out_ UUID __RPC_FAR * Uuid
    );

#ifdef UNICODE
#define UuidFromString UuidFromStringW
#define UuidToString UuidToStringW
#else /* UNICODE */
#define UuidFromString UuidFromStringA
#define UuidToString UuidToStringA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

/* client/server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
UuidToString (
    _In_ const UUID __RPC_FAR * Uuid,
    _Outptr_ RPC_CSTR __RPC_FAR * StringUuid
    );

/* client/server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
UuidFromString (
    _In_opt_ RPC_CSTR StringUuid,
    _Out_ UUID __RPC_FAR * Uuid
    );

#endif /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
signed int
RPC_ENTRY
UuidCompare (
    _In_ UUID __RPC_FAR * Uuid1,
    _In_ UUID __RPC_FAR * Uuid2,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
UuidCreateNil (
    _Out_ UUID __RPC_FAR * NilUuid
    );

RPCRTAPI
int
RPC_ENTRY
UuidEqual (
    _In_ UUID __RPC_FAR * Uuid1,
    _In_ UUID __RPC_FAR * Uuid2,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

RPCRTAPI
unsigned short
RPC_ENTRY
UuidHash (
    _In_ UUID __RPC_FAR * Uuid,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

RPCRTAPI
int
RPC_ENTRY
UuidIsNil (
    _In_ UUID __RPC_FAR * Uuid,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpRegisterNoReplaceA (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_CSTR Annotation
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpRegisterNoReplaceW (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_WSTR Annotation
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpRegisterA (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_CSTR Annotation
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpRegisterW (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_WSTR Annotation
    );

#ifdef UNICODE
#define RpcEpRegisterNoReplace RpcEpRegisterNoReplaceW
#define RpcEpRegister RpcEpRegisterW
#else /* UNICODE */
#define RpcEpRegisterNoReplace RpcEpRegisterNoReplaceA
#define RpcEpRegister RpcEpRegisterA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcEpRegisterNoReplace (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_CSTR Annotation
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcEpRegister (
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector,
    _In_opt_ RPC_CSTR Annotation
    );

#endif /* RPC_UNICODE_SUPPORTED */


RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcEpUnregister(
    _In_ RPC_IF_HANDLE IfSpec,
    _In_ RPC_BINDING_VECTOR * BindingVector,
    _In_opt_ UUID_VECTOR * UuidVector
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define DCE_C_ERROR_STRING_LEN 256

#ifdef RPC_UNICODE_SUPPORTED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
DceErrorInqTextA (
    _In_ RPC_STATUS RpcStatus,
    _Out_writes_(DCE_C_ERROR_STRING_LEN) RPC_CSTR ErrorText
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
DceErrorInqTextW (
    _In_ RPC_STATUS RpcStatus,
    _Out_writes_(DCE_C_ERROR_STRING_LEN) RPC_WSTR ErrorText
    );

#ifdef UNICODE
#define DceErrorInqText DceErrorInqTextW
#else /* UNICODE */
#define DceErrorInqText DceErrorInqTextA
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#else /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
DceErrorInqText (
    _In_ RPC_STATUS RpcStatus,
    _Out_writes_(DCE_C_ERROR_STRING_LEN) RPC_CSTR ErrorText
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* RPC_UNICODE_SUPPORTED */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef I_RPC_HANDLE * RPC_EP_INQ_HANDLE;

#define  RPC_C_EP_ALL_ELTS        0
#define  RPC_C_EP_MATCH_BY_IF     1
#define  RPC_C_EP_MATCH_BY_OBJ    2
#define  RPC_C_EP_MATCH_BY_BOTH   3

#define  RPC_C_VERS_ALL           1
#define  RPC_C_VERS_COMPATIBLE    2
#define  RPC_C_VERS_EXACT         3
#define  RPC_C_VERS_MAJOR_ONLY    4
#define  RPC_C_VERS_UPTO          5

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEpEltInqBegin (
    _In_opt_ OPTIONAL RPC_BINDING_HANDLE EpBinding,
    _In_ unsigned long InquiryType,
    _In_opt_ RPC_IF_ID __RPC_FAR * IfId,
    _In_opt_ unsigned long VersOption,
    _In_opt_ UUID __RPC_FAR * ObjectUuid,
    _Out_ RPC_EP_INQ_HANDLE __RPC_FAR * InquiryContext
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEpEltInqDone (
    _Inout_ RPC_EP_INQ_HANDLE __RPC_FAR * InquiryContext
    );

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEpEltInqNextA (
    _In_ RPC_EP_INQ_HANDLE InquiryContext,
    _Out_ RPC_IF_ID __RPC_FAR * IfId,
    _Out_opt_ RPC_BINDING_HANDLE __RPC_FAR * Binding,
    _Out_opt_ UUID __RPC_FAR * ObjectUuid,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Annotation
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEpEltInqNextW (
    _In_ RPC_EP_INQ_HANDLE InquiryContext,
    _Out_ RPC_IF_ID __RPC_FAR * IfId,
    _Out_opt_ RPC_BINDING_HANDLE __RPC_FAR * Binding,
    _Out_opt_ UUID __RPC_FAR * ObjectUuid,
    _Outptr_opt_ RPC_WSTR __RPC_FAR * Annotation
    );

#ifdef UNICODE
#define RpcMgmtEpEltInqNext RpcMgmtEpEltInqNextW
#else /* UNICODE */
#define RpcMgmtEpEltInqNext RpcMgmtEpEltInqNextA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcMgmtEpEltInqNext (
    _In_ RPC_EP_INQ_HANDLE InquiryContext,
    _Out_ RPC_IF_ID __RPC_FAR * IfId,
    _Out_opt_ RPC_BINDING_HANDLE __RPC_FAR * Binding,
    _Outptr_opt_ RPC_CSTR __RPC_FAR * Annotation
    );

#endif /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtEpUnregister (
    _In_opt_ RPC_BINDING_HANDLE EpBinding,
    _In_ RPC_IF_ID __RPC_FAR * IfId,
    _In_ RPC_BINDING_HANDLE Binding,
    _In_opt_ UUID __RPC_FAR * ObjectUuid
    );

typedef int
(__RPC_API * RPC_MGMT_AUTHORIZATION_FN) (
    _In_ RPC_BINDING_HANDLE ClientBinding,
    _In_ unsigned long RequestedMgmtOperation,
    _Out_ RPC_STATUS __RPC_FAR * Status
    );

#define RPC_C_MGMT_INQ_IF_IDS         0
#define RPC_C_MGMT_INQ_PRINC_NAME     1
#define RPC_C_MGMT_INQ_STATS          2
#define RPC_C_MGMT_IS_SERVER_LISTEN   3
#define RPC_C_MGMT_STOP_SERVER_LISTEN 4

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcMgmtSetAuthorizationFn (
    _In_ RPC_MGMT_AUTHORIZATION_FN AuthorizationFn
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_VISTA)
RPCRTAPI
int
RPC_ENTRY
RpcExceptionFilter (
    _In_ unsigned long ExceptionCode
    );
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define RPC_C_PARM_MAX_PACKET_LENGTH    1
#define RPC_C_PARM_BUFFER_LENGTH        2

#define RPC_IF_AUTOLISTEN                   0x0001
#define RPC_IF_OLE                          0x0002
#define RPC_IF_ALLOW_UNKNOWN_AUTHORITY      0x0004
#define RPC_IF_ALLOW_SECURE_ONLY            0x0008
#define RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH 0x0010
#define RPC_IF_ALLOW_LOCAL_ONLY             0x0020
#define RPC_IF_SEC_NO_CACHE                 0x0040
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_IF_SEC_CACHE_PER_PROC           0x0080
#define RPC_IF_ASYNC_CALLBACK               0x0100
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_FW_IF_FLAG_DCOM                 0x0001
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef void *RPC_INTERFACE_GROUP, **PRPC_INTERFACE_GROUP;

#ifdef RPC_UNICODE_SUPPORTED
typedef struct 
{
    unsigned long Version;
    RPC_WSTR ProtSeq;
    RPC_WSTR Endpoint;
    void * SecurityDescriptor;
    unsigned long Backlog;
} RPC_ENDPOINT_TEMPLATEW, *PRPC_ENDPOINT_TEMPLATEW;

typedef struct 
{
    unsigned long Version;
    RPC_CSTR ProtSeq;
    RPC_CSTR Endpoint;
    void * SecurityDescriptor;
    unsigned long Backlog;
} RPC_ENDPOINT_TEMPLATEA, *PRPC_ENDPOINT_TEMPLATEA;

#ifdef UNICODE
#define RPC_ENDPOINT_TEMPLATE RPC_ENDPOINT_TEMPLATEW
#define PRPC_ENDPOINT_TEMPLATE PRPC_ENDPOINT_TEMPLATEW
#else /* UNICODE */
#define RPC_ENDPOINT_TEMPLATE RPC_ENDPOINT_TEMPLATEA
#define PRPC_ENDPOINT_TEMPLATE PRPC_ENDPOINT_TEMPLATEA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */
typedef struct 
{
    unsigned long Version;
    RPC_CSTR ProtSeq;
    RPC_CSTR Endpoint;
    void * SecurityDescriptor;
    unsigned long Backlog;
} RPC_ENDPOINT_TEMPLATE, *PRPC_ENDPOINT_TEMPLATE;
#endif /* RPC_UNICODE_SUPPORTED */

#ifdef RPC_UNICODE_SUPPORTED
typedef struct 
{
    unsigned long Version;
    RPC_IF_HANDLE IfSpec;
    UUID * MgrTypeUuid;
    RPC_MGR_EPV * MgrEpv;
    unsigned int Flags;
    unsigned int MaxCalls;
    unsigned int MaxRpcSize;
    RPC_IF_CALLBACK_FN *IfCallback;
    UUID_VECTOR *UuidVector;
    RPC_CSTR Annotation;
    void * SecurityDescriptor;
} RPC_INTERFACE_TEMPLATEA, *PRPC_INTERFACE_TEMPLATEA;

typedef struct 
{
    unsigned long Version;
    RPC_IF_HANDLE IfSpec;
    UUID * MgrTypeUuid;
    RPC_MGR_EPV * MgrEpv;
    unsigned int Flags;
    unsigned int MaxCalls;
    unsigned int MaxRpcSize;
    RPC_IF_CALLBACK_FN *IfCallback;
    UUID_VECTOR *UuidVector;
    RPC_WSTR Annotation;
    void * SecurityDescriptor;
} RPC_INTERFACE_TEMPLATEW, *PRPC_INTERFACE_TEMPLATEW;

#ifdef UNICODE
#define RPC_INTERFACE_TEMPLATE RPC_INTERFACE_TEMPLATEW
#define PRPC_INTERFACE_TEMPLATE PRPC_INTERFACE_TEMPLATEW
#else /* UNICODE */
#define RPC_INTERFACE_TEMPLATE RPC_INTERFACE_TEMPLATEA
#define PRPC_INTERFACE_TEMPLATE PRPC_INTERFACE_TEMPLATEA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

typedef struct 
{
    unsigned long Version;
    RPC_IF_HANDLE IfSpec;
    UUID * MgrTypeUuid;
    RPC_MGR_EPV * MgrEpv;
    unsigned int Flags;
    unsigned int MaxCalls;
    unsigned int MaxRpcSize;
    RPC_IF_CALLBACK_FN *IfCallback;
    UUID_VECTOR *UuidVector;
    RPC_CSTR Annotation;
    void * SecurityDescriptor;
} RPC_INTERFACE_TEMPLATE, *PRPC_INTERFACE_TEMPLATE;

#endif /* RPC_UNICODE_SUPPORTED */



typedef void RPC_ENTRY
RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN (
    _In_ RPC_INTERFACE_GROUP IfGroup,
    _In_ void* IdleCallbackContext,
    _In_ unsigned long IsGroupIdle
    );

#ifdef RPC_UNICODE_SUPPORTED

/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupCreateW (
    _In_reads_(NumIfs) RPC_INTERFACE_TEMPLATEW *Interfaces,
    _In_ unsigned long NumIfs,
    _In_reads_(NumEndpoints) RPC_ENDPOINT_TEMPLATEW *Endpoints,
    _In_ unsigned long NumEndpoints,
    _In_ unsigned long IdlePeriod,
    _In_ RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN IdleCallbackFn,
    _In_ void* IdleCallbackContext,
    _Out_ PRPC_INTERFACE_GROUP IfGroup
    );


/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupCreateA (
    _In_reads_(NumIfs) RPC_INTERFACE_TEMPLATEA *Interfaces,
    _In_ unsigned long NumIfs,
    _In_reads_(NumEndpoints) RPC_ENDPOINT_TEMPLATEA *Endpoints,
    _In_ unsigned long NumEndpoints,
    _In_ unsigned long IdlePeriod,
    _In_ RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN IdleCallbackFn,
    _In_ void* IdleCallbackContext,
    _Out_ PRPC_INTERFACE_GROUP IfGroup
    );

#ifdef UNICODE
#define RpcServerInterfaceGroupCreate RpcServerInterfaceGroupCreateW
#else /* UNICODE */
#define RpcServerInterfaceGroupCreate RpcServerInterfaceGroupCreateA
#endif /* UNICODE */

#else /* RPC_UNICODE_SUPPORTED */

/* server */
RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupCreate (
    _In_reads_(NumIfs) RPC_INTERFACE_TEMPLATE *Interfaces,
    _In_ unsigned long NumIfs,
    _In_reads_(NumEndpoints) RPC_ENDPOINT_TEMPLATE *Endpoints,
    _In_ unsigned long NumEndpoints,
    _In_ unsigned long IdlePeriod,
    _In_ RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN IdleCallbackFn,
    _In_ void* IdleCallbackContext,
    _Out_ PRPC_INTERFACE_GROUP IfGroup
    );

#endif /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupClose (
    _In_ RPC_INTERFACE_GROUP IfGroup
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupActivate (
    _In_ RPC_INTERFACE_GROUP IfGroup
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupDeactivate (
    _In_ RPC_INTERFACE_GROUP IfGroup,
    _In_ unsigned long ForceDeactivation
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInterfaceGroupInqBindings (
    _In_ RPC_INTERFACE_GROUP IfGroup,
    _Outptr_ RPC_BINDING_VECTOR __RPC_FAR * __RPC_FAR * BindingVector
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#include <rpcdcep.h>

#ifdef __cplusplus
}
#endif

#endif /* __RPCDCE_H__ */

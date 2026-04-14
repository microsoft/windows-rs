

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __fwpstypes_h__
#define __fwpstypes_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "fwptypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fwpstypes_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family or AppRuntime Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME)
#if _MSC_VER >=  800
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)
#endif
typedef struct FWPS_FILTER_CONDITION0_
    {
    UINT16 fieldId;
    UINT16 reserved;
    FWP_MATCH_TYPE matchType;
    FWP_CONDITION_VALUE0 conditionValue;
    } 	FWPS_FILTER_CONDITION0;

typedef struct FWPS_ACTION0_
    {
    FWP_ACTION_TYPE type;
    UINT32 calloutId;
    } 	FWPS_ACTION0;

#define FWPS_FILTER_FLAG_CLEAR_ACTION_RIGHT    (0x0001)
#define FWPS_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED   (0x0002)
#define FWPS_FILTER_FLAG_OR_CONDITIONS   (0x0004)
#define FWPS_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT (0x0008)
#define FWPS_FILTER_FLAG_SILENT_MODE     (0x0010)
#define FWPS_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE (0x0020)
#define FWPS_FILTER_FLAG_RESERVED0 (0x0040)
#define FWPS_FILTER_FLAG_RESERVED1 (0x0080)
#define FWPS_FILTER_FLAG_RESERVED2 (0x0100)
typedef struct FWPM_PROVIDER_CONTEXT0_ FWPM_PROVIDER_CONTEXT0;

typedef struct FWPS_FILTER0_
    {
    UINT64 filterId;
    FWP_VALUE0 weight;
    UINT16 subLayerWeight;
    UINT16 flags;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPS_FILTER_CONDITION0 *filterCondition;
    FWPS_ACTION0 action;
    UINT64 context;
    /* [unique] */ FWPM_PROVIDER_CONTEXT0 *providerContext;
    } 	FWPS_FILTER0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_PROVIDER_CONTEXT1_ FWPM_PROVIDER_CONTEXT1;

typedef struct FWPS_FILTER1_
    {
    UINT64 filterId;
    FWP_VALUE0 weight;
    UINT16 subLayerWeight;
    UINT16 flags;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPS_FILTER_CONDITION0 *filterCondition;
    FWPS_ACTION0 action;
    UINT64 context;
    /* [unique] */ FWPM_PROVIDER_CONTEXT1 *providerContext;
    } 	FWPS_FILTER1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_PROVIDER_CONTEXT2_ FWPM_PROVIDER_CONTEXT2;

typedef struct FWPS_FILTER2_
    {
    UINT64 filterId;
    FWP_VALUE0 weight;
    UINT16 subLayerWeight;
    UINT16 flags;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPS_FILTER_CONDITION0 *filterCondition;
    FWPS_ACTION0 action;
    UINT64 context;
    /* [unique] */ FWPM_PROVIDER_CONTEXT2 *providerContext;
    } 	FWPS_FILTER2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef struct FWPM_PROVIDER_CONTEXT3_ FWPM_PROVIDER_CONTEXT3;

typedef struct FWPS_FILTER3_
    {
    UINT64 filterId;
    FWP_VALUE0 weight;
    UINT16 subLayerWeight;
    UINT16 flags;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPS_FILTER_CONDITION0 *filterCondition;
    FWPS_ACTION0 action;
    UINT64 context;
    /* [unique] */ FWPM_PROVIDER_CONTEXT3 *providerContext;
    } 	FWPS_FILTER3;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef struct FWPS_INCOMING_VALUE0_
    {
    FWP_VALUE0 value;
    } 	FWPS_INCOMING_VALUE0;

typedef struct FWPS_INCOMING_VALUES0_
    {
    UINT16 layerId;
    UINT32 valueCount;
    /* [ref][size_is] */ FWPS_INCOMING_VALUE0 *incomingValue;
    } 	FWPS_INCOMING_VALUES0;

typedef 
enum FWPS_DISCARD_MODULE0_
    {
        FWPS_DISCARD_MODULE_NETWORK	= 0,
        FWPS_DISCARD_MODULE_TRANSPORT	= ( FWPS_DISCARD_MODULE_NETWORK + 1 ) ,
        FWPS_DISCARD_MODULE_GENERAL	= ( FWPS_DISCARD_MODULE_TRANSPORT + 1 ) ,
        FWPS_DISCARD_MODULE_MAX	= ( FWPS_DISCARD_MODULE_GENERAL + 1 ) 
    } 	FWPS_DISCARD_MODULE0;

typedef 
enum FWPS_GENERAL_DISCARD_REASON_
    {
        FWPS_DISCARD_FIREWALL_POLICY	= 0,
        FWPS_DISCARD_IPSEC	= ( FWPS_DISCARD_FIREWALL_POLICY + 1 ) ,
        FWPS_GENERAL_DISCARD_REASON_MAX	= ( FWPS_DISCARD_IPSEC + 1 ) 
    } 	FWPS_GENERAL_DISCARD_REASON;

typedef struct FWPS_DISCARD_METADATA0_
    {
    FWPS_DISCARD_MODULE0 discardModule;
    UINT32 discardReason;
    UINT64 filterId;
    } 	FWPS_DISCARD_METADATA0;

typedef struct FWPS_INBOUND_FRAGMENT_METADATA0_
    {
    UINT32 fragmentIdentification;
    UINT16 fragmentOffset;
    ULONG fragmentLength;
    } 	FWPS_INBOUND_FRAGMENT_METADATA0;

#define FWPS_INCOMING_FLAG_CACHE_SAFE                         (0x00000001)
#define FWPS_INCOMING_FLAG_ENFORCE_QUERY                      (0x00000002)
#define FWPS_INCOMING_FLAG_ABSORB                             (0x00000004)
#define FWPS_INCOMING_FLAG_CONNECTION_FAILING_INDICATION      (0x00000008)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPS_INCOMING_FLAG_MID_STREAM_INSPECTION              (0x00000010)
#define FWPS_INCOMING_FLAG_RECLASSIFY                         (0x00000020)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define FWPS_INCOMING_FLAG_IS_LOOSE_SOURCE_FLOW               (0x00000040)
#define FWPS_INCOMING_FLAG_IS_LOCAL_ONLY_FLOW                 (0x00000080)
#define FWPS_L2_INCOMING_FLAG_IS_RAW_IPV4_FRAMING             (0x00000001)
#define FWPS_L2_INCOMING_FLAG_IS_RAW_IPV6_FRAMING             (0x00000002)
#define FWPS_L2_INCOMING_FLAG_RECLASSIFY_MULTI_DESTINATION    (0x00000008)
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
#define FWPS_INCOMING_FLAG_RESERVED0                          (0x00000100)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPS_RIGHT_ACTION_WRITE          (0x00000001)
#define FWPS_CLASSIFY_OUT_FLAG_ABSORB                         (0x00000001)
#define FWPS_CLASSIFY_OUT_FLAG_BUFFER_LIMIT_REACHED           (0x00000002)
#define FWPS_CLASSIFY_OUT_FLAG_NO_MORE_DATA                   (0x00000004)
#define FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_CHECK           (0x00000008)
#define FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_POSSIBLE        (0x00000010)
typedef struct FWPS_CLASSIFY_OUT0_
    {
    FWP_ACTION_TYPE actionType;
    UINT64 outContext;
    UINT64 filterId;
    UINT32 rights;
    UINT32 flags;
    UINT32 reserved;
    } 	FWPS_CLASSIFY_OUT0;

typedef 
enum FWPS_CALLOUT_NOTIFY_TYPE_
    {
        FWPS_CALLOUT_NOTIFY_ADD_FILTER	= 0,
        FWPS_CALLOUT_NOTIFY_DELETE_FILTER	= ( FWPS_CALLOUT_NOTIFY_ADD_FILTER + 1 ) ,
        FWPS_CALLOUT_NOTIFY_ADD_FILTER_POST_COMMIT	= ( FWPS_CALLOUT_NOTIFY_DELETE_FILTER + 1 ) ,
        FWPS_CALLOUT_NOTIFY_TYPE_MAX	= ( FWPS_CALLOUT_NOTIFY_ADD_FILTER_POST_COMMIT + 1 ) 
    } 	FWPS_CALLOUT_NOTIFY_TYPE;

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPS_ALE_ENDPOINT_FLAG_IPSEC_SECURED   (0x00000001)
typedef struct FWPS_ALE_ENDPOINT_PROPERTIES0_
    {
    UINT64 endpointId;
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localV4Address;
        /* [case()] */ UINT8 localV6Address[ 16 ];
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteV4Address;
        /* [case()] */ UINT8 remoteV6Address[ 16 ];
        } 	;
    UINT8 ipProtocol;
    UINT16 localPort;
    UINT16 remotePort;
    UINT64 localTokenModifiedId;
    UINT64 mmSaId;
    UINT64 qmSaId;
    UINT32 ipsecStatus;
    UINT32 flags;
    FWP_BYTE_BLOB appId;
    } 	FWPS_ALE_ENDPOINT_PROPERTIES0;

typedef struct FWPS_ALE_ENDPOINT_ENUM_TEMPLATE0_
    {
    FWP_CONDITION_VALUE0 localSubNet;
    FWP_CONDITION_VALUE0 remoteSubNet;
    FWP_CONDITION_VALUE0 ipProtocol;
    FWP_CONDITION_VALUE0 localPort;
    FWP_CONDITION_VALUE0 remotePort;
    } 	FWPS_ALE_ENDPOINT_ENUM_TEMPLATE0;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if _MSC_VER >=  800
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fwpstypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fwpstypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



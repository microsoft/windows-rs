

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


#ifndef __fwpmtypes_h__
#define __fwpmtypes_h__

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
#include "iketypes.h"
#include "ipsectypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fwpmtypes_0000_0000 */
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
#ifndef _DEFINE_DL_ADDRESS_TYPE_
#define _DEFINE_DL_ADDRESS_TYPE_
typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_fwpmtypes_0000_0000_0001
    {
        DlUnicast	= 0,
        DlMulticast	= ( DlUnicast + 1 ) ,
        DlBroadcast	= ( DlMulticast + 1 ) 
    } 	DL_ADDRESS_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_fwpmtypes_0000_0000_0001 *PDL_ADDRESS_TYPE;

#endif
typedef /* [v1_enum] */ 
enum FWPM_CHANGE_TYPE_
    {
        FWPM_CHANGE_ADD	= 1,
        FWPM_CHANGE_DELETE	= ( FWPM_CHANGE_ADD + 1 ) ,
        FWPM_CHANGE_TYPE_MAX	= ( FWPM_CHANGE_DELETE + 1 ) 
    } 	FWPM_CHANGE_TYPE;

#define FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_ADD    (0x00000001)
#define FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_DELETE (0x00000002)
typedef 
enum FWPM_SERVICE_STATE_
    {
        FWPM_SERVICE_STOPPED	= 0,
        FWPM_SERVICE_START_PENDING	= ( FWPM_SERVICE_STOPPED + 1 ) ,
        FWPM_SERVICE_STOP_PENDING	= ( FWPM_SERVICE_START_PENDING + 1 ) ,
        FWPM_SERVICE_RUNNING	= ( FWPM_SERVICE_STOP_PENDING + 1 ) ,
        FWPM_SERVICE_STATE_MAX	= ( FWPM_SERVICE_RUNNING + 1 ) 
    } 	FWPM_SERVICE_STATE;

#define FWPM_NET_EVENT_KEYWORD_INBOUND_MCAST      (0x00000001)
#define FWPM_NET_EVENT_KEYWORD_INBOUND_BCAST      (0x00000002)
#define FWPM_NET_EVENT_KEYWORD_CAPABILITY_DROP    (0x00000004)
#define FWPM_NET_EVENT_KEYWORD_CAPABILITY_ALLOW   (0x00000008)
#define FWPM_NET_EVENT_KEYWORD_CLASSIFY_ALLOW     (0x00000010)
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
#define FWPM_NET_EVENT_KEYWORD_PORT_SCANNING_DROP (0x00000020)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)
#define FWPM_ENGINE_OPTION_PACKET_QUEUE_NONE (0x00000000)
#define FWPM_ENGINE_OPTION_PACKET_QUEUE_INBOUND (0x00000001)
#define FWPM_ENGINE_OPTION_PACKET_QUEUE_FORWARD (0x00000002)
#define FWPM_ENGINE_OPTION_PACKET_BATCH_INBOUND (0x00000004)
typedef 
enum FWPM_ENGINE_OPTION_
    {
        FWPM_ENGINE_COLLECT_NET_EVENTS	= 0,
        FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS	= ( FWPM_ENGINE_COLLECT_NET_EVENTS + 1 ) ,
        FWPM_ENGINE_NAME_CACHE	= ( FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS + 1 ) ,
        FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS	= ( FWPM_ENGINE_NAME_CACHE + 1 ) ,
        FWPM_ENGINE_PACKET_QUEUING	= ( FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS + 1 ) ,
        FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC	= ( FWPM_ENGINE_PACKET_QUEUING + 1 ) ,
        FWPM_ENGINE_OPTION_MAX	= ( FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC + 1 ) 
    } 	FWPM_ENGINE_OPTION;

#define FWPM_SESSION_FLAG_DYNAMIC (0x00000001)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define FWPM_SESSION_FLAG_RESERVED (0x10000000)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_SESSION0_
    {
    GUID sessionKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    UINT32 txnWaitTimeoutInMSec;
    DWORD processId;
    /* [unique] */ SID *sid;
    /* [unique][string] */ wchar_t *username;
    BOOL kernelMode;
    } 	FWPM_SESSION0;

typedef struct FWPM_SESSION_ENUM_TEMPLATE0_
    {
    UINT64 reserved;
    } 	FWPM_SESSION_ENUM_TEMPLATE0;

#define FWPM_PROVIDER_FLAG_PERSISTENT  (0x00000001)
#define FWPM_PROVIDER_FLAG_DISABLED    (0x00000010)
typedef struct FWPM_PROVIDER0_
    {
    GUID providerKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    FWP_BYTE_BLOB providerData;
    /* [unique][string] */ wchar_t *serviceName;
    } 	FWPM_PROVIDER0;

typedef struct FWPM_PROVIDER_ENUM_TEMPLATE0_
    {
    UINT64 reserved;
    } 	FWPM_PROVIDER_ENUM_TEMPLATE0;

typedef struct FWPM_PROVIDER_CHANGE0_
    {
    FWPM_CHANGE_TYPE changeType;
    GUID providerKey;
    } 	FWPM_PROVIDER_CHANGE0;

typedef struct FWPM_PROVIDER_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_PROVIDER_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_PROVIDER_SUBSCRIPTION0;

#define FWPM_PROVIDER_CONTEXT_FLAG_PERSISTENT  (0x00000001)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define FWPM_PROVIDER_CONTEXT_FLAG_DOWNLEVEL  (0x00000002)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_CLASSIFY_OPTION0_
    {
    FWP_CLASSIFY_OPTION_TYPE type;
    FWP_VALUE0 value;
    } 	FWPM_CLASSIFY_OPTION0;

typedef struct FWPM_CLASSIFY_OPTIONS0_
    {
    UINT32 numOptions;
    /* [ref][size_is] */ FWPM_CLASSIFY_OPTION0 *options;
    } 	FWPM_CLASSIFY_OPTIONS0;

typedef struct FWPM_NETWORK_CONNECTION_POLICY_SETTING0_
    {
    FWP_NETWORK_CONNECTION_POLICY_SETTING_TYPE type;
    FWP_VALUE0 value;
    } 	FWPM_NETWORK_CONNECTION_POLICY_SETTING0;

typedef struct FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0_
    {
    UINT32 numSettings;
    /* [ref][size_is] */ FWPM_NETWORK_CONNECTION_POLICY_SETTING0 *settings;
    } 	FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0;

typedef /* [v1_enum] */ 
enum FWPM_PROVIDER_CONTEXT_TYPE_
    {
        FWPM_IPSEC_KEYING_CONTEXT	= 0,
        FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT	= ( FWPM_IPSEC_KEYING_CONTEXT + 1 ) ,
        FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT	= ( FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT + 1 ) ,
        FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT	= ( FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT + 1 ) ,
        FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT	= ( FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT + 1 ) ,
        FWPM_IPSEC_IKE_MM_CONTEXT	= ( FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT + 1 ) ,
        FWPM_IPSEC_AUTHIP_MM_CONTEXT	= ( FWPM_IPSEC_IKE_MM_CONTEXT + 1 ) ,
        FWPM_CLASSIFY_OPTIONS_CONTEXT	= ( FWPM_IPSEC_AUTHIP_MM_CONTEXT + 1 ) ,
        FWPM_GENERAL_CONTEXT	= ( FWPM_CLASSIFY_OPTIONS_CONTEXT + 1 ) ,
        FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT	= ( FWPM_GENERAL_CONTEXT + 1 ) ,
        FWPM_IPSEC_IKEV2_MM_CONTEXT	= ( FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT + 1 ) ,
        FWPM_IPSEC_DOSP_CONTEXT	= ( FWPM_IPSEC_IKEV2_MM_CONTEXT + 1 ) ,
        FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT	= ( FWPM_IPSEC_DOSP_CONTEXT + 1 ) ,
        FWPM_NETWORK_CONNECTION_POLICY_CONTEXT	= ( FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT + 1 ) ,
        FWPM_PROVIDER_CONTEXT_TYPE_MAX	= ( FWPM_NETWORK_CONNECTION_POLICY_CONTEXT + 1 ) 
    } 	FWPM_PROVIDER_CONTEXT_TYPE;

typedef struct FWPM_PROVIDER_CONTEXT0_
    {
    GUID providerContextKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    FWPM_PROVIDER_CONTEXT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_KEYING_POLICY0 *keyingPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY0 *ikeQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY0 *ikeQmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY0 *authipQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY0 *authipQmTunnelPolicy;
        /* [case()][unique] */ IKEEXT_POLICY0 *ikeMmPolicy;
        /* [case()][unique] */ IKEEXT_POLICY0 *authIpMmPolicy;
        /* [case()][unique] */ FWP_BYTE_BLOB *dataBuffer;
        /* [case()][unique] */ FWPM_CLASSIFY_OPTIONS0 *classifyOptions;
        /* [default] */  /* Empty union arm */ 
        } 	;
    UINT64 providerContextId;
    } 	FWPM_PROVIDER_CONTEXT0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_PROVIDER_CONTEXT1_
    {
    GUID providerContextKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    FWPM_PROVIDER_CONTEXT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_KEYING_POLICY0 *keyingPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY1 *ikeQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY1 *ikeQmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY1 *authipQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY1 *authipQmTunnelPolicy;
        /* [case()][unique] */ IKEEXT_POLICY1 *ikeMmPolicy;
        /* [case()][unique] */ IKEEXT_POLICY1 *authIpMmPolicy;
        /* [case()][unique] */ FWP_BYTE_BLOB *dataBuffer;
        /* [case()][unique] */ FWPM_CLASSIFY_OPTIONS0 *classifyOptions;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY1 *ikeV2QmTunnelPolicy;
        /* [case()][unique] */ IKEEXT_POLICY1 *ikeV2MmPolicy;
        /* [case()][unique] */ IPSEC_DOSP_OPTIONS0 *idpOptions;
        } 	;
    UINT64 providerContextId;
    } 	FWPM_PROVIDER_CONTEXT1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_PROVIDER_CONTEXT2_
    {
    GUID providerContextKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    FWPM_PROVIDER_CONTEXT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_KEYING_POLICY1 *keyingPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *ikeQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY2 *ikeQmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *authipQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY2 *authipQmTunnelPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *ikeMmPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *authIpMmPolicy;
        /* [case()][unique] */ FWP_BYTE_BLOB *dataBuffer;
        /* [case()][unique] */ FWPM_CLASSIFY_OPTIONS0 *classifyOptions;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY2 *ikeV2QmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *ikeV2QmTransportPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *ikeV2MmPolicy;
        /* [case()][unique] */ IPSEC_DOSP_OPTIONS0 *idpOptions;
        } 	;
    UINT64 providerContextId;
    } 	FWPM_PROVIDER_CONTEXT2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef struct FWPM_PROVIDER_CONTEXT3_
    {
    GUID providerContextKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    FWPM_PROVIDER_CONTEXT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_KEYING_POLICY1 *keyingPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *ikeQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY3 *ikeQmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *authipQmTransportPolicy;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY3 *authipQmTunnelPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *ikeMmPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *authIpMmPolicy;
        /* [case()][unique] */ FWP_BYTE_BLOB *dataBuffer;
        /* [case()][unique] */ FWPM_CLASSIFY_OPTIONS0 *classifyOptions;
        /* [case()][unique] */ IPSEC_TUNNEL_POLICY3 *ikeV2QmTunnelPolicy;
        /* [case()][unique] */ IPSEC_TRANSPORT_POLICY2 *ikeV2QmTransportPolicy;
        /* [case()][unique] */ IKEEXT_POLICY2 *ikeV2MmPolicy;
        /* [case()][unique] */ IPSEC_DOSP_OPTIONS0 *idpOptions;
        /* [case()][unique] */ FWPM_NETWORK_CONNECTION_POLICY_SETTINGS0 *networkConnectionPolicy;
        } 	;
    UINT64 providerContextId;
    } 	FWPM_PROVIDER_CONTEXT3;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef struct FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0_
    {
    /* [unique] */ GUID *providerKey;
    FWPM_PROVIDER_CONTEXT_TYPE providerContextType;
    } 	FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0;

typedef struct FWPM_PROVIDER_CONTEXT_CHANGE0_
    {
    FWPM_CHANGE_TYPE changeType;
    GUID providerContextKey;
    UINT64 providerContextId;
    } 	FWPM_PROVIDER_CONTEXT_CHANGE0;

typedef struct FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0;

#define FWPM_SUBLAYER_FLAG_PERSISTENT       (0x00000001)
typedef struct FWPM_SUBLAYER0_
    {
    GUID subLayerKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    UINT16 weight;
    } 	FWPM_SUBLAYER0;

typedef struct FWPM_SUBLAYER_ENUM_TEMPLATE0_
    {
    /* [unique] */ GUID *providerKey;
    } 	FWPM_SUBLAYER_ENUM_TEMPLATE0;

typedef struct FWPM_SUBLAYER_CHANGE0_
    {
    FWPM_CHANGE_TYPE changeType;
    GUID subLayerKey;
    } 	FWPM_SUBLAYER_CHANGE0;

typedef struct FWPM_SUBLAYER_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_SUBLAYER_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_SUBLAYER_SUBSCRIPTION0;

#define FWPM_LAYER_FLAG_KERNEL           (0x00000001)
#define FWPM_LAYER_FLAG_BUILTIN          (0x00000002)
#define FWPM_LAYER_FLAG_CLASSIFY_MOSTLY  (0x00000004)
#define FWPM_LAYER_FLAG_BUFFERED         (0x00000008)
typedef /* [v1_enum] */ 
enum FWPM_FIELD_TYPE_
    {
        FWPM_FIELD_RAW_DATA	= 0,
        FWPM_FIELD_IP_ADDRESS	= ( FWPM_FIELD_RAW_DATA + 1 ) ,
        FWPM_FIELD_FLAGS	= ( FWPM_FIELD_IP_ADDRESS + 1 ) ,
        FWPM_FIELD_TYPE_MAX	= ( FWPM_FIELD_FLAGS + 1 ) 
    } 	FWPM_FIELD_TYPE;

typedef struct FWPM_FIELD0_
    {
    /* [ref] */ GUID *fieldKey;
    FWPM_FIELD_TYPE type;
    FWP_DATA_TYPE dataType;
    } 	FWPM_FIELD0;

typedef struct FWPM_LAYER0_
    {
    GUID layerKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    UINT32 numFields;
    /* [ref][size_is] */ FWPM_FIELD0 *field;
    GUID defaultSubLayerKey;
    UINT16 layerId;
    } 	FWPM_LAYER0;

typedef struct FWPM_LAYER_ENUM_TEMPLATE0_
    {
    UINT64 reserved;
    } 	FWPM_LAYER_ENUM_TEMPLATE0;

#define FWPM_CALLOUT_FLAG_PERSISTENT             (0x00010000)
#define FWPM_CALLOUT_FLAG_USES_PROVIDER_CONTEXT  (0x00020000)
#define FWPM_CALLOUT_FLAG_REGISTERED             (0x00040000)
typedef struct FWPM_CALLOUT0_
    {
    GUID calloutKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    GUID applicableLayer;
    UINT32 calloutId;
    } 	FWPM_CALLOUT0;

typedef struct FWPM_CALLOUT_ENUM_TEMPLATE0_
    {
    /* [unique] */ GUID *providerKey;
    GUID layerKey;
    } 	FWPM_CALLOUT_ENUM_TEMPLATE0;

typedef struct FWPM_CALLOUT_CHANGE0_
    {
    FWPM_CHANGE_TYPE changeType;
    GUID calloutKey;
    UINT32 calloutId;
    } 	FWPM_CALLOUT_CHANGE0;

typedef struct FWPM_CALLOUT_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_CALLOUT_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_CALLOUT_SUBSCRIPTION0;

typedef struct FWPM_ACTION0_
    {
    FWP_ACTION_TYPE type;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ GUID filterType;
        /* [case()] */ GUID calloutKey;
        } 	;
    } 	FWPM_ACTION0;

typedef struct FWPM_FILTER_CONDITION0_
    {
    GUID fieldKey;
    FWP_MATCH_TYPE matchType;
    FWP_CONDITION_VALUE0 conditionValue;
    } 	FWPM_FILTER_CONDITION0;

#define FWPM_FILTER_FLAG_NONE (0x00000000)
#define FWPM_FILTER_FLAG_PERSISTENT (0x00000001)
#define FWPM_FILTER_FLAG_BOOTTIME (0x00000002)
#define FWPM_FILTER_FLAG_HAS_PROVIDER_CONTEXT  (0x00000004)
#define FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT (0x00000008)
#define FWPM_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED (0x00000010)
#define FWPM_FILTER_FLAG_DISABLED (0x00000020)
#define FWPM_FILTER_FLAG_INDEXED (0x00000040)
#define FWPM_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT (0x00000080)
#define FWPM_FILTER_FLAG_SYSTEMOS_ONLY (0x00000100)
#define FWPM_FILTER_FLAG_GAMEOS_ONLY (0x00000200)
#define FWPM_FILTER_FLAG_SILENT_MODE (0x00000400)
#define FWPM_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE   (0x00000800)
#define FWPM_FILTER_FLAG_RESERVED0   (0x00001000)
#define FWPM_FILTER_FLAG_RESERVED1   (0x00002000)
#define FWPM_FILTER_FLAG_RESERVED2   (0x00004000)
typedef struct FWPM_FILTER0_
    {
    GUID filterKey;
    FWPM_DISPLAY_DATA0 displayData;
    UINT32 flags;
    /* [unique] */ GUID *providerKey;
    FWP_BYTE_BLOB providerData;
    GUID layerKey;
    GUID subLayerKey;
    FWP_VALUE0 weight;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPM_FILTER_CONDITION0 *filterCondition;
    FWPM_ACTION0 action;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ UINT64 rawContext;
        /* [case()] */ GUID providerContextKey;
        } 	;
    /* [unique] */ GUID *reserved;
    UINT64 filterId;
    FWP_VALUE0 effectiveWeight;
    } 	FWPM_FILTER0;

typedef struct FWPM_FILTER_ENUM_TEMPLATE0_
    {
    /* [unique] */ GUID *providerKey;
    GUID layerKey;
    FWP_FILTER_ENUM_TYPE enumType;
    UINT32 flags;
    /* [unique] */ FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 *providerContextTemplate;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPM_FILTER_CONDITION0 *filterCondition;
    UINT32 actionMask;
    /* [unique] */ GUID *calloutKey;
    } 	FWPM_FILTER_ENUM_TEMPLATE0;

typedef struct FWPM_FILTER_CHANGE0_
    {
    FWPM_CHANGE_TYPE changeType;
    GUID filterKey;
    UINT64 filterId;
    } 	FWPM_FILTER_CHANGE0;

typedef struct FWPM_FILTER_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_FILTER_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_FILTER_SUBSCRIPTION0;

typedef struct FWPM_LAYER_STATISTICS0_
    {
    GUID layerId;
    UINT32 classifyPermitCount;
    UINT32 classifyBlockCount;
    UINT32 classifyVetoCount;
    UINT32 numCacheEntries;
    } 	FWPM_LAYER_STATISTICS0;

typedef struct FWPM_LAYER_STATISTICS1_
    {
    GUID layerId;
    UINT32 classifyPermitCount;
    UINT32 classifyBlockCount;
    UINT32 classifyVetoCount;
    UINT32 numCacheEntries;
    UINT32 filterCount;
    UINT32 totalFilterSize;
    } 	FWPM_LAYER_STATISTICS1;

typedef struct FWPM_STATISTICS0_
    {
    UINT32 numLayerStatistics;
    /* [ref][size_is] */ FWPM_LAYER_STATISTICS0 *layerStatistics;
    UINT32 inboundAllowedConnectionsV4;
    UINT32 inboundBlockedConnectionsV4;
    UINT32 outboundAllowedConnectionsV4;
    UINT32 outboundBlockedConnectionsV4;
    UINT32 inboundAllowedConnectionsV6;
    UINT32 inboundBlockedConnectionsV6;
    UINT32 outboundAllowedConnectionsV6;
    UINT32 outboundBlockedConnectionsV6;
    UINT32 inboundActiveConnectionsV4;
    UINT32 outboundActiveConnectionsV4;
    UINT32 inboundActiveConnectionsV6;
    UINT32 outboundActiveConnectionsV6;
    UINT64 reauthDirInbound;
    UINT64 reauthDirOutbound;
    UINT64 reauthFamilyV4;
    UINT64 reauthFamilyV6;
    UINT64 reauthProtoOther;
    UINT64 reauthProtoIPv4;
    UINT64 reauthProtoIPv6;
    UINT64 reauthProtoICMP;
    UINT64 reauthProtoICMP6;
    UINT64 reauthProtoUDP;
    UINT64 reauthProtoTCP;
    UINT64 reauthReasonPolicyChange;
    UINT64 reauthReasonNewArrivalInterface;
    UINT64 reauthReasonNewNextHopInterface;
    UINT64 reauthReasonProfileCrossing;
    UINT64 reauthReasonClassifyCompletion;
    UINT64 reauthReasonIPSecPropertiesChanged;
    UINT64 reauthReasonMidStreamInspection;
    UINT64 reauthReasonSocketPropertyChanged;
    UINT64 reauthReasonNewInboundMCastBCastPacket;
    UINT64 reauthReasonEDPPolicyChanged;
    UINT64 reauthReasonProxyHandleChanged;
    } 	FWPM_STATISTICS0;

typedef struct FWPM_STATISTICS1_
    {
    UINT32 numLayerStatistics;
    /* [ref][size_is] */ FWPM_LAYER_STATISTICS1 *layerStatistics;
    UINT32 inboundAllowedConnectionsV4;
    UINT32 inboundBlockedConnectionsV4;
    UINT32 outboundAllowedConnectionsV4;
    UINT32 outboundBlockedConnectionsV4;
    UINT32 inboundAllowedConnectionsV6;
    UINT32 inboundBlockedConnectionsV6;
    UINT32 outboundAllowedConnectionsV6;
    UINT32 outboundBlockedConnectionsV6;
    UINT32 inboundActiveConnectionsV4;
    UINT32 outboundActiveConnectionsV4;
    UINT32 inboundActiveConnectionsV6;
    UINT32 outboundActiveConnectionsV6;
    UINT64 reauthDirInbound;
    UINT64 reauthDirOutbound;
    UINT64 reauthFamilyV4;
    UINT64 reauthFamilyV6;
    UINT64 reauthProtoOther;
    UINT64 reauthProtoIPv4;
    UINT64 reauthProtoIPv6;
    UINT64 reauthProtoICMP;
    UINT64 reauthProtoICMP6;
    UINT64 reauthProtoUDP;
    UINT64 reauthProtoTCP;
    UINT64 reauthReasonPolicyChange;
    UINT64 reauthReasonNewArrivalInterface;
    UINT64 reauthReasonNewNextHopInterface;
    UINT64 reauthReasonProfileCrossing;
    UINT64 reauthReasonClassifyCompletion;
    UINT64 reauthReasonIPSecPropertiesChanged;
    UINT64 reauthReasonMidStreamInspection;
    UINT64 reauthReasonSocketPropertyChanged;
    UINT64 reauthReasonNewInboundMCastBCastPacket;
    UINT64 reauthReasonEDPPolicyChanged;
    UINT64 reauthReasonProxyHandleChanged;
    } 	FWPM_STATISTICS1;

#define FWPM_NET_EVENT_FLAG_IP_PROTOCOL_SET (0x00000001)
#define FWPM_NET_EVENT_FLAG_LOCAL_ADDR_SET  (0x00000002)
#define FWPM_NET_EVENT_FLAG_REMOTE_ADDR_SET (0x00000004)
#define FWPM_NET_EVENT_FLAG_LOCAL_PORT_SET  (0x00000008)
#define FWPM_NET_EVENT_FLAG_REMOTE_PORT_SET (0x00000010)
#define FWPM_NET_EVENT_FLAG_APP_ID_SET      (0x00000020)
#define FWPM_NET_EVENT_FLAG_USER_ID_SET     (0x00000040)
#define FWPM_NET_EVENT_FLAG_SCOPE_ID_SET    (0x00000080)
#define FWPM_NET_EVENT_FLAG_IP_VERSION_SET  (0x00000100)
#define FWPM_NET_EVENT_FLAG_REAUTH_REASON_SET (0x00000200)
#define FWPM_NET_EVENT_FLAG_PACKAGE_ID_SET  (0x00000400)
#define FWPM_NET_EVENT_FLAG_ENTERPRISE_ID_SET (0x00000800)
#define FWPM_NET_EVENT_FLAG_POLICY_FLAGS_SET (0x00001000)
#define FWPM_NET_EVENT_FLAG_EFFECTIVE_NAME_SET (0x00002000)
typedef struct FWPM_NET_EVENT_HEADER0_
    {
    FILETIME timeStamp;
    UINT32 flags;
    FWP_IP_VERSION ipVersion;
    UINT8 ipProtocol;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 localAddrV6;
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 remoteAddrV6;
        } 	;
    UINT16 localPort;
    UINT16 remotePort;
    UINT32 scopeId;
    FWP_BYTE_BLOB appId;
    /* [unique] */ SID *userId;
    } 	FWPM_NET_EVENT_HEADER0;

typedef struct FWPM_NET_EVENT_HEADER1_
    {
    FILETIME timeStamp;
    UINT32 flags;
    FWP_IP_VERSION ipVersion;
    UINT8 ipProtocol;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 localAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 remoteAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    UINT16 localPort;
    UINT16 remotePort;
    UINT32 scopeId;
    FWP_BYTE_BLOB appId;
    /* [unique] */ SID *userId;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            FWP_AF reserved1;
            /* [switch_is][switch_type] */ union 
                {
                /* [case()] */ struct 
                    {
                    FWP_BYTE_ARRAY6 reserved2;
                    FWP_BYTE_ARRAY6 reserved3;
                    UINT32 reserved4;
                    UINT32 reserved5;
                    UINT16 reserved6;
                    UINT32 reserved7;
                    UINT32 reserved8;
                    UINT16 reserved9;
                    UINT64 reserved10;
                    } 	;
                } 	;
            } 	;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    } 	FWPM_NET_EVENT_HEADER1;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_NET_EVENT_HEADER2_
    {
    FILETIME timeStamp;
    UINT32 flags;
    FWP_IP_VERSION ipVersion;
    UINT8 ipProtocol;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 localAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 remoteAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    UINT16 localPort;
    UINT16 remotePort;
    UINT32 scopeId;
    FWP_BYTE_BLOB appId;
    /* [unique] */ SID *userId;
    FWP_AF addressFamily;
    /* [unique] */ SID *packageSid;
    } 	FWPM_NET_EVENT_HEADER2;

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef struct FWPM_NET_EVENT_HEADER3_
    {
    FILETIME timeStamp;
    UINT32 flags;
    FWP_IP_VERSION ipVersion;
    UINT8 ipProtocol;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 localAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteAddrV4;
        /* [case()] */ FWP_BYTE_ARRAY16 remoteAddrV6;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    UINT16 localPort;
    UINT16 remotePort;
    UINT32 scopeId;
    FWP_BYTE_BLOB appId;
    /* [unique] */ SID *userId;
    FWP_AF addressFamily;
    /* [unique] */ SID *packageSid;
    /* [unique][string] */ wchar_t *enterpriseId;
    UINT64 policyFlags;
    FWP_BYTE_BLOB effectiveName;
    } 	FWPM_NET_EVENT_HEADER3;

#endif //(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef /* [v1_enum] */ 
enum FWPM_NET_EVENT_TYPE_
    {
        FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE	= 0,
        FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE	= ( FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE + 1 ) ,
        FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE	= ( FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE + 1 ) ,
        FWPM_NET_EVENT_TYPE_CLASSIFY_DROP	= ( FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE + 1 ) ,
        FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP	= ( FWPM_NET_EVENT_TYPE_CLASSIFY_DROP + 1 ) ,
        FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP	= ( FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP + 1 ) ,
        FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW	= ( FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP + 1 ) ,
        FWPM_NET_EVENT_TYPE_CAPABILITY_DROP	= ( FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW + 1 ) ,
        FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW	= ( FWPM_NET_EVENT_TYPE_CAPABILITY_DROP + 1 ) ,
        FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC	= ( FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW + 1 ) ,
        FWPM_NET_EVENT_TYPE_LPM_PACKET_ARRIVAL	= ( FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC + 1 ) ,
        FWPM_NET_EVENT_TYPE_MAX	= ( FWPM_NET_EVENT_TYPE_LPM_PACKET_ARRIVAL + 1 ) 
    } 	FWPM_NET_EVENT_TYPE;

#define IKEEXT_CERT_HASH_LEN 20
#define FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_BENIGN (0x00000001)
#define FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_MULTIPLE (0x00000002)
typedef struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE0_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    UINT32 flags;
    IKEEXT_KEY_MODULE_TYPE keyingModuleType;
    IKEEXT_MM_SA_STATE mmState;
    IKEEXT_SA_ROLE saRole;
    IKEEXT_AUTHENTICATION_METHOD_TYPE mmAuthMethod;
    UINT8 endCertHash[ 20 ];
    UINT64 mmId;
    UINT64 mmFilterId;
    } 	FWPM_NET_EVENT_IKEEXT_MM_FAILURE0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE1_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    UINT32 flags;
    IKEEXT_KEY_MODULE_TYPE keyingModuleType;
    IKEEXT_MM_SA_STATE mmState;
    IKEEXT_SA_ROLE saRole;
    IKEEXT_AUTHENTICATION_METHOD_TYPE mmAuthMethod;
    UINT8 endCertHash[ 20 ];
    UINT64 mmId;
    UINT64 mmFilterId;
    /* [ref][string] */ wchar_t *localPrincipalNameForAuth;
    /* [ref][string] */ wchar_t *remotePrincipalNameForAuth;
    UINT32 numLocalPrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *localPrincipalGroupSids;
    UINT32 numRemotePrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *remotePrincipalGroupSids;
    } 	FWPM_NET_EVENT_IKEEXT_MM_FAILURE1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
typedef struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    UINT32 flags;
    IKEEXT_KEY_MODULE_TYPE keyingModuleType;
    IKEEXT_MM_SA_STATE mmState;
    IKEEXT_SA_ROLE saRole;
    IKEEXT_AUTHENTICATION_METHOD_TYPE mmAuthMethod;
    UINT8 endCertHash[ 20 ];
    UINT64 mmId;
    UINT64 mmFilterId;
    /* [ref][string] */ wchar_t *localPrincipalNameForAuth;
    /* [ref][string] */ wchar_t *remotePrincipalNameForAuth;
    UINT32 numLocalPrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *localPrincipalGroupSids;
    UINT32 numRemotePrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *remotePrincipalGroupSids;
    GUID *providerContextKey;
    } 	FWPM_NET_EVENT_IKEEXT_MM_FAILURE2;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)
typedef struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    IKEEXT_KEY_MODULE_TYPE keyingModuleType;
    IKEEXT_QM_SA_STATE qmState;
    IKEEXT_SA_ROLE saRole;
    IPSEC_TRAFFIC_TYPE saTrafficType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ FWP_CONDITION_VALUE0 localSubNet;
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ FWP_CONDITION_VALUE0 remoteSubNet;
        } 	;
    UINT64 qmFilterId;
    } 	FWPM_NET_EVENT_IKEEXT_QM_FAILURE0;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
typedef struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    IKEEXT_KEY_MODULE_TYPE keyingModuleType;
    IKEEXT_QM_SA_STATE qmState;
    IKEEXT_SA_ROLE saRole;
    IPSEC_TRAFFIC_TYPE saTrafficType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ FWP_CONDITION_VALUE0 localSubNet;
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ FWP_CONDITION_VALUE0 remoteSubNet;
        } 	;
    UINT64 qmFilterId;
    UINT64 mmSaLuid;
    GUID mmProviderContextKey;
    } 	FWPM_NET_EVENT_IKEEXT_QM_FAILURE1;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_MULTIPLE (0x00000001)
#define FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_BENIGN (0x00000002)
typedef struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE0_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    UINT32 flags;
    IKEEXT_EM_SA_STATE emState;
    IKEEXT_SA_ROLE saRole;
    IKEEXT_AUTHENTICATION_METHOD_TYPE emAuthMethod;
    UINT8 endCertHash[ 20 ];
    UINT64 mmId;
    UINT64 qmFilterId;
    } 	FWPM_NET_EVENT_IKEEXT_EM_FAILURE0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE1_
    {
    UINT32 failureErrorCode;
    IPSEC_FAILURE_POINT failurePoint;
    UINT32 flags;
    IKEEXT_EM_SA_STATE emState;
    IKEEXT_SA_ROLE saRole;
    IKEEXT_AUTHENTICATION_METHOD_TYPE emAuthMethod;
    UINT8 endCertHash[ 20 ];
    UINT64 mmId;
    UINT64 qmFilterId;
    /* [ref][string] */ wchar_t *localPrincipalNameForAuth;
    /* [ref][string] */ wchar_t *remotePrincipalNameForAuth;
    UINT32 numLocalPrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *localPrincipalGroupSids;
    UINT32 numRemotePrincipalGroupSids;
    /* [unique][string][size_is] */ LPWSTR *remotePrincipalGroupSids;
    IPSEC_TRAFFIC_TYPE saTrafficType;
    } 	FWPM_NET_EVENT_IKEEXT_EM_FAILURE1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_NET_EVENT_CLASSIFY_DROP0_
    {
    UINT64 filterId;
    UINT16 layerId;
    } 	FWPM_NET_EVENT_CLASSIFY_DROP0;

typedef struct FWPM_NET_EVENT_CLASSIFY_DROP1_
    {
    UINT64 filterId;
    UINT16 layerId;
    UINT32 reauthReason;
    UINT32 originalProfile;
    UINT32 currentProfile;
    UINT32 msFwpDirection;
    BOOL isLoopback;
    } 	FWPM_NET_EVENT_CLASSIFY_DROP1;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_NET_EVENT_CLASSIFY_DROP2_
    {
    UINT64 filterId;
    UINT16 layerId;
    UINT32 reauthReason;
    UINT32 originalProfile;
    UINT32 currentProfile;
    UINT32 msFwpDirection;
    BOOL isLoopback;
    FWP_BYTE_BLOB vSwitchId;
    UINT32 vSwitchSourcePort;
    UINT32 vSwitchDestinationPort;
    } 	FWPM_NET_EVENT_CLASSIFY_DROP2;

typedef struct FWPM_NET_EVENT_CLASSIFY_DROP_MAC0_
    {
    FWP_BYTE_ARRAY6 localMacAddr;
    FWP_BYTE_ARRAY6 remoteMacAddr;
    UINT32 mediaType;
    UINT32 ifType;
    UINT16 etherType;
    UINT32 ndisPortNumber;
    UINT32 reserved;
    UINT16 vlanTag;
    UINT64 ifLuid;
    UINT64 filterId;
    UINT16 layerId;
    UINT32 reauthReason;
    UINT32 originalProfile;
    UINT32 currentProfile;
    UINT32 msFwpDirection;
    BOOL isLoopback;
    FWP_BYTE_BLOB vSwitchId;
    UINT32 vSwitchSourcePort;
    UINT32 vSwitchDestinationPort;
    } 	FWPM_NET_EVENT_CLASSIFY_DROP_MAC0;

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_NET_EVENT_CLASSIFY_ALLOW0
    {
    UINT64 filterId;
    UINT16 layerId;
    UINT32 reauthReason;
    UINT32 originalProfile;
    UINT32 currentProfile;
    UINT32 msFwpDirection;
    BOOL isLoopback;
    } 	FWPM_NET_EVENT_CLASSIFY_ALLOW0;

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_NET_EVENT_IPSEC_KERNEL_DROP0_
    {
    INT32 failureStatus;
    FWP_DIRECTION direction;
    IPSEC_SA_SPI spi;
    UINT64 filterId;
    UINT16 layerId;
    } 	FWPM_NET_EVENT_IPSEC_KERNEL_DROP0;

typedef struct FWPM_NET_EVENT_IPSEC_DOSP_DROP0_
    {
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 publicHostV4Addr;
        /* [case()] */ UINT8 publicHostV6Addr[ 16 ];
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 internalHostV4Addr;
        /* [case()] */ UINT8 internalHostV6Addr[ 16 ];
        } 	;
    INT32 failureStatus;
    FWP_DIRECTION direction;
    } 	FWPM_NET_EVENT_IPSEC_DOSP_DROP0;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef 
enum FWPM_APPC_NETWORK_CAPABILITY_TYPE_
    {
        FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT	= 0,
        FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER	= ( FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT + 1 ) ,
        FWPM_APPC_NETWORK_CAPABILITY_INTERNET_PRIVATE_NETWORK	= ( FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER + 1 ) 
    } 	FWPM_APPC_NETWORK_CAPABILITY_TYPE;

typedef struct FWPM_NET_EVENT_CAPABILITY_DROP0_
    {
    FWPM_APPC_NETWORK_CAPABILITY_TYPE networkCapabilityId;
    UINT64 filterId;
    BOOL isLoopback;
    } 	FWPM_NET_EVENT_CAPABILITY_DROP0;

typedef struct FWPM_NET_EVENT_CAPABILITY_ALLOW0_
    {
    FWPM_APPC_NETWORK_CAPABILITY_TYPE networkCapabilityId;
    UINT64 filterId;
    BOOL isLoopback;
    } 	FWPM_NET_EVENT_CAPABILITY_ALLOW0;

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
typedef struct FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_
    {
    IPSEC_SA_SPI spi;
    } 	FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0;

#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS5)
typedef struct FWPM_NET_EVENT0_
    {
    FWPM_NET_EVENT_HEADER0 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP0 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        } 	;
    } 	FWPM_NET_EVENT0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct FWPM_NET_EVENT1_
    {
    FWPM_NET_EVENT_HEADER1 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP1 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        } 	;
    } 	FWPM_NET_EVENT1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_NET_EVENT2_
    {
    FWPM_NET_EVENT_HEADER2 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP2 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_ALLOW0 *classifyAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_DROP0 *capabilityDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_ALLOW0 *capabilityAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 *classifyDropMac;
        } 	;
    } 	FWPM_NET_EVENT2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef struct FWPM_NET_EVENT3_
    {
    FWPM_NET_EVENT_HEADER3 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP2 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_ALLOW0 *classifyAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_DROP0 *capabilityDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_ALLOW0 *capabilityAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 *classifyDropMac;
        } 	;
    } 	FWPM_NET_EVENT3;

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
typedef struct FWPM_NET_EVENT4_
    {
    FWPM_NET_EVENT_HEADER3 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE1 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP2 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_ALLOW0 *classifyAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_DROP0 *capabilityDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_ALLOW0 *capabilityAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 *classifyDropMac;
        } 	;
    } 	FWPM_NET_EVENT4;

#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS4)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
typedef struct FWPM_NET_EVENT5_
    {
    FWPM_NET_EVENT_HEADER3 header;
    FWPM_NET_EVENT_TYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 *ikeMmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_QM_FAILURE1 *ikeQmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 *ikeEmFailure;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP2 *classifyDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 *ipsecDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_IPSEC_DOSP_DROP0 *idpDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_ALLOW0 *classifyAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_DROP0 *capabilityDrop;
        /* [case()][unique] */ FWPM_NET_EVENT_CAPABILITY_ALLOW0 *capabilityAllow;
        /* [case()][unique] */ FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 *classifyDropMac;
        /* [case()][unique] */ FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 *lpmPacketArrival;
        } 	;
    } 	FWPM_NET_EVENT5;

#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS5)
typedef struct FWPM_NET_EVENT_ENUM_TEMPLATE0_
    {
    FILETIME startTime;
    FILETIME endTime;
    UINT32 numFilterConditions;
    /* [unique][size_is] */ FWPM_FILTER_CONDITION0 *filterCondition;
    } 	FWPM_NET_EVENT_ENUM_TEMPLATE0;

typedef struct FWPM_NET_EVENT_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_NET_EVENT_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_NET_EVENT_SUBSCRIPTION0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef /* [v1_enum] */ 
enum FWPM_SYSTEM_PORT_TYPE_
    {
        FWPM_SYSTEM_PORT_RPC_EPMAP	= 0,
        FWPM_SYSTEM_PORT_TEREDO	= ( FWPM_SYSTEM_PORT_RPC_EPMAP + 1 ) ,
        FWPM_SYSTEM_PORT_IPHTTPS_IN	= ( FWPM_SYSTEM_PORT_TEREDO + 1 ) ,
        FWPM_SYSTEM_PORT_IPHTTPS_OUT	= ( FWPM_SYSTEM_PORT_IPHTTPS_IN + 1 ) ,
        FWPM_SYSTEM_PORT_TYPE_MAX	= ( FWPM_SYSTEM_PORT_IPHTTPS_OUT + 1 ) 
    } 	FWPM_SYSTEM_PORT_TYPE;

typedef struct FWPM_SYSTEM_PORTS_BY_TYPE0_
    {
    FWPM_SYSTEM_PORT_TYPE type;
    UINT32 numPorts;
    /* [unique][size_is] */ UINT16 *ports;
    } 	FWPM_SYSTEM_PORTS_BY_TYPE0;

typedef struct FWPM_SYSTEM_PORTS0_
    {
    UINT32 numTypes;
    /* [unique][size_is] */ FWPM_SYSTEM_PORTS_BY_TYPE0 *types;
    } 	FWPM_SYSTEM_PORTS0;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct FWPM_CONNECTION0_
    {
    UINT64 connectionId;
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
    /* [unique] */ GUID *providerKey;
    IPSEC_TRAFFIC_TYPE ipsecTrafficModeType;
    IKEEXT_KEY_MODULE_TYPE keyModuleType;
    IKEEXT_PROPOSAL0 mmCrypto;
    IKEEXT_CREDENTIAL2 mmPeer;
    IKEEXT_CREDENTIAL2 emPeer;
    UINT64 bytesTransferredIn;
    UINT64 bytesTransferredOut;
    UINT64 bytesTransferredTotal;
    FILETIME startSysTime;
    } 	FWPM_CONNECTION0;

#define FWPM_CONNECTION_ENUM_FLAG_QUERY_BYTES_TRANSFERRED (0x00000001)
typedef struct FWPM_CONNECTION_ENUM_TEMPLATE0_
    {
    UINT64 connectionId;
    UINT32 flags;
    } 	FWPM_CONNECTION_ENUM_TEMPLATE0;

typedef struct FWPM_CONNECTION_SUBSCRIPTION0_
    {
    /* [unique] */ FWPM_CONNECTION_ENUM_TEMPLATE0 *enumTemplate;
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_CONNECTION_SUBSCRIPTION0;

typedef /* [v1_enum] */ 
enum FWPM_CONNECTION_EVENT_TYPE_
    {
        FWPM_CONNECTION_EVENT_ADD	= 0,
        FWPM_CONNECTION_EVENT_DELETE	= ( FWPM_CONNECTION_EVENT_ADD + 1 ) ,
        FWPM_CONNECTION_EVENT_MAX	= ( FWPM_CONNECTION_EVENT_DELETE + 1 ) 
    } 	FWPM_CONNECTION_EVENT_TYPE;

typedef /* [v1_enum] */ 
enum FWPM_VSWITCH_EVENT_TYPE_
    {
        FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER	= 0,
        FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION	= ( FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER + 1 ) ,
        FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION	= ( FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION + 1 ) ,
        FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION	= ( FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION + 1 ) ,
        FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER	= ( FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION + 1 ) ,
        FWPM_VSWITCH_EVENT_MAX	= ( FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER + 1 ) 
    } 	FWPM_VSWITCH_EVENT_TYPE;

typedef struct FWPM_VSWITCH_EVENT0_
    {
    FWPM_VSWITCH_EVENT_TYPE eventType;
    /* [unique][string] */ wchar_t *vSwitchId;
    /* [switch_type][switch_is] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ struct 
            {
            ULONG numvSwitchFilterExtensions;
            /* [ref][string][size_is] */ LPWSTR *vSwitchFilterExtensions;
            } 	positionInfo;
        /* [case()] */ struct 
            {
            BOOL inRequiredPosition;
            ULONG numvSwitchFilterExtensions;
            /* [unique][string][size_is] */ LPWSTR *vSwitchFilterExtensions;
            } 	reorderInfo;
        } 	;
    } 	FWPM_VSWITCH_EVENT0;

typedef struct FWPM_VSWITCH_EVENT_SUBSCRIPTION0_
    {
    UINT32 flags;
    GUID sessionKey;
    } 	FWPM_VSWITCH_EVENT_SUBSCRIPTION0;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if _MSC_VER >=  800
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fwpmtypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fwpmtypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



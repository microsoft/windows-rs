

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


#ifndef __sessdirpublictypes_h__
#define __sessdirpublictypes_h__

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
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_sessdirpublictypes_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define SINGLE_SESSION 0x1
#define FORCE_REJOIN 0x2
#define FORCE_REJOIN_IN_CLUSTERMODE 0x3
#define RESERVED_FOR_LEGACY   0x4
#define KEEP_EXISTING_SESSIONS 0x8


#define SBPLUGIN_CLSID_PROP_STR     _T("CLSID")
#define SBPLUGIN_ISENABLED_PROP_STR _T("IsEnabled")
#define SBPLUGIN_PROVIDER_PROP_STR _T("Provider")


#define SB_NAME_PROP_STR             _T("SbName")
#define SB_ISREDIRECTOR_PROP_STR     _T("IsRedirector")


#define TARGET_GUID_STR                      _T("TargetGuid")
#define TARGET_SINGLESESSION                 _T("SingleSession")
#define TARGET_SERVERCAPABILITY              _T("ServerCapability")
#define TARGET_MAX_ACTIVE_SESSIONS           _T("ServerMaxActiveSessions")
#define TARGET_CLIENT_CONNECTION_MONITORING  _T("TargetClientConnectionMonitoring")
#define TARGET_OWNER_PROP                    _T("TargetOwner")
#define TARGET_REDIRECTION_GUID              _T("TargetRedirectionGuid")
#define TARGET_CERTIFICATE                   _T("TargetCertificate")
#define TARGET_SYMMETRIC_ALG_ID              _T("TargetSymmetricAlgId")
#define TARGET_SYMMETRIC_KEY                 _T("TargetSymmetricKey")
#define TARGET_IS_REMOTE_FX_ENABLED          _T("IsRemoteFxEnabled")
#define TARGET_GRANT_ADMIN_PRIVILEGE         _T("IsUserAdmin")
#define TARGET_LOAD_WAITTIMEOUT              _T("WaitForTargetTimeout")

#define CONN_REQUEST_CALLING_SERVER_NAME_STR       _T("CallingServerName")
#define CONN_REQUEST_IS_CALL_FROM_TS_REDIRECTOR    _T("IsCallFromTSRedirector")
#define CONN_REQUEST_CALLING_SERVER_FARM_NAME_STR  _T("CallingServerFarmName")
#define CONN_REQUEST_TSV_URL_STR                   _T("TsvUrl")
#define CONN_REQUEST_TARGETTYPE_FROM_TSVURL        _T("TargetTypeFromTsvUrl")
#define CONN_REQUEST_TARGETID_FROM_TSVURL          _T("TargetIdFromTsvUrl")
#define CONN_REQUEST_CPUBPLUGINID_FROM_TSVURL      _T("CPubPluginIdFromTsvUrl")
#define CONN_REQUEST_RESOURCEPLUGIN_FROM_TSVURL    _T("ResourcePluginFromTsvUrl")
#define CONN_REQUEST_GUID                          _T("ConnectionRequestGUID")
#define CONN_REQUEST_CLIENT_NAME                   _T("ConnectionRequestClientName")
#define CONN_REQUEST_CLIENT_ADDRESS_FAMILY         _T("ConnectionRequestClientAddressFamily")
#define CONN_REQUEST_CLIENT_ADDRESS                _T("ConnectionRequestClientAddress")
#define CONN_REQUEST_CLIENT_BUILD_NUMBER           _T("ConnectionRequestClientBuildNumber")
#define CONN_REQUEST_PROTOCOL_TYPE                 _T("ConnectionRequestProtocolType")
#define CONN_REQUEST_CLIENT_TIME_ZONE_BIAS         _T("ConnectionRequestClinetTimeZoneBias")
#define CONN_REQUEST_CLIENT_TIME_ZONE_STANDARD_BIAS _T("ConnectionRequestClinetTimeZoneStandardBias")
#define CONN_REQUEST_CLIENT_TIME_ZONE_DAYLIGHT_BIAS _T("ConnectionRequestClinetTimeZoneDaylightBias")
#define CONN_REQUEST_CLIENT_TIME_ZONE_STANDARD_NAME _T("ConnectionRequestClinetTimeZoneStandardName")
#define CONN_REQUEST_CLIENT_TIME_ZONE_DAYLIGHT_NAME _T("ConnectionRequestClinetTimeZoneDaylightName")
typedef /* [v1_enum] */ 
enum _TSSD_AddrV46Type
    {
        TSSD_ADDR_UNDEFINED	= 0,
        TSSD_ADDR_IPv4	= 4,
        TSSD_ADDR_IPv6	= 6
    } 	TSSD_AddrV46Type;

typedef /* [v1_enum] */ 
enum _TSSB_NOTIFICATION_TYPE
    {
        TSSB_NOTIFY_INVALID	= 0,
        TSSB_NOTIFY_TARGET_CHANGE	= 0x1,
        TSSB_NOTIFY_SESSION_CHANGE	= 0x2,
        TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE	= 0x4
    } 	TSSB_NOTIFICATION_TYPE;

// begin_wpp config
// CUSTOM_TYPE(TssbNotificationType, ItemEnum(_TSSB_NOTIFICATION_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TARGET_STATE
    {
        TARGET_UNKNOWN	= 0x1,
        TARGET_INITIALIZING	= ( TARGET_UNKNOWN + 1 ) ,
        TARGET_RUNNING	= ( TARGET_INITIALIZING + 1 ) ,
        TARGET_DOWN	= ( TARGET_RUNNING + 1 ) ,
        TARGET_HIBERNATED	= ( TARGET_DOWN + 1 ) ,
        TARGET_CHECKED_OUT	= ( TARGET_HIBERNATED + 1 ) ,
        TARGET_STOPPED	= ( TARGET_CHECKED_OUT + 1 ) ,
        TARGET_INVALID	= ( TARGET_STOPPED + 1 ) ,
        TARGET_STARTING	= ( TARGET_INVALID + 1 ) ,
        TARGET_STOPPING	= ( TARGET_STARTING + 1 ) ,
        TARGET_MAXSTATE	= ( TARGET_STOPPING + 1 ) 
    } 	TARGET_STATE;

// begin_wpp config
// CUSTOM_TYPE(TargetState, ItemEnum(_TARGET_STATE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TARGET_CHANGE_TYPE
    {
        TARGET_CHANGE_UNSPEC	= 0x1,
        TARGET_EXTERNALIP_CHANGED	= 0x2,
        TARGET_INTERNALIP_CHANGED	= 0x4,
        TARGET_JOINED	= 0x8,
        TARGET_REMOVED	= 0x10,
        TARGET_STATE_CHANGED	= 0x20,
        TARGET_IDLE	= 0x40,
        TARGET_PENDING	= 0x80,
        TARGET_INUSE	= 0x100,
        TARGET_PATCH_STATE_CHANGED	= 0x200,
        TARGET_FARM_MEMBERSHIP_CHANGED	= 0x400
    } 	TARGET_CHANGE_TYPE;

// begin_wpp config
// CUSTOM_TYPE(TargetChangeType, ItemEnum(_TARGET_CHANGE_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TARGET_TYPE
    {
        UNKNOWN	= 0,
        FARM	= 1,
        NONFARM	= 2
    } 	TARGET_TYPE;

// begin_wpp config
// CUSTOM_TYPE(TargetType, ItemEnum(_TARGET_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TARGET_PATCH_STATE
    {
        TARGET_PATCH_UNKNOWN	= 0,
        TARGET_PATCH_NOT_STARTED	= 1,
        TARGET_PATCH_IN_PROGRESS	= 2,
        TARGET_PATCH_COMPLETED	= 3,
        TARGET_PATCH_FAILED	= 4
    } 	TARGET_PATCH_STATE;

// begin_wpp config
// CUSTOM_TYPE(TargetPatchState, ItemEnum(_TARGET_PATCH_STATE));
// end_wpp
typedef /* [v1_enum] */ 
enum _CLIENT_MESSAGE_TYPE
    {
        CLIENT_MESSAGE_CONNECTION_INVALID	= 0,
        CLIENT_MESSAGE_CONNECTION_STATUS	= ( CLIENT_MESSAGE_CONNECTION_INVALID + 1 ) ,
        CLIENT_MESSAGE_CONNECTION_ERROR	= ( CLIENT_MESSAGE_CONNECTION_STATUS + 1 ) 
    } 	CLIENT_MESSAGE_TYPE;

// begin_wpp config
// CUSTOM_TYPE(ClientMessageType, ItemEnum(_CLIENT_MESSAGE_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _CONNECTION_CHANGE_NOTIFICATION
    {
        CONNECTION_REQUEST_INVALID	= 0,
        CONNECTION_REQUEST_PENDING	= ( CONNECTION_REQUEST_INVALID + 1 ) ,
        CONNECTION_REQUEST_FAILED	= ( CONNECTION_REQUEST_PENDING + 1 ) ,
        CONNECTION_REQUEST_TIMEDOUT	= ( CONNECTION_REQUEST_FAILED + 1 ) ,
        CONNECTION_REQUEST_SUCCEEDED	= ( CONNECTION_REQUEST_TIMEDOUT + 1 ) ,
        CONNECTION_REQUEST_CANCELLED	= ( CONNECTION_REQUEST_SUCCEEDED + 1 ) ,
        CONNECTION_REQUEST_LB_COMPLETED	= ( CONNECTION_REQUEST_CANCELLED + 1 ) ,
        CONNECTION_REQUEST_QUERY_PL_COMPLETED	= ( CONNECTION_REQUEST_LB_COMPLETED + 1 ) ,
        CONNECTION_REQUEST_ORCH_COMPLETED	= ( CONNECTION_REQUEST_QUERY_PL_COMPLETED + 1 ) 
    } 	CONNECTION_CHANGE_NOTIFICATION;

// begin_wpp config
// CUSTOM_TYPE(ConnChangeNotification, ItemEnum(_CONNECTION_CHANGE_NOTIFICATION));
// end_wpp
typedef /* [v1_enum] */ 
enum _RD_FARM_TYPE
    {
        RD_FARM_RDSH	= 0,
        RD_FARM_TEMP_VM	= 1,
        RD_FARM_MANUAL_PERSONAL_VM	= 2,
        RD_FARM_AUTO_PERSONAL_VM	= 3,
        RD_FARM_MANUAL_PERSONAL_RDSH	= 4,
        RD_FARM_AUTO_PERSONAL_RDSH	= 5,
        RD_FARM_TYPE_UNKNOWN	= 0xffffffff
    } 	RD_FARM_TYPE;

// begin_wpp config
// CUSTOM_TYPE(RdFarmType, ItemEnum(_RD_FARM_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _PLUGIN_TYPE
    {
        UNKNOWN_PLUGIN	= 0,
        POLICY_PLUGIN	= 0x1,
        RESOURCE_PLUGIN	= 0x2,
        LOAD_BALANCING_PLUGIN	= 0x4,
        PLACEMENT_PLUGIN	= 0x8,
        ORCHESTRATION_PLUGIN	= 0x10,
        PROVISIONING_PLUGIN	= 0x20,
        TASK_PLUGIN	= 0x40
    } 	PLUGIN_TYPE;

// begin_wpp config
// CUSTOM_TYPE(PluginType, ItemEnum(_PLUGIN_TYPE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TSSESSION_STATE
    {
        STATE_INVALID	= -1,
        STATE_ACTIVE	= ( STATE_INVALID + 1 ) ,
        STATE_CONNECTED	= ( STATE_ACTIVE + 1 ) ,
        STATE_CONNECTQUERY	= ( STATE_CONNECTED + 1 ) ,
        STATE_SHADOW	= ( STATE_CONNECTQUERY + 1 ) ,
        STATE_DISCONNECTED	= ( STATE_SHADOW + 1 ) ,
        STATE_IDLE	= ( STATE_DISCONNECTED + 1 ) ,
        STATE_LISTEN	= ( STATE_IDLE + 1 ) ,
        STATE_RESET	= ( STATE_LISTEN + 1 ) ,
        STATE_DOWN	= ( STATE_RESET + 1 ) ,
        STATE_INIT	= ( STATE_DOWN + 1 ) ,
        STATE_MAX	= ( STATE_INIT + 1 ) 
    } 	TSSESSION_STATE;

// begin_wpp config
// CUSTOM_TYPE(TsSessionState, ItemEnum(_TSSESSION_STATE));
// end_wpp
typedef /* [v1_enum] */ 
enum _TARGET_OWNER
    {
        OWNER_UNKNOWN	= 0,
        OWNER_MS_TS_PLUGIN	= 0x1,
        OWNER_MS_VM_PLUGIN	= 0x2
    } 	TARGET_OWNER;

// begin_wpp config
// CUSTOM_TYPE(TargetOwner, ItemEnum(TARGET_OWNER));
// end_wpp
typedef /* [public] */ struct __MIDL___MIDL_itf_sessdirpublictypes_0000_0000_0001
    {
    DWORD HorizontalResolution;
    DWORD VerticalResolution;
    DWORD ColorDepth;
    } 	CLIENT_DISPLAY;

typedef struct __MIDL___MIDL_itf_sessdirpublictypes_0000_0000_0001 *PCLIENT_DISPLAY;

typedef /* [public] */ struct __MIDL___MIDL_itf_sessdirpublictypes_0000_0000_0002
    {
    byte ServerAddressB[ 16 ];
    TSSD_AddrV46Type AddressType;
    USHORT PortNumber;
    ULONG AddressScope;
    } 	TSSD_ConnectionPoint;

typedef struct __MIDL___MIDL_itf_sessdirpublictypes_0000_0000_0002 *PTSSD_ConnectionPoint;

typedef /* [v1_enum] */ 
enum _VM_NOTIFY_STATUS
    {
        VM_NOTIFY_STATUS_PENDING	= 0,
        VM_NOTIFY_STATUS_IN_PROGRESS	= 1,
        VM_NOTIFY_STATUS_COMPLETE	= 2,
        VM_NOTIFY_STATUS_FAILED	= 3,
        VM_NOTIFY_STATUS_CANCELED	= 4
    } 	VM_NOTIFY_STATUS;

typedef struct _VM_NOTIFY_ENTRY
    {
    WCHAR VmName[ 128 ];
    WCHAR VmHost[ 128 ];
    } 	VM_NOTIFY_ENTRY;

typedef struct _VM_PATCH_INFO
    {
    DWORD dwNumEntries;
    /* [size_is] */ LPWSTR *pVmNames;
    } 	VM_PATCH_INFO;

typedef struct _VM_NOTIFY_INFO
    {
    DWORD dwNumEntries;
    /* [size_is] */ VM_NOTIFY_ENTRY **ppVmEntries;
    } 	VM_NOTIFY_INFO;

typedef /* [v1_enum] */ 
enum _VM_HOST_NOTIFY_STATUS
    {
        VM_HOST_STATUS_INIT_PENDING	= 0,
        VM_HOST_STATUS_INIT_IN_PROGRESS	= 1,
        VM_HOST_STATUS_INIT_COMPLETE	= 2,
        VM_HOST_STATUS_INIT_FAILED	= 3
    } 	VM_HOST_NOTIFY_STATUS;

typedef /* [v1_enum] */ 
enum _RDV_TASK_STATUS
    {
        RDV_TASK_STATUS_UNKNOWN	= 0,
        RDV_TASK_STATUS_SEARCHING	= ( RDV_TASK_STATUS_UNKNOWN + 1 ) ,
        RDV_TASK_STATUS_DOWNLOADING	= ( RDV_TASK_STATUS_SEARCHING + 1 ) ,
        RDV_TASK_STATUS_APPLYING	= ( RDV_TASK_STATUS_DOWNLOADING + 1 ) ,
        RDV_TASK_STATUS_REBOOTING	= ( RDV_TASK_STATUS_APPLYING + 1 ) ,
        RDV_TASK_STATUS_REBOOTED	= ( RDV_TASK_STATUS_REBOOTING + 1 ) ,
        RDV_TASK_STATUS_SUCCESS	= ( RDV_TASK_STATUS_REBOOTED + 1 ) ,
        RDV_TASK_STATUS_FAILED	= ( RDV_TASK_STATUS_SUCCESS + 1 ) ,
        RDV_TASK_STATUS_TIMEOUT	= ( RDV_TASK_STATUS_FAILED + 1 ) 
    } 	RDV_TASK_STATUS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_sessdirpublictypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sessdirpublictypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



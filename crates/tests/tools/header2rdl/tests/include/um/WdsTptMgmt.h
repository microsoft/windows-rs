

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __wdstptmgmt_h__
#define __wdstptmgmt_h__

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

#ifndef __IWdsTransportCacheable_FWD_DEFINED__
#define __IWdsTransportCacheable_FWD_DEFINED__
typedef interface IWdsTransportCacheable IWdsTransportCacheable;

#endif 	/* __IWdsTransportCacheable_FWD_DEFINED__ */


#ifndef __IWdsTransportCollection_FWD_DEFINED__
#define __IWdsTransportCollection_FWD_DEFINED__
typedef interface IWdsTransportCollection IWdsTransportCollection;

#endif 	/* __IWdsTransportCollection_FWD_DEFINED__ */


#ifndef __IWdsTransportManager_FWD_DEFINED__
#define __IWdsTransportManager_FWD_DEFINED__
typedef interface IWdsTransportManager IWdsTransportManager;

#endif 	/* __IWdsTransportManager_FWD_DEFINED__ */


#ifndef __IWdsTransportServer_FWD_DEFINED__
#define __IWdsTransportServer_FWD_DEFINED__
typedef interface IWdsTransportServer IWdsTransportServer;

#endif 	/* __IWdsTransportServer_FWD_DEFINED__ */


#ifndef __IWdsTransportServer2_FWD_DEFINED__
#define __IWdsTransportServer2_FWD_DEFINED__
typedef interface IWdsTransportServer2 IWdsTransportServer2;

#endif 	/* __IWdsTransportServer2_FWD_DEFINED__ */


#ifndef __IWdsTransportSetupManager_FWD_DEFINED__
#define __IWdsTransportSetupManager_FWD_DEFINED__
typedef interface IWdsTransportSetupManager IWdsTransportSetupManager;

#endif 	/* __IWdsTransportSetupManager_FWD_DEFINED__ */


#ifndef __IWdsTransportSetupManager2_FWD_DEFINED__
#define __IWdsTransportSetupManager2_FWD_DEFINED__
typedef interface IWdsTransportSetupManager2 IWdsTransportSetupManager2;

#endif 	/* __IWdsTransportSetupManager2_FWD_DEFINED__ */


#ifndef __IWdsTransportConfigurationManager_FWD_DEFINED__
#define __IWdsTransportConfigurationManager_FWD_DEFINED__
typedef interface IWdsTransportConfigurationManager IWdsTransportConfigurationManager;

#endif 	/* __IWdsTransportConfigurationManager_FWD_DEFINED__ */


#ifndef __IWdsTransportConfigurationManager2_FWD_DEFINED__
#define __IWdsTransportConfigurationManager2_FWD_DEFINED__
typedef interface IWdsTransportConfigurationManager2 IWdsTransportConfigurationManager2;

#endif 	/* __IWdsTransportConfigurationManager2_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespaceManager_FWD_DEFINED__
#define __IWdsTransportNamespaceManager_FWD_DEFINED__
typedef interface IWdsTransportNamespaceManager IWdsTransportNamespaceManager;

#endif 	/* __IWdsTransportNamespaceManager_FWD_DEFINED__ */


#ifndef __IWdsTransportTftpManager_FWD_DEFINED__
#define __IWdsTransportTftpManager_FWD_DEFINED__
typedef interface IWdsTransportTftpManager IWdsTransportTftpManager;

#endif 	/* __IWdsTransportTftpManager_FWD_DEFINED__ */


#ifndef __IWdsTransportServicePolicy_FWD_DEFINED__
#define __IWdsTransportServicePolicy_FWD_DEFINED__
typedef interface IWdsTransportServicePolicy IWdsTransportServicePolicy;

#endif 	/* __IWdsTransportServicePolicy_FWD_DEFINED__ */


#ifndef __IWdsTransportServicePolicy2_FWD_DEFINED__
#define __IWdsTransportServicePolicy2_FWD_DEFINED__
typedef interface IWdsTransportServicePolicy2 IWdsTransportServicePolicy2;

#endif 	/* __IWdsTransportServicePolicy2_FWD_DEFINED__ */


#ifndef __IWdsTransportDiagnosticsPolicy_FWD_DEFINED__
#define __IWdsTransportDiagnosticsPolicy_FWD_DEFINED__
typedef interface IWdsTransportDiagnosticsPolicy IWdsTransportDiagnosticsPolicy;

#endif 	/* __IWdsTransportDiagnosticsPolicy_FWD_DEFINED__ */


#ifndef __IWdsTransportMulticastSessionPolicy_FWD_DEFINED__
#define __IWdsTransportMulticastSessionPolicy_FWD_DEFINED__
typedef interface IWdsTransportMulticastSessionPolicy IWdsTransportMulticastSessionPolicy;

#endif 	/* __IWdsTransportMulticastSessionPolicy_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespace_FWD_DEFINED__
#define __IWdsTransportNamespace_FWD_DEFINED__
typedef interface IWdsTransportNamespace IWdsTransportNamespace;

#endif 	/* __IWdsTransportNamespace_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespaceAutoCast_FWD_DEFINED__
#define __IWdsTransportNamespaceAutoCast_FWD_DEFINED__
typedef interface IWdsTransportNamespaceAutoCast IWdsTransportNamespaceAutoCast;

#endif 	/* __IWdsTransportNamespaceAutoCast_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCast_FWD_DEFINED__
#define __IWdsTransportNamespaceScheduledCast_FWD_DEFINED__
typedef interface IWdsTransportNamespaceScheduledCast IWdsTransportNamespaceScheduledCast;

#endif 	/* __IWdsTransportNamespaceScheduledCast_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__
#define __IWdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__
typedef interface IWdsTransportNamespaceScheduledCastManualStart IWdsTransportNamespaceScheduledCastManualStart;

#endif 	/* __IWdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__
#define __IWdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__
typedef interface IWdsTransportNamespaceScheduledCastAutoStart IWdsTransportNamespaceScheduledCastAutoStart;

#endif 	/* __IWdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__ */


#ifndef __IWdsTransportContent_FWD_DEFINED__
#define __IWdsTransportContent_FWD_DEFINED__
typedef interface IWdsTransportContent IWdsTransportContent;

#endif 	/* __IWdsTransportContent_FWD_DEFINED__ */


#ifndef __IWdsTransportSession_FWD_DEFINED__
#define __IWdsTransportSession_FWD_DEFINED__
typedef interface IWdsTransportSession IWdsTransportSession;

#endif 	/* __IWdsTransportSession_FWD_DEFINED__ */


#ifndef __IWdsTransportClient_FWD_DEFINED__
#define __IWdsTransportClient_FWD_DEFINED__
typedef interface IWdsTransportClient IWdsTransportClient;

#endif 	/* __IWdsTransportClient_FWD_DEFINED__ */


#ifndef __IWdsTransportTftpClient_FWD_DEFINED__
#define __IWdsTransportTftpClient_FWD_DEFINED__
typedef interface IWdsTransportTftpClient IWdsTransportTftpClient;

#endif 	/* __IWdsTransportTftpClient_FWD_DEFINED__ */


#ifndef __IWdsTransportContentProvider_FWD_DEFINED__
#define __IWdsTransportContentProvider_FWD_DEFINED__
typedef interface IWdsTransportContentProvider IWdsTransportContentProvider;

#endif 	/* __IWdsTransportContentProvider_FWD_DEFINED__ */


#ifndef __WdsTransportCacheable_FWD_DEFINED__
#define __WdsTransportCacheable_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportCacheable WdsTransportCacheable;
#else
typedef struct WdsTransportCacheable WdsTransportCacheable;
#endif /* __cplusplus */

#endif 	/* __WdsTransportCacheable_FWD_DEFINED__ */


#ifndef __WdsTransportCollection_FWD_DEFINED__
#define __WdsTransportCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportCollection WdsTransportCollection;
#else
typedef struct WdsTransportCollection WdsTransportCollection;
#endif /* __cplusplus */

#endif 	/* __WdsTransportCollection_FWD_DEFINED__ */


#ifndef __WdsTransportManager_FWD_DEFINED__
#define __WdsTransportManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportManager WdsTransportManager;
#else
typedef struct WdsTransportManager WdsTransportManager;
#endif /* __cplusplus */

#endif 	/* __WdsTransportManager_FWD_DEFINED__ */


#ifndef __WdsTransportServer_FWD_DEFINED__
#define __WdsTransportServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportServer WdsTransportServer;
#else
typedef struct WdsTransportServer WdsTransportServer;
#endif /* __cplusplus */

#endif 	/* __WdsTransportServer_FWD_DEFINED__ */


#ifndef __WdsTransportSetupManager_FWD_DEFINED__
#define __WdsTransportSetupManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportSetupManager WdsTransportSetupManager;
#else
typedef struct WdsTransportSetupManager WdsTransportSetupManager;
#endif /* __cplusplus */

#endif 	/* __WdsTransportSetupManager_FWD_DEFINED__ */


#ifndef __WdsTransportConfigurationManager_FWD_DEFINED__
#define __WdsTransportConfigurationManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportConfigurationManager WdsTransportConfigurationManager;
#else
typedef struct WdsTransportConfigurationManager WdsTransportConfigurationManager;
#endif /* __cplusplus */

#endif 	/* __WdsTransportConfigurationManager_FWD_DEFINED__ */


#ifndef __WdsTransportNamespaceManager_FWD_DEFINED__
#define __WdsTransportNamespaceManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespaceManager WdsTransportNamespaceManager;
#else
typedef struct WdsTransportNamespaceManager WdsTransportNamespaceManager;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespaceManager_FWD_DEFINED__ */


#ifndef __WdsTransportServicePolicy_FWD_DEFINED__
#define __WdsTransportServicePolicy_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportServicePolicy WdsTransportServicePolicy;
#else
typedef struct WdsTransportServicePolicy WdsTransportServicePolicy;
#endif /* __cplusplus */

#endif 	/* __WdsTransportServicePolicy_FWD_DEFINED__ */


#ifndef __WdsTransportDiagnosticsPolicy_FWD_DEFINED__
#define __WdsTransportDiagnosticsPolicy_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportDiagnosticsPolicy WdsTransportDiagnosticsPolicy;
#else
typedef struct WdsTransportDiagnosticsPolicy WdsTransportDiagnosticsPolicy;
#endif /* __cplusplus */

#endif 	/* __WdsTransportDiagnosticsPolicy_FWD_DEFINED__ */


#ifndef __WdsTransportMulticastSessionPolicy_FWD_DEFINED__
#define __WdsTransportMulticastSessionPolicy_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportMulticastSessionPolicy WdsTransportMulticastSessionPolicy;
#else
typedef struct WdsTransportMulticastSessionPolicy WdsTransportMulticastSessionPolicy;
#endif /* __cplusplus */

#endif 	/* __WdsTransportMulticastSessionPolicy_FWD_DEFINED__ */


#ifndef __WdsTransportNamespace_FWD_DEFINED__
#define __WdsTransportNamespace_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespace WdsTransportNamespace;
#else
typedef struct WdsTransportNamespace WdsTransportNamespace;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespace_FWD_DEFINED__ */


#ifndef __WdsTransportNamespaceAutoCast_FWD_DEFINED__
#define __WdsTransportNamespaceAutoCast_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespaceAutoCast WdsTransportNamespaceAutoCast;
#else
typedef struct WdsTransportNamespaceAutoCast WdsTransportNamespaceAutoCast;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespaceAutoCast_FWD_DEFINED__ */


#ifndef __WdsTransportNamespaceScheduledCast_FWD_DEFINED__
#define __WdsTransportNamespaceScheduledCast_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespaceScheduledCast WdsTransportNamespaceScheduledCast;
#else
typedef struct WdsTransportNamespaceScheduledCast WdsTransportNamespaceScheduledCast;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespaceScheduledCast_FWD_DEFINED__ */


#ifndef __WdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__
#define __WdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespaceScheduledCastManualStart WdsTransportNamespaceScheduledCastManualStart;
#else
typedef struct WdsTransportNamespaceScheduledCastManualStart WdsTransportNamespaceScheduledCastManualStart;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespaceScheduledCastManualStart_FWD_DEFINED__ */


#ifndef __WdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__
#define __WdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportNamespaceScheduledCastAutoStart WdsTransportNamespaceScheduledCastAutoStart;
#else
typedef struct WdsTransportNamespaceScheduledCastAutoStart WdsTransportNamespaceScheduledCastAutoStart;
#endif /* __cplusplus */

#endif 	/* __WdsTransportNamespaceScheduledCastAutoStart_FWD_DEFINED__ */


#ifndef __WdsTransportContent_FWD_DEFINED__
#define __WdsTransportContent_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportContent WdsTransportContent;
#else
typedef struct WdsTransportContent WdsTransportContent;
#endif /* __cplusplus */

#endif 	/* __WdsTransportContent_FWD_DEFINED__ */


#ifndef __WdsTransportSession_FWD_DEFINED__
#define __WdsTransportSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportSession WdsTransportSession;
#else
typedef struct WdsTransportSession WdsTransportSession;
#endif /* __cplusplus */

#endif 	/* __WdsTransportSession_FWD_DEFINED__ */


#ifndef __WdsTransportClient_FWD_DEFINED__
#define __WdsTransportClient_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportClient WdsTransportClient;
#else
typedef struct WdsTransportClient WdsTransportClient;
#endif /* __cplusplus */

#endif 	/* __WdsTransportClient_FWD_DEFINED__ */


#ifndef __WdsTransportTftpClient_FWD_DEFINED__
#define __WdsTransportTftpClient_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportTftpClient WdsTransportTftpClient;
#else
typedef struct WdsTransportTftpClient WdsTransportTftpClient;
#endif /* __cplusplus */

#endif 	/* __WdsTransportTftpClient_FWD_DEFINED__ */


#ifndef __WdsTransportTftpManager_FWD_DEFINED__
#define __WdsTransportTftpManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportTftpManager WdsTransportTftpManager;
#else
typedef struct WdsTransportTftpManager WdsTransportTftpManager;
#endif /* __cplusplus */

#endif 	/* __WdsTransportTftpManager_FWD_DEFINED__ */


#ifndef __WdsTransportContentProvider_FWD_DEFINED__
#define __WdsTransportContentProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class WdsTransportContentProvider WdsTransportContentProvider;
#else
typedef struct WdsTransportContentProvider WdsTransportContentProvider;
#endif /* __cplusplus */

#endif 	/* __WdsTransportContentProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wdstptmgmt_0000_0000 */
/* [local] */ 

//
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
//
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

























typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0001
    {
        WdsTptFeatureAdminPack	= 0x1,
        WdsTptFeatureTransportServer	= 0x2,
        WdsTptFeatureDeploymentServer	= 0x4
    } 	WDSTRANSPORT_FEATURE_FLAGS;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0001 *PWDSTRANSPORT_FEATURE_FLAGS;

#define WDSTRANSPORT_FEATURE_FLAGS_ALL  ( WdsTptFeatureAdminPack | WdsTptFeatureTransportServer | WdsTptFeatureDeploymentServer )
typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0002
    {
        WdsTptProtocolUnicast	= 0x1,
        WdsTptProtocolMulticast	= 0x2
    } 	WDSTRANSPORT_PROTOCOL_FLAGS;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0002 *PWDSTRANSPORT_PROTOCOL_FLAGS;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0003
    {
        WdsTptNamespaceTypeUnknown	= 0,
        WdsTptNamespaceTypeAutoCast	= 1,
        WdsTptNamespaceTypeScheduledCastManualStart	= 2,
        WdsTptNamespaceTypeScheduledCastAutoStart	= 3
    } 	WDSTRANSPORT_NAMESPACE_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0003 *PWDSTRANSPORT_NAMESPACE_TYPE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0004
    {
        WdsTptDisconnectUnknown	= 0,
        WdsTptDisconnectFallback	= 1,
        WdsTptDisconnectAbort	= 2
    } 	WDSTRANSPORT_DISCONNECT_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0004 *PWDSTRANSPORT_DISCONNECT_TYPE;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0005
    {
        WdsTptServiceNotifyUnknown	= 0,
        WdsTptServiceNotifyReadSettings	= 1
    } 	WDSTRANSPORT_SERVICE_NOTIFICATION;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0005 *PWDSTRANSPORT_SERVICE_NOTIFICATION;

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0006
    {
        WdsTptIpAddressUnknown	= 0,
        WdsTptIpAddressIpv4	= 1,
        WdsTptIpAddressIpv6	= 2
    } 	WDSTRANSPORT_IP_ADDRESS_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0006 *PWDSTRANSPORT_IP_ADDRESS_TYPE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0007
    {
        WdsTptIpAddressSourceUnknown	= 0,
        WdsTptIpAddressSourceDhcp	= 1,
        WdsTptIpAddressSourceRange	= 2
    } 	WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0007 *PWDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0008
    {
        WdsTptNetworkProfileUnknown	= 0,
        WdsTptNetworkProfileCustom	= 1,
        WdsTptNetworkProfile10Mbps	= 2,
        WdsTptNetworkProfile100Mbps	= 3,
        WdsTptNetworkProfile1Gbps	= 4
    } 	WDSTRANSPORT_NETWORK_PROFILE_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0008 *PWDSTRANSPORT_NETWORK_PROFILE_TYPE;

typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0009
    {
        WdsTptDiagnosticsComponentPxe	= 0x1,
        WdsTptDiagnosticsComponentTftp	= 0x2,
        WdsTptDiagnosticsComponentImageServer	= 0x4,
        WdsTptDiagnosticsComponentMulticast	= 0x8
    } 	WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0009 *PWDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS;

#define WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS_ALL  ( WdsTptDiagnosticsComponentPxe | WdsTptDiagnosticsComponentTftp | WdsTptDiagnosticsComponentImageServer | WdsTptDiagnosticsComponentMulticast )
#define WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN    0xFF
typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0010
    {
        WdsTptSlowClientHandlingUnknown	= 0,
        WdsTptSlowClientHandlingNone	= 1,
        WdsTptSlowClientHandlingAutoDisconnect	= 2,
        WdsTptSlowClientHandlingMultistream	= 3
    } 	WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0010 *PWDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0011
    {
        WdsTptUdpPortPolicyDynamic	= 0,
        WdsTptUdpPortPolicyFixed	= 1
    } 	WDSTRANSPORT_UDP_PORT_POLICY;

typedef /* [v1_enum] */ enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0011 *PWDSTRANSPORT_UDP_PORT_POLICY;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0012
    {
        WdsTptTftpCapMaximumBlockSize	= 0x1,
        WdsTptTftpCapVariableWindow	= 0x2
    } 	WDSTRANSPORT_TFTP_CAPABILITY;

typedef enum __MIDL___MIDL_itf_wdstptmgmt_0000_0000_0012 *PWDSTRANSPORT_TFTP_CAPABILITY;



extern RPC_IF_HANDLE __MIDL_itf_wdstptmgmt_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wdstptmgmt_0000_0000_v0_0_s_ifspec;

#ifndef __IWdsTransportCacheable_INTERFACE_DEFINED__
#define __IWdsTransportCacheable_INTERFACE_DEFINED__

/* interface IWdsTransportCacheable */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportCacheable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46AD894B-0BAB-47DC-84B2-7B553F1D8F80")
    IWdsTransportCacheable : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dirty( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Discard( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportCacheableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportCacheable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportCacheable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportCacheable * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportCacheable * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportCacheable * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportCacheable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportCacheable * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, get_Dirty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IWdsTransportCacheable * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Discard)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Discard )( 
            __RPC__in IWdsTransportCacheable * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportCacheable * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWdsTransportCacheable * This);
        
        END_INTERFACE
    } IWdsTransportCacheableVtbl;

    interface IWdsTransportCacheable
    {
        CONST_VTBL struct IWdsTransportCacheableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportCacheable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportCacheable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportCacheable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportCacheable_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportCacheable_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportCacheable_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportCacheable_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportCacheable_get_Dirty(This,pbDirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,pbDirty) ) 

#define IWdsTransportCacheable_Discard(This)	\
    ( (This)->lpVtbl -> Discard(This) ) 

#define IWdsTransportCacheable_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportCacheable_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportCacheable_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportCollection_INTERFACE_DEFINED__
#define __IWdsTransportCollection_INTERFACE_DEFINED__

/* interface IWdsTransportCollection */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B8BA4B1A-2FF4-43AB-996C-B2B10A91A6EB")
    IWdsTransportCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *pulCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IWdsTransportCollection * This,
            /* [retval][out] */ __RPC__out ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IWdsTransportCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IWdsTransportCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppVal);
        
        DECLSPEC_XFGVIRT(IWdsTransportCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IWdsTransportCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal);
        
        END_INTERFACE
    } IWdsTransportCollectionVtbl;

    interface IWdsTransportCollection
    {
        CONST_VTBL struct IWdsTransportCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportCollection_get_Count(This,pulCount)	\
    ( (This)->lpVtbl -> get_Count(This,pulCount) ) 

#define IWdsTransportCollection_get_Item(This,ulIndex,ppVal)	\
    ( (This)->lpVtbl -> get_Item(This,ulIndex,ppVal) ) 

#define IWdsTransportCollection_get__NewEnum(This,ppVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportCollection_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportManager_INTERFACE_DEFINED__
#define __IWdsTransportManager_INTERFACE_DEFINED__

/* interface IWdsTransportManager */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B0D35F5-1B13-4AFD-B878-6526DC340B5D")
    IWdsTransportManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWdsTransportServer( 
            /* [in] */ __RPC__in BSTR bszServerName,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportServer **ppWdsTransportServer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportManager, GetWdsTransportServer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWdsTransportServer )( 
            __RPC__in IWdsTransportManager * This,
            /* [in] */ __RPC__in BSTR bszServerName,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportServer **ppWdsTransportServer);
        
        END_INTERFACE
    } IWdsTransportManagerVtbl;

    interface IWdsTransportManager
    {
        CONST_VTBL struct IWdsTransportManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportManager_GetWdsTransportServer(This,bszServerName,ppWdsTransportServer)	\
    ( (This)->lpVtbl -> GetWdsTransportServer(This,bszServerName,ppWdsTransportServer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportManager_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportServer_INTERFACE_DEFINED__
#define __IWdsTransportServer_INTERFACE_DEFINED__

/* interface IWdsTransportServer */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09CCD093-830D-4344-A30A-73AE8E8FCA90")
    IWdsTransportServer : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SetupManager( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportSetupManager **ppWdsTransportSetupManager) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConfigurationManager( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportConfigurationManager **ppWdsTransportConfigurationManager) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NamespaceManager( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespaceManager **ppWdsTransportNamespaceManager) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisconnectClient( 
            /* [in] */ ULONG ulClientId,
            WDSTRANSPORT_DISCONNECT_TYPE DisconnectionType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportServer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportServer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportServer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportServer * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportServer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_SetupManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SetupManager )( 
            __RPC__in IWdsTransportServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportSetupManager **ppWdsTransportSetupManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_ConfigurationManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigurationManager )( 
            __RPC__in IWdsTransportServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportConfigurationManager **ppWdsTransportConfigurationManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_NamespaceManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceManager )( 
            __RPC__in IWdsTransportServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespaceManager **ppWdsTransportNamespaceManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, DisconnectClient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisconnectClient )( 
            __RPC__in IWdsTransportServer * This,
            /* [in] */ ULONG ulClientId,
            WDSTRANSPORT_DISCONNECT_TYPE DisconnectionType);
        
        END_INTERFACE
    } IWdsTransportServerVtbl;

    interface IWdsTransportServer
    {
        CONST_VTBL struct IWdsTransportServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportServer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportServer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportServer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportServer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportServer_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportServer_get_SetupManager(This,ppWdsTransportSetupManager)	\
    ( (This)->lpVtbl -> get_SetupManager(This,ppWdsTransportSetupManager) ) 

#define IWdsTransportServer_get_ConfigurationManager(This,ppWdsTransportConfigurationManager)	\
    ( (This)->lpVtbl -> get_ConfigurationManager(This,ppWdsTransportConfigurationManager) ) 

#define IWdsTransportServer_get_NamespaceManager(This,ppWdsTransportNamespaceManager)	\
    ( (This)->lpVtbl -> get_NamespaceManager(This,ppWdsTransportNamespaceManager) ) 

#define IWdsTransportServer_DisconnectClient(This,ulClientId,DisconnectionType)	\
    ( (This)->lpVtbl -> DisconnectClient(This,ulClientId,DisconnectionType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportServer_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportServer2_INTERFACE_DEFINED__
#define __IWdsTransportServer2_INTERFACE_DEFINED__

/* interface IWdsTransportServer2 */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportServer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("256e999f-6df4-4538-81b9-857b9ab8fb47")
    IWdsTransportServer2 : public IWdsTransportServer
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TftpManager( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportTftpManager **ppWdsTransportTftpManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportServer2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportServer2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportServer2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportServer2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_SetupManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SetupManager )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportSetupManager **ppWdsTransportSetupManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_ConfigurationManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfigurationManager )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportConfigurationManager **ppWdsTransportConfigurationManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, get_NamespaceManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceManager )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespaceManager **ppWdsTransportNamespaceManager);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer, DisconnectClient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisconnectClient )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [in] */ ULONG ulClientId,
            WDSTRANSPORT_DISCONNECT_TYPE DisconnectionType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServer2, get_TftpManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TftpManager )( 
            __RPC__in IWdsTransportServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportTftpManager **ppWdsTransportTftpManager);
        
        END_INTERFACE
    } IWdsTransportServer2Vtbl;

    interface IWdsTransportServer2
    {
        CONST_VTBL struct IWdsTransportServer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportServer2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportServer2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportServer2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportServer2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportServer2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportServer2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportServer2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportServer2_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportServer2_get_SetupManager(This,ppWdsTransportSetupManager)	\
    ( (This)->lpVtbl -> get_SetupManager(This,ppWdsTransportSetupManager) ) 

#define IWdsTransportServer2_get_ConfigurationManager(This,ppWdsTransportConfigurationManager)	\
    ( (This)->lpVtbl -> get_ConfigurationManager(This,ppWdsTransportConfigurationManager) ) 

#define IWdsTransportServer2_get_NamespaceManager(This,ppWdsTransportNamespaceManager)	\
    ( (This)->lpVtbl -> get_NamespaceManager(This,ppWdsTransportNamespaceManager) ) 

#define IWdsTransportServer2_DisconnectClient(This,ulClientId,DisconnectionType)	\
    ( (This)->lpVtbl -> DisconnectClient(This,ulClientId,DisconnectionType) ) 


#define IWdsTransportServer2_get_TftpManager(This,ppWdsTransportTftpManager)	\
    ( (This)->lpVtbl -> get_TftpManager(This,ppWdsTransportTftpManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportServer2_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportSetupManager_INTERFACE_DEFINED__
#define __IWdsTransportSetupManager_INTERFACE_DEFINED__

/* interface IWdsTransportSetupManager */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportSetupManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F7238425-EFA8-40A4-AEF9-C98D969C0B75")
    IWdsTransportSetupManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__out ULONGLONG *pullVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InstalledFeatures( 
            /* [retval][out] */ __RPC__out ULONG *pulInstalledFeatures) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Protocols( 
            /* [retval][out] */ __RPC__out ULONG *pulProtocols) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterContentProvider( 
            /* [in] */ __RPC__in BSTR bszName,
            /* [in] */ __RPC__in BSTR bszDescription,
            /* [in] */ __RPC__in BSTR bszFilePath,
            /* [in] */ __RPC__in BSTR bszInitializationRoutine) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeregisterContentProvider( 
            /* [in] */ __RPC__in BSTR bszName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportSetupManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportSetupManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportSetupManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportSetupManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_Version)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [retval][out] */ __RPC__out ULONGLONG *pullVersion);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_InstalledFeatures)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InstalledFeatures )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [retval][out] */ __RPC__out ULONG *pulInstalledFeatures);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_Protocols)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Protocols )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [retval][out] */ __RPC__out ULONG *pulProtocols);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, RegisterContentProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterContentProvider )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [in] */ __RPC__in BSTR bszName,
            /* [in] */ __RPC__in BSTR bszDescription,
            /* [in] */ __RPC__in BSTR bszFilePath,
            /* [in] */ __RPC__in BSTR bszInitializationRoutine);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, DeregisterContentProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeregisterContentProvider )( 
            __RPC__in IWdsTransportSetupManager * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        END_INTERFACE
    } IWdsTransportSetupManagerVtbl;

    interface IWdsTransportSetupManager
    {
        CONST_VTBL struct IWdsTransportSetupManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportSetupManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportSetupManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportSetupManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportSetupManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportSetupManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportSetupManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportSetupManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportSetupManager_get_Version(This,pullVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pullVersion) ) 

#define IWdsTransportSetupManager_get_InstalledFeatures(This,pulInstalledFeatures)	\
    ( (This)->lpVtbl -> get_InstalledFeatures(This,pulInstalledFeatures) ) 

#define IWdsTransportSetupManager_get_Protocols(This,pulProtocols)	\
    ( (This)->lpVtbl -> get_Protocols(This,pulProtocols) ) 

#define IWdsTransportSetupManager_RegisterContentProvider(This,bszName,bszDescription,bszFilePath,bszInitializationRoutine)	\
    ( (This)->lpVtbl -> RegisterContentProvider(This,bszName,bszDescription,bszFilePath,bszInitializationRoutine) ) 

#define IWdsTransportSetupManager_DeregisterContentProvider(This,bszName)	\
    ( (This)->lpVtbl -> DeregisterContentProvider(This,bszName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportSetupManager_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportSetupManager2_INTERFACE_DEFINED__
#define __IWdsTransportSetupManager2_INTERFACE_DEFINED__

/* interface IWdsTransportSetupManager2 */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportSetupManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02BE79DA-7E9E-4366-8B6E-2AA9A91BE47F")
    IWdsTransportSetupManager2 : public IWdsTransportSetupManager
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TftpCapabilities( 
            /* [retval][out] */ __RPC__out ULONG *pulTftpCapabilities) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ContentProviders( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppProviderCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportSetupManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportSetupManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportSetupManager2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportSetupManager2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_Version)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [retval][out] */ __RPC__out ULONGLONG *pullVersion);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_InstalledFeatures)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InstalledFeatures )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulInstalledFeatures);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, get_Protocols)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Protocols )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulProtocols);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, RegisterContentProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterContentProvider )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [in] */ __RPC__in BSTR bszName,
            /* [in] */ __RPC__in BSTR bszDescription,
            /* [in] */ __RPC__in BSTR bszFilePath,
            /* [in] */ __RPC__in BSTR bszInitializationRoutine);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager, DeregisterContentProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeregisterContentProvider )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager2, get_TftpCapabilities)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TftpCapabilities )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulTftpCapabilities);
        
        DECLSPEC_XFGVIRT(IWdsTransportSetupManager2, get_ContentProviders)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProviders )( 
            __RPC__in IWdsTransportSetupManager2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppProviderCollection);
        
        END_INTERFACE
    } IWdsTransportSetupManager2Vtbl;

    interface IWdsTransportSetupManager2
    {
        CONST_VTBL struct IWdsTransportSetupManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportSetupManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportSetupManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportSetupManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportSetupManager2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportSetupManager2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportSetupManager2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportSetupManager2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportSetupManager2_get_Version(This,pullVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pullVersion) ) 

#define IWdsTransportSetupManager2_get_InstalledFeatures(This,pulInstalledFeatures)	\
    ( (This)->lpVtbl -> get_InstalledFeatures(This,pulInstalledFeatures) ) 

#define IWdsTransportSetupManager2_get_Protocols(This,pulProtocols)	\
    ( (This)->lpVtbl -> get_Protocols(This,pulProtocols) ) 

#define IWdsTransportSetupManager2_RegisterContentProvider(This,bszName,bszDescription,bszFilePath,bszInitializationRoutine)	\
    ( (This)->lpVtbl -> RegisterContentProvider(This,bszName,bszDescription,bszFilePath,bszInitializationRoutine) ) 

#define IWdsTransportSetupManager2_DeregisterContentProvider(This,bszName)	\
    ( (This)->lpVtbl -> DeregisterContentProvider(This,bszName) ) 


#define IWdsTransportSetupManager2_get_TftpCapabilities(This,pulTftpCapabilities)	\
    ( (This)->lpVtbl -> get_TftpCapabilities(This,pulTftpCapabilities) ) 

#define IWdsTransportSetupManager2_get_ContentProviders(This,ppProviderCollection)	\
    ( (This)->lpVtbl -> get_ContentProviders(This,ppProviderCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportSetupManager2_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportConfigurationManager_INTERFACE_DEFINED__
#define __IWdsTransportConfigurationManager_INTERFACE_DEFINED__

/* interface IWdsTransportConfigurationManager */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportConfigurationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("84CC4779-42DD-4792-891E-1321D6D74B44")
    IWdsTransportConfigurationManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServicePolicy( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportServicePolicy **ppWdsTransportServicePolicy) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiagnosticsPolicy( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportDiagnosticsPolicy **ppWdsTransportDiagnosticsPolicy) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WdsTransportServicesRunning( 
            /* [in] */ VARIANT_BOOL bRealtimeStatus,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbServicesRunning) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableWdsTransportServices( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisableWdsTransportServices( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartWdsTransportServices( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopWdsTransportServices( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RestartWdsTransportServices( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyWdsTransportServices( 
            /* [in] */ WDSTRANSPORT_SERVICE_NOTIFICATION ServiceNotification) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportConfigurationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportConfigurationManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_ServicePolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServicePolicy )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportServicePolicy **ppWdsTransportServicePolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_DiagnosticsPolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiagnosticsPolicy )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportDiagnosticsPolicy **ppWdsTransportDiagnosticsPolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_WdsTransportServicesRunning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WdsTransportServicesRunning )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [in] */ VARIANT_BOOL bRealtimeStatus,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbServicesRunning);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, EnableWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, DisableWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisableWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, StartWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, StopWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, RestartWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestartWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, NotifyWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager * This,
            /* [in] */ WDSTRANSPORT_SERVICE_NOTIFICATION ServiceNotification);
        
        END_INTERFACE
    } IWdsTransportConfigurationManagerVtbl;

    interface IWdsTransportConfigurationManager
    {
        CONST_VTBL struct IWdsTransportConfigurationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportConfigurationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportConfigurationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportConfigurationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportConfigurationManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportConfigurationManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportConfigurationManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportConfigurationManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportConfigurationManager_get_ServicePolicy(This,ppWdsTransportServicePolicy)	\
    ( (This)->lpVtbl -> get_ServicePolicy(This,ppWdsTransportServicePolicy) ) 

#define IWdsTransportConfigurationManager_get_DiagnosticsPolicy(This,ppWdsTransportDiagnosticsPolicy)	\
    ( (This)->lpVtbl -> get_DiagnosticsPolicy(This,ppWdsTransportDiagnosticsPolicy) ) 

#define IWdsTransportConfigurationManager_get_WdsTransportServicesRunning(This,bRealtimeStatus,pbServicesRunning)	\
    ( (This)->lpVtbl -> get_WdsTransportServicesRunning(This,bRealtimeStatus,pbServicesRunning) ) 

#define IWdsTransportConfigurationManager_EnableWdsTransportServices(This)	\
    ( (This)->lpVtbl -> EnableWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager_DisableWdsTransportServices(This)	\
    ( (This)->lpVtbl -> DisableWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager_StartWdsTransportServices(This)	\
    ( (This)->lpVtbl -> StartWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager_StopWdsTransportServices(This)	\
    ( (This)->lpVtbl -> StopWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager_RestartWdsTransportServices(This)	\
    ( (This)->lpVtbl -> RestartWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager_NotifyWdsTransportServices(This,ServiceNotification)	\
    ( (This)->lpVtbl -> NotifyWdsTransportServices(This,ServiceNotification) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportConfigurationManager_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportConfigurationManager2_INTERFACE_DEFINED__
#define __IWdsTransportConfigurationManager2_INTERFACE_DEFINED__

/* interface IWdsTransportConfigurationManager2 */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportConfigurationManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D0D85CAF-A153-4F1D-A9DD-96F431C50717")
    IWdsTransportConfigurationManager2 : public IWdsTransportConfigurationManager
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MulticastSessionPolicy( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportMulticastSessionPolicy **ppWdsTransportMulticastSessionPolicy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportConfigurationManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportConfigurationManager2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_ServicePolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServicePolicy )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportServicePolicy **ppWdsTransportServicePolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_DiagnosticsPolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiagnosticsPolicy )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportDiagnosticsPolicy **ppWdsTransportDiagnosticsPolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, get_WdsTransportServicesRunning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WdsTransportServicesRunning )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [in] */ VARIANT_BOOL bRealtimeStatus,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbServicesRunning);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, EnableWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, DisableWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisableWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, StartWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, StopWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, RestartWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestartWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager, NotifyWdsTransportServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyWdsTransportServices )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [in] */ WDSTRANSPORT_SERVICE_NOTIFICATION ServiceNotification);
        
        DECLSPEC_XFGVIRT(IWdsTransportConfigurationManager2, get_MulticastSessionPolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MulticastSessionPolicy )( 
            __RPC__in IWdsTransportConfigurationManager2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportMulticastSessionPolicy **ppWdsTransportMulticastSessionPolicy);
        
        END_INTERFACE
    } IWdsTransportConfigurationManager2Vtbl;

    interface IWdsTransportConfigurationManager2
    {
        CONST_VTBL struct IWdsTransportConfigurationManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportConfigurationManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportConfigurationManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportConfigurationManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportConfigurationManager2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportConfigurationManager2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportConfigurationManager2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportConfigurationManager2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportConfigurationManager2_get_ServicePolicy(This,ppWdsTransportServicePolicy)	\
    ( (This)->lpVtbl -> get_ServicePolicy(This,ppWdsTransportServicePolicy) ) 

#define IWdsTransportConfigurationManager2_get_DiagnosticsPolicy(This,ppWdsTransportDiagnosticsPolicy)	\
    ( (This)->lpVtbl -> get_DiagnosticsPolicy(This,ppWdsTransportDiagnosticsPolicy) ) 

#define IWdsTransportConfigurationManager2_get_WdsTransportServicesRunning(This,bRealtimeStatus,pbServicesRunning)	\
    ( (This)->lpVtbl -> get_WdsTransportServicesRunning(This,bRealtimeStatus,pbServicesRunning) ) 

#define IWdsTransportConfigurationManager2_EnableWdsTransportServices(This)	\
    ( (This)->lpVtbl -> EnableWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager2_DisableWdsTransportServices(This)	\
    ( (This)->lpVtbl -> DisableWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager2_StartWdsTransportServices(This)	\
    ( (This)->lpVtbl -> StartWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager2_StopWdsTransportServices(This)	\
    ( (This)->lpVtbl -> StopWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager2_RestartWdsTransportServices(This)	\
    ( (This)->lpVtbl -> RestartWdsTransportServices(This) ) 

#define IWdsTransportConfigurationManager2_NotifyWdsTransportServices(This,ServiceNotification)	\
    ( (This)->lpVtbl -> NotifyWdsTransportServices(This,ServiceNotification) ) 


#define IWdsTransportConfigurationManager2_get_MulticastSessionPolicy(This,ppWdsTransportMulticastSessionPolicy)	\
    ( (This)->lpVtbl -> get_MulticastSessionPolicy(This,ppWdsTransportMulticastSessionPolicy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportConfigurationManager2_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespaceManager_INTERFACE_DEFINED__
#define __IWdsTransportNamespaceManager_INTERFACE_DEFINED__

/* interface IWdsTransportNamespaceManager */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespaceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3E22D9F6-3777-4D98-83E1-F98696717BA3")
    IWdsTransportNamespaceManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateNamespace( 
            /* [in] */ WDSTRANSPORT_NAMESPACE_TYPE NamespaceType,
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [in] */ __RPC__in BSTR bszContentProvider,
            /* [in] */ __RPC__in BSTR bszConfiguration,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveNamespace( 
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveNamespaces( 
            /* [in] */ __RPC__in BSTR bszContentProvider,
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [in] */ VARIANT_BOOL bIncludeTombstones,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportNamespaces) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespaceManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceManager, CreateNamespace)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateNamespace )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ WDSTRANSPORT_NAMESPACE_TYPE NamespaceType,
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [in] */ __RPC__in BSTR bszContentProvider,
            /* [in] */ __RPC__in BSTR bszConfiguration,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceManager, RetrieveNamespace)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveNamespace )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceManager, RetrieveNamespaces)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveNamespaces )( 
            __RPC__in IWdsTransportNamespaceManager * This,
            /* [in] */ __RPC__in BSTR bszContentProvider,
            /* [in] */ __RPC__in BSTR bszNamespaceName,
            /* [in] */ VARIANT_BOOL bIncludeTombstones,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportNamespaces);
        
        END_INTERFACE
    } IWdsTransportNamespaceManagerVtbl;

    interface IWdsTransportNamespaceManager
    {
        CONST_VTBL struct IWdsTransportNamespaceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespaceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespaceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespaceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespaceManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespaceManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespaceManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespaceManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespaceManager_CreateNamespace(This,NamespaceType,bszNamespaceName,bszContentProvider,bszConfiguration,ppWdsTransportNamespace)	\
    ( (This)->lpVtbl -> CreateNamespace(This,NamespaceType,bszNamespaceName,bszContentProvider,bszConfiguration,ppWdsTransportNamespace) ) 

#define IWdsTransportNamespaceManager_RetrieveNamespace(This,bszNamespaceName,ppWdsTransportNamespace)	\
    ( (This)->lpVtbl -> RetrieveNamespace(This,bszNamespaceName,ppWdsTransportNamespace) ) 

#define IWdsTransportNamespaceManager_RetrieveNamespaces(This,bszContentProvider,bszNamespaceName,bIncludeTombstones,ppWdsTransportNamespaces)	\
    ( (This)->lpVtbl -> RetrieveNamespaces(This,bszContentProvider,bszNamespaceName,bIncludeTombstones,ppWdsTransportNamespaces) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespaceManager_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportTftpManager_INTERFACE_DEFINED__
#define __IWdsTransportTftpManager_INTERFACE_DEFINED__

/* interface IWdsTransportTftpManager */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportTftpManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1327a7c8-ae8a-4fb3-8150-136227c37e9a")
    IWdsTransportTftpManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveTftpClients( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportTftpClients) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportTftpManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportTftpManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportTftpManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportTftpManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportTftpManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportTftpManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportTftpManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportTftpManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpManager, RetrieveTftpClients)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveTftpClients )( 
            __RPC__in IWdsTransportTftpManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportTftpClients);
        
        END_INTERFACE
    } IWdsTransportTftpManagerVtbl;

    interface IWdsTransportTftpManager
    {
        CONST_VTBL struct IWdsTransportTftpManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportTftpManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportTftpManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportTftpManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportTftpManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportTftpManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportTftpManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportTftpManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportTftpManager_RetrieveTftpClients(This,ppWdsTransportTftpClients)	\
    ( (This)->lpVtbl -> RetrieveTftpClients(This,ppWdsTransportTftpClients) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportTftpManager_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportServicePolicy_INTERFACE_DEFINED__
#define __IWdsTransportServicePolicy_INTERFACE_DEFINED__

/* interface IWdsTransportServicePolicy */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportServicePolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B9468578-9F2B-48CC-B27A-A60799C2750C")
    IWdsTransportServicePolicy : public IWdsTransportCacheable
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IpAddressSource( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE *pSourceType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IpAddressSource( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE SourceType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartIpAddress( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszStartIpAddress) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartIpAddress( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszStartIpAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EndIpAddress( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszEndIpAddress) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EndIpAddress( 
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszEndIpAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartPort( 
            /* [retval][out] */ __RPC__out ULONG *pulStartPort) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartPort( 
            /* [in] */ ULONG ulStartPort) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EndPort( 
            /* [retval][out] */ __RPC__out ULONG *pulEndPort) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EndPort( 
            /* [in] */ ULONG ulEndPort) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkProfile( 
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NETWORK_PROFILE_TYPE *pProfileType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NetworkProfile( 
            /* [in] */ WDSTRANSPORT_NETWORK_PROFILE_TYPE ProfileType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportServicePolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportServicePolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportServicePolicy * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportServicePolicy * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, get_Dirty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Discard)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Discard )( 
            __RPC__in IWdsTransportServicePolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportServicePolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWdsTransportServicePolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_IpAddressSource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IpAddressSource )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE *pSourceType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_IpAddressSource)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IpAddressSource )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE SourceType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_StartIpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartIpAddress )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszStartIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_StartIpAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartIpAddress )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszStartIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_EndIpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndIpAddress )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszEndIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_EndIpAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndIpAddress )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszEndIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_StartPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartPort )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [retval][out] */ __RPC__out ULONG *pulStartPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_StartPort)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartPort )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ ULONG ulStartPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_EndPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndPort )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [retval][out] */ __RPC__out ULONG *pulEndPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_EndPort)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndPort )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ ULONG ulEndPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_NetworkProfile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkProfile )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NETWORK_PROFILE_TYPE *pProfileType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_NetworkProfile)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkProfile )( 
            __RPC__in IWdsTransportServicePolicy * This,
            /* [in] */ WDSTRANSPORT_NETWORK_PROFILE_TYPE ProfileType);
        
        END_INTERFACE
    } IWdsTransportServicePolicyVtbl;

    interface IWdsTransportServicePolicy
    {
        CONST_VTBL struct IWdsTransportServicePolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportServicePolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportServicePolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportServicePolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportServicePolicy_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportServicePolicy_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportServicePolicy_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportServicePolicy_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportServicePolicy_get_Dirty(This,pbDirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,pbDirty) ) 

#define IWdsTransportServicePolicy_Discard(This)	\
    ( (This)->lpVtbl -> Discard(This) ) 

#define IWdsTransportServicePolicy_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportServicePolicy_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IWdsTransportServicePolicy_get_IpAddressSource(This,AddressType,pSourceType)	\
    ( (This)->lpVtbl -> get_IpAddressSource(This,AddressType,pSourceType) ) 

#define IWdsTransportServicePolicy_put_IpAddressSource(This,AddressType,SourceType)	\
    ( (This)->lpVtbl -> put_IpAddressSource(This,AddressType,SourceType) ) 

#define IWdsTransportServicePolicy_get_StartIpAddress(This,AddressType,pbszStartIpAddress)	\
    ( (This)->lpVtbl -> get_StartIpAddress(This,AddressType,pbszStartIpAddress) ) 

#define IWdsTransportServicePolicy_put_StartIpAddress(This,AddressType,bszStartIpAddress)	\
    ( (This)->lpVtbl -> put_StartIpAddress(This,AddressType,bszStartIpAddress) ) 

#define IWdsTransportServicePolicy_get_EndIpAddress(This,AddressType,pbszEndIpAddress)	\
    ( (This)->lpVtbl -> get_EndIpAddress(This,AddressType,pbszEndIpAddress) ) 

#define IWdsTransportServicePolicy_put_EndIpAddress(This,AddressType,bszEndIpAddress)	\
    ( (This)->lpVtbl -> put_EndIpAddress(This,AddressType,bszEndIpAddress) ) 

#define IWdsTransportServicePolicy_get_StartPort(This,pulStartPort)	\
    ( (This)->lpVtbl -> get_StartPort(This,pulStartPort) ) 

#define IWdsTransportServicePolicy_put_StartPort(This,ulStartPort)	\
    ( (This)->lpVtbl -> put_StartPort(This,ulStartPort) ) 

#define IWdsTransportServicePolicy_get_EndPort(This,pulEndPort)	\
    ( (This)->lpVtbl -> get_EndPort(This,pulEndPort) ) 

#define IWdsTransportServicePolicy_put_EndPort(This,ulEndPort)	\
    ( (This)->lpVtbl -> put_EndPort(This,ulEndPort) ) 

#define IWdsTransportServicePolicy_get_NetworkProfile(This,pProfileType)	\
    ( (This)->lpVtbl -> get_NetworkProfile(This,pProfileType) ) 

#define IWdsTransportServicePolicy_put_NetworkProfile(This,ProfileType)	\
    ( (This)->lpVtbl -> put_NetworkProfile(This,ProfileType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportServicePolicy_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportServicePolicy2_INTERFACE_DEFINED__
#define __IWdsTransportServicePolicy2_INTERFACE_DEFINED__

/* interface IWdsTransportServicePolicy2 */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportServicePolicy2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("65C19E5C-AA7E-4B91-8944-91E0E5572797")
    IWdsTransportServicePolicy2 : public IWdsTransportServicePolicy
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UdpPortPolicy( 
            /* [retval][out] */ __RPC__out WDSTRANSPORT_UDP_PORT_POLICY *pUdpPortPolicy) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UdpPortPolicy( 
            /* [in] */ WDSTRANSPORT_UDP_PORT_POLICY UdpPortPolicy) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TftpMaximumBlockSize( 
            /* [retval][out] */ __RPC__out ULONG *pulTftpMaximumBlockSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TftpMaximumBlockSize( 
            /* [in] */ ULONG ulTftpMaximumBlockSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnableTftpVariableWindowExtension( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnableTftpVariableWindowExtension) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnableTftpVariableWindowExtension( 
            /* [in] */ VARIANT_BOOL bEnableTftpVariableWindowExtension) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportServicePolicy2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportServicePolicy2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportServicePolicy2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportServicePolicy2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, get_Dirty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Discard)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Discard )( 
            __RPC__in IWdsTransportServicePolicy2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportServicePolicy2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWdsTransportServicePolicy2 * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_IpAddressSource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IpAddressSource )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE *pSourceType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_IpAddressSource)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IpAddressSource )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE SourceType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_StartIpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartIpAddress )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszStartIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_StartIpAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartIpAddress )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszStartIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_EndIpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndIpAddress )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszEndIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_EndIpAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndIpAddress )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_IP_ADDRESS_TYPE AddressType,
            /* [in] */ __RPC__in BSTR bszEndIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_StartPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartPort )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulStartPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_StartPort)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartPort )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ ULONG ulStartPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_EndPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndPort )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulEndPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_EndPort)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndPort )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ ULONG ulEndPort);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, get_NetworkProfile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkProfile )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NETWORK_PROFILE_TYPE *pProfileType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy, put_NetworkProfile)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkProfile )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_NETWORK_PROFILE_TYPE ProfileType);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, get_UdpPortPolicy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UdpPortPolicy )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_UDP_PORT_POLICY *pUdpPortPolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, put_UdpPortPolicy)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UdpPortPolicy )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ WDSTRANSPORT_UDP_PORT_POLICY UdpPortPolicy);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, get_TftpMaximumBlockSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TftpMaximumBlockSize )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulTftpMaximumBlockSize);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, put_TftpMaximumBlockSize)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TftpMaximumBlockSize )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ ULONG ulTftpMaximumBlockSize);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, get_EnableTftpVariableWindowExtension)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableTftpVariableWindowExtension )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnableTftpVariableWindowExtension);
        
        DECLSPEC_XFGVIRT(IWdsTransportServicePolicy2, put_EnableTftpVariableWindowExtension)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableTftpVariableWindowExtension )( 
            __RPC__in IWdsTransportServicePolicy2 * This,
            /* [in] */ VARIANT_BOOL bEnableTftpVariableWindowExtension);
        
        END_INTERFACE
    } IWdsTransportServicePolicy2Vtbl;

    interface IWdsTransportServicePolicy2
    {
        CONST_VTBL struct IWdsTransportServicePolicy2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportServicePolicy2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportServicePolicy2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportServicePolicy2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportServicePolicy2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportServicePolicy2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportServicePolicy2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportServicePolicy2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportServicePolicy2_get_Dirty(This,pbDirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,pbDirty) ) 

#define IWdsTransportServicePolicy2_Discard(This)	\
    ( (This)->lpVtbl -> Discard(This) ) 

#define IWdsTransportServicePolicy2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportServicePolicy2_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IWdsTransportServicePolicy2_get_IpAddressSource(This,AddressType,pSourceType)	\
    ( (This)->lpVtbl -> get_IpAddressSource(This,AddressType,pSourceType) ) 

#define IWdsTransportServicePolicy2_put_IpAddressSource(This,AddressType,SourceType)	\
    ( (This)->lpVtbl -> put_IpAddressSource(This,AddressType,SourceType) ) 

#define IWdsTransportServicePolicy2_get_StartIpAddress(This,AddressType,pbszStartIpAddress)	\
    ( (This)->lpVtbl -> get_StartIpAddress(This,AddressType,pbszStartIpAddress) ) 

#define IWdsTransportServicePolicy2_put_StartIpAddress(This,AddressType,bszStartIpAddress)	\
    ( (This)->lpVtbl -> put_StartIpAddress(This,AddressType,bszStartIpAddress) ) 

#define IWdsTransportServicePolicy2_get_EndIpAddress(This,AddressType,pbszEndIpAddress)	\
    ( (This)->lpVtbl -> get_EndIpAddress(This,AddressType,pbszEndIpAddress) ) 

#define IWdsTransportServicePolicy2_put_EndIpAddress(This,AddressType,bszEndIpAddress)	\
    ( (This)->lpVtbl -> put_EndIpAddress(This,AddressType,bszEndIpAddress) ) 

#define IWdsTransportServicePolicy2_get_StartPort(This,pulStartPort)	\
    ( (This)->lpVtbl -> get_StartPort(This,pulStartPort) ) 

#define IWdsTransportServicePolicy2_put_StartPort(This,ulStartPort)	\
    ( (This)->lpVtbl -> put_StartPort(This,ulStartPort) ) 

#define IWdsTransportServicePolicy2_get_EndPort(This,pulEndPort)	\
    ( (This)->lpVtbl -> get_EndPort(This,pulEndPort) ) 

#define IWdsTransportServicePolicy2_put_EndPort(This,ulEndPort)	\
    ( (This)->lpVtbl -> put_EndPort(This,ulEndPort) ) 

#define IWdsTransportServicePolicy2_get_NetworkProfile(This,pProfileType)	\
    ( (This)->lpVtbl -> get_NetworkProfile(This,pProfileType) ) 

#define IWdsTransportServicePolicy2_put_NetworkProfile(This,ProfileType)	\
    ( (This)->lpVtbl -> put_NetworkProfile(This,ProfileType) ) 


#define IWdsTransportServicePolicy2_get_UdpPortPolicy(This,pUdpPortPolicy)	\
    ( (This)->lpVtbl -> get_UdpPortPolicy(This,pUdpPortPolicy) ) 

#define IWdsTransportServicePolicy2_put_UdpPortPolicy(This,UdpPortPolicy)	\
    ( (This)->lpVtbl -> put_UdpPortPolicy(This,UdpPortPolicy) ) 

#define IWdsTransportServicePolicy2_get_TftpMaximumBlockSize(This,pulTftpMaximumBlockSize)	\
    ( (This)->lpVtbl -> get_TftpMaximumBlockSize(This,pulTftpMaximumBlockSize) ) 

#define IWdsTransportServicePolicy2_put_TftpMaximumBlockSize(This,ulTftpMaximumBlockSize)	\
    ( (This)->lpVtbl -> put_TftpMaximumBlockSize(This,ulTftpMaximumBlockSize) ) 

#define IWdsTransportServicePolicy2_get_EnableTftpVariableWindowExtension(This,pbEnableTftpVariableWindowExtension)	\
    ( (This)->lpVtbl -> get_EnableTftpVariableWindowExtension(This,pbEnableTftpVariableWindowExtension) ) 

#define IWdsTransportServicePolicy2_put_EnableTftpVariableWindowExtension(This,bEnableTftpVariableWindowExtension)	\
    ( (This)->lpVtbl -> put_EnableTftpVariableWindowExtension(This,bEnableTftpVariableWindowExtension) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportServicePolicy2_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportDiagnosticsPolicy_INTERFACE_DEFINED__
#define __IWdsTransportDiagnosticsPolicy_INTERFACE_DEFINED__

/* interface IWdsTransportDiagnosticsPolicy */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportDiagnosticsPolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13B33EFC-7856-4F61-9A59-8DE67B6B87B6")
    IWdsTransportDiagnosticsPolicy : public IWdsTransportCacheable
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL bEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Components( 
            /* [retval][out] */ __RPC__out ULONG *pulComponents) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Components( 
            /* [in] */ ULONG ulComponents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportDiagnosticsPolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportDiagnosticsPolicy * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, get_Dirty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Discard)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Discard )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportDiagnosticsPolicy, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled);
        
        DECLSPEC_XFGVIRT(IWdsTransportDiagnosticsPolicy, put_Enabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [in] */ VARIANT_BOOL bEnabled);
        
        DECLSPEC_XFGVIRT(IWdsTransportDiagnosticsPolicy, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [retval][out] */ __RPC__out ULONG *pulComponents);
        
        DECLSPEC_XFGVIRT(IWdsTransportDiagnosticsPolicy, put_Components)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Components )( 
            __RPC__in IWdsTransportDiagnosticsPolicy * This,
            /* [in] */ ULONG ulComponents);
        
        END_INTERFACE
    } IWdsTransportDiagnosticsPolicyVtbl;

    interface IWdsTransportDiagnosticsPolicy
    {
        CONST_VTBL struct IWdsTransportDiagnosticsPolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportDiagnosticsPolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportDiagnosticsPolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportDiagnosticsPolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportDiagnosticsPolicy_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportDiagnosticsPolicy_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportDiagnosticsPolicy_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportDiagnosticsPolicy_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportDiagnosticsPolicy_get_Dirty(This,pbDirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,pbDirty) ) 

#define IWdsTransportDiagnosticsPolicy_Discard(This)	\
    ( (This)->lpVtbl -> Discard(This) ) 

#define IWdsTransportDiagnosticsPolicy_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportDiagnosticsPolicy_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IWdsTransportDiagnosticsPolicy_get_Enabled(This,pbEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pbEnabled) ) 

#define IWdsTransportDiagnosticsPolicy_put_Enabled(This,bEnabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,bEnabled) ) 

#define IWdsTransportDiagnosticsPolicy_get_Components(This,pulComponents)	\
    ( (This)->lpVtbl -> get_Components(This,pulComponents) ) 

#define IWdsTransportDiagnosticsPolicy_put_Components(This,ulComponents)	\
    ( (This)->lpVtbl -> put_Components(This,ulComponents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportDiagnosticsPolicy_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportMulticastSessionPolicy_INTERFACE_DEFINED__
#define __IWdsTransportMulticastSessionPolicy_INTERFACE_DEFINED__

/* interface IWdsTransportMulticastSessionPolicy */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportMulticastSessionPolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4E5753CF-68EC-4504-A951-4A003266606B")
    IWdsTransportMulticastSessionPolicy : public IWdsTransportCacheable
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SlowClientHandling( 
            /* [retval][out] */ __RPC__out WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE *pSlowClientHandling) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SlowClientHandling( 
            /* [in] */ WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE SlowClientHandling) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoDisconnectThreshold( 
            /* [retval][out] */ __RPC__out ULONG *pulThreshold) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoDisconnectThreshold( 
            /* [in] */ ULONG ulThreshold) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MultistreamStreamCount( 
            /* [retval][out] */ __RPC__out ULONG *pulStreamCount) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MultistreamStreamCount( 
            /* [in] */ ULONG ulStreamCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SlowClientFallback( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbClientFallback) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SlowClientFallback( 
            /* [in] */ VARIANT_BOOL bClientFallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportMulticastSessionPolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportMulticastSessionPolicy * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, get_Dirty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDirty);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Discard)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Discard )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportCacheable, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, get_SlowClientHandling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SlowClientHandling )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE *pSlowClientHandling);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, put_SlowClientHandling)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SlowClientHandling )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE SlowClientHandling);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, get_AutoDisconnectThreshold)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoDisconnectThreshold )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [retval][out] */ __RPC__out ULONG *pulThreshold);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, put_AutoDisconnectThreshold)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoDisconnectThreshold )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ ULONG ulThreshold);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, get_MultistreamStreamCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultistreamStreamCount )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [retval][out] */ __RPC__out ULONG *pulStreamCount);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, put_MultistreamStreamCount)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultistreamStreamCount )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ ULONG ulStreamCount);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, get_SlowClientFallback)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SlowClientFallback )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbClientFallback);
        
        DECLSPEC_XFGVIRT(IWdsTransportMulticastSessionPolicy, put_SlowClientFallback)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SlowClientFallback )( 
            __RPC__in IWdsTransportMulticastSessionPolicy * This,
            /* [in] */ VARIANT_BOOL bClientFallback);
        
        END_INTERFACE
    } IWdsTransportMulticastSessionPolicyVtbl;

    interface IWdsTransportMulticastSessionPolicy
    {
        CONST_VTBL struct IWdsTransportMulticastSessionPolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportMulticastSessionPolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportMulticastSessionPolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportMulticastSessionPolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportMulticastSessionPolicy_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportMulticastSessionPolicy_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportMulticastSessionPolicy_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportMulticastSessionPolicy_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportMulticastSessionPolicy_get_Dirty(This,pbDirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,pbDirty) ) 

#define IWdsTransportMulticastSessionPolicy_Discard(This)	\
    ( (This)->lpVtbl -> Discard(This) ) 

#define IWdsTransportMulticastSessionPolicy_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportMulticastSessionPolicy_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IWdsTransportMulticastSessionPolicy_get_SlowClientHandling(This,pSlowClientHandling)	\
    ( (This)->lpVtbl -> get_SlowClientHandling(This,pSlowClientHandling) ) 

#define IWdsTransportMulticastSessionPolicy_put_SlowClientHandling(This,SlowClientHandling)	\
    ( (This)->lpVtbl -> put_SlowClientHandling(This,SlowClientHandling) ) 

#define IWdsTransportMulticastSessionPolicy_get_AutoDisconnectThreshold(This,pulThreshold)	\
    ( (This)->lpVtbl -> get_AutoDisconnectThreshold(This,pulThreshold) ) 

#define IWdsTransportMulticastSessionPolicy_put_AutoDisconnectThreshold(This,ulThreshold)	\
    ( (This)->lpVtbl -> put_AutoDisconnectThreshold(This,ulThreshold) ) 

#define IWdsTransportMulticastSessionPolicy_get_MultistreamStreamCount(This,pulStreamCount)	\
    ( (This)->lpVtbl -> get_MultistreamStreamCount(This,pulStreamCount) ) 

#define IWdsTransportMulticastSessionPolicy_put_MultistreamStreamCount(This,ulStreamCount)	\
    ( (This)->lpVtbl -> put_MultistreamStreamCount(This,ulStreamCount) ) 

#define IWdsTransportMulticastSessionPolicy_get_SlowClientFallback(This,pbClientFallback)	\
    ( (This)->lpVtbl -> get_SlowClientFallback(This,pbClientFallback) ) 

#define IWdsTransportMulticastSessionPolicy_put_SlowClientFallback(This,bClientFallback)	\
    ( (This)->lpVtbl -> put_SlowClientFallback(This,bClientFallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportMulticastSessionPolicy_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespace_INTERFACE_DEFINED__
#define __IWdsTransportNamespace_INTERFACE_DEFINED__

/* interface IWdsTransportNamespace */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA561F57-FBEF-4ED3-B056-127CB1B33B84")
    IWdsTransportNamespace : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out ULONG *pulId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bszName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FriendlyName( 
            /* [in] */ __RPC__in BSTR bszFriendlyName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bszDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ContentProvider( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ContentProvider( 
            /* [in] */ __RPC__in BSTR bszContentProvider) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Configuration( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Configuration( 
            /* [in] */ __RPC__in BSTR bszConfiguration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Registered( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tombstoned( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TombstoneTime( 
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStarted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Register( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Deregister( 
            /* [in] */ VARIANT_BOOL bTerminateSessions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveContents( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespace * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in BSTR bszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in BSTR bszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_ContentProvider)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProvider )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_ContentProvider)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentProvider )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in BSTR bszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Configuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ __RPC__in BSTR bszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Registered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Registered )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Tombstoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tombstoned )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TombstoneTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TombstoneTime )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TransmissionStarted)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStarted )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Register)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IWdsTransportNamespace * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Deregister)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deregister )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [in] */ VARIANT_BOOL bTerminateSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportNamespace * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, RetrieveContents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveContents )( 
            __RPC__in IWdsTransportNamespace * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents);
        
        END_INTERFACE
    } IWdsTransportNamespaceVtbl;

    interface IWdsTransportNamespace
    {
        CONST_VTBL struct IWdsTransportNamespaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespace_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWdsTransportNamespace_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportNamespace_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportNamespace_put_Name(This,bszName)	\
    ( (This)->lpVtbl -> put_Name(This,bszName) ) 

#define IWdsTransportNamespace_get_FriendlyName(This,pbszFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbszFriendlyName) ) 

#define IWdsTransportNamespace_put_FriendlyName(This,bszFriendlyName)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,bszFriendlyName) ) 

#define IWdsTransportNamespace_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportNamespace_put_Description(This,bszDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bszDescription) ) 

#define IWdsTransportNamespace_get_ContentProvider(This,pbszContentProvider)	\
    ( (This)->lpVtbl -> get_ContentProvider(This,pbszContentProvider) ) 

#define IWdsTransportNamespace_put_ContentProvider(This,bszContentProvider)	\
    ( (This)->lpVtbl -> put_ContentProvider(This,bszContentProvider) ) 

#define IWdsTransportNamespace_get_Configuration(This,pbszConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pbszConfiguration) ) 

#define IWdsTransportNamespace_put_Configuration(This,bszConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,bszConfiguration) ) 

#define IWdsTransportNamespace_get_Registered(This,pbRegistered)	\
    ( (This)->lpVtbl -> get_Registered(This,pbRegistered) ) 

#define IWdsTransportNamespace_get_Tombstoned(This,pbTombstoned)	\
    ( (This)->lpVtbl -> get_Tombstoned(This,pbTombstoned) ) 

#define IWdsTransportNamespace_get_TombstoneTime(This,pTombstoneTime)	\
    ( (This)->lpVtbl -> get_TombstoneTime(This,pTombstoneTime) ) 

#define IWdsTransportNamespace_get_TransmissionStarted(This,pbTransmissionStarted)	\
    ( (This)->lpVtbl -> get_TransmissionStarted(This,pbTransmissionStarted) ) 

#define IWdsTransportNamespace_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IWdsTransportNamespace_Deregister(This,bTerminateSessions)	\
    ( (This)->lpVtbl -> Deregister(This,bTerminateSessions) ) 

#define IWdsTransportNamespace_Clone(This,ppWdsTransportNamespaceClone)	\
    ( (This)->lpVtbl -> Clone(This,ppWdsTransportNamespaceClone) ) 

#define IWdsTransportNamespace_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportNamespace_RetrieveContents(This,ppWdsTransportContents)	\
    ( (This)->lpVtbl -> RetrieveContents(This,ppWdsTransportContents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespace_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespaceAutoCast_INTERFACE_DEFINED__
#define __IWdsTransportNamespaceAutoCast_INTERFACE_DEFINED__

/* interface IWdsTransportNamespaceAutoCast */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespaceAutoCast;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD931A72-C4BD-4C41-8FBC-59C9C748DF9E")
    IWdsTransportNamespaceAutoCast : public IWdsTransportNamespace
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceAutoCastVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespaceAutoCast * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in BSTR bszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in BSTR bszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_ContentProvider)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_ContentProvider)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in BSTR bszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Configuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ __RPC__in BSTR bszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Registered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Registered )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Tombstoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tombstoned )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TombstoneTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TombstoneTime )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TransmissionStarted)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStarted )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Register)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Deregister)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deregister )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [in] */ VARIANT_BOOL bTerminateSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, RetrieveContents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveContents )( 
            __RPC__in IWdsTransportNamespaceAutoCast * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents);
        
        END_INTERFACE
    } IWdsTransportNamespaceAutoCastVtbl;

    interface IWdsTransportNamespaceAutoCast
    {
        CONST_VTBL struct IWdsTransportNamespaceAutoCastVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespaceAutoCast_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespaceAutoCast_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespaceAutoCast_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespaceAutoCast_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespaceAutoCast_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespaceAutoCast_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespaceAutoCast_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespaceAutoCast_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWdsTransportNamespaceAutoCast_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportNamespaceAutoCast_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportNamespaceAutoCast_put_Name(This,bszName)	\
    ( (This)->lpVtbl -> put_Name(This,bszName) ) 

#define IWdsTransportNamespaceAutoCast_get_FriendlyName(This,pbszFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbszFriendlyName) ) 

#define IWdsTransportNamespaceAutoCast_put_FriendlyName(This,bszFriendlyName)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,bszFriendlyName) ) 

#define IWdsTransportNamespaceAutoCast_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportNamespaceAutoCast_put_Description(This,bszDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bszDescription) ) 

#define IWdsTransportNamespaceAutoCast_get_ContentProvider(This,pbszContentProvider)	\
    ( (This)->lpVtbl -> get_ContentProvider(This,pbszContentProvider) ) 

#define IWdsTransportNamespaceAutoCast_put_ContentProvider(This,bszContentProvider)	\
    ( (This)->lpVtbl -> put_ContentProvider(This,bszContentProvider) ) 

#define IWdsTransportNamespaceAutoCast_get_Configuration(This,pbszConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pbszConfiguration) ) 

#define IWdsTransportNamespaceAutoCast_put_Configuration(This,bszConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,bszConfiguration) ) 

#define IWdsTransportNamespaceAutoCast_get_Registered(This,pbRegistered)	\
    ( (This)->lpVtbl -> get_Registered(This,pbRegistered) ) 

#define IWdsTransportNamespaceAutoCast_get_Tombstoned(This,pbTombstoned)	\
    ( (This)->lpVtbl -> get_Tombstoned(This,pbTombstoned) ) 

#define IWdsTransportNamespaceAutoCast_get_TombstoneTime(This,pTombstoneTime)	\
    ( (This)->lpVtbl -> get_TombstoneTime(This,pTombstoneTime) ) 

#define IWdsTransportNamespaceAutoCast_get_TransmissionStarted(This,pbTransmissionStarted)	\
    ( (This)->lpVtbl -> get_TransmissionStarted(This,pbTransmissionStarted) ) 

#define IWdsTransportNamespaceAutoCast_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IWdsTransportNamespaceAutoCast_Deregister(This,bTerminateSessions)	\
    ( (This)->lpVtbl -> Deregister(This,bTerminateSessions) ) 

#define IWdsTransportNamespaceAutoCast_Clone(This,ppWdsTransportNamespaceClone)	\
    ( (This)->lpVtbl -> Clone(This,ppWdsTransportNamespaceClone) ) 

#define IWdsTransportNamespaceAutoCast_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportNamespaceAutoCast_RetrieveContents(This,ppWdsTransportContents)	\
    ( (This)->lpVtbl -> RetrieveContents(This,ppWdsTransportContents) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespaceAutoCast_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCast_INTERFACE_DEFINED__
#define __IWdsTransportNamespaceScheduledCast_INTERFACE_DEFINED__

/* interface IWdsTransportNamespaceScheduledCast */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespaceScheduledCast;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3840CECF-D76C-416E-A4CC-31C741D2874B")
    IWdsTransportNamespaceScheduledCast : public IWdsTransportNamespace
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartTransmission( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceScheduledCastVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespaceScheduledCast * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in BSTR bszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in BSTR bszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_ContentProvider)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_ContentProvider)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in BSTR bszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Configuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ __RPC__in BSTR bszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Registered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Registered )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Tombstoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tombstoned )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TombstoneTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TombstoneTime )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TransmissionStarted)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStarted )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Register)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Deregister)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deregister )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [in] */ VARIANT_BOOL bTerminateSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, RetrieveContents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveContents )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCast, StartTransmission)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartTransmission )( 
            __RPC__in IWdsTransportNamespaceScheduledCast * This);
        
        END_INTERFACE
    } IWdsTransportNamespaceScheduledCastVtbl;

    interface IWdsTransportNamespaceScheduledCast
    {
        CONST_VTBL struct IWdsTransportNamespaceScheduledCastVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespaceScheduledCast_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespaceScheduledCast_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespaceScheduledCast_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespaceScheduledCast_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespaceScheduledCast_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespaceScheduledCast_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespaceScheduledCast_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespaceScheduledCast_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWdsTransportNamespaceScheduledCast_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportNamespaceScheduledCast_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportNamespaceScheduledCast_put_Name(This,bszName)	\
    ( (This)->lpVtbl -> put_Name(This,bszName) ) 

#define IWdsTransportNamespaceScheduledCast_get_FriendlyName(This,pbszFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCast_put_FriendlyName(This,bszFriendlyName)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,bszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCast_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportNamespaceScheduledCast_put_Description(This,bszDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bszDescription) ) 

#define IWdsTransportNamespaceScheduledCast_get_ContentProvider(This,pbszContentProvider)	\
    ( (This)->lpVtbl -> get_ContentProvider(This,pbszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCast_put_ContentProvider(This,bszContentProvider)	\
    ( (This)->lpVtbl -> put_ContentProvider(This,bszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCast_get_Configuration(This,pbszConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pbszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCast_put_Configuration(This,bszConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,bszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCast_get_Registered(This,pbRegistered)	\
    ( (This)->lpVtbl -> get_Registered(This,pbRegistered) ) 

#define IWdsTransportNamespaceScheduledCast_get_Tombstoned(This,pbTombstoned)	\
    ( (This)->lpVtbl -> get_Tombstoned(This,pbTombstoned) ) 

#define IWdsTransportNamespaceScheduledCast_get_TombstoneTime(This,pTombstoneTime)	\
    ( (This)->lpVtbl -> get_TombstoneTime(This,pTombstoneTime) ) 

#define IWdsTransportNamespaceScheduledCast_get_TransmissionStarted(This,pbTransmissionStarted)	\
    ( (This)->lpVtbl -> get_TransmissionStarted(This,pbTransmissionStarted) ) 

#define IWdsTransportNamespaceScheduledCast_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IWdsTransportNamespaceScheduledCast_Deregister(This,bTerminateSessions)	\
    ( (This)->lpVtbl -> Deregister(This,bTerminateSessions) ) 

#define IWdsTransportNamespaceScheduledCast_Clone(This,ppWdsTransportNamespaceClone)	\
    ( (This)->lpVtbl -> Clone(This,ppWdsTransportNamespaceClone) ) 

#define IWdsTransportNamespaceScheduledCast_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportNamespaceScheduledCast_RetrieveContents(This,ppWdsTransportContents)	\
    ( (This)->lpVtbl -> RetrieveContents(This,ppWdsTransportContents) ) 


#define IWdsTransportNamespaceScheduledCast_StartTransmission(This)	\
    ( (This)->lpVtbl -> StartTransmission(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespaceScheduledCast_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCastManualStart_INTERFACE_DEFINED__
#define __IWdsTransportNamespaceScheduledCastManualStart_INTERFACE_DEFINED__

/* interface IWdsTransportNamespaceScheduledCastManualStart */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespaceScheduledCastManualStart;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("013E6E4C-E6A7-4FB5-B7FF-D9F5DA805C31")
    IWdsTransportNamespaceScheduledCastManualStart : public IWdsTransportNamespaceScheduledCast
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceScheduledCastManualStartVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in BSTR bszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in BSTR bszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_ContentProvider)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_ContentProvider)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in BSTR bszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Configuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ __RPC__in BSTR bszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Registered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Registered )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Tombstoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tombstoned )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TombstoneTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TombstoneTime )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TransmissionStarted)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStarted )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Register)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Deregister)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deregister )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [in] */ VARIANT_BOOL bTerminateSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, RetrieveContents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveContents )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCast, StartTransmission)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartTransmission )( 
            __RPC__in IWdsTransportNamespaceScheduledCastManualStart * This);
        
        END_INTERFACE
    } IWdsTransportNamespaceScheduledCastManualStartVtbl;

    interface IWdsTransportNamespaceScheduledCastManualStart
    {
        CONST_VTBL struct IWdsTransportNamespaceScheduledCastManualStartVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespaceScheduledCastManualStart_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespaceScheduledCastManualStart_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespaceScheduledCastManualStart_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_put_Name(This,bszName)	\
    ( (This)->lpVtbl -> put_Name(This,bszName) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_FriendlyName(This,pbszFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_put_FriendlyName(This,bszFriendlyName)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,bszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_put_Description(This,bszDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bszDescription) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_ContentProvider(This,pbszContentProvider)	\
    ( (This)->lpVtbl -> get_ContentProvider(This,pbszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_put_ContentProvider(This,bszContentProvider)	\
    ( (This)->lpVtbl -> put_ContentProvider(This,bszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Configuration(This,pbszConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pbszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_put_Configuration(This,bszConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,bszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Registered(This,pbRegistered)	\
    ( (This)->lpVtbl -> get_Registered(This,pbRegistered) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_Tombstoned(This,pbTombstoned)	\
    ( (This)->lpVtbl -> get_Tombstoned(This,pbTombstoned) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_TombstoneTime(This,pTombstoneTime)	\
    ( (This)->lpVtbl -> get_TombstoneTime(This,pTombstoneTime) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_get_TransmissionStarted(This,pbTransmissionStarted)	\
    ( (This)->lpVtbl -> get_TransmissionStarted(This,pbTransmissionStarted) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Deregister(This,bTerminateSessions)	\
    ( (This)->lpVtbl -> Deregister(This,bTerminateSessions) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Clone(This,ppWdsTransportNamespaceClone)	\
    ( (This)->lpVtbl -> Clone(This,ppWdsTransportNamespaceClone) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportNamespaceScheduledCastManualStart_RetrieveContents(This,ppWdsTransportContents)	\
    ( (This)->lpVtbl -> RetrieveContents(This,ppWdsTransportContents) ) 


#define IWdsTransportNamespaceScheduledCastManualStart_StartTransmission(This)	\
    ( (This)->lpVtbl -> StartTransmission(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespaceScheduledCastManualStart_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportNamespaceScheduledCastAutoStart_INTERFACE_DEFINED__
#define __IWdsTransportNamespaceScheduledCastAutoStart_INTERFACE_DEFINED__

/* interface IWdsTransportNamespaceScheduledCastAutoStart */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportNamespaceScheduledCastAutoStart;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D606AF3D-EA9C-4219-961E-7491D618D9B9")
    IWdsTransportNamespaceScheduledCastAutoStart : public IWdsTransportNamespaceScheduledCast
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinimumClients( 
            /* [retval][out] */ __RPC__out ULONG *pulMinimumClients) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinimumClients( 
            /* [in] */ ULONG ulMinimumClients) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out DATE *pStartTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StartTime( 
            /* [in] */ DATE StartTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportNamespaceScheduledCastAutoStartVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out WDSTRANSPORT_NAMESPACE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in BSTR bszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in BSTR bszFriendlyName);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in BSTR bszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_ContentProvider)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_ContentProvider)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentProvider )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in BSTR bszContentProvider);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, put_Configuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ __RPC__in BSTR bszConfiguration);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Registered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Registered )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegistered);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_Tombstoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tombstoned )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTombstoned);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TombstoneTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TombstoneTime )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out DATE *pTombstoneTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, get_TransmissionStarted)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStarted )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbTransmissionStarted);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Register)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Deregister)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Deregister )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ VARIANT_BOOL bTerminateSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespaceClone);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespace, RetrieveContents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveContents )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportContents);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCast, StartTransmission)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartTransmission )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCastAutoStart, get_MinimumClients)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinimumClients )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out ULONG *pulMinimumClients);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCastAutoStart, put_MinimumClients)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinimumClients )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ ULONG ulMinimumClients);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCastAutoStart, get_StartTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [retval][out] */ __RPC__out DATE *pStartTime);
        
        DECLSPEC_XFGVIRT(IWdsTransportNamespaceScheduledCastAutoStart, put_StartTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartTime )( 
            __RPC__in IWdsTransportNamespaceScheduledCastAutoStart * This,
            /* [in] */ DATE StartTime);
        
        END_INTERFACE
    } IWdsTransportNamespaceScheduledCastAutoStartVtbl;

    interface IWdsTransportNamespaceScheduledCastAutoStart
    {
        CONST_VTBL struct IWdsTransportNamespaceScheduledCastAutoStartVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportNamespaceScheduledCastAutoStart_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportNamespaceScheduledCastAutoStart_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportNamespaceScheduledCastAutoStart_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_Name(This,bszName)	\
    ( (This)->lpVtbl -> put_Name(This,bszName) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_FriendlyName(This,pbszFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_FriendlyName(This,bszFriendlyName)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,bszFriendlyName) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_Description(This,bszDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bszDescription) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_ContentProvider(This,pbszContentProvider)	\
    ( (This)->lpVtbl -> get_ContentProvider(This,pbszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_ContentProvider(This,bszContentProvider)	\
    ( (This)->lpVtbl -> put_ContentProvider(This,bszContentProvider) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Configuration(This,pbszConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pbszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_Configuration(This,bszConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,bszConfiguration) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Registered(This,pbRegistered)	\
    ( (This)->lpVtbl -> get_Registered(This,pbRegistered) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_Tombstoned(This,pbTombstoned)	\
    ( (This)->lpVtbl -> get_Tombstoned(This,pbTombstoned) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_TombstoneTime(This,pTombstoneTime)	\
    ( (This)->lpVtbl -> get_TombstoneTime(This,pTombstoneTime) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_TransmissionStarted(This,pbTransmissionStarted)	\
    ( (This)->lpVtbl -> get_TransmissionStarted(This,pbTransmissionStarted) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Deregister(This,bTerminateSessions)	\
    ( (This)->lpVtbl -> Deregister(This,bTerminateSessions) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Clone(This,ppWdsTransportNamespaceClone)	\
    ( (This)->lpVtbl -> Clone(This,ppWdsTransportNamespaceClone) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_RetrieveContents(This,ppWdsTransportContents)	\
    ( (This)->lpVtbl -> RetrieveContents(This,ppWdsTransportContents) ) 


#define IWdsTransportNamespaceScheduledCastAutoStart_StartTransmission(This)	\
    ( (This)->lpVtbl -> StartTransmission(This) ) 


#define IWdsTransportNamespaceScheduledCastAutoStart_get_MinimumClients(This,pulMinimumClients)	\
    ( (This)->lpVtbl -> get_MinimumClients(This,pulMinimumClients) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_MinimumClients(This,ulMinimumClients)	\
    ( (This)->lpVtbl -> put_MinimumClients(This,ulMinimumClients) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_get_StartTime(This,pStartTime)	\
    ( (This)->lpVtbl -> get_StartTime(This,pStartTime) ) 

#define IWdsTransportNamespaceScheduledCastAutoStart_put_StartTime(This,StartTime)	\
    ( (This)->lpVtbl -> put_StartTime(This,StartTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportNamespaceScheduledCastAutoStart_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportContent_INTERFACE_DEFINED__
#define __IWdsTransportContent_INTERFACE_DEFINED__

/* interface IWdsTransportContent */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D405D711-0296-4AB4-A860-AC7D32E65798")
    IWdsTransportContent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Namespace( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out ULONG *pulId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveSessions( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportSessions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Terminate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportContent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportContent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportContent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportContent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportContent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportContent * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportContent, get_Namespace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Namespace )( 
            __RPC__in IWdsTransportContent * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportNamespace **ppWdsTransportNamespace);
        
        DECLSPEC_XFGVIRT(IWdsTransportContent, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportContent * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportContent, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportContent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportContent, RetrieveSessions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveSessions )( 
            __RPC__in IWdsTransportContent * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportSessions);
        
        DECLSPEC_XFGVIRT(IWdsTransportContent, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IWdsTransportContent * This);
        
        END_INTERFACE
    } IWdsTransportContentVtbl;

    interface IWdsTransportContent
    {
        CONST_VTBL struct IWdsTransportContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportContent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportContent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportContent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportContent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportContent_get_Namespace(This,ppWdsTransportNamespace)	\
    ( (This)->lpVtbl -> get_Namespace(This,ppWdsTransportNamespace) ) 

#define IWdsTransportContent_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportContent_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportContent_RetrieveSessions(This,ppWdsTransportSessions)	\
    ( (This)->lpVtbl -> RetrieveSessions(This,ppWdsTransportSessions) ) 

#define IWdsTransportContent_Terminate(This)	\
    ( (This)->lpVtbl -> Terminate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportContent_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportSession_INTERFACE_DEFINED__
#define __IWdsTransportSession_INTERFACE_DEFINED__

/* interface IWdsTransportSession */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4EFEA88-65B1-4F30-A4B9-2793987796FB")
    IWdsTransportSession : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Content( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportContent **ppWdsTransportContent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out ULONG *pulId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkInterfaceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszNetworkInterfaceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkInterfaceAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszNetworkInterfaceAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransferRate( 
            /* [retval][out] */ __RPC__out ULONG *pulTransferRate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MasterClientId( 
            /* [retval][out] */ __RPC__out ULONG *pulMasterClientId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RetrieveClients( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportClients) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Terminate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportSession * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportSession * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportSession * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportSession * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_Content)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Content )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportContent **ppWdsTransportContent);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_NetworkInterfaceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkInterfaceName )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszNetworkInterfaceName);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_NetworkInterfaceAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkInterfaceAddress )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszNetworkInterfaceAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_TransferRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransferRate )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__out ULONG *pulTransferRate);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, get_MasterClientId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MasterClientId )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__out ULONG *pulMasterClientId);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, RetrieveClients)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveClients )( 
            __RPC__in IWdsTransportSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportCollection **ppWdsTransportClients);
        
        DECLSPEC_XFGVIRT(IWdsTransportSession, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IWdsTransportSession * This);
        
        END_INTERFACE
    } IWdsTransportSessionVtbl;

    interface IWdsTransportSession
    {
        CONST_VTBL struct IWdsTransportSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportSession_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportSession_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportSession_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportSession_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportSession_get_Content(This,ppWdsTransportContent)	\
    ( (This)->lpVtbl -> get_Content(This,ppWdsTransportContent) ) 

#define IWdsTransportSession_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportSession_get_NetworkInterfaceName(This,pbszNetworkInterfaceName)	\
    ( (This)->lpVtbl -> get_NetworkInterfaceName(This,pbszNetworkInterfaceName) ) 

#define IWdsTransportSession_get_NetworkInterfaceAddress(This,pbszNetworkInterfaceAddress)	\
    ( (This)->lpVtbl -> get_NetworkInterfaceAddress(This,pbszNetworkInterfaceAddress) ) 

#define IWdsTransportSession_get_TransferRate(This,pulTransferRate)	\
    ( (This)->lpVtbl -> get_TransferRate(This,pulTransferRate) ) 

#define IWdsTransportSession_get_MasterClientId(This,pulMasterClientId)	\
    ( (This)->lpVtbl -> get_MasterClientId(This,pulMasterClientId) ) 

#define IWdsTransportSession_RetrieveClients(This,ppWdsTransportClients)	\
    ( (This)->lpVtbl -> RetrieveClients(This,ppWdsTransportClients) ) 

#define IWdsTransportSession_Terminate(This)	\
    ( (This)->lpVtbl -> Terminate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportSession_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportClient_INTERFACE_DEFINED__
#define __IWdsTransportClient_INTERFACE_DEFINED__

/* interface IWdsTransportClient */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B5DBC93A-CABE-46CA-837F-3E44E93C6545")
    IWdsTransportClient : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportSession **ppWdsTransportSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out ULONG *pulId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MacAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszMacAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IpAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszIpAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PercentCompletion( 
            /* [retval][out] */ __RPC__out ULONG *pulPercentCompletion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_JoinDuration( 
            /* [retval][out] */ __RPC__out ULONG *pulJoinDuration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CpuUtilization( 
            /* [retval][out] */ __RPC__out ULONG *pulCpuUtilization) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MemoryUtilization( 
            /* [retval][out] */ __RPC__out ULONG *pulMemoryUtilization) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkUtilization( 
            /* [retval][out] */ __RPC__out ULONG *pulNetworkUtilization) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserIdentity( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszUserIdentity) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [in] */ WDSTRANSPORT_DISCONNECT_TYPE DisconnectionType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportClient * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportClient * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportClient * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportClient * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__deref_out_opt IWdsTransportSession **ppWdsTransportSession);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulId);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_MacAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MacAddress )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszMacAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_IpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IpAddress )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_PercentCompletion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PercentCompletion )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulPercentCompletion);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_JoinDuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JoinDuration )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulJoinDuration);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_CpuUtilization)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CpuUtilization )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulCpuUtilization);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_MemoryUtilization)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MemoryUtilization )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulMemoryUtilization);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_NetworkUtilization)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkUtilization )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulNetworkUtilization);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, get_UserIdentity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserIdentity )( 
            __RPC__in IWdsTransportClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszUserIdentity);
        
        DECLSPEC_XFGVIRT(IWdsTransportClient, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IWdsTransportClient * This,
            /* [in] */ WDSTRANSPORT_DISCONNECT_TYPE DisconnectionType);
        
        END_INTERFACE
    } IWdsTransportClientVtbl;

    interface IWdsTransportClient
    {
        CONST_VTBL struct IWdsTransportClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportClient_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportClient_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportClient_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportClient_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportClient_get_Session(This,ppWdsTransportSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppWdsTransportSession) ) 

#define IWdsTransportClient_get_Id(This,pulId)	\
    ( (This)->lpVtbl -> get_Id(This,pulId) ) 

#define IWdsTransportClient_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportClient_get_MacAddress(This,pbszMacAddress)	\
    ( (This)->lpVtbl -> get_MacAddress(This,pbszMacAddress) ) 

#define IWdsTransportClient_get_IpAddress(This,pbszIpAddress)	\
    ( (This)->lpVtbl -> get_IpAddress(This,pbszIpAddress) ) 

#define IWdsTransportClient_get_PercentCompletion(This,pulPercentCompletion)	\
    ( (This)->lpVtbl -> get_PercentCompletion(This,pulPercentCompletion) ) 

#define IWdsTransportClient_get_JoinDuration(This,pulJoinDuration)	\
    ( (This)->lpVtbl -> get_JoinDuration(This,pulJoinDuration) ) 

#define IWdsTransportClient_get_CpuUtilization(This,pulCpuUtilization)	\
    ( (This)->lpVtbl -> get_CpuUtilization(This,pulCpuUtilization) ) 

#define IWdsTransportClient_get_MemoryUtilization(This,pulMemoryUtilization)	\
    ( (This)->lpVtbl -> get_MemoryUtilization(This,pulMemoryUtilization) ) 

#define IWdsTransportClient_get_NetworkUtilization(This,pulNetworkUtilization)	\
    ( (This)->lpVtbl -> get_NetworkUtilization(This,pulNetworkUtilization) ) 

#define IWdsTransportClient_get_UserIdentity(This,pbszUserIdentity)	\
    ( (This)->lpVtbl -> get_UserIdentity(This,pbszUserIdentity) ) 

#define IWdsTransportClient_Disconnect(This,DisconnectionType)	\
    ( (This)->lpVtbl -> Disconnect(This,DisconnectionType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportClient_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportTftpClient_INTERFACE_DEFINED__
#define __IWdsTransportTftpClient_INTERFACE_DEFINED__

/* interface IWdsTransportTftpClient */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportTftpClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b022d3ae-884d-4d85-b146-53320e76ef62")
    IWdsTransportTftpClient : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFileName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IpAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszIpAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Timeout( 
            /* [retval][out] */ __RPC__out ULONG *pulTimeout) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentFileOffset( 
            /* [retval][out] */ __RPC__out ULONG64 *pul64CurrentOffset) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileSize( 
            /* [retval][out] */ __RPC__out ULONG64 *pul64FileSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockSize( 
            /* [retval][out] */ __RPC__out ULONG *pulBlockSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WindowSize( 
            /* [retval][out] */ __RPC__out ULONG *pulWindowSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportTftpClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportTftpClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportTftpClient * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportTftpClient * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_FileName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFileName);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_IpAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IpAddress )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszIpAddress);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_Timeout)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timeout )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulTimeout);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_CurrentFileOffset)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentFileOffset )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__out ULONG64 *pul64CurrentOffset);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_FileSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSize )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__out ULONG64 *pul64FileSize);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_BlockSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockSize )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulBlockSize);
        
        DECLSPEC_XFGVIRT(IWdsTransportTftpClient, get_WindowSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowSize )( 
            __RPC__in IWdsTransportTftpClient * This,
            /* [retval][out] */ __RPC__out ULONG *pulWindowSize);
        
        END_INTERFACE
    } IWdsTransportTftpClientVtbl;

    interface IWdsTransportTftpClient
    {
        CONST_VTBL struct IWdsTransportTftpClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportTftpClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportTftpClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportTftpClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportTftpClient_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportTftpClient_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportTftpClient_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportTftpClient_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportTftpClient_get_FileName(This,pbszFileName)	\
    ( (This)->lpVtbl -> get_FileName(This,pbszFileName) ) 

#define IWdsTransportTftpClient_get_IpAddress(This,pbszIpAddress)	\
    ( (This)->lpVtbl -> get_IpAddress(This,pbszIpAddress) ) 

#define IWdsTransportTftpClient_get_Timeout(This,pulTimeout)	\
    ( (This)->lpVtbl -> get_Timeout(This,pulTimeout) ) 

#define IWdsTransportTftpClient_get_CurrentFileOffset(This,pul64CurrentOffset)	\
    ( (This)->lpVtbl -> get_CurrentFileOffset(This,pul64CurrentOffset) ) 

#define IWdsTransportTftpClient_get_FileSize(This,pul64FileSize)	\
    ( (This)->lpVtbl -> get_FileSize(This,pul64FileSize) ) 

#define IWdsTransportTftpClient_get_BlockSize(This,pulBlockSize)	\
    ( (This)->lpVtbl -> get_BlockSize(This,pulBlockSize) ) 

#define IWdsTransportTftpClient_get_WindowSize(This,pulWindowSize)	\
    ( (This)->lpVtbl -> get_WindowSize(This,pulWindowSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportTftpClient_INTERFACE_DEFINED__ */


#ifndef __IWdsTransportContentProvider_INTERFACE_DEFINED__
#define __IWdsTransportContentProvider_INTERFACE_DEFINED__

/* interface IWdsTransportContentProvider */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWdsTransportContentProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B9489F24-F219-4ACF-AAD7-265C7C08A6AE")
    IWdsTransportContentProvider : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FilePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFilePath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitializationRoutine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszInitializationRoutine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWdsTransportContentProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWdsTransportContentProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWdsTransportContentProvider * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWdsTransportContentProvider * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWdsTransportContentProvider, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszName);
        
        DECLSPEC_XFGVIRT(IWdsTransportContentProvider, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszDescription);
        
        DECLSPEC_XFGVIRT(IWdsTransportContentProvider, get_FilePath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilePath )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszFilePath);
        
        DECLSPEC_XFGVIRT(IWdsTransportContentProvider, get_InitializationRoutine)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitializationRoutine )( 
            __RPC__in IWdsTransportContentProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbszInitializationRoutine);
        
        END_INTERFACE
    } IWdsTransportContentProviderVtbl;

    interface IWdsTransportContentProvider
    {
        CONST_VTBL struct IWdsTransportContentProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWdsTransportContentProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWdsTransportContentProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWdsTransportContentProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWdsTransportContentProvider_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWdsTransportContentProvider_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWdsTransportContentProvider_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWdsTransportContentProvider_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWdsTransportContentProvider_get_Name(This,pbszName)	\
    ( (This)->lpVtbl -> get_Name(This,pbszName) ) 

#define IWdsTransportContentProvider_get_Description(This,pbszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbszDescription) ) 

#define IWdsTransportContentProvider_get_FilePath(This,pbszFilePath)	\
    ( (This)->lpVtbl -> get_FilePath(This,pbszFilePath) ) 

#define IWdsTransportContentProvider_get_InitializationRoutine(This,pbszInitializationRoutine)	\
    ( (This)->lpVtbl -> get_InitializationRoutine(This,pbszInitializationRoutine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWdsTransportContentProvider_INTERFACE_DEFINED__ */



#ifndef __WdsTptMgmtLib_LIBRARY_DEFINED__
#define __WdsTptMgmtLib_LIBRARY_DEFINED__

/* library WdsTptMgmtLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WdsTptMgmtLib;

EXTERN_C const CLSID CLSID_WdsTransportCacheable;

#ifdef __cplusplus

class DECLSPEC_UUID("70590B16-F146-46BD-BD9D-4AAA90084BF5")
WdsTransportCacheable;
#endif

EXTERN_C const CLSID CLSID_WdsTransportCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("C7F18B09-391E-436E-B10B-C3EF46F2C34F")
WdsTransportCollection;
#endif

EXTERN_C const CLSID CLSID_WdsTransportManager;

#ifdef __cplusplus

class DECLSPEC_UUID("F21523F6-837C-4A58-AF99-8A7E27F8FF59")
WdsTransportManager;
#endif

EXTERN_C const CLSID CLSID_WdsTransportServer;

#ifdef __cplusplus

class DECLSPEC_UUID("EA19B643-4ADF-4413-942C-14F379118760")
WdsTransportServer;
#endif

EXTERN_C const CLSID CLSID_WdsTransportSetupManager;

#ifdef __cplusplus

class DECLSPEC_UUID("C7BEEAAD-9F04-4923-9F0C-FBF52BC7590F")
WdsTransportSetupManager;
#endif

EXTERN_C const CLSID CLSID_WdsTransportConfigurationManager;

#ifdef __cplusplus

class DECLSPEC_UUID("8743F674-904C-47CA-8512-35FE98F6B0AC")
WdsTransportConfigurationManager;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespaceManager;

#ifdef __cplusplus

class DECLSPEC_UUID("F08CDB63-85DE-4A28-A1A9-5CA3E7EFDA73")
WdsTransportNamespaceManager;
#endif

EXTERN_C const CLSID CLSID_WdsTransportServicePolicy;

#ifdef __cplusplus

class DECLSPEC_UUID("65ACEADC-2F0B-4F43-9F4D-811865D8CEAD")
WdsTransportServicePolicy;
#endif

EXTERN_C const CLSID CLSID_WdsTransportDiagnosticsPolicy;

#ifdef __cplusplus

class DECLSPEC_UUID("EB3333E1-A7AD-46F5-80D6-6B740204E509")
WdsTransportDiagnosticsPolicy;
#endif

EXTERN_C const CLSID CLSID_WdsTransportMulticastSessionPolicy;

#ifdef __cplusplus

class DECLSPEC_UUID("3C6BC3F4-6418-472A-B6F1-52D457195437")
WdsTransportMulticastSessionPolicy;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespace;

#ifdef __cplusplus

class DECLSPEC_UUID("D8385768-0732-4EC1-95EA-16DA581908A1")
WdsTransportNamespace;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespaceAutoCast;

#ifdef __cplusplus

class DECLSPEC_UUID("B091F5A8-6A99-478D-B23B-09E8FEE04574")
WdsTransportNamespaceAutoCast;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespaceScheduledCast;

#ifdef __cplusplus

class DECLSPEC_UUID("BADC1897-7025-44EB-9108-FB61C4055792")
WdsTransportNamespaceScheduledCast;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespaceScheduledCastManualStart;

#ifdef __cplusplus

class DECLSPEC_UUID("D3E1A2AA-CAAC-460E-B98A-47F9F318A1FA")
WdsTransportNamespaceScheduledCastManualStart;
#endif

EXTERN_C const CLSID CLSID_WdsTransportNamespaceScheduledCastAutoStart;

#ifdef __cplusplus

class DECLSPEC_UUID("A1107052-122C-4B81-9B7C-386E6855383F")
WdsTransportNamespaceScheduledCastAutoStart;
#endif

EXTERN_C const CLSID CLSID_WdsTransportContent;

#ifdef __cplusplus

class DECLSPEC_UUID("0A891FE7-4A3F-4C65-B6F2-1467619679EA")
WdsTransportContent;
#endif

EXTERN_C const CLSID CLSID_WdsTransportSession;

#ifdef __cplusplus

class DECLSPEC_UUID("749AC4E0-67BC-4743-BFE5-CACB1F26F57F")
WdsTransportSession;
#endif

EXTERN_C const CLSID CLSID_WdsTransportClient;

#ifdef __cplusplus

class DECLSPEC_UUID("66D2C5E9-0FF6-49EC-9733-DAFB1E01DF1C")
WdsTransportClient;
#endif

EXTERN_C const CLSID CLSID_WdsTransportTftpClient;

#ifdef __cplusplus

class DECLSPEC_UUID("50343925-7c5c-4c8c-96c4-ad9fa5005fba")
WdsTransportTftpClient;
#endif

EXTERN_C const CLSID CLSID_WdsTransportTftpManager;

#ifdef __cplusplus

class DECLSPEC_UUID("c8e9dca2-3241-4e4d-b806-bc74019dfeda")
WdsTransportTftpManager;
#endif

EXTERN_C const CLSID CLSID_WdsTransportContentProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("E0BE741F-5A75-4EB9-8A2D-5E189B45F327")
WdsTransportContentProvider;
#endif
#endif /* __WdsTptMgmtLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wdstptmgmt_0000_0026 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wdstptmgmt_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wdstptmgmt_0000_0026_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



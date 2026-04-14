

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

#ifndef __sbtsv_h__
#define __sbtsv_h__

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

#ifndef __ITsSbPlugin_FWD_DEFINED__
#define __ITsSbPlugin_FWD_DEFINED__
typedef interface ITsSbPlugin ITsSbPlugin;

#endif 	/* __ITsSbPlugin_FWD_DEFINED__ */


#ifndef __ITsSbResourcePlugin_FWD_DEFINED__
#define __ITsSbResourcePlugin_FWD_DEFINED__
typedef interface ITsSbResourcePlugin ITsSbResourcePlugin;

#endif 	/* __ITsSbResourcePlugin_FWD_DEFINED__ */


#ifndef __ITsSbServiceNotification_FWD_DEFINED__
#define __ITsSbServiceNotification_FWD_DEFINED__
typedef interface ITsSbServiceNotification ITsSbServiceNotification;

#endif 	/* __ITsSbServiceNotification_FWD_DEFINED__ */


#ifndef __ITsSbLoadBalancing_FWD_DEFINED__
#define __ITsSbLoadBalancing_FWD_DEFINED__
typedef interface ITsSbLoadBalancing ITsSbLoadBalancing;

#endif 	/* __ITsSbLoadBalancing_FWD_DEFINED__ */


#ifndef __ITsSbPlacement_FWD_DEFINED__
#define __ITsSbPlacement_FWD_DEFINED__
typedef interface ITsSbPlacement ITsSbPlacement;

#endif 	/* __ITsSbPlacement_FWD_DEFINED__ */


#ifndef __ITsSbOrchestration_FWD_DEFINED__
#define __ITsSbOrchestration_FWD_DEFINED__
typedef interface ITsSbOrchestration ITsSbOrchestration;

#endif 	/* __ITsSbOrchestration_FWD_DEFINED__ */


#ifndef __ITsSbEnvironment_FWD_DEFINED__
#define __ITsSbEnvironment_FWD_DEFINED__
typedef interface ITsSbEnvironment ITsSbEnvironment;

#endif 	/* __ITsSbEnvironment_FWD_DEFINED__ */


#ifndef __ITsSbLoadBalanceResult_FWD_DEFINED__
#define __ITsSbLoadBalanceResult_FWD_DEFINED__
typedef interface ITsSbLoadBalanceResult ITsSbLoadBalanceResult;

#endif 	/* __ITsSbLoadBalanceResult_FWD_DEFINED__ */


#ifndef __ITsSbTarget_FWD_DEFINED__
#define __ITsSbTarget_FWD_DEFINED__
typedef interface ITsSbTarget ITsSbTarget;

#endif 	/* __ITsSbTarget_FWD_DEFINED__ */


#ifndef __ITsSbSession_FWD_DEFINED__
#define __ITsSbSession_FWD_DEFINED__
typedef interface ITsSbSession ITsSbSession;

#endif 	/* __ITsSbSession_FWD_DEFINED__ */


#ifndef __ITsSbResourceNotification_FWD_DEFINED__
#define __ITsSbResourceNotification_FWD_DEFINED__
typedef interface ITsSbResourceNotification ITsSbResourceNotification;

#endif 	/* __ITsSbResourceNotification_FWD_DEFINED__ */


#ifndef __ITsSbResourceNotificationEx_FWD_DEFINED__
#define __ITsSbResourceNotificationEx_FWD_DEFINED__
typedef interface ITsSbResourceNotificationEx ITsSbResourceNotificationEx;

#endif 	/* __ITsSbResourceNotificationEx_FWD_DEFINED__ */


#ifndef __ITsSbTaskInfo_FWD_DEFINED__
#define __ITsSbTaskInfo_FWD_DEFINED__
typedef interface ITsSbTaskInfo ITsSbTaskInfo;

#endif 	/* __ITsSbTaskInfo_FWD_DEFINED__ */


#ifndef __ITsSbTaskPlugin_FWD_DEFINED__
#define __ITsSbTaskPlugin_FWD_DEFINED__
typedef interface ITsSbTaskPlugin ITsSbTaskPlugin;

#endif 	/* __ITsSbTaskPlugin_FWD_DEFINED__ */


#ifndef __ITsSbPropertySet_FWD_DEFINED__
#define __ITsSbPropertySet_FWD_DEFINED__
typedef interface ITsSbPropertySet ITsSbPropertySet;

#endif 	/* __ITsSbPropertySet_FWD_DEFINED__ */


#ifndef __ITsSbPluginPropertySet_FWD_DEFINED__
#define __ITsSbPluginPropertySet_FWD_DEFINED__
typedef interface ITsSbPluginPropertySet ITsSbPluginPropertySet;

#endif 	/* __ITsSbPluginPropertySet_FWD_DEFINED__ */


#ifndef __ITsSbClientConnectionPropertySet_FWD_DEFINED__
#define __ITsSbClientConnectionPropertySet_FWD_DEFINED__
typedef interface ITsSbClientConnectionPropertySet ITsSbClientConnectionPropertySet;

#endif 	/* __ITsSbClientConnectionPropertySet_FWD_DEFINED__ */


#ifndef __ITsSbTargetPropertySet_FWD_DEFINED__
#define __ITsSbTargetPropertySet_FWD_DEFINED__
typedef interface ITsSbTargetPropertySet ITsSbTargetPropertySet;

#endif 	/* __ITsSbTargetPropertySet_FWD_DEFINED__ */


#ifndef __ITsSbEnvironmentPropertySet_FWD_DEFINED__
#define __ITsSbEnvironmentPropertySet_FWD_DEFINED__
typedef interface ITsSbEnvironmentPropertySet ITsSbEnvironmentPropertySet;

#endif 	/* __ITsSbEnvironmentPropertySet_FWD_DEFINED__ */


#ifndef __ITsSbBaseNotifySink_FWD_DEFINED__
#define __ITsSbBaseNotifySink_FWD_DEFINED__
typedef interface ITsSbBaseNotifySink ITsSbBaseNotifySink;

#endif 	/* __ITsSbBaseNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbPluginNotifySink_FWD_DEFINED__
#define __ITsSbPluginNotifySink_FWD_DEFINED__
typedef interface ITsSbPluginNotifySink ITsSbPluginNotifySink;

#endif 	/* __ITsSbPluginNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbLoadBalancingNotifySink_FWD_DEFINED__
#define __ITsSbLoadBalancingNotifySink_FWD_DEFINED__
typedef interface ITsSbLoadBalancingNotifySink ITsSbLoadBalancingNotifySink;

#endif 	/* __ITsSbLoadBalancingNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbPlacementNotifySink_FWD_DEFINED__
#define __ITsSbPlacementNotifySink_FWD_DEFINED__
typedef interface ITsSbPlacementNotifySink ITsSbPlacementNotifySink;

#endif 	/* __ITsSbPlacementNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbOrchestrationNotifySink_FWD_DEFINED__
#define __ITsSbOrchestrationNotifySink_FWD_DEFINED__
typedef interface ITsSbOrchestrationNotifySink ITsSbOrchestrationNotifySink;

#endif 	/* __ITsSbOrchestrationNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbTaskPluginNotifySink_FWD_DEFINED__
#define __ITsSbTaskPluginNotifySink_FWD_DEFINED__
typedef interface ITsSbTaskPluginNotifySink ITsSbTaskPluginNotifySink;

#endif 	/* __ITsSbTaskPluginNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbClientConnection_FWD_DEFINED__
#define __ITsSbClientConnection_FWD_DEFINED__
typedef interface ITsSbClientConnection ITsSbClientConnection;

#endif 	/* __ITsSbClientConnection_FWD_DEFINED__ */


#ifndef __ITsSbProvider_FWD_DEFINED__
#define __ITsSbProvider_FWD_DEFINED__
typedef interface ITsSbProvider ITsSbProvider;

#endif 	/* __ITsSbProvider_FWD_DEFINED__ */


#ifndef __ITsSbResourcePluginStore_FWD_DEFINED__
#define __ITsSbResourcePluginStore_FWD_DEFINED__
typedef interface ITsSbResourcePluginStore ITsSbResourcePluginStore;

#endif 	/* __ITsSbResourcePluginStore_FWD_DEFINED__ */


#ifndef __ITsSbFilterPluginStore_FWD_DEFINED__
#define __ITsSbFilterPluginStore_FWD_DEFINED__
typedef interface ITsSbFilterPluginStore ITsSbFilterPluginStore;

#endif 	/* __ITsSbFilterPluginStore_FWD_DEFINED__ */


#ifndef __ITsSbGlobalStore_FWD_DEFINED__
#define __ITsSbGlobalStore_FWD_DEFINED__
typedef interface ITsSbGlobalStore ITsSbGlobalStore;

#endif 	/* __ITsSbGlobalStore_FWD_DEFINED__ */


#ifndef __ITsSbProvisioningPluginNotifySink_FWD_DEFINED__
#define __ITsSbProvisioningPluginNotifySink_FWD_DEFINED__
typedef interface ITsSbProvisioningPluginNotifySink ITsSbProvisioningPluginNotifySink;

#endif 	/* __ITsSbProvisioningPluginNotifySink_FWD_DEFINED__ */


#ifndef __ITsSbProvisioning_FWD_DEFINED__
#define __ITsSbProvisioning_FWD_DEFINED__
typedef interface ITsSbProvisioning ITsSbProvisioning;

#endif 	/* __ITsSbProvisioning_FWD_DEFINED__ */


#ifndef __ITsSbGenericNotifySink_FWD_DEFINED__
#define __ITsSbGenericNotifySink_FWD_DEFINED__
typedef interface ITsSbGenericNotifySink ITsSbGenericNotifySink;

#endif 	/* __ITsSbGenericNotifySink_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "SessdirPublicTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_sbtsv_0000_0000 */
/* [local] */ 

#pragma once
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum _TS_SB_SORT_BY
    {
        TS_SB_SORT_BY_NONE	= 0,
        TS_SB_SORT_BY_NAME	= 0x1,
        TS_SB_SORT_BY_PROP	= 0x2
    } 	TS_SB_SORT_BY;

// 
// If plugins get synch error (E_SB_SYNCH_CONFLICT) while saving resources to broker store;
// Plugins should attempt to retry saving the object.
// 
#define SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS                  100
#define ACQUIRE_TARGET_LOCK_TIMEOUT                        300000


































extern RPC_IF_HANDLE __MIDL_itf_sbtsv_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sbtsv_0000_0000_v0_0_s_ifspec;

#ifndef __ITsSbPlugin_INTERFACE_DEFINED__
#define __ITsSbPlugin_INTERFACE_DEFINED__

/* interface ITsSbPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("48cd7406-caab-465f-a5d6-baa863b9ea4f")
    ITsSbPlugin : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Terminate( 
            /* [in] */ HRESULT hr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPlugin * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbPlugin * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbPlugin * This,
            /* [in] */ HRESULT hr);
        
        END_INTERFACE
    } ITsSbPluginVtbl;

    interface ITsSbPlugin
    {
        CONST_VTBL struct ITsSbPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPlugin_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbPlugin_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPlugin_INTERFACE_DEFINED__ */


#ifndef __ITsSbResourcePlugin_INTERFACE_DEFINED__
#define __ITsSbResourcePlugin_INTERFACE_DEFINED__

/* interface ITsSbResourcePlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbResourcePlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EA8DB42C-98ED-4535-A88B-2A164F35490F")
    ITsSbResourcePlugin : public ITsSbPlugin
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbResourcePluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbResourcePlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbResourcePlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbResourcePlugin * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbResourcePlugin * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbResourcePlugin * This,
            /* [in] */ HRESULT hr);
        
        END_INTERFACE
    } ITsSbResourcePluginVtbl;

    interface ITsSbResourcePlugin
    {
        CONST_VTBL struct ITsSbResourcePluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbResourcePlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbResourcePlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbResourcePlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbResourcePlugin_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbResourcePlugin_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbResourcePlugin_INTERFACE_DEFINED__ */


#ifndef __ITsSbServiceNotification_INTERFACE_DEFINED__
#define __ITsSbServiceNotification_INTERFACE_DEFINED__

/* interface ITsSbServiceNotification */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbServiceNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86CB68AE-86E0-4F57-8A64-BB7406BC5550")
    ITsSbServiceNotification : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyServiceFailure( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyServiceSuccess( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbServiceNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbServiceNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbServiceNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbServiceNotification * This);
        
        DECLSPEC_XFGVIRT(ITsSbServiceNotification, NotifyServiceFailure)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyServiceFailure )( 
            __RPC__in ITsSbServiceNotification * This);
        
        DECLSPEC_XFGVIRT(ITsSbServiceNotification, NotifyServiceSuccess)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyServiceSuccess )( 
            __RPC__in ITsSbServiceNotification * This);
        
        END_INTERFACE
    } ITsSbServiceNotificationVtbl;

    interface ITsSbServiceNotification
    {
        CONST_VTBL struct ITsSbServiceNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbServiceNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbServiceNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbServiceNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbServiceNotification_NotifyServiceFailure(This)	\
    ( (This)->lpVtbl -> NotifyServiceFailure(This) ) 

#define ITsSbServiceNotification_NotifyServiceSuccess(This)	\
    ( (This)->lpVtbl -> NotifyServiceSuccess(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbServiceNotification_INTERFACE_DEFINED__ */


#ifndef __ITsSbLoadBalancing_INTERFACE_DEFINED__
#define __ITsSbLoadBalancing_INTERFACE_DEFINED__

/* interface ITsSbLoadBalancing */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbLoadBalancing;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24329274-9EB7-11DC-AE98-F2B456D89593")
    ITsSbLoadBalancing : public ITsSbPlugin
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMostSuitableTarget( 
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbLoadBalancingNotifySink *pLBSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbLoadBalancingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbLoadBalancing * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbLoadBalancing * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbLoadBalancing * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbLoadBalancing * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbLoadBalancing * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbLoadBalancing, GetMostSuitableTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMostSuitableTarget )( 
            __RPC__in ITsSbLoadBalancing * This,
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbLoadBalancingNotifySink *pLBSink);
        
        END_INTERFACE
    } ITsSbLoadBalancingVtbl;

    interface ITsSbLoadBalancing
    {
        CONST_VTBL struct ITsSbLoadBalancingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbLoadBalancing_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbLoadBalancing_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbLoadBalancing_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbLoadBalancing_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbLoadBalancing_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#define ITsSbLoadBalancing_GetMostSuitableTarget(This,pConnection,pLBSink)	\
    ( (This)->lpVtbl -> GetMostSuitableTarget(This,pConnection,pLBSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbLoadBalancing_INTERFACE_DEFINED__ */


#ifndef __ITsSbPlacement_INTERFACE_DEFINED__
#define __ITsSbPlacement_INTERFACE_DEFINED__

/* interface ITsSbPlacement */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPlacement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DAADEE5F-6D32-480E-9E36-DDAB2329F06D")
    ITsSbPlacement : public ITsSbPlugin
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryEnvironmentForTarget( 
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbPlacementNotifySink *pPlacementSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPlacementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPlacement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPlacement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPlacement * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbPlacement * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbPlacement * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbPlacement, QueryEnvironmentForTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryEnvironmentForTarget )( 
            __RPC__in ITsSbPlacement * This,
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbPlacementNotifySink *pPlacementSink);
        
        END_INTERFACE
    } ITsSbPlacementVtbl;

    interface ITsSbPlacement
    {
        CONST_VTBL struct ITsSbPlacementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPlacement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPlacement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPlacement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPlacement_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbPlacement_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#define ITsSbPlacement_QueryEnvironmentForTarget(This,pConnection,pPlacementSink)	\
    ( (This)->lpVtbl -> QueryEnvironmentForTarget(This,pConnection,pPlacementSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPlacement_INTERFACE_DEFINED__ */


#ifndef __ITsSbOrchestration_INTERFACE_DEFINED__
#define __ITsSbOrchestration_INTERFACE_DEFINED__

/* interface ITsSbOrchestration */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbOrchestration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64FC1172-9EB7-11DC-8B00-3ABA56D89593")
    ITsSbOrchestration : public ITsSbPlugin
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PrepareTargetForConnect( 
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbOrchestrationNotifySink *pOrchestrationNotifySink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbOrchestrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbOrchestration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbOrchestration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbOrchestration * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbOrchestration * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbOrchestration * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbOrchestration, PrepareTargetForConnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PrepareTargetForConnect )( 
            __RPC__in ITsSbOrchestration * This,
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection,
            /* [in] */ __RPC__in_opt ITsSbOrchestrationNotifySink *pOrchestrationNotifySink);
        
        END_INTERFACE
    } ITsSbOrchestrationVtbl;

    interface ITsSbOrchestration
    {
        CONST_VTBL struct ITsSbOrchestrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbOrchestration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbOrchestration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbOrchestration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbOrchestration_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbOrchestration_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#define ITsSbOrchestration_PrepareTargetForConnect(This,pConnection,pOrchestrationNotifySink)	\
    ( (This)->lpVtbl -> PrepareTargetForConnect(This,pConnection,pOrchestrationNotifySink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbOrchestration_INTERFACE_DEFINED__ */


#ifndef __ITsSbEnvironment_INTERFACE_DEFINED__
#define __ITsSbEnvironment_INTERFACE_DEFINED__

/* interface ITsSbEnvironment */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbEnvironment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8c87f7f7-bf51-4a5c-87bf-8e94fb6e2256")
    ITsSbEnvironment : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerWeight( 
            /* [retval][out] */ __RPC__out DWORD *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnvironmentPropertySet( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbEnvironmentPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnvironmentPropertySet( 
            /* [in] */ __RPC__in_opt ITsSbEnvironmentPropertySet *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbEnvironmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbEnvironment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbEnvironment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbEnvironment * This);
        
        DECLSPEC_XFGVIRT(ITsSbEnvironment, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITsSbEnvironment * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbEnvironment, get_ServerWeight)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerWeight )( 
            __RPC__in ITsSbEnvironment * This,
            /* [retval][out] */ __RPC__out DWORD *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbEnvironment, get_EnvironmentPropertySet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnvironmentPropertySet )( 
            __RPC__in ITsSbEnvironment * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbEnvironmentPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbEnvironment, put_EnvironmentPropertySet)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnvironmentPropertySet )( 
            __RPC__in ITsSbEnvironment * This,
            /* [in] */ __RPC__in_opt ITsSbEnvironmentPropertySet *pVal);
        
        END_INTERFACE
    } ITsSbEnvironmentVtbl;

    interface ITsSbEnvironment
    {
        CONST_VTBL struct ITsSbEnvironmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbEnvironment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbEnvironment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbEnvironment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbEnvironment_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define ITsSbEnvironment_get_ServerWeight(This,pVal)	\
    ( (This)->lpVtbl -> get_ServerWeight(This,pVal) ) 

#define ITsSbEnvironment_get_EnvironmentPropertySet(This,ppPropertySet)	\
    ( (This)->lpVtbl -> get_EnvironmentPropertySet(This,ppPropertySet) ) 

#define ITsSbEnvironment_put_EnvironmentPropertySet(This,pVal)	\
    ( (This)->lpVtbl -> put_EnvironmentPropertySet(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbEnvironment_INTERFACE_DEFINED__ */


#ifndef __ITsSbLoadBalanceResult_INTERFACE_DEFINED__
#define __ITsSbLoadBalanceResult_INTERFACE_DEFINED__

/* interface ITsSbLoadBalanceResult */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbLoadBalanceResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24FDB7AC-FEA6-11DC-9672-9A8956D89593")
    ITsSbLoadBalanceResult : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbLoadBalanceResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbLoadBalanceResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbLoadBalanceResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbLoadBalanceResult * This);
        
        DECLSPEC_XFGVIRT(ITsSbLoadBalanceResult, get_TargetName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetName )( 
            __RPC__in ITsSbLoadBalanceResult * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } ITsSbLoadBalanceResultVtbl;

    interface ITsSbLoadBalanceResult
    {
        CONST_VTBL struct ITsSbLoadBalanceResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbLoadBalanceResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbLoadBalanceResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbLoadBalanceResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbLoadBalanceResult_get_TargetName(This,pVal)	\
    ( (This)->lpVtbl -> get_TargetName(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbLoadBalanceResult_INTERFACE_DEFINED__ */


#ifndef __ITsSbTarget_INTERFACE_DEFINED__
#define __ITsSbTarget_INTERFACE_DEFINED__

/* interface ITsSbTarget */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("16616ECC-272D-411D-B324-126893033856")
    ITsSbTarget : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetName( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FarmName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FarmName( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetFQDN( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TargetFqdnName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetFQDN( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetNetbios( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TargetNetbiosName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetNetbios( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IpAddresses( 
            /* [size_is][out] */ __RPC__out_ecount_full(*numAddresses) TSSD_ConnectionPoint *sockaddr,
            /* [out][in] */ __RPC__inout DWORD *numAddresses) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IpAddresses( 
            /* [size_is][in] */ __RPC__in_ecount_full(numAddresses) TSSD_ConnectionPoint *sockaddr,
            /* [in] */ DWORD numAddresses) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetState( 
            /* [retval][out] */ __RPC__out TARGET_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetState( 
            /* [in] */ TARGET_STATE State) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetPropertySet( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbTargetPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetPropertySet( 
            /* [in] */ __RPC__in_opt ITsSbTargetPropertySet *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnvironmentName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnvironmentName( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumSessions( 
            /* [retval][out] */ __RPC__out DWORD *pNumSessions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumPendingConnections( 
            /* [retval][out] */ __RPC__out DWORD *pNumPendingConnections) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetLoad( 
            /* [retval][out] */ __RPC__out DWORD *pTargetLoad) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbTarget * This);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetName )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_TargetName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetName )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_FarmName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FarmName )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_FarmName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FarmName )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetFQDN)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetFQDN )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TargetFqdnName);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_TargetFQDN)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetFQDN )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetNetbios)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetNetbios )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TargetNetbiosName);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_TargetNetbios)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetNetbios )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_IpAddresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IpAddresses )( 
            __RPC__in ITsSbTarget * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*numAddresses) TSSD_ConnectionPoint *sockaddr,
            /* [out][in] */ __RPC__inout DWORD *numAddresses);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_IpAddresses)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IpAddresses )( 
            __RPC__in ITsSbTarget * This,
            /* [size_is][in] */ __RPC__in_ecount_full(numAddresses) TSSD_ConnectionPoint *sockaddr,
            /* [in] */ DWORD numAddresses);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetState )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__out TARGET_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_TargetState)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetState )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ TARGET_STATE State);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetPropertySet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetPropertySet )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbTargetPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_TargetPropertySet)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetPropertySet )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in_opt ITsSbTargetPropertySet *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_EnvironmentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnvironmentName )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, put_EnvironmentName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnvironmentName )( 
            __RPC__in ITsSbTarget * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_NumSessions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumSessions )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__out DWORD *pNumSessions);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_NumPendingConnections)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumPendingConnections )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__out DWORD *pNumPendingConnections);
        
        DECLSPEC_XFGVIRT(ITsSbTarget, get_TargetLoad)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetLoad )( 
            __RPC__in ITsSbTarget * This,
            /* [retval][out] */ __RPC__out DWORD *pTargetLoad);
        
        END_INTERFACE
    } ITsSbTargetVtbl;

    interface ITsSbTarget
    {
        CONST_VTBL struct ITsSbTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbTarget_get_TargetName(This,pVal)	\
    ( (This)->lpVtbl -> get_TargetName(This,pVal) ) 

#define ITsSbTarget_put_TargetName(This,Val)	\
    ( (This)->lpVtbl -> put_TargetName(This,Val) ) 

#define ITsSbTarget_get_FarmName(This,pVal)	\
    ( (This)->lpVtbl -> get_FarmName(This,pVal) ) 

#define ITsSbTarget_put_FarmName(This,Val)	\
    ( (This)->lpVtbl -> put_FarmName(This,Val) ) 

#define ITsSbTarget_get_TargetFQDN(This,TargetFqdnName)	\
    ( (This)->lpVtbl -> get_TargetFQDN(This,TargetFqdnName) ) 

#define ITsSbTarget_put_TargetFQDN(This,Val)	\
    ( (This)->lpVtbl -> put_TargetFQDN(This,Val) ) 

#define ITsSbTarget_get_TargetNetbios(This,TargetNetbiosName)	\
    ( (This)->lpVtbl -> get_TargetNetbios(This,TargetNetbiosName) ) 

#define ITsSbTarget_put_TargetNetbios(This,Val)	\
    ( (This)->lpVtbl -> put_TargetNetbios(This,Val) ) 

#define ITsSbTarget_get_IpAddresses(This,sockaddr,numAddresses)	\
    ( (This)->lpVtbl -> get_IpAddresses(This,sockaddr,numAddresses) ) 

#define ITsSbTarget_put_IpAddresses(This,sockaddr,numAddresses)	\
    ( (This)->lpVtbl -> put_IpAddresses(This,sockaddr,numAddresses) ) 

#define ITsSbTarget_get_TargetState(This,pState)	\
    ( (This)->lpVtbl -> get_TargetState(This,pState) ) 

#define ITsSbTarget_put_TargetState(This,State)	\
    ( (This)->lpVtbl -> put_TargetState(This,State) ) 

#define ITsSbTarget_get_TargetPropertySet(This,ppPropertySet)	\
    ( (This)->lpVtbl -> get_TargetPropertySet(This,ppPropertySet) ) 

#define ITsSbTarget_put_TargetPropertySet(This,pVal)	\
    ( (This)->lpVtbl -> put_TargetPropertySet(This,pVal) ) 

#define ITsSbTarget_get_EnvironmentName(This,pVal)	\
    ( (This)->lpVtbl -> get_EnvironmentName(This,pVal) ) 

#define ITsSbTarget_put_EnvironmentName(This,Val)	\
    ( (This)->lpVtbl -> put_EnvironmentName(This,Val) ) 

#define ITsSbTarget_get_NumSessions(This,pNumSessions)	\
    ( (This)->lpVtbl -> get_NumSessions(This,pNumSessions) ) 

#define ITsSbTarget_get_NumPendingConnections(This,pNumPendingConnections)	\
    ( (This)->lpVtbl -> get_NumPendingConnections(This,pNumPendingConnections) ) 

#define ITsSbTarget_get_TargetLoad(This,pTargetLoad)	\
    ( (This)->lpVtbl -> get_TargetLoad(This,pTargetLoad) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbTarget_INTERFACE_DEFINED__ */


#ifndef __ITsSbSession_INTERFACE_DEFINED__
#define __ITsSbSession_INTERFACE_DEFINED__

/* interface ITsSbSession */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D453AAC7-B1D8-4C5E-BA34-9AFB4C8C5510")
    ITsSbSession : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SessionId( 
            /* [retval][out] */ __RPC__out DWORD *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *targetName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TargetName( 
            /* [in] */ __RPC__in BSTR targetName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Username( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Domain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *domain) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out TSSESSION_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_State( 
            /* [in] */ TSSESSION_STATE State) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CreateTime( 
            /* [retval][out] */ __RPC__out FILETIME *pTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CreateTime( 
            /* [in] */ FILETIME Time) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisconnectTime( 
            /* [retval][out] */ __RPC__out FILETIME *pTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisconnectTime( 
            /* [in] */ FILETIME Time) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitialProgram( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *app) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InitialProgram( 
            /* [in] */ __RPC__in BSTR Application) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientDisplay( 
            /* [retval][out] */ __RPC__out CLIENT_DISPLAY *pClientDisplay) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ClientDisplay( 
            /* [in] */ CLIENT_DISPLAY pClientDisplay) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProtocolType( 
            /* [retval][out] */ __RPC__out DWORD *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProtocolType( 
            /* [in] */ DWORD Val) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbSession * This);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_SessionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionId )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out DWORD *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_TargetName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetName )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *targetName);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_TargetName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TargetName )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ __RPC__in BSTR targetName);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_Username)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Username )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userName);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *domain);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out TSSESSION_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_State)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ TSSESSION_STATE State);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_CreateTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreateTime )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out FILETIME *pTime);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_CreateTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreateTime )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ FILETIME Time);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_DisconnectTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisconnectTime )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out FILETIME *pTime);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_DisconnectTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisconnectTime )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ FILETIME Time);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_InitialProgram)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitialProgram )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *app);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_InitialProgram)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InitialProgram )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ __RPC__in BSTR Application);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_ClientDisplay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientDisplay )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out CLIENT_DISPLAY *pClientDisplay);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_ClientDisplay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientDisplay )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ CLIENT_DISPLAY pClientDisplay);
        
        DECLSPEC_XFGVIRT(ITsSbSession, get_ProtocolType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProtocolType )( 
            __RPC__in ITsSbSession * This,
            /* [retval][out] */ __RPC__out DWORD *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbSession, put_ProtocolType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProtocolType )( 
            __RPC__in ITsSbSession * This,
            /* [in] */ DWORD Val);
        
        END_INTERFACE
    } ITsSbSessionVtbl;

    interface ITsSbSession
    {
        CONST_VTBL struct ITsSbSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbSession_get_SessionId(This,pVal)	\
    ( (This)->lpVtbl -> get_SessionId(This,pVal) ) 

#define ITsSbSession_get_TargetName(This,targetName)	\
    ( (This)->lpVtbl -> get_TargetName(This,targetName) ) 

#define ITsSbSession_put_TargetName(This,targetName)	\
    ( (This)->lpVtbl -> put_TargetName(This,targetName) ) 

#define ITsSbSession_get_Username(This,userName)	\
    ( (This)->lpVtbl -> get_Username(This,userName) ) 

#define ITsSbSession_get_Domain(This,domain)	\
    ( (This)->lpVtbl -> get_Domain(This,domain) ) 

#define ITsSbSession_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define ITsSbSession_put_State(This,State)	\
    ( (This)->lpVtbl -> put_State(This,State) ) 

#define ITsSbSession_get_CreateTime(This,pTime)	\
    ( (This)->lpVtbl -> get_CreateTime(This,pTime) ) 

#define ITsSbSession_put_CreateTime(This,Time)	\
    ( (This)->lpVtbl -> put_CreateTime(This,Time) ) 

#define ITsSbSession_get_DisconnectTime(This,pTime)	\
    ( (This)->lpVtbl -> get_DisconnectTime(This,pTime) ) 

#define ITsSbSession_put_DisconnectTime(This,Time)	\
    ( (This)->lpVtbl -> put_DisconnectTime(This,Time) ) 

#define ITsSbSession_get_InitialProgram(This,app)	\
    ( (This)->lpVtbl -> get_InitialProgram(This,app) ) 

#define ITsSbSession_put_InitialProgram(This,Application)	\
    ( (This)->lpVtbl -> put_InitialProgram(This,Application) ) 

#define ITsSbSession_get_ClientDisplay(This,pClientDisplay)	\
    ( (This)->lpVtbl -> get_ClientDisplay(This,pClientDisplay) ) 

#define ITsSbSession_put_ClientDisplay(This,pClientDisplay)	\
    ( (This)->lpVtbl -> put_ClientDisplay(This,pClientDisplay) ) 

#define ITsSbSession_get_ProtocolType(This,pVal)	\
    ( (This)->lpVtbl -> get_ProtocolType(This,pVal) ) 

#define ITsSbSession_put_ProtocolType(This,Val)	\
    ( (This)->lpVtbl -> put_ProtocolType(This,Val) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbSession_INTERFACE_DEFINED__ */


#ifndef __ITsSbResourceNotification_INTERFACE_DEFINED__
#define __ITsSbResourceNotification_INTERFACE_DEFINED__

/* interface ITsSbResourceNotification */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbResourceNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("65D3E85A-C39B-11DC-B92D-3CD255D89593")
    ITsSbResourceNotification : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifySessionChange( 
            /* [in] */ TSSESSION_STATE changeType,
            /* [in] */ __RPC__in_opt ITsSbSession *pSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyTargetChange( 
            /* [in] */ DWORD TargetChangeType,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyClientConnectionStateChange( 
            /* [in] */ CONNECTION_CHANGE_NOTIFICATION ChangeType,
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbResourceNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbResourceNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbResourceNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbResourceNotification * This);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotification, NotifySessionChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifySessionChange )( 
            __RPC__in ITsSbResourceNotification * This,
            /* [in] */ TSSESSION_STATE changeType,
            /* [in] */ __RPC__in_opt ITsSbSession *pSession);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotification, NotifyTargetChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyTargetChange )( 
            __RPC__in ITsSbResourceNotification * This,
            /* [in] */ DWORD TargetChangeType,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotification, NotifyClientConnectionStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyClientConnectionStateChange )( 
            __RPC__in ITsSbResourceNotification * This,
            /* [in] */ CONNECTION_CHANGE_NOTIFICATION ChangeType,
            /* [in] */ __RPC__in_opt ITsSbClientConnection *pConnection);
        
        END_INTERFACE
    } ITsSbResourceNotificationVtbl;

    interface ITsSbResourceNotification
    {
        CONST_VTBL struct ITsSbResourceNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbResourceNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbResourceNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbResourceNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbResourceNotification_NotifySessionChange(This,changeType,pSession)	\
    ( (This)->lpVtbl -> NotifySessionChange(This,changeType,pSession) ) 

#define ITsSbResourceNotification_NotifyTargetChange(This,TargetChangeType,pTarget)	\
    ( (This)->lpVtbl -> NotifyTargetChange(This,TargetChangeType,pTarget) ) 

#define ITsSbResourceNotification_NotifyClientConnectionStateChange(This,ChangeType,pConnection)	\
    ( (This)->lpVtbl -> NotifyClientConnectionStateChange(This,ChangeType,pConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbResourceNotification_INTERFACE_DEFINED__ */


#ifndef __ITsSbResourceNotificationEx_INTERFACE_DEFINED__
#define __ITsSbResourceNotificationEx_INTERFACE_DEFINED__

/* interface ITsSbResourceNotificationEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbResourceNotificationEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A8A47FDE-CA91-44D2-B897-3AA28A43B2B7")
    ITsSbResourceNotificationEx : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifySessionChangeEx( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR domain,
            /* [in] */ DWORD sessionId,
            /* [in] */ TSSESSION_STATE sessionState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyTargetChangeEx( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ DWORD targetChangeType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NotifyClientConnectionStateChangeEx( 
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR domain,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ CONNECTION_CHANGE_NOTIFICATION connectionChangeType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbResourceNotificationExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbResourceNotificationEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbResourceNotificationEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbResourceNotificationEx * This);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotificationEx, NotifySessionChangeEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifySessionChangeEx )( 
            __RPC__in ITsSbResourceNotificationEx * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR domain,
            /* [in] */ DWORD sessionId,
            /* [in] */ TSSESSION_STATE sessionState);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotificationEx, NotifyTargetChangeEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyTargetChangeEx )( 
            __RPC__in ITsSbResourceNotificationEx * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ DWORD targetChangeType);
        
        DECLSPEC_XFGVIRT(ITsSbResourceNotificationEx, NotifyClientConnectionStateChangeEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NotifyClientConnectionStateChangeEx )( 
            __RPC__in ITsSbResourceNotificationEx * This,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR domain,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ CONNECTION_CHANGE_NOTIFICATION connectionChangeType);
        
        END_INTERFACE
    } ITsSbResourceNotificationExVtbl;

    interface ITsSbResourceNotificationEx
    {
        CONST_VTBL struct ITsSbResourceNotificationExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbResourceNotificationEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbResourceNotificationEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbResourceNotificationEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbResourceNotificationEx_NotifySessionChangeEx(This,targetName,userName,domain,sessionId,sessionState)	\
    ( (This)->lpVtbl -> NotifySessionChangeEx(This,targetName,userName,domain,sessionId,sessionState) ) 

#define ITsSbResourceNotificationEx_NotifyTargetChangeEx(This,targetName,targetChangeType)	\
    ( (This)->lpVtbl -> NotifyTargetChangeEx(This,targetName,targetChangeType) ) 

#define ITsSbResourceNotificationEx_NotifyClientConnectionStateChangeEx(This,userName,domain,initialProgram,poolName,targetName,connectionChangeType)	\
    ( (This)->lpVtbl -> NotifyClientConnectionStateChangeEx(This,userName,domain,initialProgram,poolName,targetName,connectionChangeType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbResourceNotificationEx_INTERFACE_DEFINED__ */


#ifndef __ITsSbTaskInfo_INTERFACE_DEFINED__
#define __ITsSbTaskInfo_INTERFACE_DEFINED__

/* interface ITsSbTaskInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbTaskInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("523D1083-89BE-48DD-99EA-04E82FFA7265")
    ITsSbTaskInfo : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out FILETIME *pStartTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EndTime( 
            /* [retval][out] */ __RPC__out FILETIME *pEndTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Deadline( 
            /* [retval][out] */ __RPC__out FILETIME *pDeadline) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Identifier( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pIdentifier) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pLabel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Context( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pContext) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Plugin( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPlugin) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out RDV_TASK_STATUS *pStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbTaskInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbTaskInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbTaskInfo * This);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_TargetId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetId )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_StartTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__out FILETIME *pStartTime);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_EndTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndTime )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__out FILETIME *pEndTime);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Deadline)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Deadline )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__out FILETIME *pDeadline);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Identifier)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Identifier )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pIdentifier);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Label)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pLabel);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Context)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Context )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pContext);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Plugin)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Plugin )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPlugin);
        
        DECLSPEC_XFGVIRT(ITsSbTaskInfo, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in ITsSbTaskInfo * This,
            /* [retval][out] */ __RPC__out RDV_TASK_STATUS *pStatus);
        
        END_INTERFACE
    } ITsSbTaskInfoVtbl;

    interface ITsSbTaskInfo
    {
        CONST_VTBL struct ITsSbTaskInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbTaskInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbTaskInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbTaskInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbTaskInfo_get_TargetId(This,pName)	\
    ( (This)->lpVtbl -> get_TargetId(This,pName) ) 

#define ITsSbTaskInfo_get_StartTime(This,pStartTime)	\
    ( (This)->lpVtbl -> get_StartTime(This,pStartTime) ) 

#define ITsSbTaskInfo_get_EndTime(This,pEndTime)	\
    ( (This)->lpVtbl -> get_EndTime(This,pEndTime) ) 

#define ITsSbTaskInfo_get_Deadline(This,pDeadline)	\
    ( (This)->lpVtbl -> get_Deadline(This,pDeadline) ) 

#define ITsSbTaskInfo_get_Identifier(This,pIdentifier)	\
    ( (This)->lpVtbl -> get_Identifier(This,pIdentifier) ) 

#define ITsSbTaskInfo_get_Label(This,pLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pLabel) ) 

#define ITsSbTaskInfo_get_Context(This,pContext)	\
    ( (This)->lpVtbl -> get_Context(This,pContext) ) 

#define ITsSbTaskInfo_get_Plugin(This,pPlugin)	\
    ( (This)->lpVtbl -> get_Plugin(This,pPlugin) ) 

#define ITsSbTaskInfo_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbTaskInfo_INTERFACE_DEFINED__ */


#ifndef __ITsSbTaskPlugin_INTERFACE_DEFINED__
#define __ITsSbTaskPlugin_INTERFACE_DEFINED__

/* interface ITsSbTaskPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbTaskPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA22EF0F-8705-41BE-93BC-44BDBCF1C9C4")
    ITsSbTaskPlugin : public ITsSbPlugin
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeTaskPlugin( 
            /* [in] */ __RPC__in_opt ITsSbTaskPluginNotifySink *pITsSbTaskPluginNotifySink) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTaskQueue( 
            /* [in] */ __RPC__in BSTR pszHostName,
            /* [in] */ DWORD SbTaskInfoSize,
            /* [size_is][in] */ __RPC__in_ecount_full(SbTaskInfoSize) ITsSbTaskInfo *pITsSbTaskInfo[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbTaskPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbTaskPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbTaskPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbTaskPlugin * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbTaskPlugin * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbTaskPlugin * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPlugin, InitializeTaskPlugin)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeTaskPlugin )( 
            __RPC__in ITsSbTaskPlugin * This,
            /* [in] */ __RPC__in_opt ITsSbTaskPluginNotifySink *pITsSbTaskPluginNotifySink);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPlugin, SetTaskQueue)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTaskQueue )( 
            __RPC__in ITsSbTaskPlugin * This,
            /* [in] */ __RPC__in BSTR pszHostName,
            /* [in] */ DWORD SbTaskInfoSize,
            /* [size_is][in] */ __RPC__in_ecount_full(SbTaskInfoSize) ITsSbTaskInfo *pITsSbTaskInfo[  ]);
        
        END_INTERFACE
    } ITsSbTaskPluginVtbl;

    interface ITsSbTaskPlugin
    {
        CONST_VTBL struct ITsSbTaskPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbTaskPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbTaskPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbTaskPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbTaskPlugin_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbTaskPlugin_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#define ITsSbTaskPlugin_InitializeTaskPlugin(This,pITsSbTaskPluginNotifySink)	\
    ( (This)->lpVtbl -> InitializeTaskPlugin(This,pITsSbTaskPluginNotifySink) ) 

#define ITsSbTaskPlugin_SetTaskQueue(This,pszHostName,SbTaskInfoSize,pITsSbTaskInfo)	\
    ( (This)->lpVtbl -> SetTaskQueue(This,pszHostName,SbTaskInfoSize,pITsSbTaskInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbTaskPlugin_INTERFACE_DEFINED__ */


#ifndef __ITsSbPropertySet_INTERFACE_DEFINED__
#define __ITsSbPropertySet_INTERFACE_DEFINED__

/* interface ITsSbPropertySet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5c025171-bb1e-4baf-a212-6d5e9774b33b")
    ITsSbPropertySet : public IPropertyBag
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPropertySet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPropertySet * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ITsSbPropertySet * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in ITsSbPropertySet * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } ITsSbPropertySetVtbl;

    interface ITsSbPropertySet
    {
        CONST_VTBL struct ITsSbPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPropertySet_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define ITsSbPropertySet_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPropertySet_INTERFACE_DEFINED__ */


#ifndef __ITsSbPluginPropertySet_INTERFACE_DEFINED__
#define __ITsSbPluginPropertySet_INTERFACE_DEFINED__

/* interface ITsSbPluginPropertySet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPluginPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95006e34-7eff-4b6c-bb40-49a4fda7cea6")
    ITsSbPluginPropertySet : public ITsSbPropertySet
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPluginPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPluginPropertySet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPluginPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPluginPropertySet * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ITsSbPluginPropertySet * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in ITsSbPluginPropertySet * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } ITsSbPluginPropertySetVtbl;

    interface ITsSbPluginPropertySet
    {
        CONST_VTBL struct ITsSbPluginPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPluginPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPluginPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPluginPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPluginPropertySet_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define ITsSbPluginPropertySet_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 



#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPluginPropertySet_INTERFACE_DEFINED__ */


#ifndef __ITsSbClientConnectionPropertySet_INTERFACE_DEFINED__
#define __ITsSbClientConnectionPropertySet_INTERFACE_DEFINED__

/* interface ITsSbClientConnectionPropertySet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbClientConnectionPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E51995B0-46D6-11DD-AA21-CEDC55D89593")
    ITsSbClientConnectionPropertySet : public ITsSbPropertySet
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbClientConnectionPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbClientConnectionPropertySet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbClientConnectionPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbClientConnectionPropertySet * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ITsSbClientConnectionPropertySet * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in ITsSbClientConnectionPropertySet * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } ITsSbClientConnectionPropertySetVtbl;

    interface ITsSbClientConnectionPropertySet
    {
        CONST_VTBL struct ITsSbClientConnectionPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbClientConnectionPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbClientConnectionPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbClientConnectionPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbClientConnectionPropertySet_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define ITsSbClientConnectionPropertySet_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 



#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbClientConnectionPropertySet_INTERFACE_DEFINED__ */


#ifndef __ITsSbTargetPropertySet_INTERFACE_DEFINED__
#define __ITsSbTargetPropertySet_INTERFACE_DEFINED__

/* interface ITsSbTargetPropertySet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbTargetPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7bda5d6-994c-4e11-a079-2763b61830ac")
    ITsSbTargetPropertySet : public ITsSbPropertySet
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbTargetPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbTargetPropertySet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbTargetPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbTargetPropertySet * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ITsSbTargetPropertySet * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in ITsSbTargetPropertySet * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } ITsSbTargetPropertySetVtbl;

    interface ITsSbTargetPropertySet
    {
        CONST_VTBL struct ITsSbTargetPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbTargetPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbTargetPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbTargetPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbTargetPropertySet_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define ITsSbTargetPropertySet_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 



#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbTargetPropertySet_INTERFACE_DEFINED__ */


#ifndef __ITsSbEnvironmentPropertySet_INTERFACE_DEFINED__
#define __ITsSbEnvironmentPropertySet_INTERFACE_DEFINED__

/* interface ITsSbEnvironmentPropertySet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbEnvironmentPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D0D1BF7E-7ACF-11DD-A243-E51156D89593")
    ITsSbEnvironmentPropertySet : public ITsSbPropertySet
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbEnvironmentPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbEnvironmentPropertySet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbEnvironmentPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbEnvironmentPropertySet * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ITsSbEnvironmentPropertySet * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in ITsSbEnvironmentPropertySet * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } ITsSbEnvironmentPropertySetVtbl;

    interface ITsSbEnvironmentPropertySet
    {
        CONST_VTBL struct ITsSbEnvironmentPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbEnvironmentPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbEnvironmentPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbEnvironmentPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbEnvironmentPropertySet_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define ITsSbEnvironmentPropertySet_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 



#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbEnvironmentPropertySet_INTERFACE_DEFINED__ */


#ifndef __ITsSbBaseNotifySink_INTERFACE_DEFINED__
#define __ITsSbBaseNotifySink_INTERFACE_DEFINED__

/* interface ITsSbBaseNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbBaseNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("808a6537-1282-4989-9e09-f43938b71722")
    ITsSbBaseNotifySink : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnError( 
            /* [in] */ HRESULT hrError) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReportStatus( 
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbBaseNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbBaseNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbBaseNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbBaseNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbBaseNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbBaseNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        END_INTERFACE
    } ITsSbBaseNotifySinkVtbl;

    interface ITsSbBaseNotifySink
    {
        CONST_VTBL struct ITsSbBaseNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbBaseNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbBaseNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbBaseNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbBaseNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbBaseNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbBaseNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbPluginNotifySink_INTERFACE_DEFINED__
#define __ITsSbPluginNotifySink_INTERFACE_DEFINED__

/* interface ITsSbPluginNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPluginNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("44dfe30b-c3be-40f5-bf82-7a95bb795adf")
    ITsSbPluginNotifySink : public ITsSbBaseNotifySink
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInitialized( 
            /* [in] */ HRESULT hr) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnTerminated( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPluginNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPluginNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbPluginNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbPluginNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        DECLSPEC_XFGVIRT(ITsSbPluginNotifySink, OnInitialized)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInitialized )( 
            __RPC__in ITsSbPluginNotifySink * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbPluginNotifySink, OnTerminated)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnTerminated )( 
            __RPC__in ITsSbPluginNotifySink * This);
        
        END_INTERFACE
    } ITsSbPluginNotifySinkVtbl;

    interface ITsSbPluginNotifySink
    {
        CONST_VTBL struct ITsSbPluginNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPluginNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPluginNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPluginNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPluginNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbPluginNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 


#define ITsSbPluginNotifySink_OnInitialized(This,hr)	\
    ( (This)->lpVtbl -> OnInitialized(This,hr) ) 

#define ITsSbPluginNotifySink_OnTerminated(This)	\
    ( (This)->lpVtbl -> OnTerminated(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPluginNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbLoadBalancingNotifySink_INTERFACE_DEFINED__
#define __ITsSbLoadBalancingNotifySink_INTERFACE_DEFINED__

/* interface ITsSbLoadBalancingNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbLoadBalancingNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5F8A8297-3244-4E6A-958A-27C822C1E141")
    ITsSbLoadBalancingNotifySink : public ITsSbBaseNotifySink
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnGetMostSuitableTarget( 
            /* [in] */ __RPC__in_opt ITsSbLoadBalanceResult *pLBResult,
            /* [in] */ BOOL fIsNewConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbLoadBalancingNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        DECLSPEC_XFGVIRT(ITsSbLoadBalancingNotifySink, OnGetMostSuitableTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnGetMostSuitableTarget )( 
            __RPC__in ITsSbLoadBalancingNotifySink * This,
            /* [in] */ __RPC__in_opt ITsSbLoadBalanceResult *pLBResult,
            /* [in] */ BOOL fIsNewConnection);
        
        END_INTERFACE
    } ITsSbLoadBalancingNotifySinkVtbl;

    interface ITsSbLoadBalancingNotifySink
    {
        CONST_VTBL struct ITsSbLoadBalancingNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbLoadBalancingNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbLoadBalancingNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbLoadBalancingNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbLoadBalancingNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbLoadBalancingNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 


#define ITsSbLoadBalancingNotifySink_OnGetMostSuitableTarget(This,pLBResult,fIsNewConnection)	\
    ( (This)->lpVtbl -> OnGetMostSuitableTarget(This,pLBResult,fIsNewConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbLoadBalancingNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbPlacementNotifySink_INTERFACE_DEFINED__
#define __ITsSbPlacementNotifySink_INTERFACE_DEFINED__

/* interface ITsSbPlacementNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbPlacementNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68A0C487-2B4F-46C2-94A1-6CE685183634")
    ITsSbPlacementNotifySink : public ITsSbBaseNotifySink
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnQueryEnvironmentCompleted( 
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbPlacementNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbPlacementNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbPlacementNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbPlacementNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbPlacementNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbPlacementNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        DECLSPEC_XFGVIRT(ITsSbPlacementNotifySink, OnQueryEnvironmentCompleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnQueryEnvironmentCompleted )( 
            __RPC__in ITsSbPlacementNotifySink * This,
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment);
        
        END_INTERFACE
    } ITsSbPlacementNotifySinkVtbl;

    interface ITsSbPlacementNotifySink
    {
        CONST_VTBL struct ITsSbPlacementNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbPlacementNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbPlacementNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbPlacementNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbPlacementNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbPlacementNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 


#define ITsSbPlacementNotifySink_OnQueryEnvironmentCompleted(This,pEnvironment)	\
    ( (This)->lpVtbl -> OnQueryEnvironmentCompleted(This,pEnvironment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbPlacementNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbOrchestrationNotifySink_INTERFACE_DEFINED__
#define __ITsSbOrchestrationNotifySink_INTERFACE_DEFINED__

/* interface ITsSbOrchestrationNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbOrchestrationNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36c37d61-926b-442f-bca5-118c6d50dcf2")
    ITsSbOrchestrationNotifySink : public ITsSbBaseNotifySink
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReadyToConnect( 
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbOrchestrationNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbOrchestrationNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbOrchestrationNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbOrchestrationNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbOrchestrationNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbOrchestrationNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        DECLSPEC_XFGVIRT(ITsSbOrchestrationNotifySink, OnReadyToConnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReadyToConnect )( 
            __RPC__in ITsSbOrchestrationNotifySink * This,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget);
        
        END_INTERFACE
    } ITsSbOrchestrationNotifySinkVtbl;

    interface ITsSbOrchestrationNotifySink
    {
        CONST_VTBL struct ITsSbOrchestrationNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbOrchestrationNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbOrchestrationNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbOrchestrationNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbOrchestrationNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbOrchestrationNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 


#define ITsSbOrchestrationNotifySink_OnReadyToConnect(This,pTarget)	\
    ( (This)->lpVtbl -> OnReadyToConnect(This,pTarget) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbOrchestrationNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbTaskPluginNotifySink_INTERFACE_DEFINED__
#define __ITsSbTaskPluginNotifySink_INTERFACE_DEFINED__

/* interface ITsSbTaskPluginNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbTaskPluginNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6AAF899E-C2EC-45EE-AA37-45E60895261A")
    ITsSbTaskPluginNotifySink : public ITsSbBaseNotifySink
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetTaskTime( 
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ FILETIME TaskStartTime,
            /* [in] */ FILETIME TaskEndTime,
            /* [in] */ FILETIME TaskDeadline,
            /* [in] */ __RPC__in BSTR szTaskLabel,
            /* [in] */ __RPC__in BSTR szTaskIdentifier,
            /* [in] */ __RPC__in BSTR szTaskPlugin,
            /* [in] */ DWORD dwTaskStatus,
            /* [in] */ __RPC__in SAFEARRAY * saContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDeleteTaskTime( 
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ __RPC__in BSTR szTaskIdentifier) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnUpdateTaskStatus( 
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ __RPC__in BSTR TaskIdentifier,
            /* [in] */ RDV_TASK_STATUS TaskStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReportTasks( 
            /* [in] */ __RPC__in BSTR szHostName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbTaskPluginNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbTaskPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbTaskPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(ITsSbBaseNotifySink, OnReportStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportStatus )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ CLIENT_MESSAGE_TYPE messageType,
            /* [in] */ DWORD messageID);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPluginNotifySink, OnSetTaskTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetTaskTime )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ FILETIME TaskStartTime,
            /* [in] */ FILETIME TaskEndTime,
            /* [in] */ FILETIME TaskDeadline,
            /* [in] */ __RPC__in BSTR szTaskLabel,
            /* [in] */ __RPC__in BSTR szTaskIdentifier,
            /* [in] */ __RPC__in BSTR szTaskPlugin,
            /* [in] */ DWORD dwTaskStatus,
            /* [in] */ __RPC__in SAFEARRAY * saContext);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPluginNotifySink, OnDeleteTaskTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDeleteTaskTime )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ __RPC__in BSTR szTaskIdentifier);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPluginNotifySink, OnUpdateTaskStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnUpdateTaskStatus )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ __RPC__in BSTR szTargetName,
            /* [in] */ __RPC__in BSTR TaskIdentifier,
            /* [in] */ RDV_TASK_STATUS TaskStatus);
        
        DECLSPEC_XFGVIRT(ITsSbTaskPluginNotifySink, OnReportTasks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReportTasks )( 
            __RPC__in ITsSbTaskPluginNotifySink * This,
            /* [in] */ __RPC__in BSTR szHostName);
        
        END_INTERFACE
    } ITsSbTaskPluginNotifySinkVtbl;

    interface ITsSbTaskPluginNotifySink
    {
        CONST_VTBL struct ITsSbTaskPluginNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbTaskPluginNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbTaskPluginNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbTaskPluginNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbTaskPluginNotifySink_OnError(This,hrError)	\
    ( (This)->lpVtbl -> OnError(This,hrError) ) 

#define ITsSbTaskPluginNotifySink_OnReportStatus(This,messageType,messageID)	\
    ( (This)->lpVtbl -> OnReportStatus(This,messageType,messageID) ) 


#define ITsSbTaskPluginNotifySink_OnSetTaskTime(This,szTargetName,TaskStartTime,TaskEndTime,TaskDeadline,szTaskLabel,szTaskIdentifier,szTaskPlugin,dwTaskStatus,saContext)	\
    ( (This)->lpVtbl -> OnSetTaskTime(This,szTargetName,TaskStartTime,TaskEndTime,TaskDeadline,szTaskLabel,szTaskIdentifier,szTaskPlugin,dwTaskStatus,saContext) ) 

#define ITsSbTaskPluginNotifySink_OnDeleteTaskTime(This,szTargetName,szTaskIdentifier)	\
    ( (This)->lpVtbl -> OnDeleteTaskTime(This,szTargetName,szTaskIdentifier) ) 

#define ITsSbTaskPluginNotifySink_OnUpdateTaskStatus(This,szTargetName,TaskIdentifier,TaskStatus)	\
    ( (This)->lpVtbl -> OnUpdateTaskStatus(This,szTargetName,TaskIdentifier,TaskStatus) ) 

#define ITsSbTaskPluginNotifySink_OnReportTasks(This,szHostName)	\
    ( (This)->lpVtbl -> OnReportTasks(This,szHostName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbTaskPluginNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbClientConnection_INTERFACE_DEFINED__
#define __ITsSbClientConnection_INTERFACE_DEFINED__

/* interface ITsSbClientConnection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbClientConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18857499-AD61-4B1B-B7DF-CBCD41FB8338")
    ITsSbClientConnection : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Domain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitialProgram( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LoadBalanceResult( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbLoadBalanceResult **ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FarmName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][local] */ HRESULT STDMETHODCALLTYPE PutContext( 
            /* [annotation] */ 
            _In_  BSTR contextId,
            /* [annotation] */ 
            _In_  VARIANT context,
            /* [annotation] */ 
            _Out_opt_  VARIANT *existingContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetContext( 
            /* [in] */ __RPC__in BSTR contextId,
            /* [retval][out] */ __RPC__out VARIANT *context) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Environment( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectionError( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SamUserAccount( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClientConnectionPropertySet( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbClientConnectionPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsFirstAssignment( 
            /* [retval][out] */ __RPC__out BOOL *ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RdFarmType( 
            /* [retval][out] */ __RPC__out RD_FARM_TYPE *pRdFarmType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserSidString( 
            /* [retval][out] */ __RPC__deref_out_opt LPTSTR *pszUserSidString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDisconnectedSession( 
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbClientConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbClientConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbClientConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbClientConnection * This);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_UserName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_InitialProgram)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitialProgram )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_LoadBalanceResult)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoadBalanceResult )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbLoadBalanceResult **ppVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_FarmName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FarmName )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, PutContext)
        /* [helpstring][id][local] */ HRESULT ( STDMETHODCALLTYPE *PutContext )( 
            ITsSbClientConnection * This,
            /* [annotation] */ 
            _In_  BSTR contextId,
            /* [annotation] */ 
            _In_  VARIANT context,
            /* [annotation] */ 
            _Out_opt_  VARIANT *existingContext);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, GetContext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITsSbClientConnection * This,
            /* [in] */ __RPC__in BSTR contextId,
            /* [retval][out] */ __RPC__out VARIANT *context);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_Environment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Environment )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_ConnectionError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionError )( 
            __RPC__in ITsSbClientConnection * This);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_SamUserAccount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SamUserAccount )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_ClientConnectionPropertySet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientConnectionPropertySet )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbClientConnectionPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_IsFirstAssignment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstAssignment )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__out BOOL *ppVal);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_RdFarmType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RdFarmType )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__out RD_FARM_TYPE *pRdFarmType);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, get_UserSidString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSidString )( 
            __RPC__in ITsSbClientConnection * This,
            /* [retval][out] */ __RPC__deref_out_opt LPTSTR *pszUserSidString);
        
        DECLSPEC_XFGVIRT(ITsSbClientConnection, GetDisconnectedSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDisconnectedSession )( 
            __RPC__in ITsSbClientConnection * This,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession);
        
        END_INTERFACE
    } ITsSbClientConnectionVtbl;

    interface ITsSbClientConnection
    {
        CONST_VTBL struct ITsSbClientConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbClientConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbClientConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbClientConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbClientConnection_get_UserName(This,pVal)	\
    ( (This)->lpVtbl -> get_UserName(This,pVal) ) 

#define ITsSbClientConnection_get_Domain(This,pVal)	\
    ( (This)->lpVtbl -> get_Domain(This,pVal) ) 

#define ITsSbClientConnection_get_InitialProgram(This,pVal)	\
    ( (This)->lpVtbl -> get_InitialProgram(This,pVal) ) 

#define ITsSbClientConnection_get_LoadBalanceResult(This,ppVal)	\
    ( (This)->lpVtbl -> get_LoadBalanceResult(This,ppVal) ) 

#define ITsSbClientConnection_get_FarmName(This,pVal)	\
    ( (This)->lpVtbl -> get_FarmName(This,pVal) ) 

#define ITsSbClientConnection_PutContext(This,contextId,context,existingContext)	\
    ( (This)->lpVtbl -> PutContext(This,contextId,context,existingContext) ) 

#define ITsSbClientConnection_GetContext(This,contextId,context)	\
    ( (This)->lpVtbl -> GetContext(This,contextId,context) ) 

#define ITsSbClientConnection_get_Environment(This,ppEnvironment)	\
    ( (This)->lpVtbl -> get_Environment(This,ppEnvironment) ) 

#define ITsSbClientConnection_get_ConnectionError(This)	\
    ( (This)->lpVtbl -> get_ConnectionError(This) ) 

#define ITsSbClientConnection_get_SamUserAccount(This,pVal)	\
    ( (This)->lpVtbl -> get_SamUserAccount(This,pVal) ) 

#define ITsSbClientConnection_get_ClientConnectionPropertySet(This,ppPropertySet)	\
    ( (This)->lpVtbl -> get_ClientConnectionPropertySet(This,ppPropertySet) ) 

#define ITsSbClientConnection_get_IsFirstAssignment(This,ppVal)	\
    ( (This)->lpVtbl -> get_IsFirstAssignment(This,ppVal) ) 

#define ITsSbClientConnection_get_RdFarmType(This,pRdFarmType)	\
    ( (This)->lpVtbl -> get_RdFarmType(This,pRdFarmType) ) 

#define ITsSbClientConnection_get_UserSidString(This,pszUserSidString)	\
    ( (This)->lpVtbl -> get_UserSidString(This,pszUserSidString) ) 

#define ITsSbClientConnection_GetDisconnectedSession(This,ppSession)	\
    ( (This)->lpVtbl -> GetDisconnectedSession(This,ppSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbClientConnection_INTERFACE_DEFINED__ */


#ifndef __ITsSbProvider_INTERFACE_DEFINED__
#define __ITsSbProvider_INTERFACE_DEFINED__

/* interface ITsSbProvider */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("87A4098F-6D7B-44DD-BC17-8CE44E370D52")
    ITsSbProvider : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTargetObject( 
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateLoadBalanceResultObject( 
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbLoadBalanceResult **ppLBResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSessionObject( 
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR UserName,
            /* [in] */ __RPC__in BSTR Domain,
            /* [in] */ DWORD SessionId,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreatePluginPropertySet( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbPluginPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTargetPropertySetObject( 
            /* [out] */ __RPC__deref_out_opt ITsSbTargetPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateEnvironmentObject( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DWORD ServerWeight,
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetResourcePluginStore( 
            /* [out] */ __RPC__deref_out_opt ITsSbResourcePluginStore **ppStore) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFilterPluginStore( 
            /* [out] */ __RPC__deref_out_opt ITsSbFilterPluginStore **ppStore) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterForNotification( 
            /* [in] */ DWORD notificationType,
            /* [in] */ __RPC__in BSTR ResourceToMonitor,
            /* [in] */ __RPC__in_opt ITsSbResourceNotification *pPluginNotification) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnRegisterForNotification( 
            /* [in] */ DWORD notificationType,
            /* [in] */ __RPC__in BSTR ResourceToMonitor) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInstanceOfGlobalStore( 
            /* [out] */ __RPC__deref_out_opt ITsSbGlobalStore **ppGlobalStore) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateEnvironmentPropertySetObject( 
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironmentPropertySet **ppPropertySet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbProvider * This);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateTargetObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTargetObject )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateLoadBalanceResultObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateLoadBalanceResultObject )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbLoadBalanceResult **ppLBResult);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateSessionObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSessionObject )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR UserName,
            /* [in] */ __RPC__in BSTR Domain,
            /* [in] */ DWORD SessionId,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreatePluginPropertySet)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePluginPropertySet )( 
            __RPC__in ITsSbProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbPluginPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateTargetPropertySetObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTargetPropertySetObject )( 
            __RPC__in ITsSbProvider * This,
            /* [out] */ __RPC__deref_out_opt ITsSbTargetPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateEnvironmentObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateEnvironmentObject )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ DWORD ServerWeight,
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, GetResourcePluginStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetResourcePluginStore )( 
            __RPC__in ITsSbProvider * This,
            /* [out] */ __RPC__deref_out_opt ITsSbResourcePluginStore **ppStore);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, GetFilterPluginStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFilterPluginStore )( 
            __RPC__in ITsSbProvider * This,
            /* [out] */ __RPC__deref_out_opt ITsSbFilterPluginStore **ppStore);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, RegisterForNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterForNotification )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ DWORD notificationType,
            /* [in] */ __RPC__in BSTR ResourceToMonitor,
            /* [in] */ __RPC__in_opt ITsSbResourceNotification *pPluginNotification);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, UnRegisterForNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnRegisterForNotification )( 
            __RPC__in ITsSbProvider * This,
            /* [in] */ DWORD notificationType,
            /* [in] */ __RPC__in BSTR ResourceToMonitor);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, GetInstanceOfGlobalStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInstanceOfGlobalStore )( 
            __RPC__in ITsSbProvider * This,
            /* [out] */ __RPC__deref_out_opt ITsSbGlobalStore **ppGlobalStore);
        
        DECLSPEC_XFGVIRT(ITsSbProvider, CreateEnvironmentPropertySetObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateEnvironmentPropertySetObject )( 
            __RPC__in ITsSbProvider * This,
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironmentPropertySet **ppPropertySet);
        
        END_INTERFACE
    } ITsSbProviderVtbl;

    interface ITsSbProvider
    {
        CONST_VTBL struct ITsSbProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbProvider_CreateTargetObject(This,TargetName,EnvironmentName,ppTarget)	\
    ( (This)->lpVtbl -> CreateTargetObject(This,TargetName,EnvironmentName,ppTarget) ) 

#define ITsSbProvider_CreateLoadBalanceResultObject(This,TargetName,ppLBResult)	\
    ( (This)->lpVtbl -> CreateLoadBalanceResultObject(This,TargetName,ppLBResult) ) 

#define ITsSbProvider_CreateSessionObject(This,TargetName,UserName,Domain,SessionId,ppSession)	\
    ( (This)->lpVtbl -> CreateSessionObject(This,TargetName,UserName,Domain,SessionId,ppSession) ) 

#define ITsSbProvider_CreatePluginPropertySet(This,ppPropertySet)	\
    ( (This)->lpVtbl -> CreatePluginPropertySet(This,ppPropertySet) ) 

#define ITsSbProvider_CreateTargetPropertySetObject(This,ppPropertySet)	\
    ( (This)->lpVtbl -> CreateTargetPropertySetObject(This,ppPropertySet) ) 

#define ITsSbProvider_CreateEnvironmentObject(This,Name,ServerWeight,ppEnvironment)	\
    ( (This)->lpVtbl -> CreateEnvironmentObject(This,Name,ServerWeight,ppEnvironment) ) 

#define ITsSbProvider_GetResourcePluginStore(This,ppStore)	\
    ( (This)->lpVtbl -> GetResourcePluginStore(This,ppStore) ) 

#define ITsSbProvider_GetFilterPluginStore(This,ppStore)	\
    ( (This)->lpVtbl -> GetFilterPluginStore(This,ppStore) ) 

#define ITsSbProvider_RegisterForNotification(This,notificationType,ResourceToMonitor,pPluginNotification)	\
    ( (This)->lpVtbl -> RegisterForNotification(This,notificationType,ResourceToMonitor,pPluginNotification) ) 

#define ITsSbProvider_UnRegisterForNotification(This,notificationType,ResourceToMonitor)	\
    ( (This)->lpVtbl -> UnRegisterForNotification(This,notificationType,ResourceToMonitor) ) 

#define ITsSbProvider_GetInstanceOfGlobalStore(This,ppGlobalStore)	\
    ( (This)->lpVtbl -> GetInstanceOfGlobalStore(This,ppGlobalStore) ) 

#define ITsSbProvider_CreateEnvironmentPropertySetObject(This,ppPropertySet)	\
    ( (This)->lpVtbl -> CreateEnvironmentPropertySetObject(This,ppPropertySet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbProvider_INTERFACE_DEFINED__ */


#ifndef __ITsSbResourcePluginStore_INTERFACE_DEFINED__
#define __ITsSbResourcePluginStore_INTERFACE_DEFINED__

/* interface ITsSbResourcePluginStore */
/* [unique][helpstring][uuid][object] */ 



EXTERN_C const IID IID_ITsSbResourcePluginStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5C38F65F-BCF1-4036-A6BF-9E3CCCAE0B63")
    ITsSbResourcePluginStore : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryTarget( 
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QuerySessionBySessionId( 
            /* [in] */ DWORD dwSessionId,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddTargetToStore( 
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddSessionToStore( 
            /* [in] */ __RPC__in_opt ITsSbSession *pSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddEnvironmentToStore( 
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveEnvironmentFromStore( 
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [optional][in] */ BOOL bIgnoreOwner) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateFarms( 
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryEnvironment( 
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateEnvironments( 
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbEnvironment **pVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveTarget( 
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget,
            /* [in] */ BOOL bForceWrite) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveEnvironment( 
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment,
            /* [in] */ BOOL bForceWrite) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveSession( 
            /* [in] */ __RPC__in_opt ITsSbSession *pSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTargetProperty( 
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEnvironmentProperty( 
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTargetState( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ TARGET_STATE newState,
            /* [out] */ __RPC__out TARGET_STATE *pOldState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSessionState( 
            /* [in] */ __RPC__in_opt ITsSbSession *sbSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateTargets( 
            /* [in] */ __RPC__in BSTR FarmName,
            /* [in] */ __RPC__in BSTR EnvName,
            /* [in] */ TS_SB_SORT_BY sortByFieldId,
            /* [in] */ __RPC__in BSTR sortyByPropName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [annotation][size_is][out] */ 
            __RPC__deref_out_ecount_full(*pdwCount)  ITsSbTarget **pVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateSessions( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in TSSESSION_STATE *pSessionState,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [annotation][size_is][out] */ 
            __RPC__deref_out_ecount_full(*pdwCount)  ITsSbSession **ppVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFarmProperty( 
            /* [in] */ __RPC__in BSTR farmName,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ __RPC__in VARIANT *pVarValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteTarget( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR hostName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTargetPropertyWithVersionCheck( 
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEnvironmentPropertyWithVersionCheck( 
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AcquireTargetLock( 
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseTargetLock( 
            /* [in] */ __RPC__in_opt IUnknown *pContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TestAndSetServerState( 
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [in] */ TARGET_STATE NewState,
            /* [in] */ TARGET_STATE TestState,
            /* [out] */ __RPC__out TARGET_STATE *pInitState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetServerWaitingToStart( 
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR serverName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetServerState( 
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [out] */ __RPC__out TARGET_STATE *pState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetServerDrainMode( 
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [in] */ DWORD DrainMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbResourcePluginStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbResourcePluginStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbResourcePluginStore * This);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, QueryTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryTarget )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, QuerySessionBySessionId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QuerySessionBySessionId )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ DWORD dwSessionId,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, AddTargetToStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTargetToStore )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, AddSessionToStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddSessionToStore )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbSession *pSession);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, AddEnvironmentToStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddEnvironmentToStore )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, RemoveEnvironmentFromStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveEnvironmentFromStore )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [optional][in] */ BOOL bIgnoreOwner);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, EnumerateFarms)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateFarms )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, QueryEnvironment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryEnvironment )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [out] */ __RPC__deref_out_opt ITsSbEnvironment **ppEnvironment);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, EnumerateEnvironments)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateEnvironments )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbEnvironment **pVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SaveTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveTarget )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget,
            /* [in] */ BOOL bForceWrite);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SaveEnvironment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveEnvironment )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment,
            /* [in] */ BOOL bForceWrite);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SaveSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveSession )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbSession *pSession);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetTargetProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTargetProperty )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetEnvironmentProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnvironmentProperty )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR EnvironmentName,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetTargetState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTargetState )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ TARGET_STATE newState,
            /* [out] */ __RPC__out TARGET_STATE *pOldState);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetSessionState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSessionState )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbSession *sbSession);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, EnumerateTargets)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTargets )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [in] */ __RPC__in BSTR EnvName,
            /* [in] */ TS_SB_SORT_BY sortByFieldId,
            /* [in] */ __RPC__in BSTR sortyByPropName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [annotation][size_is][out] */ 
            __RPC__deref_out_ecount_full(*pdwCount)  ITsSbTarget **pVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, EnumerateSessions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateSessions )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in TSSESSION_STATE *pSessionState,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [annotation][size_is][out] */ 
            __RPC__deref_out_ecount_full(*pdwCount)  ITsSbSession **ppVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, GetFarmProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFarmProperty )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR farmName,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ __RPC__in VARIANT *pVarValue);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, DeleteTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteTarget )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR hostName);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetTargetPropertyWithVersionCheck)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTargetPropertyWithVersionCheck )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbTarget *pTarget,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetEnvironmentPropertyWithVersionCheck)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnvironmentPropertyWithVersionCheck )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbEnvironment *pEnvironment,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [in] */ __RPC__in VARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, AcquireTargetLock)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AcquireTargetLock )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppContext);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, ReleaseTargetLock)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseTargetLock )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in_opt IUnknown *pContext);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, TestAndSetServerState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TestAndSetServerState )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [in] */ TARGET_STATE NewState,
            /* [in] */ TARGET_STATE TestState,
            /* [out] */ __RPC__out TARGET_STATE *pInitState);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetServerWaitingToStart)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetServerWaitingToStart )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR serverName);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, GetServerState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetServerState )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR PoolName,
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [out] */ __RPC__out TARGET_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITsSbResourcePluginStore, SetServerDrainMode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetServerDrainMode )( 
            __RPC__in ITsSbResourcePluginStore * This,
            /* [in] */ __RPC__in BSTR ServerFQDN,
            /* [in] */ DWORD DrainMode);
        
        END_INTERFACE
    } ITsSbResourcePluginStoreVtbl;

    interface ITsSbResourcePluginStore
    {
        CONST_VTBL struct ITsSbResourcePluginStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbResourcePluginStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbResourcePluginStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbResourcePluginStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbResourcePluginStore_QueryTarget(This,TargetName,FarmName,ppTarget)	\
    ( (This)->lpVtbl -> QueryTarget(This,TargetName,FarmName,ppTarget) ) 

#define ITsSbResourcePluginStore_QuerySessionBySessionId(This,dwSessionId,TargetName,ppSession)	\
    ( (This)->lpVtbl -> QuerySessionBySessionId(This,dwSessionId,TargetName,ppSession) ) 

#define ITsSbResourcePluginStore_AddTargetToStore(This,pTarget)	\
    ( (This)->lpVtbl -> AddTargetToStore(This,pTarget) ) 

#define ITsSbResourcePluginStore_AddSessionToStore(This,pSession)	\
    ( (This)->lpVtbl -> AddSessionToStore(This,pSession) ) 

#define ITsSbResourcePluginStore_AddEnvironmentToStore(This,pEnvironment)	\
    ( (This)->lpVtbl -> AddEnvironmentToStore(This,pEnvironment) ) 

#define ITsSbResourcePluginStore_RemoveEnvironmentFromStore(This,EnvironmentName,bIgnoreOwner)	\
    ( (This)->lpVtbl -> RemoveEnvironmentFromStore(This,EnvironmentName,bIgnoreOwner) ) 

#define ITsSbResourcePluginStore_EnumerateFarms(This,pdwCount,pVal)	\
    ( (This)->lpVtbl -> EnumerateFarms(This,pdwCount,pVal) ) 

#define ITsSbResourcePluginStore_QueryEnvironment(This,EnvironmentName,ppEnvironment)	\
    ( (This)->lpVtbl -> QueryEnvironment(This,EnvironmentName,ppEnvironment) ) 

#define ITsSbResourcePluginStore_EnumerateEnvironments(This,pdwCount,pVal)	\
    ( (This)->lpVtbl -> EnumerateEnvironments(This,pdwCount,pVal) ) 

#define ITsSbResourcePluginStore_SaveTarget(This,pTarget,bForceWrite)	\
    ( (This)->lpVtbl -> SaveTarget(This,pTarget,bForceWrite) ) 

#define ITsSbResourcePluginStore_SaveEnvironment(This,pEnvironment,bForceWrite)	\
    ( (This)->lpVtbl -> SaveEnvironment(This,pEnvironment,bForceWrite) ) 

#define ITsSbResourcePluginStore_SaveSession(This,pSession)	\
    ( (This)->lpVtbl -> SaveSession(This,pSession) ) 

#define ITsSbResourcePluginStore_SetTargetProperty(This,TargetName,PropertyName,pProperty)	\
    ( (This)->lpVtbl -> SetTargetProperty(This,TargetName,PropertyName,pProperty) ) 

#define ITsSbResourcePluginStore_SetEnvironmentProperty(This,EnvironmentName,PropertyName,pProperty)	\
    ( (This)->lpVtbl -> SetEnvironmentProperty(This,EnvironmentName,PropertyName,pProperty) ) 

#define ITsSbResourcePluginStore_SetTargetState(This,targetName,newState,pOldState)	\
    ( (This)->lpVtbl -> SetTargetState(This,targetName,newState,pOldState) ) 

#define ITsSbResourcePluginStore_SetSessionState(This,sbSession)	\
    ( (This)->lpVtbl -> SetSessionState(This,sbSession) ) 

#define ITsSbResourcePluginStore_EnumerateTargets(This,FarmName,EnvName,sortByFieldId,sortyByPropName,pdwCount,pVal)	\
    ( (This)->lpVtbl -> EnumerateTargets(This,FarmName,EnvName,sortByFieldId,sortyByPropName,pdwCount,pVal) ) 

#define ITsSbResourcePluginStore_EnumerateSessions(This,targetName,userName,userDomain,poolName,initialProgram,pSessionState,pdwCount,ppVal)	\
    ( (This)->lpVtbl -> EnumerateSessions(This,targetName,userName,userDomain,poolName,initialProgram,pSessionState,pdwCount,ppVal) ) 

#define ITsSbResourcePluginStore_GetFarmProperty(This,farmName,propertyName,pVarValue)	\
    ( (This)->lpVtbl -> GetFarmProperty(This,farmName,propertyName,pVarValue) ) 

#define ITsSbResourcePluginStore_DeleteTarget(This,targetName,hostName)	\
    ( (This)->lpVtbl -> DeleteTarget(This,targetName,hostName) ) 

#define ITsSbResourcePluginStore_SetTargetPropertyWithVersionCheck(This,pTarget,PropertyName,pProperty)	\
    ( (This)->lpVtbl -> SetTargetPropertyWithVersionCheck(This,pTarget,PropertyName,pProperty) ) 

#define ITsSbResourcePluginStore_SetEnvironmentPropertyWithVersionCheck(This,pEnvironment,PropertyName,pProperty)	\
    ( (This)->lpVtbl -> SetEnvironmentPropertyWithVersionCheck(This,pEnvironment,PropertyName,pProperty) ) 

#define ITsSbResourcePluginStore_AcquireTargetLock(This,targetName,dwTimeout,ppContext)	\
    ( (This)->lpVtbl -> AcquireTargetLock(This,targetName,dwTimeout,ppContext) ) 

#define ITsSbResourcePluginStore_ReleaseTargetLock(This,pContext)	\
    ( (This)->lpVtbl -> ReleaseTargetLock(This,pContext) ) 

#define ITsSbResourcePluginStore_TestAndSetServerState(This,PoolName,ServerFQDN,NewState,TestState,pInitState)	\
    ( (This)->lpVtbl -> TestAndSetServerState(This,PoolName,ServerFQDN,NewState,TestState,pInitState) ) 

#define ITsSbResourcePluginStore_SetServerWaitingToStart(This,PoolName,serverName)	\
    ( (This)->lpVtbl -> SetServerWaitingToStart(This,PoolName,serverName) ) 

#define ITsSbResourcePluginStore_GetServerState(This,PoolName,ServerFQDN,pState)	\
    ( (This)->lpVtbl -> GetServerState(This,PoolName,ServerFQDN,pState) ) 

#define ITsSbResourcePluginStore_SetServerDrainMode(This,ServerFQDN,DrainMode)	\
    ( (This)->lpVtbl -> SetServerDrainMode(This,ServerFQDN,DrainMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbResourcePluginStore_INTERFACE_DEFINED__ */


#ifndef __ITsSbFilterPluginStore_INTERFACE_DEFINED__
#define __ITsSbFilterPluginStore_INTERFACE_DEFINED__

/* interface ITsSbFilterPluginStore */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbFilterPluginStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85b44b0f-ed78-413f-9702-fa6d3b5ee755")
    ITsSbFilterPluginStore : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveProperties( 
            /* [in] */ __RPC__in_opt ITsSbPropertySet *pPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ITsSbPropertySet **ppPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteProperties( 
            /* [in] */ __RPC__in BSTR propertyName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbFilterPluginStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbFilterPluginStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbFilterPluginStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbFilterPluginStore * This);
        
        DECLSPEC_XFGVIRT(ITsSbFilterPluginStore, SaveProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveProperties )( 
            __RPC__in ITsSbFilterPluginStore * This,
            /* [in] */ __RPC__in_opt ITsSbPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbFilterPluginStore, EnumerateProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateProperties )( 
            __RPC__in ITsSbFilterPluginStore * This,
            /* [retval][out] */ __RPC__deref_out_opt ITsSbPropertySet **ppPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbFilterPluginStore, DeleteProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteProperties )( 
            __RPC__in ITsSbFilterPluginStore * This,
            /* [in] */ __RPC__in BSTR propertyName);
        
        END_INTERFACE
    } ITsSbFilterPluginStoreVtbl;

    interface ITsSbFilterPluginStore
    {
        CONST_VTBL struct ITsSbFilterPluginStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbFilterPluginStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbFilterPluginStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbFilterPluginStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbFilterPluginStore_SaveProperties(This,pPropertySet)	\
    ( (This)->lpVtbl -> SaveProperties(This,pPropertySet) ) 

#define ITsSbFilterPluginStore_EnumerateProperties(This,ppPropertySet)	\
    ( (This)->lpVtbl -> EnumerateProperties(This,ppPropertySet) ) 

#define ITsSbFilterPluginStore_DeleteProperties(This,propertyName)	\
    ( (This)->lpVtbl -> DeleteProperties(This,propertyName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbFilterPluginStore_INTERFACE_DEFINED__ */


#ifndef __ITsSbGlobalStore_INTERFACE_DEFINED__
#define __ITsSbGlobalStore_INTERFACE_DEFINED__

/* interface ITsSbGlobalStore */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbGlobalStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9AB60F7B-BD72-4D9F-8A3A-A0EA5574E635")
    ITsSbGlobalStore : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryTarget( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QuerySessionBySessionId( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ DWORD dwSessionId,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateFarms( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateTargets( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [in] */ __RPC__in BSTR EnvName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbTarget **pVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateEnvironmentsByProvider( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbEnvironment **ppVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateSessions( 
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in TSSESSION_STATE *pSessionState,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbSession **ppVal[  ]) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFarmProperty( 
            /* [in] */ __RPC__in BSTR farmName,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ __RPC__in VARIANT *pVarValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbGlobalStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbGlobalStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbGlobalStore * This);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, QueryTarget)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryTarget )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [out] */ __RPC__deref_out_opt ITsSbTarget **ppTarget);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, QuerySessionBySessionId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QuerySessionBySessionId )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ DWORD dwSessionId,
            /* [in] */ __RPC__in BSTR TargetName,
            /* [out] */ __RPC__deref_out_opt ITsSbSession **ppSession);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, EnumerateFarms)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateFarms )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, EnumerateTargets)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTargets )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR FarmName,
            /* [in] */ __RPC__in BSTR EnvName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbTarget **pVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, EnumerateEnvironmentsByProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateEnvironmentsByProvider )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbEnvironment **ppVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, EnumerateSessions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateSessions )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR ProviderName,
            /* [in] */ __RPC__in BSTR targetName,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ __RPC__in BSTR poolName,
            /* [in] */ __RPC__in BSTR initialProgram,
            /* [in] */ __RPC__in TSSESSION_STATE *pSessionState,
            /* [out][in] */ __RPC__inout DWORD *pdwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwCount) ITsSbSession **ppVal[  ]);
        
        DECLSPEC_XFGVIRT(ITsSbGlobalStore, GetFarmProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFarmProperty )( 
            __RPC__in ITsSbGlobalStore * This,
            /* [in] */ __RPC__in BSTR farmName,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ __RPC__in VARIANT *pVarValue);
        
        END_INTERFACE
    } ITsSbGlobalStoreVtbl;

    interface ITsSbGlobalStore
    {
        CONST_VTBL struct ITsSbGlobalStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbGlobalStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbGlobalStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbGlobalStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbGlobalStore_QueryTarget(This,ProviderName,TargetName,FarmName,ppTarget)	\
    ( (This)->lpVtbl -> QueryTarget(This,ProviderName,TargetName,FarmName,ppTarget) ) 

#define ITsSbGlobalStore_QuerySessionBySessionId(This,ProviderName,dwSessionId,TargetName,ppSession)	\
    ( (This)->lpVtbl -> QuerySessionBySessionId(This,ProviderName,dwSessionId,TargetName,ppSession) ) 

#define ITsSbGlobalStore_EnumerateFarms(This,ProviderName,pdwCount,pVal)	\
    ( (This)->lpVtbl -> EnumerateFarms(This,ProviderName,pdwCount,pVal) ) 

#define ITsSbGlobalStore_EnumerateTargets(This,ProviderName,FarmName,EnvName,pdwCount,pVal)	\
    ( (This)->lpVtbl -> EnumerateTargets(This,ProviderName,FarmName,EnvName,pdwCount,pVal) ) 

#define ITsSbGlobalStore_EnumerateEnvironmentsByProvider(This,ProviderName,pdwCount,ppVal)	\
    ( (This)->lpVtbl -> EnumerateEnvironmentsByProvider(This,ProviderName,pdwCount,ppVal) ) 

#define ITsSbGlobalStore_EnumerateSessions(This,ProviderName,targetName,userName,userDomain,poolName,initialProgram,pSessionState,pdwCount,ppVal)	\
    ( (This)->lpVtbl -> EnumerateSessions(This,ProviderName,targetName,userName,userDomain,poolName,initialProgram,pSessionState,pdwCount,ppVal) ) 

#define ITsSbGlobalStore_GetFarmProperty(This,farmName,propertyName,pVarValue)	\
    ( (This)->lpVtbl -> GetFarmProperty(This,farmName,propertyName,pVarValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbGlobalStore_INTERFACE_DEFINED__ */


#ifndef __ITsSbProvisioningPluginNotifySink_INTERFACE_DEFINED__
#define __ITsSbProvisioningPluginNotifySink_INTERFACE_DEFINED__

/* interface ITsSbProvisioningPluginNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbProvisioningPluginNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aca87a8e-818b-4581-a032-49c3dfb9c701")
    ITsSbProvisioningPluginNotifySink : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnJobCreated( 
            /* [in] */ __RPC__in VM_NOTIFY_INFO *pVmNotifyInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnVirtualMachineStatusChanged( 
            /* [in] */ __RPC__in VM_NOTIFY_ENTRY *pVmNotifyEntry,
            /* [in] */ VM_NOTIFY_STATUS VmNotifyStatus,
            /* [in] */ HRESULT ErrorCode,
            /* [in] */ __RPC__in BSTR ErrorDescr) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnJobCompleted( 
            /* [in] */ HRESULT ResultCode,
            /* [in] */ __RPC__in BSTR ResultDescription) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnJobCancelled( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LockVirtualMachine( 
            /* [in] */ __RPC__in VM_NOTIFY_ENTRY *pVmNotifyEntry) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnVirtualMachineHostStatusChanged( 
            /* [in] */ __RPC__in BSTR VmHost,
            /* [in] */ VM_HOST_NOTIFY_STATUS VmHostNotifyStatus,
            /* [in] */ HRESULT ErrorCode,
            /* [in] */ __RPC__in BSTR ErrorDescr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbProvisioningPluginNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, OnJobCreated)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnJobCreated )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ __RPC__in VM_NOTIFY_INFO *pVmNotifyInfo);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, OnVirtualMachineStatusChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnVirtualMachineStatusChanged )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ __RPC__in VM_NOTIFY_ENTRY *pVmNotifyEntry,
            /* [in] */ VM_NOTIFY_STATUS VmNotifyStatus,
            /* [in] */ HRESULT ErrorCode,
            /* [in] */ __RPC__in BSTR ErrorDescr);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, OnJobCompleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnJobCompleted )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ HRESULT ResultCode,
            /* [in] */ __RPC__in BSTR ResultDescription);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, OnJobCancelled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnJobCancelled )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, LockVirtualMachine)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LockVirtualMachine )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ __RPC__in VM_NOTIFY_ENTRY *pVmNotifyEntry);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioningPluginNotifySink, OnVirtualMachineHostStatusChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnVirtualMachineHostStatusChanged )( 
            __RPC__in ITsSbProvisioningPluginNotifySink * This,
            /* [in] */ __RPC__in BSTR VmHost,
            /* [in] */ VM_HOST_NOTIFY_STATUS VmHostNotifyStatus,
            /* [in] */ HRESULT ErrorCode,
            /* [in] */ __RPC__in BSTR ErrorDescr);
        
        END_INTERFACE
    } ITsSbProvisioningPluginNotifySinkVtbl;

    interface ITsSbProvisioningPluginNotifySink
    {
        CONST_VTBL struct ITsSbProvisioningPluginNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbProvisioningPluginNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbProvisioningPluginNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbProvisioningPluginNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbProvisioningPluginNotifySink_OnJobCreated(This,pVmNotifyInfo)	\
    ( (This)->lpVtbl -> OnJobCreated(This,pVmNotifyInfo) ) 

#define ITsSbProvisioningPluginNotifySink_OnVirtualMachineStatusChanged(This,pVmNotifyEntry,VmNotifyStatus,ErrorCode,ErrorDescr)	\
    ( (This)->lpVtbl -> OnVirtualMachineStatusChanged(This,pVmNotifyEntry,VmNotifyStatus,ErrorCode,ErrorDescr) ) 

#define ITsSbProvisioningPluginNotifySink_OnJobCompleted(This,ResultCode,ResultDescription)	\
    ( (This)->lpVtbl -> OnJobCompleted(This,ResultCode,ResultDescription) ) 

#define ITsSbProvisioningPluginNotifySink_OnJobCancelled(This)	\
    ( (This)->lpVtbl -> OnJobCancelled(This) ) 

#define ITsSbProvisioningPluginNotifySink_LockVirtualMachine(This,pVmNotifyEntry)	\
    ( (This)->lpVtbl -> LockVirtualMachine(This,pVmNotifyEntry) ) 

#define ITsSbProvisioningPluginNotifySink_OnVirtualMachineHostStatusChanged(This,VmHost,VmHostNotifyStatus,ErrorCode,ErrorDescr)	\
    ( (This)->lpVtbl -> OnVirtualMachineHostStatusChanged(This,VmHost,VmHostNotifyStatus,ErrorCode,ErrorDescr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbProvisioningPluginNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITsSbProvisioning_INTERFACE_DEFINED__
#define __ITsSbProvisioning_INTERFACE_DEFINED__

/* interface ITsSbProvisioning */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbProvisioning;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2f6f0dbb-9e4f-462b-9c3f-fccc3dcb6232")
    ITsSbProvisioning : public ITsSbPlugin
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateVirtualMachines( 
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PatchVirtualMachines( 
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink,
            /* [optional][in] */ __RPC__in VM_PATCH_INFO *pVMPatchInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteVirtualMachines( 
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CancelJob( 
            /* [in] */ __RPC__in BSTR JobGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbProvisioningVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbProvisioning * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbProvisioning * This);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in_opt ITsSbProvider *pProvider,
            /* [in] */ __RPC__in_opt ITsSbPluginNotifySink *pNotifySink,
            /* [in] */ __RPC__in_opt ITsSbPluginPropertySet *pPropertySet);
        
        DECLSPEC_XFGVIRT(ITsSbPlugin, Terminate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioning, CreateVirtualMachines)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateVirtualMachines )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioning, PatchVirtualMachines)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PatchVirtualMachines )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink,
            /* [optional][in] */ __RPC__in VM_PATCH_INFO *pVMPatchInfo);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioning, DeleteVirtualMachines)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteVirtualMachines )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in BSTR JobXmlString,
            /* [in] */ __RPC__in BSTR JobGuid,
            /* [in] */ __RPC__in_opt ITsSbProvisioningPluginNotifySink *pSink);
        
        DECLSPEC_XFGVIRT(ITsSbProvisioning, CancelJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CancelJob )( 
            __RPC__in ITsSbProvisioning * This,
            /* [in] */ __RPC__in BSTR JobGuid);
        
        END_INTERFACE
    } ITsSbProvisioningVtbl;

    interface ITsSbProvisioning
    {
        CONST_VTBL struct ITsSbProvisioningVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbProvisioning_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbProvisioning_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbProvisioning_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbProvisioning_Initialize(This,pProvider,pNotifySink,pPropertySet)	\
    ( (This)->lpVtbl -> Initialize(This,pProvider,pNotifySink,pPropertySet) ) 

#define ITsSbProvisioning_Terminate(This,hr)	\
    ( (This)->lpVtbl -> Terminate(This,hr) ) 


#define ITsSbProvisioning_CreateVirtualMachines(This,JobXmlString,JobGuid,pSink)	\
    ( (This)->lpVtbl -> CreateVirtualMachines(This,JobXmlString,JobGuid,pSink) ) 

#define ITsSbProvisioning_PatchVirtualMachines(This,JobXmlString,JobGuid,pSink,pVMPatchInfo)	\
    ( (This)->lpVtbl -> PatchVirtualMachines(This,JobXmlString,JobGuid,pSink,pVMPatchInfo) ) 

#define ITsSbProvisioning_DeleteVirtualMachines(This,JobXmlString,JobGuid,pSink)	\
    ( (This)->lpVtbl -> DeleteVirtualMachines(This,JobXmlString,JobGuid,pSink) ) 

#define ITsSbProvisioning_CancelJob(This,JobGuid)	\
    ( (This)->lpVtbl -> CancelJob(This,JobGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbProvisioning_INTERFACE_DEFINED__ */


#ifndef __ITsSbGenericNotifySink_INTERFACE_DEFINED__
#define __ITsSbGenericNotifySink_INTERFACE_DEFINED__

/* interface ITsSbGenericNotifySink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITsSbGenericNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c4c8c4f-300b-46ad-9164-8468a7e7568c")
    ITsSbGenericNotifySink : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnCompleted( 
            /* [in] */ HRESULT Status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWaitTimeout( 
            /* [out] */ __RPC__out FILETIME *pftTimeout) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITsSbGenericNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITsSbGenericNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITsSbGenericNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITsSbGenericNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITsSbGenericNotifySink, OnCompleted)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnCompleted )( 
            __RPC__in ITsSbGenericNotifySink * This,
            /* [in] */ HRESULT Status);
        
        DECLSPEC_XFGVIRT(ITsSbGenericNotifySink, GetWaitTimeout)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWaitTimeout )( 
            __RPC__in ITsSbGenericNotifySink * This,
            /* [out] */ __RPC__out FILETIME *pftTimeout);
        
        END_INTERFACE
    } ITsSbGenericNotifySinkVtbl;

    interface ITsSbGenericNotifySink
    {
        CONST_VTBL struct ITsSbGenericNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITsSbGenericNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITsSbGenericNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITsSbGenericNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITsSbGenericNotifySink_OnCompleted(This,Status)	\
    ( (This)->lpVtbl -> OnCompleted(This,Status) ) 

#define ITsSbGenericNotifySink_GetWaitTimeout(This,pftTimeout)	\
    ( (This)->lpVtbl -> GetWaitTimeout(This,pftTimeout) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITsSbGenericNotifySink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_sbtsv_0000_0033 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_sbtsv_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sbtsv_0000_0033_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



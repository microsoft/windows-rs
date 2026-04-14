

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

#ifndef __txdtc_h__
#define __txdtc_h__

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

#ifndef __IXATransLookup_FWD_DEFINED__
#define __IXATransLookup_FWD_DEFINED__
typedef interface IXATransLookup IXATransLookup;

#endif 	/* __IXATransLookup_FWD_DEFINED__ */


#ifndef __IXATransLookup2_FWD_DEFINED__
#define __IXATransLookup2_FWD_DEFINED__
typedef interface IXATransLookup2 IXATransLookup2;

#endif 	/* __IXATransLookup2_FWD_DEFINED__ */


#ifndef __IResourceManagerSink_FWD_DEFINED__
#define __IResourceManagerSink_FWD_DEFINED__
typedef interface IResourceManagerSink IResourceManagerSink;

#endif 	/* __IResourceManagerSink_FWD_DEFINED__ */


#ifndef __IResourceManager_FWD_DEFINED__
#define __IResourceManager_FWD_DEFINED__
typedef interface IResourceManager IResourceManager;

#endif 	/* __IResourceManager_FWD_DEFINED__ */


#ifndef __ILastResourceManager_FWD_DEFINED__
#define __ILastResourceManager_FWD_DEFINED__
typedef interface ILastResourceManager ILastResourceManager;

#endif 	/* __ILastResourceManager_FWD_DEFINED__ */


#ifndef __IResourceManager2_FWD_DEFINED__
#define __IResourceManager2_FWD_DEFINED__
typedef interface IResourceManager2 IResourceManager2;

#endif 	/* __IResourceManager2_FWD_DEFINED__ */


#ifndef __IResourceManagerRejoinable_FWD_DEFINED__
#define __IResourceManagerRejoinable_FWD_DEFINED__
typedef interface IResourceManagerRejoinable IResourceManagerRejoinable;

#endif 	/* __IResourceManagerRejoinable_FWD_DEFINED__ */


#ifndef __IXAConfig_FWD_DEFINED__
#define __IXAConfig_FWD_DEFINED__
typedef interface IXAConfig IXAConfig;

#endif 	/* __IXAConfig_FWD_DEFINED__ */


#ifndef __IRMHelper_FWD_DEFINED__
#define __IRMHelper_FWD_DEFINED__
typedef interface IRMHelper IRMHelper;

#endif 	/* __IRMHelper_FWD_DEFINED__ */


#ifndef __IXAObtainRMInfo_FWD_DEFINED__
#define __IXAObtainRMInfo_FWD_DEFINED__
typedef interface IXAObtainRMInfo IXAObtainRMInfo;

#endif 	/* __IXAObtainRMInfo_FWD_DEFINED__ */


#ifndef __IResourceManagerFactory_FWD_DEFINED__
#define __IResourceManagerFactory_FWD_DEFINED__
typedef interface IResourceManagerFactory IResourceManagerFactory;

#endif 	/* __IResourceManagerFactory_FWD_DEFINED__ */


#ifndef __IResourceManagerFactory2_FWD_DEFINED__
#define __IResourceManagerFactory2_FWD_DEFINED__
typedef interface IResourceManagerFactory2 IResourceManagerFactory2;

#endif 	/* __IResourceManagerFactory2_FWD_DEFINED__ */


#ifndef __IPrepareInfo_FWD_DEFINED__
#define __IPrepareInfo_FWD_DEFINED__
typedef interface IPrepareInfo IPrepareInfo;

#endif 	/* __IPrepareInfo_FWD_DEFINED__ */


#ifndef __IPrepareInfo2_FWD_DEFINED__
#define __IPrepareInfo2_FWD_DEFINED__
typedef interface IPrepareInfo2 IPrepareInfo2;

#endif 	/* __IPrepareInfo2_FWD_DEFINED__ */


#ifndef __IGetDispenser_FWD_DEFINED__
#define __IGetDispenser_FWD_DEFINED__
typedef interface IGetDispenser IGetDispenser;

#endif 	/* __IGetDispenser_FWD_DEFINED__ */


#ifndef __ITransactionVoterBallotAsync2_FWD_DEFINED__
#define __ITransactionVoterBallotAsync2_FWD_DEFINED__
typedef interface ITransactionVoterBallotAsync2 ITransactionVoterBallotAsync2;

#endif 	/* __ITransactionVoterBallotAsync2_FWD_DEFINED__ */


#ifndef __ITransactionVoterNotifyAsync2_FWD_DEFINED__
#define __ITransactionVoterNotifyAsync2_FWD_DEFINED__
typedef interface ITransactionVoterNotifyAsync2 ITransactionVoterNotifyAsync2;

#endif 	/* __ITransactionVoterNotifyAsync2_FWD_DEFINED__ */


#ifndef __ITransactionVoterFactory2_FWD_DEFINED__
#define __ITransactionVoterFactory2_FWD_DEFINED__
typedef interface ITransactionVoterFactory2 ITransactionVoterFactory2;

#endif 	/* __ITransactionVoterFactory2_FWD_DEFINED__ */


#ifndef __ITransactionPhase0EnlistmentAsync_FWD_DEFINED__
#define __ITransactionPhase0EnlistmentAsync_FWD_DEFINED__
typedef interface ITransactionPhase0EnlistmentAsync ITransactionPhase0EnlistmentAsync;

#endif 	/* __ITransactionPhase0EnlistmentAsync_FWD_DEFINED__ */


#ifndef __ITransactionPhase0NotifyAsync_FWD_DEFINED__
#define __ITransactionPhase0NotifyAsync_FWD_DEFINED__
typedef interface ITransactionPhase0NotifyAsync ITransactionPhase0NotifyAsync;

#endif 	/* __ITransactionPhase0NotifyAsync_FWD_DEFINED__ */


#ifndef __ITransactionPhase0Factory_FWD_DEFINED__
#define __ITransactionPhase0Factory_FWD_DEFINED__
typedef interface ITransactionPhase0Factory ITransactionPhase0Factory;

#endif 	/* __ITransactionPhase0Factory_FWD_DEFINED__ */


#ifndef __ITransactionTransmitter_FWD_DEFINED__
#define __ITransactionTransmitter_FWD_DEFINED__
typedef interface ITransactionTransmitter ITransactionTransmitter;

#endif 	/* __ITransactionTransmitter_FWD_DEFINED__ */


#ifndef __ITransactionTransmitterFactory_FWD_DEFINED__
#define __ITransactionTransmitterFactory_FWD_DEFINED__
typedef interface ITransactionTransmitterFactory ITransactionTransmitterFactory;

#endif 	/* __ITransactionTransmitterFactory_FWD_DEFINED__ */


#ifndef __ITransactionReceiver_FWD_DEFINED__
#define __ITransactionReceiver_FWD_DEFINED__
typedef interface ITransactionReceiver ITransactionReceiver;

#endif 	/* __ITransactionReceiver_FWD_DEFINED__ */


#ifndef __ITransactionReceiverFactory_FWD_DEFINED__
#define __ITransactionReceiverFactory_FWD_DEFINED__
typedef interface ITransactionReceiverFactory ITransactionReceiverFactory;

#endif 	/* __ITransactionReceiverFactory_FWD_DEFINED__ */


#ifndef __IDtcLuConfigure_FWD_DEFINED__
#define __IDtcLuConfigure_FWD_DEFINED__
typedef interface IDtcLuConfigure IDtcLuConfigure;

#endif 	/* __IDtcLuConfigure_FWD_DEFINED__ */


#ifndef __IDtcLuRecovery_FWD_DEFINED__
#define __IDtcLuRecovery_FWD_DEFINED__
typedef interface IDtcLuRecovery IDtcLuRecovery;

#endif 	/* __IDtcLuRecovery_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryFactory_FWD_DEFINED__
#define __IDtcLuRecoveryFactory_FWD_DEFINED__
typedef interface IDtcLuRecoveryFactory IDtcLuRecoveryFactory;

#endif 	/* __IDtcLuRecoveryFactory_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByDtcTransWork_FWD_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtcTransWork_FWD_DEFINED__
typedef interface IDtcLuRecoveryInitiatedByDtcTransWork IDtcLuRecoveryInitiatedByDtcTransWork;

#endif 	/* __IDtcLuRecoveryInitiatedByDtcTransWork_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByDtcStatusWork_FWD_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtcStatusWork_FWD_DEFINED__
typedef interface IDtcLuRecoveryInitiatedByDtcStatusWork IDtcLuRecoveryInitiatedByDtcStatusWork;

#endif 	/* __IDtcLuRecoveryInitiatedByDtcStatusWork_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByDtc_FWD_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtc_FWD_DEFINED__
typedef interface IDtcLuRecoveryInitiatedByDtc IDtcLuRecoveryInitiatedByDtc;

#endif 	/* __IDtcLuRecoveryInitiatedByDtc_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByLuWork_FWD_DEFINED__
#define __IDtcLuRecoveryInitiatedByLuWork_FWD_DEFINED__
typedef interface IDtcLuRecoveryInitiatedByLuWork IDtcLuRecoveryInitiatedByLuWork;

#endif 	/* __IDtcLuRecoveryInitiatedByLuWork_FWD_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByLu_FWD_DEFINED__
#define __IDtcLuRecoveryInitiatedByLu_FWD_DEFINED__
typedef interface IDtcLuRecoveryInitiatedByLu IDtcLuRecoveryInitiatedByLu;

#endif 	/* __IDtcLuRecoveryInitiatedByLu_FWD_DEFINED__ */


#ifndef __IDtcLuRmEnlistment_FWD_DEFINED__
#define __IDtcLuRmEnlistment_FWD_DEFINED__
typedef interface IDtcLuRmEnlistment IDtcLuRmEnlistment;

#endif 	/* __IDtcLuRmEnlistment_FWD_DEFINED__ */


#ifndef __IDtcLuRmEnlistmentSink_FWD_DEFINED__
#define __IDtcLuRmEnlistmentSink_FWD_DEFINED__
typedef interface IDtcLuRmEnlistmentSink IDtcLuRmEnlistmentSink;

#endif 	/* __IDtcLuRmEnlistmentSink_FWD_DEFINED__ */


#ifndef __IDtcLuRmEnlistmentFactory_FWD_DEFINED__
#define __IDtcLuRmEnlistmentFactory_FWD_DEFINED__
typedef interface IDtcLuRmEnlistmentFactory IDtcLuRmEnlistmentFactory;

#endif 	/* __IDtcLuRmEnlistmentFactory_FWD_DEFINED__ */


#ifndef __IDtcLuSubordinateDtc_FWD_DEFINED__
#define __IDtcLuSubordinateDtc_FWD_DEFINED__
typedef interface IDtcLuSubordinateDtc IDtcLuSubordinateDtc;

#endif 	/* __IDtcLuSubordinateDtc_FWD_DEFINED__ */


#ifndef __IDtcLuSubordinateDtcSink_FWD_DEFINED__
#define __IDtcLuSubordinateDtcSink_FWD_DEFINED__
typedef interface IDtcLuSubordinateDtcSink IDtcLuSubordinateDtcSink;

#endif 	/* __IDtcLuSubordinateDtcSink_FWD_DEFINED__ */


#ifndef __IDtcLuSubordinateDtcFactory_FWD_DEFINED__
#define __IDtcLuSubordinateDtcFactory_FWD_DEFINED__
typedef interface IDtcLuSubordinateDtcFactory IDtcLuSubordinateDtcFactory;

#endif 	/* __IDtcLuSubordinateDtcFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "txcoord.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_txdtc_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define XACTTOMSG(dwXact) (dwXact-0x00040000+0x40000000)
typedef 
enum XACT_DTC_CONSTANTS
    {
        XACT_E_CONNECTION_REQUEST_DENIED	= 0x8004d100L,
        XACT_E_TOOMANY_ENLISTMENTS	= 0x8004d101L,
        XACT_E_DUPLICATE_GUID	= 0x8004d102L,
        XACT_E_NOTSINGLEPHASE	= 0x8004d103L,
        XACT_E_RECOVERYALREADYDONE	= 0x8004d104L,
        XACT_E_PROTOCOL	= 0x8004d105L,
        XACT_E_RM_FAILURE	= 0x8004d106L,
        XACT_E_RECOVERY_FAILED	= 0x8004d107L,
        XACT_E_LU_NOT_FOUND	= 0x8004d108L,
        XACT_E_DUPLICATE_LU	= 0x8004d109L,
        XACT_E_LU_NOT_CONNECTED	= 0x8004d10aL,
        XACT_E_DUPLICATE_TRANSID	= 0x8004d10bL,
        XACT_E_LU_BUSY	= 0x8004d10cL,
        XACT_E_LU_NO_RECOVERY_PROCESS	= 0x8004d10dL,
        XACT_E_LU_DOWN	= 0x8004d10eL,
        XACT_E_LU_RECOVERING	= 0x8004d10fL,
        XACT_E_LU_RECOVERY_MISMATCH	= 0x8004d110L,
        XACT_E_RM_UNAVAILABLE	= 0x8004d111L,
        XACT_E_LRMRECOVERYALREADYDONE	= 0x8004d112L,
        XACT_E_NOLASTRESOURCEINTERFACE	= 0x8004d113L,
        XACT_S_NONOTIFY	= 0x4d100L,
        XACT_OK_NONOTIFY	= 0x4d101L,
        dwUSER_MS_SQLSERVER	= 0xffff
    } 	XACT_DTC_CONSTANTS;

#ifndef _XID_T_DEFINED
#define _XID_T_DEFINED
typedef struct xid_t
    {
    long formatID;
    long gtrid_length;
    long bqual_length;
    char data[ 128 ];
    } 	XID;

#endif
#ifndef _XA_SWITCH_T_DEFINED
#define _XA_SWITCH_T_DEFINED
typedef struct xa_switch_t
    {
    char name[ 32 ];
    long flags;
    long version;
    int ( __cdecl *xa_open_entry )( 
        char *__MIDL____MIDL_itf_txdtc_0000_00000000,
        int __MIDL____MIDL_itf_txdtc_0000_00000001,
        long __MIDL____MIDL_itf_txdtc_0000_00000002);
    int ( __cdecl *xa_close_entry )( 
        char *__MIDL____MIDL_itf_txdtc_0000_00000004,
        int __MIDL____MIDL_itf_txdtc_0000_00000005,
        long __MIDL____MIDL_itf_txdtc_0000_00000006);
    int ( __cdecl *xa_start_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000008,
        int __MIDL____MIDL_itf_txdtc_0000_00000009,
        long __MIDL____MIDL_itf_txdtc_0000_00000010);
    int ( __cdecl *xa_end_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000012,
        int __MIDL____MIDL_itf_txdtc_0000_00000013,
        long __MIDL____MIDL_itf_txdtc_0000_00000014);
    int ( __cdecl *xa_rollback_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000016,
        int __MIDL____MIDL_itf_txdtc_0000_00000017,
        long __MIDL____MIDL_itf_txdtc_0000_00000018);
    int ( __cdecl *xa_prepare_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000020,
        int __MIDL____MIDL_itf_txdtc_0000_00000021,
        long __MIDL____MIDL_itf_txdtc_0000_00000022);
    int ( __cdecl *xa_commit_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000024,
        int __MIDL____MIDL_itf_txdtc_0000_00000025,
        long __MIDL____MIDL_itf_txdtc_0000_00000026);
    int ( __cdecl *xa_recover_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000028,
        long __MIDL____MIDL_itf_txdtc_0000_00000029,
        int __MIDL____MIDL_itf_txdtc_0000_00000030,
        long __MIDL____MIDL_itf_txdtc_0000_00000031);
    int ( __cdecl *xa_forget_entry )( 
        XID *__MIDL____MIDL_itf_txdtc_0000_00000033,
        int __MIDL____MIDL_itf_txdtc_0000_00000034,
        long __MIDL____MIDL_itf_txdtc_0000_00000035);
    int ( __cdecl *xa_complete_entry )( 
        int *__MIDL____MIDL_itf_txdtc_0000_00000037,
        int *__MIDL____MIDL_itf_txdtc_0000_00000038,
        int __MIDL____MIDL_itf_txdtc_0000_00000039,
        long __MIDL____MIDL_itf_txdtc_0000_00000040);
    } 	xa_switch_t;

#endif


extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0000_v0_0_s_ifspec;

#ifndef __IXATransLookup_INTERFACE_DEFINED__
#define __IXATransLookup_INTERFACE_DEFINED__

/* interface IXATransLookup */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IXATransLookup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F3B1F131-EEDA-11ce-AED4-00AA0051E2C4")
    IXATransLookup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lookup( 
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXATransLookupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXATransLookup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXATransLookup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXATransLookup * This);
        
        DECLSPEC_XFGVIRT(IXATransLookup, Lookup)
        HRESULT ( STDMETHODCALLTYPE *Lookup )( 
            __RPC__in IXATransLookup * This,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction);
        
        END_INTERFACE
    } IXATransLookupVtbl;

    interface IXATransLookup
    {
        CONST_VTBL struct IXATransLookupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXATransLookup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXATransLookup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXATransLookup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXATransLookup_Lookup(This,ppTransaction)	\
    ( (This)->lpVtbl -> Lookup(This,ppTransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXATransLookup_INTERFACE_DEFINED__ */


#ifndef __IXATransLookup2_INTERFACE_DEFINED__
#define __IXATransLookup2_INTERFACE_DEFINED__

/* interface IXATransLookup2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IXATransLookup2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF193C85-0D1A-4290-B88F-D2CB8873D1E7")
    IXATransLookup2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lookup( 
            /* [in] */ __RPC__in XID *pXID,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXATransLookup2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXATransLookup2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXATransLookup2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXATransLookup2 * This);
        
        DECLSPEC_XFGVIRT(IXATransLookup2, Lookup)
        HRESULT ( STDMETHODCALLTYPE *Lookup )( 
            __RPC__in IXATransLookup2 * This,
            /* [in] */ __RPC__in XID *pXID,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction);
        
        END_INTERFACE
    } IXATransLookup2Vtbl;

    interface IXATransLookup2
    {
        CONST_VTBL struct IXATransLookup2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXATransLookup2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXATransLookup2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXATransLookup2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXATransLookup2_Lookup(This,pXID,ppTransaction)	\
    ( (This)->lpVtbl -> Lookup(This,pXID,ppTransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXATransLookup2_INTERFACE_DEFINED__ */


#ifndef __IResourceManagerSink_INTERFACE_DEFINED__
#define __IResourceManagerSink_INTERFACE_DEFINED__

/* interface IResourceManagerSink */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManagerSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0D563181-DEFB-11ce-AED1-00AA0051E2C4")
    IResourceManagerSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TMDown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManagerSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManagerSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManagerSink * This);
        
        DECLSPEC_XFGVIRT(IResourceManagerSink, TMDown)
        HRESULT ( STDMETHODCALLTYPE *TMDown )( 
            __RPC__in IResourceManagerSink * This);
        
        END_INTERFACE
    } IResourceManagerSinkVtbl;

    interface IResourceManagerSink
    {
        CONST_VTBL struct IResourceManagerSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManagerSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManagerSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManagerSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManagerSink_TMDown(This)	\
    ( (This)->lpVtbl -> TMDown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManagerSink_INTERFACE_DEFINED__ */


#ifndef __IResourceManager_INTERFACE_DEFINED__
#define __IResourceManager_INTERFACE_DEFINED__

/* interface IResourceManager */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13741d21-87eb-11ce-8081-0080c758527e")
    IResourceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Enlist( 
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pRes,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reenlist( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReenlistmentComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDistributedTransactionManager( 
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManager * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, Enlist)
        HRESULT ( STDMETHODCALLTYPE *Enlist )( 
            __RPC__in IResourceManager * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pRes,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist);
        
        DECLSPEC_XFGVIRT(IResourceManager, Reenlist)
        HRESULT ( STDMETHODCALLTYPE *Reenlist )( 
            __RPC__in IResourceManager * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        DECLSPEC_XFGVIRT(IResourceManager, ReenlistmentComplete)
        HRESULT ( STDMETHODCALLTYPE *ReenlistmentComplete )( 
            __RPC__in IResourceManager * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, GetDistributedTransactionManager)
        HRESULT ( STDMETHODCALLTYPE *GetDistributedTransactionManager )( 
            __RPC__in IResourceManager * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject);
        
        END_INTERFACE
    } IResourceManagerVtbl;

    interface IResourceManager
    {
        CONST_VTBL struct IResourceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManager_Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist)	\
    ( (This)->lpVtbl -> Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist) ) 

#define IResourceManager_Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat) ) 

#define IResourceManager_ReenlistmentComplete(This)	\
    ( (This)->lpVtbl -> ReenlistmentComplete(This) ) 

#define IResourceManager_GetDistributedTransactionManager(This,iid,ppvObject)	\
    ( (This)->lpVtbl -> GetDistributedTransactionManager(This,iid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManager_INTERFACE_DEFINED__ */


#ifndef __ILastResourceManager_INTERFACE_DEFINED__
#define __ILastResourceManager_INTERFACE_DEFINED__

/* interface ILastResourceManager */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ILastResourceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D964AD4-5B33-11d3-8A91-00C04F79EB6D")
    ILastResourceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransactionCommitted( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecoveryDone( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILastResourceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILastResourceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILastResourceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILastResourceManager * This);
        
        DECLSPEC_XFGVIRT(ILastResourceManager, TransactionCommitted)
        HRESULT ( STDMETHODCALLTYPE *TransactionCommitted )( 
            __RPC__in ILastResourceManager * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo);
        
        DECLSPEC_XFGVIRT(ILastResourceManager, RecoveryDone)
        HRESULT ( STDMETHODCALLTYPE *RecoveryDone )( 
            __RPC__in ILastResourceManager * This);
        
        END_INTERFACE
    } ILastResourceManagerVtbl;

    interface ILastResourceManager
    {
        CONST_VTBL struct ILastResourceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILastResourceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILastResourceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILastResourceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILastResourceManager_TransactionCommitted(This,pPrepInfo,cbPrepInfo)	\
    ( (This)->lpVtbl -> TransactionCommitted(This,pPrepInfo,cbPrepInfo) ) 

#define ILastResourceManager_RecoveryDone(This)	\
    ( (This)->lpVtbl -> RecoveryDone(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILastResourceManager_INTERFACE_DEFINED__ */


#ifndef __IResourceManager2_INTERFACE_DEFINED__
#define __IResourceManager2_INTERFACE_DEFINED__

/* interface IResourceManager2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D136C69A-F749-11d1-8F47-00C04F8EE57D")
    IResourceManager2 : public IResourceManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Enlist2( 
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pResAsync,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__out XID *pXid,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reenlist2( 
            /* [in] */ __RPC__in XID *pXid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManager2 * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, Enlist)
        HRESULT ( STDMETHODCALLTYPE *Enlist )( 
            __RPC__in IResourceManager2 * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pRes,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist);
        
        DECLSPEC_XFGVIRT(IResourceManager, Reenlist)
        HRESULT ( STDMETHODCALLTYPE *Reenlist )( 
            __RPC__in IResourceManager2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        DECLSPEC_XFGVIRT(IResourceManager, ReenlistmentComplete)
        HRESULT ( STDMETHODCALLTYPE *ReenlistmentComplete )( 
            __RPC__in IResourceManager2 * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, GetDistributedTransactionManager)
        HRESULT ( STDMETHODCALLTYPE *GetDistributedTransactionManager )( 
            __RPC__in IResourceManager2 * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject);
        
        DECLSPEC_XFGVIRT(IResourceManager2, Enlist2)
        HRESULT ( STDMETHODCALLTYPE *Enlist2 )( 
            __RPC__in IResourceManager2 * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pResAsync,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__out XID *pXid,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist);
        
        DECLSPEC_XFGVIRT(IResourceManager2, Reenlist2)
        HRESULT ( STDMETHODCALLTYPE *Reenlist2 )( 
            __RPC__in IResourceManager2 * This,
            /* [in] */ __RPC__in XID *pXid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        END_INTERFACE
    } IResourceManager2Vtbl;

    interface IResourceManager2
    {
        CONST_VTBL struct IResourceManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManager2_Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist)	\
    ( (This)->lpVtbl -> Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist) ) 

#define IResourceManager2_Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat) ) 

#define IResourceManager2_ReenlistmentComplete(This)	\
    ( (This)->lpVtbl -> ReenlistmentComplete(This) ) 

#define IResourceManager2_GetDistributedTransactionManager(This,iid,ppvObject)	\
    ( (This)->lpVtbl -> GetDistributedTransactionManager(This,iid,ppvObject) ) 


#define IResourceManager2_Enlist2(This,pTransaction,pResAsync,pUOW,pisoLevel,pXid,ppEnlist)	\
    ( (This)->lpVtbl -> Enlist2(This,pTransaction,pResAsync,pUOW,pisoLevel,pXid,ppEnlist) ) 

#define IResourceManager2_Reenlist2(This,pXid,dwTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Reenlist2(This,pXid,dwTimeout,pXactStat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManager2_INTERFACE_DEFINED__ */


#ifndef __IResourceManagerRejoinable_INTERFACE_DEFINED__
#define __IResourceManagerRejoinable_INTERFACE_DEFINED__

/* interface IResourceManagerRejoinable */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManagerRejoinable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6f6de620-b5df-4f3e-9cfa-c8aebd05172b")
    IResourceManagerRejoinable : public IResourceManager2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Rejoin( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerRejoinableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManagerRejoinable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManagerRejoinable * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, Enlist)
        HRESULT ( STDMETHODCALLTYPE *Enlist )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pRes,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist);
        
        DECLSPEC_XFGVIRT(IResourceManager, Reenlist)
        HRESULT ( STDMETHODCALLTYPE *Reenlist )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        DECLSPEC_XFGVIRT(IResourceManager, ReenlistmentComplete)
        HRESULT ( STDMETHODCALLTYPE *ReenlistmentComplete )( 
            __RPC__in IResourceManagerRejoinable * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, GetDistributedTransactionManager)
        HRESULT ( STDMETHODCALLTYPE *GetDistributedTransactionManager )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject);
        
        DECLSPEC_XFGVIRT(IResourceManager2, Enlist2)
        HRESULT ( STDMETHODCALLTYPE *Enlist2 )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionResourceAsync *pResAsync,
            /* [out] */ __RPC__out XACTUOW *pUOW,
            /* [out] */ __RPC__out LONG *pisoLevel,
            /* [out] */ __RPC__out XID *pXid,
            /* [out] */ __RPC__deref_out_opt ITransactionEnlistmentAsync **ppEnlist);
        
        DECLSPEC_XFGVIRT(IResourceManager2, Reenlist2)
        HRESULT ( STDMETHODCALLTYPE *Reenlist2 )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [in] */ __RPC__in XID *pXid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        DECLSPEC_XFGVIRT(IResourceManagerRejoinable, Rejoin)
        HRESULT ( STDMETHODCALLTYPE *Rejoin )( 
            __RPC__in IResourceManagerRejoinable * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPrepInfo) byte *pPrepInfo,
            /* [in] */ ULONG cbPrepInfo,
            /* [in] */ DWORD lTimeout,
            /* [out] */ __RPC__out XACTSTAT *pXactStat);
        
        END_INTERFACE
    } IResourceManagerRejoinableVtbl;

    interface IResourceManagerRejoinable
    {
        CONST_VTBL struct IResourceManagerRejoinableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManagerRejoinable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManagerRejoinable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManagerRejoinable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManagerRejoinable_Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist)	\
    ( (This)->lpVtbl -> Enlist(This,pTransaction,pRes,pUOW,pisoLevel,ppEnlist) ) 

#define IResourceManagerRejoinable_Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Reenlist(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat) ) 

#define IResourceManagerRejoinable_ReenlistmentComplete(This)	\
    ( (This)->lpVtbl -> ReenlistmentComplete(This) ) 

#define IResourceManagerRejoinable_GetDistributedTransactionManager(This,iid,ppvObject)	\
    ( (This)->lpVtbl -> GetDistributedTransactionManager(This,iid,ppvObject) ) 


#define IResourceManagerRejoinable_Enlist2(This,pTransaction,pResAsync,pUOW,pisoLevel,pXid,ppEnlist)	\
    ( (This)->lpVtbl -> Enlist2(This,pTransaction,pResAsync,pUOW,pisoLevel,pXid,ppEnlist) ) 

#define IResourceManagerRejoinable_Reenlist2(This,pXid,dwTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Reenlist2(This,pXid,dwTimeout,pXactStat) ) 


#define IResourceManagerRejoinable_Rejoin(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat)	\
    ( (This)->lpVtbl -> Rejoin(This,pPrepInfo,cbPrepInfo,lTimeout,pXactStat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManagerRejoinable_INTERFACE_DEFINED__ */


#ifndef __IXAConfig_INTERFACE_DEFINED__
#define __IXAConfig_INTERFACE_DEFINED__

/* interface IXAConfig */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IXAConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C8A6E3A1-9A8C-11cf-A308-00A0C905416E")
    IXAConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ GUID clsidHelperDll) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Terminate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXAConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXAConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXAConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXAConfig * This);
        
        DECLSPEC_XFGVIRT(IXAConfig, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IXAConfig * This,
            /* [in] */ GUID clsidHelperDll);
        
        DECLSPEC_XFGVIRT(IXAConfig, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IXAConfig * This);
        
        END_INTERFACE
    } IXAConfigVtbl;

    interface IXAConfig
    {
        CONST_VTBL struct IXAConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXAConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXAConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXAConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXAConfig_Initialize(This,clsidHelperDll)	\
    ( (This)->lpVtbl -> Initialize(This,clsidHelperDll) ) 

#define IXAConfig_Terminate(This)	\
    ( (This)->lpVtbl -> Terminate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXAConfig_INTERFACE_DEFINED__ */


#ifndef __IRMHelper_INTERFACE_DEFINED__
#define __IRMHelper_INTERFACE_DEFINED__

/* interface IRMHelper */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_IRMHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E793F6D1-F53D-11cf-A60D-00A0C905416E")
    IRMHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RMCount( 
            /* [in] */ DWORD dwcTotalNumberOfRMs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RMInfo( 
            /* [in] */ xa_switch_t *pXa_Switch,
            /* [in] */ BOOL fCDeclCallingConv,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszOpenString,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszCloseString,
            /* [in] */ GUID guidRMRecovery) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRMHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRMHelper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRMHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRMHelper * This);
        
        DECLSPEC_XFGVIRT(IRMHelper, RMCount)
        HRESULT ( STDMETHODCALLTYPE *RMCount )( 
            IRMHelper * This,
            /* [in] */ DWORD dwcTotalNumberOfRMs);
        
        DECLSPEC_XFGVIRT(IRMHelper, RMInfo)
        HRESULT ( STDMETHODCALLTYPE *RMInfo )( 
            IRMHelper * This,
            /* [in] */ xa_switch_t *pXa_Switch,
            /* [in] */ BOOL fCDeclCallingConv,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszOpenString,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszCloseString,
            /* [in] */ GUID guidRMRecovery);
        
        END_INTERFACE
    } IRMHelperVtbl;

    interface IRMHelper
    {
        CONST_VTBL struct IRMHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRMHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRMHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRMHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRMHelper_RMCount(This,dwcTotalNumberOfRMs)	\
    ( (This)->lpVtbl -> RMCount(This,dwcTotalNumberOfRMs) ) 

#define IRMHelper_RMInfo(This,pXa_Switch,fCDeclCallingConv,pszOpenString,pszCloseString,guidRMRecovery)	\
    ( (This)->lpVtbl -> RMInfo(This,pXa_Switch,fCDeclCallingConv,pszOpenString,pszCloseString,guidRMRecovery) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRMHelper_INTERFACE_DEFINED__ */


#ifndef __IXAObtainRMInfo_INTERFACE_DEFINED__
#define __IXAObtainRMInfo_INTERFACE_DEFINED__

/* interface IXAObtainRMInfo */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IXAObtainRMInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E793F6D2-F53D-11cf-A60D-00A0C905416E")
    IXAObtainRMInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ObtainRMInfo( 
            /* [in] */ __RPC__in_opt IRMHelper *pIRMHelper) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXAObtainRMInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXAObtainRMInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXAObtainRMInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXAObtainRMInfo * This);
        
        DECLSPEC_XFGVIRT(IXAObtainRMInfo, ObtainRMInfo)
        HRESULT ( STDMETHODCALLTYPE *ObtainRMInfo )( 
            __RPC__in IXAObtainRMInfo * This,
            /* [in] */ __RPC__in_opt IRMHelper *pIRMHelper);
        
        END_INTERFACE
    } IXAObtainRMInfoVtbl;

    interface IXAObtainRMInfo
    {
        CONST_VTBL struct IXAObtainRMInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXAObtainRMInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXAObtainRMInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXAObtainRMInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXAObtainRMInfo_ObtainRMInfo(This,pIRMHelper)	\
    ( (This)->lpVtbl -> ObtainRMInfo(This,pIRMHelper) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXAObtainRMInfo_INTERFACE_DEFINED__ */


#ifndef __IResourceManagerFactory_INTERFACE_DEFINED__
#define __IResourceManagerFactory_INTERFACE_DEFINED__

/* interface IResourceManagerFactory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManagerFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13741d20-87eb-11ce-8081-0080c758527e")
    IResourceManagerFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in GUID *pguidRM,
            /* [string][in] */ __RPC__in_string CHAR *pszRMName,
            /* [in] */ __RPC__in_opt IResourceManagerSink *pIResMgrSink,
            /* [out] */ __RPC__deref_out_opt IResourceManager **ppResMgr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManagerFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManagerFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManagerFactory * This);
        
        DECLSPEC_XFGVIRT(IResourceManagerFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IResourceManagerFactory * This,
            /* [in] */ __RPC__in GUID *pguidRM,
            /* [string][in] */ __RPC__in_string CHAR *pszRMName,
            /* [in] */ __RPC__in_opt IResourceManagerSink *pIResMgrSink,
            /* [out] */ __RPC__deref_out_opt IResourceManager **ppResMgr);
        
        END_INTERFACE
    } IResourceManagerFactoryVtbl;

    interface IResourceManagerFactory
    {
        CONST_VTBL struct IResourceManagerFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManagerFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManagerFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManagerFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManagerFactory_Create(This,pguidRM,pszRMName,pIResMgrSink,ppResMgr)	\
    ( (This)->lpVtbl -> Create(This,pguidRM,pszRMName,pIResMgrSink,ppResMgr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManagerFactory_INTERFACE_DEFINED__ */


#ifndef __IResourceManagerFactory2_INTERFACE_DEFINED__
#define __IResourceManagerFactory2_INTERFACE_DEFINED__

/* interface IResourceManagerFactory2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IResourceManagerFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B369C21-FBD2-11d1-8F47-00C04F8EE57D")
    IResourceManagerFactory2 : public IResourceManagerFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateEx( 
            /* [in] */ __RPC__in GUID *pguidRM,
            /* [string][in] */ __RPC__in_string CHAR *pszRMName,
            /* [in] */ __RPC__in_opt IResourceManagerSink *pIResMgrSink,
            /* [in] */ __RPC__in REFIID riidRequested,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvResMgr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResourceManagerFactory2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResourceManagerFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResourceManagerFactory2 * This);
        
        DECLSPEC_XFGVIRT(IResourceManagerFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IResourceManagerFactory2 * This,
            /* [in] */ __RPC__in GUID *pguidRM,
            /* [string][in] */ __RPC__in_string CHAR *pszRMName,
            /* [in] */ __RPC__in_opt IResourceManagerSink *pIResMgrSink,
            /* [out] */ __RPC__deref_out_opt IResourceManager **ppResMgr);
        
        DECLSPEC_XFGVIRT(IResourceManagerFactory2, CreateEx)
        HRESULT ( STDMETHODCALLTYPE *CreateEx )( 
            __RPC__in IResourceManagerFactory2 * This,
            /* [in] */ __RPC__in GUID *pguidRM,
            /* [string][in] */ __RPC__in_string CHAR *pszRMName,
            /* [in] */ __RPC__in_opt IResourceManagerSink *pIResMgrSink,
            /* [in] */ __RPC__in REFIID riidRequested,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvResMgr);
        
        END_INTERFACE
    } IResourceManagerFactory2Vtbl;

    interface IResourceManagerFactory2
    {
        CONST_VTBL struct IResourceManagerFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManagerFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManagerFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManagerFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManagerFactory2_Create(This,pguidRM,pszRMName,pIResMgrSink,ppResMgr)	\
    ( (This)->lpVtbl -> Create(This,pguidRM,pszRMName,pIResMgrSink,ppResMgr) ) 


#define IResourceManagerFactory2_CreateEx(This,pguidRM,pszRMName,pIResMgrSink,riidRequested,ppvResMgr)	\
    ( (This)->lpVtbl -> CreateEx(This,pguidRM,pszRMName,pIResMgrSink,riidRequested,ppvResMgr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManagerFactory2_INTERFACE_DEFINED__ */


#ifndef __IPrepareInfo_INTERFACE_DEFINED__
#define __IPrepareInfo_INTERFACE_DEFINED__

/* interface IPrepareInfo */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_IPrepareInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80c7bfd0-87ee-11ce-8081-0080c758527e")
    IPrepareInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPrepareInfoSize( 
            /* [out] */ ULONG *pcbPrepInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrepareInfo( 
            /* [out] */ byte *pPrepInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrepareInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrepareInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrepareInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrepareInfo * This);
        
        DECLSPEC_XFGVIRT(IPrepareInfo, GetPrepareInfoSize)
        HRESULT ( STDMETHODCALLTYPE *GetPrepareInfoSize )( 
            IPrepareInfo * This,
            /* [out] */ ULONG *pcbPrepInfo);
        
        DECLSPEC_XFGVIRT(IPrepareInfo, GetPrepareInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPrepareInfo )( 
            IPrepareInfo * This,
            /* [out] */ byte *pPrepInfo);
        
        END_INTERFACE
    } IPrepareInfoVtbl;

    interface IPrepareInfo
    {
        CONST_VTBL struct IPrepareInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrepareInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrepareInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrepareInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrepareInfo_GetPrepareInfoSize(This,pcbPrepInfo)	\
    ( (This)->lpVtbl -> GetPrepareInfoSize(This,pcbPrepInfo) ) 

#define IPrepareInfo_GetPrepareInfo(This,pPrepInfo)	\
    ( (This)->lpVtbl -> GetPrepareInfo(This,pPrepInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrepareInfo_INTERFACE_DEFINED__ */


#ifndef __IPrepareInfo2_INTERFACE_DEFINED__
#define __IPrepareInfo2_INTERFACE_DEFINED__

/* interface IPrepareInfo2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPrepareInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5FAB2547-9779-11d1-B886-00C04FB9618A")
    IPrepareInfo2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPrepareInfoSize( 
            /* [out] */ __RPC__out ULONG *pcbPrepInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrepareInfo( 
            /* [in] */ ULONG cbPrepareInfo,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPrepareInfo) byte *pPrepInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrepareInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrepareInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrepareInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrepareInfo2 * This);
        
        DECLSPEC_XFGVIRT(IPrepareInfo2, GetPrepareInfoSize)
        HRESULT ( STDMETHODCALLTYPE *GetPrepareInfoSize )( 
            __RPC__in IPrepareInfo2 * This,
            /* [out] */ __RPC__out ULONG *pcbPrepInfo);
        
        DECLSPEC_XFGVIRT(IPrepareInfo2, GetPrepareInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPrepareInfo )( 
            __RPC__in IPrepareInfo2 * This,
            /* [in] */ ULONG cbPrepareInfo,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPrepareInfo) byte *pPrepInfo);
        
        END_INTERFACE
    } IPrepareInfo2Vtbl;

    interface IPrepareInfo2
    {
        CONST_VTBL struct IPrepareInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrepareInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrepareInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrepareInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrepareInfo2_GetPrepareInfoSize(This,pcbPrepInfo)	\
    ( (This)->lpVtbl -> GetPrepareInfoSize(This,pcbPrepInfo) ) 

#define IPrepareInfo2_GetPrepareInfo(This,cbPrepareInfo,pPrepInfo)	\
    ( (This)->lpVtbl -> GetPrepareInfo(This,cbPrepareInfo,pPrepInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrepareInfo2_INTERFACE_DEFINED__ */


#ifndef __IGetDispenser_INTERFACE_DEFINED__
#define __IGetDispenser_INTERFACE_DEFINED__

/* interface IGetDispenser */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IGetDispenser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c23cc370-87ef-11ce-8081-0080c758527e")
    IGetDispenser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDispenser( 
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetDispenserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetDispenser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetDispenser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetDispenser * This);
        
        DECLSPEC_XFGVIRT(IGetDispenser, GetDispenser)
        HRESULT ( STDMETHODCALLTYPE *GetDispenser )( 
            __RPC__in IGetDispenser * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject);
        
        END_INTERFACE
    } IGetDispenserVtbl;

    interface IGetDispenser
    {
        CONST_VTBL struct IGetDispenserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetDispenser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetDispenser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetDispenser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetDispenser_GetDispenser(This,iid,ppvObject)	\
    ( (This)->lpVtbl -> GetDispenser(This,iid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetDispenser_INTERFACE_DEFINED__ */


#ifndef __ITransactionVoterBallotAsync2_INTERFACE_DEFINED__
#define __ITransactionVoterBallotAsync2_INTERFACE_DEFINED__

/* interface ITransactionVoterBallotAsync2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionVoterBallotAsync2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5433376C-414D-11d3-B206-00C04FC2F3EF")
    ITransactionVoterBallotAsync2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE VoteRequestDone( 
            /* [in] */ HRESULT hr,
            /* [unique][in] */ __RPC__in_opt BOID *pboidReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionVoterBallotAsync2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionVoterBallotAsync2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionVoterBallotAsync2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionVoterBallotAsync2 * This);
        
        DECLSPEC_XFGVIRT(ITransactionVoterBallotAsync2, VoteRequestDone)
        HRESULT ( STDMETHODCALLTYPE *VoteRequestDone )( 
            __RPC__in ITransactionVoterBallotAsync2 * This,
            /* [in] */ HRESULT hr,
            /* [unique][in] */ __RPC__in_opt BOID *pboidReason);
        
        END_INTERFACE
    } ITransactionVoterBallotAsync2Vtbl;

    interface ITransactionVoterBallotAsync2
    {
        CONST_VTBL struct ITransactionVoterBallotAsync2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionVoterBallotAsync2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionVoterBallotAsync2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionVoterBallotAsync2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionVoterBallotAsync2_VoteRequestDone(This,hr,pboidReason)	\
    ( (This)->lpVtbl -> VoteRequestDone(This,hr,pboidReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionVoterBallotAsync2_INTERFACE_DEFINED__ */


#ifndef __ITransactionVoterNotifyAsync2_INTERFACE_DEFINED__
#define __ITransactionVoterNotifyAsync2_INTERFACE_DEFINED__

/* interface ITransactionVoterNotifyAsync2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionVoterNotifyAsync2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5433376B-414D-11d3-B206-00C04FC2F3EF")
    ITransactionVoterNotifyAsync2 : public ITransactionOutcomeEvents
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE VoteRequest( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionVoterNotifyAsync2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This);
        
        DECLSPEC_XFGVIRT(ITransactionOutcomeEvents, Committed)
        HRESULT ( STDMETHODCALLTYPE *Committed )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This,
            /* [in] */ BOOL fRetaining,
            /* [unique][in] */ __RPC__in_opt XACTUOW *pNewUOW,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITransactionOutcomeEvents, Aborted)
        HRESULT ( STDMETHODCALLTYPE *Aborted )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This,
            /* [unique][in] */ __RPC__in_opt BOID *pboidReason,
            /* [in] */ BOOL fRetaining,
            /* [unique][in] */ __RPC__in_opt XACTUOW *pNewUOW,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITransactionOutcomeEvents, HeuristicDecision)
        HRESULT ( STDMETHODCALLTYPE *HeuristicDecision )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This,
            /* [in] */ DWORD dwDecision,
            /* [unique][in] */ __RPC__in_opt BOID *pboidReason,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(ITransactionOutcomeEvents, Indoubt)
        HRESULT ( STDMETHODCALLTYPE *Indoubt )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This);
        
        DECLSPEC_XFGVIRT(ITransactionVoterNotifyAsync2, VoteRequest)
        HRESULT ( STDMETHODCALLTYPE *VoteRequest )( 
            __RPC__in ITransactionVoterNotifyAsync2 * This);
        
        END_INTERFACE
    } ITransactionVoterNotifyAsync2Vtbl;

    interface ITransactionVoterNotifyAsync2
    {
        CONST_VTBL struct ITransactionVoterNotifyAsync2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionVoterNotifyAsync2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionVoterNotifyAsync2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionVoterNotifyAsync2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionVoterNotifyAsync2_Committed(This,fRetaining,pNewUOW,hr)	\
    ( (This)->lpVtbl -> Committed(This,fRetaining,pNewUOW,hr) ) 

#define ITransactionVoterNotifyAsync2_Aborted(This,pboidReason,fRetaining,pNewUOW,hr)	\
    ( (This)->lpVtbl -> Aborted(This,pboidReason,fRetaining,pNewUOW,hr) ) 

#define ITransactionVoterNotifyAsync2_HeuristicDecision(This,dwDecision,pboidReason,hr)	\
    ( (This)->lpVtbl -> HeuristicDecision(This,dwDecision,pboidReason,hr) ) 

#define ITransactionVoterNotifyAsync2_Indoubt(This)	\
    ( (This)->lpVtbl -> Indoubt(This) ) 


#define ITransactionVoterNotifyAsync2_VoteRequest(This)	\
    ( (This)->lpVtbl -> VoteRequest(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionVoterNotifyAsync2_INTERFACE_DEFINED__ */


#ifndef __ITransactionVoterFactory2_INTERFACE_DEFINED__
#define __ITransactionVoterFactory2_INTERFACE_DEFINED__

/* interface ITransactionVoterFactory2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionVoterFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5433376A-414D-11d3-B206-00C04FC2F3EF")
    ITransactionVoterFactory2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionVoterNotifyAsync2 *pVoterNotify,
            /* [out] */ __RPC__deref_out_opt ITransactionVoterBallotAsync2 **ppVoterBallot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionVoterFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionVoterFactory2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionVoterFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionVoterFactory2 * This);
        
        DECLSPEC_XFGVIRT(ITransactionVoterFactory2, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in ITransactionVoterFactory2 * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction,
            /* [in] */ __RPC__in_opt ITransactionVoterNotifyAsync2 *pVoterNotify,
            /* [out] */ __RPC__deref_out_opt ITransactionVoterBallotAsync2 **ppVoterBallot);
        
        END_INTERFACE
    } ITransactionVoterFactory2Vtbl;

    interface ITransactionVoterFactory2
    {
        CONST_VTBL struct ITransactionVoterFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionVoterFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionVoterFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionVoterFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionVoterFactory2_Create(This,pTransaction,pVoterNotify,ppVoterBallot)	\
    ( (This)->lpVtbl -> Create(This,pTransaction,pVoterNotify,ppVoterBallot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionVoterFactory2_INTERFACE_DEFINED__ */


#ifndef __ITransactionPhase0EnlistmentAsync_INTERFACE_DEFINED__
#define __ITransactionPhase0EnlistmentAsync_INTERFACE_DEFINED__

/* interface ITransactionPhase0EnlistmentAsync */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionPhase0EnlistmentAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82DC88E1-A954-11d1-8F88-00600895E7D5")
    ITransactionPhase0EnlistmentAsync : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Enable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitForEnlistment( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Phase0Done( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unenlist( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransaction( 
            /* [out] */ __RPC__deref_out_opt ITransaction **ppITransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionPhase0EnlistmentAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0EnlistmentAsync, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0EnlistmentAsync, WaitForEnlistment)
        HRESULT ( STDMETHODCALLTYPE *WaitForEnlistment )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0EnlistmentAsync, Phase0Done)
        HRESULT ( STDMETHODCALLTYPE *Phase0Done )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0EnlistmentAsync, Unenlist)
        HRESULT ( STDMETHODCALLTYPE *Unenlist )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0EnlistmentAsync, GetTransaction)
        HRESULT ( STDMETHODCALLTYPE *GetTransaction )( 
            __RPC__in ITransactionPhase0EnlistmentAsync * This,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppITransaction);
        
        END_INTERFACE
    } ITransactionPhase0EnlistmentAsyncVtbl;

    interface ITransactionPhase0EnlistmentAsync
    {
        CONST_VTBL struct ITransactionPhase0EnlistmentAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionPhase0EnlistmentAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionPhase0EnlistmentAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionPhase0EnlistmentAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionPhase0EnlistmentAsync_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define ITransactionPhase0EnlistmentAsync_WaitForEnlistment(This)	\
    ( (This)->lpVtbl -> WaitForEnlistment(This) ) 

#define ITransactionPhase0EnlistmentAsync_Phase0Done(This)	\
    ( (This)->lpVtbl -> Phase0Done(This) ) 

#define ITransactionPhase0EnlistmentAsync_Unenlist(This)	\
    ( (This)->lpVtbl -> Unenlist(This) ) 

#define ITransactionPhase0EnlistmentAsync_GetTransaction(This,ppITransaction)	\
    ( (This)->lpVtbl -> GetTransaction(This,ppITransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionPhase0EnlistmentAsync_INTERFACE_DEFINED__ */


#ifndef __ITransactionPhase0NotifyAsync_INTERFACE_DEFINED__
#define __ITransactionPhase0NotifyAsync_INTERFACE_DEFINED__

/* interface ITransactionPhase0NotifyAsync */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionPhase0NotifyAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF081809-0C76-11d2-87A6-00C04F990F34")
    ITransactionPhase0NotifyAsync : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Phase0Request( 
            BOOL fAbortingHint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnlistCompleted( 
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionPhase0NotifyAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionPhase0NotifyAsync * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionPhase0NotifyAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionPhase0NotifyAsync * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0NotifyAsync, Phase0Request)
        HRESULT ( STDMETHODCALLTYPE *Phase0Request )( 
            __RPC__in ITransactionPhase0NotifyAsync * This,
            BOOL fAbortingHint);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0NotifyAsync, EnlistCompleted)
        HRESULT ( STDMETHODCALLTYPE *EnlistCompleted )( 
            __RPC__in ITransactionPhase0NotifyAsync * This,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } ITransactionPhase0NotifyAsyncVtbl;

    interface ITransactionPhase0NotifyAsync
    {
        CONST_VTBL struct ITransactionPhase0NotifyAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionPhase0NotifyAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionPhase0NotifyAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionPhase0NotifyAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionPhase0NotifyAsync_Phase0Request(This,fAbortingHint)	\
    ( (This)->lpVtbl -> Phase0Request(This,fAbortingHint) ) 

#define ITransactionPhase0NotifyAsync_EnlistCompleted(This,status)	\
    ( (This)->lpVtbl -> EnlistCompleted(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionPhase0NotifyAsync_INTERFACE_DEFINED__ */


#ifndef __ITransactionPhase0Factory_INTERFACE_DEFINED__
#define __ITransactionPhase0Factory_INTERFACE_DEFINED__

/* interface ITransactionPhase0Factory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionPhase0Factory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82DC88E0-A954-11d1-8F88-00600895E7D5")
    ITransactionPhase0Factory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in_opt ITransactionPhase0NotifyAsync *pPhase0Notify,
            /* [out] */ __RPC__deref_out_opt ITransactionPhase0EnlistmentAsync **ppPhase0Enlistment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionPhase0FactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionPhase0Factory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionPhase0Factory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionPhase0Factory * This);
        
        DECLSPEC_XFGVIRT(ITransactionPhase0Factory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in ITransactionPhase0Factory * This,
            /* [in] */ __RPC__in_opt ITransactionPhase0NotifyAsync *pPhase0Notify,
            /* [out] */ __RPC__deref_out_opt ITransactionPhase0EnlistmentAsync **ppPhase0Enlistment);
        
        END_INTERFACE
    } ITransactionPhase0FactoryVtbl;

    interface ITransactionPhase0Factory
    {
        CONST_VTBL struct ITransactionPhase0FactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionPhase0Factory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionPhase0Factory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionPhase0Factory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionPhase0Factory_Create(This,pPhase0Notify,ppPhase0Enlistment)	\
    ( (This)->lpVtbl -> Create(This,pPhase0Notify,ppPhase0Enlistment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionPhase0Factory_INTERFACE_DEFINED__ */


#ifndef __ITransactionTransmitter_INTERFACE_DEFINED__
#define __ITransactionTransmitter_INTERFACE_DEFINED__

/* interface ITransactionTransmitter */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionTransmitter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59313E01-B36C-11cf-A539-00AA006887C3")
    ITransactionTransmitter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ __RPC__in_opt ITransaction *pTransaction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropagationTokenSize( 
            /* [out] */ __RPC__out ULONG *pcbToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MarshalPropagationToken( 
            /* [in] */ ULONG cbToken,
            /* [size_is][out] */ __RPC__out_ecount_full(cbToken) byte *rgbToken,
            /* [out] */ __RPC__out ULONG *pcbUsed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnmarshalReturnToken( 
            /* [in] */ ULONG cbReturnToken,
            /* [size_is][in] */ __RPC__in_ecount_full(cbReturnToken) byte *rgbReturnToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionTransmitterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionTransmitter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionTransmitter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionTransmitter * This);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitter, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            __RPC__in ITransactionTransmitter * This,
            /* [in] */ __RPC__in_opt ITransaction *pTransaction);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitter, GetPropagationTokenSize)
        HRESULT ( STDMETHODCALLTYPE *GetPropagationTokenSize )( 
            __RPC__in ITransactionTransmitter * This,
            /* [out] */ __RPC__out ULONG *pcbToken);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitter, MarshalPropagationToken)
        HRESULT ( STDMETHODCALLTYPE *MarshalPropagationToken )( 
            __RPC__in ITransactionTransmitter * This,
            /* [in] */ ULONG cbToken,
            /* [size_is][out] */ __RPC__out_ecount_full(cbToken) byte *rgbToken,
            /* [out] */ __RPC__out ULONG *pcbUsed);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitter, UnmarshalReturnToken)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalReturnToken )( 
            __RPC__in ITransactionTransmitter * This,
            /* [in] */ ULONG cbReturnToken,
            /* [size_is][in] */ __RPC__in_ecount_full(cbReturnToken) byte *rgbReturnToken);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitter, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITransactionTransmitter * This);
        
        END_INTERFACE
    } ITransactionTransmitterVtbl;

    interface ITransactionTransmitter
    {
        CONST_VTBL struct ITransactionTransmitterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionTransmitter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionTransmitter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionTransmitter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionTransmitter_Set(This,pTransaction)	\
    ( (This)->lpVtbl -> Set(This,pTransaction) ) 

#define ITransactionTransmitter_GetPropagationTokenSize(This,pcbToken)	\
    ( (This)->lpVtbl -> GetPropagationTokenSize(This,pcbToken) ) 

#define ITransactionTransmitter_MarshalPropagationToken(This,cbToken,rgbToken,pcbUsed)	\
    ( (This)->lpVtbl -> MarshalPropagationToken(This,cbToken,rgbToken,pcbUsed) ) 

#define ITransactionTransmitter_UnmarshalReturnToken(This,cbReturnToken,rgbReturnToken)	\
    ( (This)->lpVtbl -> UnmarshalReturnToken(This,cbReturnToken,rgbReturnToken) ) 

#define ITransactionTransmitter_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionTransmitter_INTERFACE_DEFINED__ */


#ifndef __ITransactionTransmitterFactory_INTERFACE_DEFINED__
#define __ITransactionTransmitterFactory_INTERFACE_DEFINED__

/* interface ITransactionTransmitterFactory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionTransmitterFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59313E00-B36C-11cf-A539-00AA006887C3")
    ITransactionTransmitterFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [out] */ __RPC__deref_out_opt ITransactionTransmitter **ppTransmitter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionTransmitterFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionTransmitterFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionTransmitterFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionTransmitterFactory * This);
        
        DECLSPEC_XFGVIRT(ITransactionTransmitterFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in ITransactionTransmitterFactory * This,
            /* [out] */ __RPC__deref_out_opt ITransactionTransmitter **ppTransmitter);
        
        END_INTERFACE
    } ITransactionTransmitterFactoryVtbl;

    interface ITransactionTransmitterFactory
    {
        CONST_VTBL struct ITransactionTransmitterFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionTransmitterFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionTransmitterFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionTransmitterFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionTransmitterFactory_Create(This,ppTransmitter)	\
    ( (This)->lpVtbl -> Create(This,ppTransmitter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionTransmitterFactory_INTERFACE_DEFINED__ */


#ifndef __ITransactionReceiver_INTERFACE_DEFINED__
#define __ITransactionReceiver_INTERFACE_DEFINED__

/* interface ITransactionReceiver */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionReceiver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59313E03-B36C-11cf-A539-00AA006887C3")
    ITransactionReceiver : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UnmarshalPropagationToken( 
            /* [in] */ ULONG cbToken,
            /* [size_is][in] */ __RPC__in_ecount_full(cbToken) byte *rgbToken,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReturnTokenSize( 
            /* [out] */ __RPC__out ULONG *pcbReturnToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MarshalReturnToken( 
            /* [in] */ ULONG cbReturnToken,
            /* [size_is][out] */ __RPC__out_ecount_full(cbReturnToken) byte *rgbReturnToken,
            /* [out] */ __RPC__out ULONG *pcbUsed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionReceiverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionReceiver * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionReceiver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionReceiver * This);
        
        DECLSPEC_XFGVIRT(ITransactionReceiver, UnmarshalPropagationToken)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalPropagationToken )( 
            __RPC__in ITransactionReceiver * This,
            /* [in] */ ULONG cbToken,
            /* [size_is][in] */ __RPC__in_ecount_full(cbToken) byte *rgbToken,
            /* [out] */ __RPC__deref_out_opt ITransaction **ppTransaction);
        
        DECLSPEC_XFGVIRT(ITransactionReceiver, GetReturnTokenSize)
        HRESULT ( STDMETHODCALLTYPE *GetReturnTokenSize )( 
            __RPC__in ITransactionReceiver * This,
            /* [out] */ __RPC__out ULONG *pcbReturnToken);
        
        DECLSPEC_XFGVIRT(ITransactionReceiver, MarshalReturnToken)
        HRESULT ( STDMETHODCALLTYPE *MarshalReturnToken )( 
            __RPC__in ITransactionReceiver * This,
            /* [in] */ ULONG cbReturnToken,
            /* [size_is][out] */ __RPC__out_ecount_full(cbReturnToken) byte *rgbReturnToken,
            /* [out] */ __RPC__out ULONG *pcbUsed);
        
        DECLSPEC_XFGVIRT(ITransactionReceiver, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITransactionReceiver * This);
        
        END_INTERFACE
    } ITransactionReceiverVtbl;

    interface ITransactionReceiver
    {
        CONST_VTBL struct ITransactionReceiverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionReceiver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionReceiver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionReceiver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionReceiver_UnmarshalPropagationToken(This,cbToken,rgbToken,ppTransaction)	\
    ( (This)->lpVtbl -> UnmarshalPropagationToken(This,cbToken,rgbToken,ppTransaction) ) 

#define ITransactionReceiver_GetReturnTokenSize(This,pcbReturnToken)	\
    ( (This)->lpVtbl -> GetReturnTokenSize(This,pcbReturnToken) ) 

#define ITransactionReceiver_MarshalReturnToken(This,cbReturnToken,rgbReturnToken,pcbUsed)	\
    ( (This)->lpVtbl -> MarshalReturnToken(This,cbReturnToken,rgbReturnToken,pcbUsed) ) 

#define ITransactionReceiver_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionReceiver_INTERFACE_DEFINED__ */


#ifndef __ITransactionReceiverFactory_INTERFACE_DEFINED__
#define __ITransactionReceiverFactory_INTERFACE_DEFINED__

/* interface ITransactionReceiverFactory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ITransactionReceiverFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59313E02-B36C-11cf-A539-00AA006887C3")
    ITransactionReceiverFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [out] */ __RPC__deref_out_opt ITransactionReceiver **ppReceiver) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransactionReceiverFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransactionReceiverFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransactionReceiverFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransactionReceiverFactory * This);
        
        DECLSPEC_XFGVIRT(ITransactionReceiverFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in ITransactionReceiverFactory * This,
            /* [out] */ __RPC__deref_out_opt ITransactionReceiver **ppReceiver);
        
        END_INTERFACE
    } ITransactionReceiverFactoryVtbl;

    interface ITransactionReceiverFactory
    {
        CONST_VTBL struct ITransactionReceiverFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransactionReceiverFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransactionReceiverFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransactionReceiverFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransactionReceiverFactory_Create(This,ppReceiver)	\
    ( (This)->lpVtbl -> Create(This,ppReceiver) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransactionReceiverFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_txdtc_0000_0025 */
/* [local] */ 

typedef struct _ProxyConfigParams
    {
    WORD wcThreadsMax;
    } 	PROXY_CONFIG_PARAMS;



extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0025_v0_0_s_ifspec;

#ifndef __IDtcLuConfigure_INTERFACE_DEFINED__
#define __IDtcLuConfigure_INTERFACE_DEFINED__

/* interface IDtcLuConfigure */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuConfigure;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E760-1AEA-11d0-944B-00A0C905416E")
    IDtcLuConfigure : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuConfigureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuConfigure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuConfigure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuConfigure * This);
        
        DECLSPEC_XFGVIRT(IDtcLuConfigure, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IDtcLuConfigure * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair);
        
        DECLSPEC_XFGVIRT(IDtcLuConfigure, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IDtcLuConfigure * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair);
        
        END_INTERFACE
    } IDtcLuConfigureVtbl;

    interface IDtcLuConfigure
    {
        CONST_VTBL struct IDtcLuConfigureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuConfigure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuConfigure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuConfigure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuConfigure_Add(This,pucLuPair,cbLuPair)	\
    ( (This)->lpVtbl -> Add(This,pucLuPair,cbLuPair) ) 

#define IDtcLuConfigure_Delete(This,pucLuPair,cbLuPair)	\
    ( (This)->lpVtbl -> Delete(This,pucLuPair,cbLuPair) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuConfigure_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecovery_INTERFACE_DEFINED__
#define __IDtcLuRecovery_INTERFACE_DEFINED__

/* interface IDtcLuRecovery */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecovery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC2B8AD2-D6F0-11d0-B386-00A0C9083365")
    IDtcLuRecovery : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRecovery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRecovery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRecovery * This);
        
        END_INTERFACE
    } IDtcLuRecoveryVtbl;

    interface IDtcLuRecovery
    {
        CONST_VTBL struct IDtcLuRecoveryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecovery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecovery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecovery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecovery_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecoveryFactory_INTERFACE_DEFINED__
#define __IDtcLuRecoveryFactory_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryFactory */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E762-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRecoveryFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [out] */ __RPC__deref_out_opt IDtcLuRecovery **ppRecovery) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRecoveryFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRecoveryFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRecoveryFactory * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IDtcLuRecoveryFactory * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLuPair) byte *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [out] */ __RPC__deref_out_opt IDtcLuRecovery **ppRecovery);
        
        END_INTERFACE
    } IDtcLuRecoveryFactoryVtbl;

    interface IDtcLuRecoveryFactory
    {
        CONST_VTBL struct IDtcLuRecoveryFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryFactory_Create(This,pucLuPair,cbLuPair,ppRecovery)	\
    ( (This)->lpVtbl -> Create(This,pucLuPair,cbLuPair,ppRecovery) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_txdtc_0000_0028 */
/* [local] */ 

typedef 
enum _DtcLu_LocalRecovery_Work
    {
        DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS	= 1,
        DTCINITIATEDRECOVERYWORK_TRANS	= ( DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS + 1 ) ,
        DTCINITIATEDRECOVERYWORK_TMDOWN	= ( DTCINITIATEDRECOVERYWORK_TRANS + 1 ) 
    } 	DTCINITIATEDRECOVERYWORK;

typedef 
enum _DtcLu_Xln
    {
        DTCLUXLN_COLD	= 1,
        DTCLUXLN_WARM	= ( DTCLUXLN_COLD + 1 ) 
    } 	DTCLUXLN;

typedef 
enum _DtcLu_Xln_Confirmation
    {
        DTCLUXLNCONFIRMATION_CONFIRM	= 1,
        DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH	= ( DTCLUXLNCONFIRMATION_CONFIRM + 1 ) ,
        DTCLUXLNCONFIRMATION_COLDWARMMISMATCH	= ( DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH + 1 ) ,
        DTCLUXLNCONFIRMATION_OBSOLETE	= ( DTCLUXLNCONFIRMATION_COLDWARMMISMATCH + 1 ) 
    } 	DTCLUXLNCONFIRMATION;

typedef 
enum _DtcLu_Xln_Response
    {
        DTCLUXLNRESPONSE_OK_SENDOURXLNBACK	= 1,
        DTCLUXLNRESPONSE_OK_SENDCONFIRMATION	= ( DTCLUXLNRESPONSE_OK_SENDOURXLNBACK + 1 ) ,
        DTCLUXLNRESPONSE_LOGNAMEMISMATCH	= ( DTCLUXLNRESPONSE_OK_SENDCONFIRMATION + 1 ) ,
        DTCLUXLNRESPONSE_COLDWARMMISMATCH	= ( DTCLUXLNRESPONSE_LOGNAMEMISMATCH + 1 ) 
    } 	DTCLUXLNRESPONSE;

typedef 
enum _DtcLu_Xln_Error
    {
        DTCLUXLNERROR_PROTOCOL	= 1,
        DTCLUXLNERROR_LOGNAMEMISMATCH	= ( DTCLUXLNERROR_PROTOCOL + 1 ) ,
        DTCLUXLNERROR_COLDWARMMISMATCH	= ( DTCLUXLNERROR_LOGNAMEMISMATCH + 1 ) 
    } 	DTCLUXLNERROR;

typedef 
enum _DtcLu_CompareState
    {
        DTCLUCOMPARESTATE_COMMITTED	= 1,
        DTCLUCOMPARESTATE_HEURISTICCOMMITTED	= ( DTCLUCOMPARESTATE_COMMITTED + 1 ) ,
        DTCLUCOMPARESTATE_HEURISTICMIXED	= ( DTCLUCOMPARESTATE_HEURISTICCOMMITTED + 1 ) ,
        DTCLUCOMPARESTATE_HEURISTICRESET	= ( DTCLUCOMPARESTATE_HEURISTICMIXED + 1 ) ,
        DTCLUCOMPARESTATE_INDOUBT	= ( DTCLUCOMPARESTATE_HEURISTICRESET + 1 ) ,
        DTCLUCOMPARESTATE_RESET	= ( DTCLUCOMPARESTATE_INDOUBT + 1 ) 
    } 	DTCLUCOMPARESTATE;

typedef 
enum _DtcLu_CompareStates_Confirmation
    {
        DTCLUCOMPARESTATESCONFIRMATION_CONFIRM	= 1,
        DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL	= ( DTCLUCOMPARESTATESCONFIRMATION_CONFIRM + 1 ) 
    } 	DTCLUCOMPARESTATESCONFIRMATION;

typedef 
enum _DtcLu_CompareStates_Error
    {
        DTCLUCOMPARESTATESERROR_PROTOCOL	= 1
    } 	DTCLUCOMPARESTATESERROR;

typedef 
enum _DtcLu_CompareStates_Response
    {
        DTCLUCOMPARESTATESRESPONSE_OK	= 1,
        DTCLUCOMPARESTATESRESPONSE_PROTOCOL	= ( DTCLUCOMPARESTATESRESPONSE_OK + 1 ) 
    } 	DTCLUCOMPARESTATESRESPONSE;



extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0028_v0_0_s_ifspec;

#ifndef __IDtcLuRecoveryInitiatedByDtcTransWork_INTERFACE_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtcTransWork_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryInitiatedByDtcTransWork */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryInitiatedByDtcTransWork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E765-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRecoveryInitiatedByDtcTransWork : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLogNameSizes( 
            /* [out] */ DWORD *pcbOurLogName,
            /* [out] */ DWORD *pcbRemoteLogName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOurXln( 
            /* [out] */ DTCLUXLN *pXln,
            /* [out][in] */ unsigned char *pOurLogName,
            /* [out][in] */ unsigned char *pRemoteLogName,
            /* [out] */ DWORD *pdwProtocol) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleConfirmationFromOurXln( 
            /* [in] */ DTCLUXLNCONFIRMATION Confirmation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleTheirXlnResponse( 
            /* [in] */ DTCLUXLN Xln,
            /* [in] */ unsigned char *pRemoteLogName,
            /* [in] */ DWORD cbRemoteLogName,
            /* [in] */ DWORD dwProtocol,
            /* [out] */ DTCLUXLNCONFIRMATION *pConfirmation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleErrorFromOurXln( 
            /* [in] */ DTCLUXLNERROR Error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckForCompareStates( 
            /* [out] */ BOOL *fCompareStates) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOurTransIdSize( 
            /* [out][in] */ DWORD *pcbOurTransId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOurCompareStates( 
            /* [out][in] */ unsigned char *pOurTransId,
            /* [out] */ DTCLUCOMPARESTATE *pCompareState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleTheirCompareStatesResponse( 
            /* [in] */ DTCLUCOMPARESTATE CompareState,
            /* [out] */ DTCLUCOMPARESTATESCONFIRMATION *pConfirmation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleErrorFromOurCompareStates( 
            /* [in] */ DTCLUCOMPARESTATESERROR Error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConversationLost( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecoverySeqNum( 
            /* [out] */ LONG *plRecoverySeqNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ObsoleteRecoverySeqNum( 
            /* [in] */ LONG lNewRecoverySeqNum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryInitiatedByDtcTransWorkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, GetLogNameSizes)
        HRESULT ( STDMETHODCALLTYPE *GetLogNameSizes )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out] */ DWORD *pcbOurLogName,
            /* [out] */ DWORD *pcbRemoteLogName);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, GetOurXln)
        HRESULT ( STDMETHODCALLTYPE *GetOurXln )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out] */ DTCLUXLN *pXln,
            /* [out][in] */ unsigned char *pOurLogName,
            /* [out][in] */ unsigned char *pRemoteLogName,
            /* [out] */ DWORD *pdwProtocol);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, HandleConfirmationFromOurXln)
        HRESULT ( STDMETHODCALLTYPE *HandleConfirmationFromOurXln )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ DTCLUXLNCONFIRMATION Confirmation);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, HandleTheirXlnResponse)
        HRESULT ( STDMETHODCALLTYPE *HandleTheirXlnResponse )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ DTCLUXLN Xln,
            /* [in] */ unsigned char *pRemoteLogName,
            /* [in] */ DWORD cbRemoteLogName,
            /* [in] */ DWORD dwProtocol,
            /* [out] */ DTCLUXLNCONFIRMATION *pConfirmation);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, HandleErrorFromOurXln)
        HRESULT ( STDMETHODCALLTYPE *HandleErrorFromOurXln )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ DTCLUXLNERROR Error);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, CheckForCompareStates)
        HRESULT ( STDMETHODCALLTYPE *CheckForCompareStates )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out] */ BOOL *fCompareStates);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, GetOurTransIdSize)
        HRESULT ( STDMETHODCALLTYPE *GetOurTransIdSize )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out][in] */ DWORD *pcbOurTransId);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, GetOurCompareStates)
        HRESULT ( STDMETHODCALLTYPE *GetOurCompareStates )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out][in] */ unsigned char *pOurTransId,
            /* [out] */ DTCLUCOMPARESTATE *pCompareState);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, HandleTheirCompareStatesResponse)
        HRESULT ( STDMETHODCALLTYPE *HandleTheirCompareStatesResponse )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ DTCLUCOMPARESTATE CompareState,
            /* [out] */ DTCLUCOMPARESTATESCONFIRMATION *pConfirmation);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, HandleErrorFromOurCompareStates)
        HRESULT ( STDMETHODCALLTYPE *HandleErrorFromOurCompareStates )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ DTCLUCOMPARESTATESERROR Error);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, ConversationLost)
        HRESULT ( STDMETHODCALLTYPE *ConversationLost )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, GetRecoverySeqNum)
        HRESULT ( STDMETHODCALLTYPE *GetRecoverySeqNum )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [out] */ LONG *plRecoverySeqNum);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcTransWork, ObsoleteRecoverySeqNum)
        HRESULT ( STDMETHODCALLTYPE *ObsoleteRecoverySeqNum )( 
            IDtcLuRecoveryInitiatedByDtcTransWork * This,
            /* [in] */ LONG lNewRecoverySeqNum);
        
        END_INTERFACE
    } IDtcLuRecoveryInitiatedByDtcTransWorkVtbl;

    interface IDtcLuRecoveryInitiatedByDtcTransWork
    {
        CONST_VTBL struct IDtcLuRecoveryInitiatedByDtcTransWorkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryInitiatedByDtcTransWork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryInitiatedByDtcTransWork_GetLogNameSizes(This,pcbOurLogName,pcbRemoteLogName)	\
    ( (This)->lpVtbl -> GetLogNameSizes(This,pcbOurLogName,pcbRemoteLogName) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_GetOurXln(This,pXln,pOurLogName,pRemoteLogName,pdwProtocol)	\
    ( (This)->lpVtbl -> GetOurXln(This,pXln,pOurLogName,pRemoteLogName,pdwProtocol) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_HandleConfirmationFromOurXln(This,Confirmation)	\
    ( (This)->lpVtbl -> HandleConfirmationFromOurXln(This,Confirmation) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_HandleTheirXlnResponse(This,Xln,pRemoteLogName,cbRemoteLogName,dwProtocol,pConfirmation)	\
    ( (This)->lpVtbl -> HandleTheirXlnResponse(This,Xln,pRemoteLogName,cbRemoteLogName,dwProtocol,pConfirmation) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_HandleErrorFromOurXln(This,Error)	\
    ( (This)->lpVtbl -> HandleErrorFromOurXln(This,Error) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_CheckForCompareStates(This,fCompareStates)	\
    ( (This)->lpVtbl -> CheckForCompareStates(This,fCompareStates) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_GetOurTransIdSize(This,pcbOurTransId)	\
    ( (This)->lpVtbl -> GetOurTransIdSize(This,pcbOurTransId) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_GetOurCompareStates(This,pOurTransId,pCompareState)	\
    ( (This)->lpVtbl -> GetOurCompareStates(This,pOurTransId,pCompareState) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_HandleTheirCompareStatesResponse(This,CompareState,pConfirmation)	\
    ( (This)->lpVtbl -> HandleTheirCompareStatesResponse(This,CompareState,pConfirmation) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_HandleErrorFromOurCompareStates(This,Error)	\
    ( (This)->lpVtbl -> HandleErrorFromOurCompareStates(This,Error) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_ConversationLost(This)	\
    ( (This)->lpVtbl -> ConversationLost(This) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_GetRecoverySeqNum(This,plRecoverySeqNum)	\
    ( (This)->lpVtbl -> GetRecoverySeqNum(This,plRecoverySeqNum) ) 

#define IDtcLuRecoveryInitiatedByDtcTransWork_ObsoleteRecoverySeqNum(This,lNewRecoverySeqNum)	\
    ( (This)->lpVtbl -> ObsoleteRecoverySeqNum(This,lNewRecoverySeqNum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryInitiatedByDtcTransWork_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByDtcStatusWork_INTERFACE_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtcStatusWork_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryInitiatedByDtcStatusWork */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryInitiatedByDtcStatusWork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E766-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRecoveryInitiatedByDtcStatusWork : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandleCheckLuStatus( 
            /* [in] */ LONG lRecoverySeqNum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRecoveryInitiatedByDtcStatusWork * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRecoveryInitiatedByDtcStatusWork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRecoveryInitiatedByDtcStatusWork * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtcStatusWork, HandleCheckLuStatus)
        HRESULT ( STDMETHODCALLTYPE *HandleCheckLuStatus )( 
            __RPC__in IDtcLuRecoveryInitiatedByDtcStatusWork * This,
            /* [in] */ LONG lRecoverySeqNum);
        
        END_INTERFACE
    } IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl;

    interface IDtcLuRecoveryInitiatedByDtcStatusWork
    {
        CONST_VTBL struct IDtcLuRecoveryInitiatedByDtcStatusWorkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryInitiatedByDtcStatusWork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryInitiatedByDtcStatusWork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryInitiatedByDtcStatusWork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryInitiatedByDtcStatusWork_HandleCheckLuStatus(This,lRecoverySeqNum)	\
    ( (This)->lpVtbl -> HandleCheckLuStatus(This,lRecoverySeqNum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryInitiatedByDtcStatusWork_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByDtc_INTERFACE_DEFINED__
#define __IDtcLuRecoveryInitiatedByDtc_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryInitiatedByDtc */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryInitiatedByDtc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E764-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRecoveryInitiatedByDtc : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWork( 
            /* [out][in] */ DTCINITIATEDRECOVERYWORK *pWork,
            /* [out][in] */ void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryInitiatedByDtcVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuRecoveryInitiatedByDtc * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuRecoveryInitiatedByDtc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuRecoveryInitiatedByDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByDtc, GetWork)
        HRESULT ( STDMETHODCALLTYPE *GetWork )( 
            IDtcLuRecoveryInitiatedByDtc * This,
            /* [out][in] */ DTCINITIATEDRECOVERYWORK *pWork,
            /* [out][in] */ void **ppv);
        
        END_INTERFACE
    } IDtcLuRecoveryInitiatedByDtcVtbl;

    interface IDtcLuRecoveryInitiatedByDtc
    {
        CONST_VTBL struct IDtcLuRecoveryInitiatedByDtcVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryInitiatedByDtc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryInitiatedByDtc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryInitiatedByDtc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryInitiatedByDtc_GetWork(This,pWork,ppv)	\
    ( (This)->lpVtbl -> GetWork(This,pWork,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryInitiatedByDtc_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByLuWork_INTERFACE_DEFINED__
#define __IDtcLuRecoveryInitiatedByLuWork_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryInitiatedByLuWork */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryInitiatedByLuWork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC2B8AD1-D6F0-11d0-B386-00A0C9083365")
    IDtcLuRecoveryInitiatedByLuWork : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandleTheirXln( 
            /* [in] */ LONG lRecoverySeqNum,
            /* [in] */ DTCLUXLN Xln,
            /* [in] */ unsigned char *pRemoteLogName,
            /* [in] */ DWORD cbRemoteLogName,
            /* [in] */ unsigned char *pOurLogName,
            /* [in] */ DWORD cbOurLogName,
            /* [in] */ DWORD dwProtocol,
            /* [out] */ DTCLUXLNRESPONSE *pResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOurLogNameSize( 
            /* [out][in] */ DWORD *pcbOurLogName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOurXln( 
            /* [out] */ DTCLUXLN *pXln,
            /* [out][in] */ unsigned char *pOurLogName,
            /* [out] */ DWORD *pdwProtocol) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleConfirmationOfOurXln( 
            /* [in] */ DTCLUXLNCONFIRMATION Confirmation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleTheirCompareStates( 
            /* [out][in] */ unsigned char *pRemoteTransId,
            /* [in] */ DWORD cbRemoteTransId,
            /* [in] */ DTCLUCOMPARESTATE CompareState,
            /* [out] */ DTCLUCOMPARESTATESRESPONSE *pResponse,
            /* [out] */ DTCLUCOMPARESTATE *pCompareState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleConfirmationOfOurCompareStates( 
            /* [in] */ DTCLUCOMPARESTATESCONFIRMATION Confirmation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleErrorFromOurCompareStates( 
            /* [in] */ DTCLUCOMPARESTATESERROR Error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConversationLost( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryInitiatedByLuWorkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuRecoveryInitiatedByLuWork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuRecoveryInitiatedByLuWork * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, HandleTheirXln)
        HRESULT ( STDMETHODCALLTYPE *HandleTheirXln )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [in] */ LONG lRecoverySeqNum,
            /* [in] */ DTCLUXLN Xln,
            /* [in] */ unsigned char *pRemoteLogName,
            /* [in] */ DWORD cbRemoteLogName,
            /* [in] */ unsigned char *pOurLogName,
            /* [in] */ DWORD cbOurLogName,
            /* [in] */ DWORD dwProtocol,
            /* [out] */ DTCLUXLNRESPONSE *pResponse);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, GetOurLogNameSize)
        HRESULT ( STDMETHODCALLTYPE *GetOurLogNameSize )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [out][in] */ DWORD *pcbOurLogName);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, GetOurXln)
        HRESULT ( STDMETHODCALLTYPE *GetOurXln )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [out] */ DTCLUXLN *pXln,
            /* [out][in] */ unsigned char *pOurLogName,
            /* [out] */ DWORD *pdwProtocol);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, HandleConfirmationOfOurXln)
        HRESULT ( STDMETHODCALLTYPE *HandleConfirmationOfOurXln )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [in] */ DTCLUXLNCONFIRMATION Confirmation);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, HandleTheirCompareStates)
        HRESULT ( STDMETHODCALLTYPE *HandleTheirCompareStates )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [out][in] */ unsigned char *pRemoteTransId,
            /* [in] */ DWORD cbRemoteTransId,
            /* [in] */ DTCLUCOMPARESTATE CompareState,
            /* [out] */ DTCLUCOMPARESTATESRESPONSE *pResponse,
            /* [out] */ DTCLUCOMPARESTATE *pCompareState);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, HandleConfirmationOfOurCompareStates)
        HRESULT ( STDMETHODCALLTYPE *HandleConfirmationOfOurCompareStates )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [in] */ DTCLUCOMPARESTATESCONFIRMATION Confirmation);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, HandleErrorFromOurCompareStates)
        HRESULT ( STDMETHODCALLTYPE *HandleErrorFromOurCompareStates )( 
            IDtcLuRecoveryInitiatedByLuWork * This,
            /* [in] */ DTCLUCOMPARESTATESERROR Error);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLuWork, ConversationLost)
        HRESULT ( STDMETHODCALLTYPE *ConversationLost )( 
            IDtcLuRecoveryInitiatedByLuWork * This);
        
        END_INTERFACE
    } IDtcLuRecoveryInitiatedByLuWorkVtbl;

    interface IDtcLuRecoveryInitiatedByLuWork
    {
        CONST_VTBL struct IDtcLuRecoveryInitiatedByLuWorkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryInitiatedByLuWork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryInitiatedByLuWork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryInitiatedByLuWork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryInitiatedByLuWork_HandleTheirXln(This,lRecoverySeqNum,Xln,pRemoteLogName,cbRemoteLogName,pOurLogName,cbOurLogName,dwProtocol,pResponse)	\
    ( (This)->lpVtbl -> HandleTheirXln(This,lRecoverySeqNum,Xln,pRemoteLogName,cbRemoteLogName,pOurLogName,cbOurLogName,dwProtocol,pResponse) ) 

#define IDtcLuRecoveryInitiatedByLuWork_GetOurLogNameSize(This,pcbOurLogName)	\
    ( (This)->lpVtbl -> GetOurLogNameSize(This,pcbOurLogName) ) 

#define IDtcLuRecoveryInitiatedByLuWork_GetOurXln(This,pXln,pOurLogName,pdwProtocol)	\
    ( (This)->lpVtbl -> GetOurXln(This,pXln,pOurLogName,pdwProtocol) ) 

#define IDtcLuRecoveryInitiatedByLuWork_HandleConfirmationOfOurXln(This,Confirmation)	\
    ( (This)->lpVtbl -> HandleConfirmationOfOurXln(This,Confirmation) ) 

#define IDtcLuRecoveryInitiatedByLuWork_HandleTheirCompareStates(This,pRemoteTransId,cbRemoteTransId,CompareState,pResponse,pCompareState)	\
    ( (This)->lpVtbl -> HandleTheirCompareStates(This,pRemoteTransId,cbRemoteTransId,CompareState,pResponse,pCompareState) ) 

#define IDtcLuRecoveryInitiatedByLuWork_HandleConfirmationOfOurCompareStates(This,Confirmation)	\
    ( (This)->lpVtbl -> HandleConfirmationOfOurCompareStates(This,Confirmation) ) 

#define IDtcLuRecoveryInitiatedByLuWork_HandleErrorFromOurCompareStates(This,Error)	\
    ( (This)->lpVtbl -> HandleErrorFromOurCompareStates(This,Error) ) 

#define IDtcLuRecoveryInitiatedByLuWork_ConversationLost(This)	\
    ( (This)->lpVtbl -> ConversationLost(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryInitiatedByLuWork_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRecoveryInitiatedByLu_INTERFACE_DEFINED__
#define __IDtcLuRecoveryInitiatedByLu_INTERFACE_DEFINED__

/* interface IDtcLuRecoveryInitiatedByLu */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRecoveryInitiatedByLu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E768-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRecoveryInitiatedByLu : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetObjectToHandleWorkFromLu( 
            /* [out] */ __RPC__deref_out_opt IDtcLuRecoveryInitiatedByLuWork **ppWork) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRecoveryInitiatedByLuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRecoveryInitiatedByLu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRecoveryInitiatedByLu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRecoveryInitiatedByLu * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRecoveryInitiatedByLu, GetObjectToHandleWorkFromLu)
        HRESULT ( STDMETHODCALLTYPE *GetObjectToHandleWorkFromLu )( 
            __RPC__in IDtcLuRecoveryInitiatedByLu * This,
            /* [out] */ __RPC__deref_out_opt IDtcLuRecoveryInitiatedByLuWork **ppWork);
        
        END_INTERFACE
    } IDtcLuRecoveryInitiatedByLuVtbl;

    interface IDtcLuRecoveryInitiatedByLu
    {
        CONST_VTBL struct IDtcLuRecoveryInitiatedByLuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRecoveryInitiatedByLu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRecoveryInitiatedByLu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRecoveryInitiatedByLu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRecoveryInitiatedByLu_GetObjectToHandleWorkFromLu(This,ppWork)	\
    ( (This)->lpVtbl -> GetObjectToHandleWorkFromLu(This,ppWork) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRecoveryInitiatedByLu_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRmEnlistment_INTERFACE_DEFINED__
#define __IDtcLuRmEnlistment_INTERFACE_DEFINED__

/* interface IDtcLuRmEnlistment */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRmEnlistment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E769-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRmEnlistment : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Unplug( 
            /* [in] */ BOOL fConversationLost) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackedOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Committed( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Forget( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestCommit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRmEnlistmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRmEnlistment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, Unplug)
        HRESULT ( STDMETHODCALLTYPE *Unplug )( 
            __RPC__in IDtcLuRmEnlistment * This,
            /* [in] */ BOOL fConversationLost);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, BackedOut)
        HRESULT ( STDMETHODCALLTYPE *BackedOut )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, BackOut)
        HRESULT ( STDMETHODCALLTYPE *BackOut )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, Committed)
        HRESULT ( STDMETHODCALLTYPE *Committed )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, Forget)
        HRESULT ( STDMETHODCALLTYPE *Forget )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistment, RequestCommit)
        HRESULT ( STDMETHODCALLTYPE *RequestCommit )( 
            __RPC__in IDtcLuRmEnlistment * This);
        
        END_INTERFACE
    } IDtcLuRmEnlistmentVtbl;

    interface IDtcLuRmEnlistment
    {
        CONST_VTBL struct IDtcLuRmEnlistmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRmEnlistment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRmEnlistment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRmEnlistment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRmEnlistment_Unplug(This,fConversationLost)	\
    ( (This)->lpVtbl -> Unplug(This,fConversationLost) ) 

#define IDtcLuRmEnlistment_BackedOut(This)	\
    ( (This)->lpVtbl -> BackedOut(This) ) 

#define IDtcLuRmEnlistment_BackOut(This)	\
    ( (This)->lpVtbl -> BackOut(This) ) 

#define IDtcLuRmEnlistment_Committed(This)	\
    ( (This)->lpVtbl -> Committed(This) ) 

#define IDtcLuRmEnlistment_Forget(This)	\
    ( (This)->lpVtbl -> Forget(This) ) 

#define IDtcLuRmEnlistment_RequestCommit(This)	\
    ( (This)->lpVtbl -> RequestCommit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRmEnlistment_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRmEnlistmentSink_INTERFACE_DEFINED__
#define __IDtcLuRmEnlistmentSink_INTERFACE_DEFINED__

/* interface IDtcLuRmEnlistmentSink */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRmEnlistmentSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E770-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRmEnlistmentSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AckUnplug( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TmDown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SessionLost( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackedOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Committed( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Forget( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Prepare( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestCommit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRmEnlistmentSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuRmEnlistmentSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, AckUnplug)
        HRESULT ( STDMETHODCALLTYPE *AckUnplug )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, TmDown)
        HRESULT ( STDMETHODCALLTYPE *TmDown )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, SessionLost)
        HRESULT ( STDMETHODCALLTYPE *SessionLost )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, BackedOut)
        HRESULT ( STDMETHODCALLTYPE *BackedOut )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, BackOut)
        HRESULT ( STDMETHODCALLTYPE *BackOut )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, Committed)
        HRESULT ( STDMETHODCALLTYPE *Committed )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, Forget)
        HRESULT ( STDMETHODCALLTYPE *Forget )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentSink, RequestCommit)
        HRESULT ( STDMETHODCALLTYPE *RequestCommit )( 
            __RPC__in IDtcLuRmEnlistmentSink * This);
        
        END_INTERFACE
    } IDtcLuRmEnlistmentSinkVtbl;

    interface IDtcLuRmEnlistmentSink
    {
        CONST_VTBL struct IDtcLuRmEnlistmentSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRmEnlistmentSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRmEnlistmentSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRmEnlistmentSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRmEnlistmentSink_AckUnplug(This)	\
    ( (This)->lpVtbl -> AckUnplug(This) ) 

#define IDtcLuRmEnlistmentSink_TmDown(This)	\
    ( (This)->lpVtbl -> TmDown(This) ) 

#define IDtcLuRmEnlistmentSink_SessionLost(This)	\
    ( (This)->lpVtbl -> SessionLost(This) ) 

#define IDtcLuRmEnlistmentSink_BackedOut(This)	\
    ( (This)->lpVtbl -> BackedOut(This) ) 

#define IDtcLuRmEnlistmentSink_BackOut(This)	\
    ( (This)->lpVtbl -> BackOut(This) ) 

#define IDtcLuRmEnlistmentSink_Committed(This)	\
    ( (This)->lpVtbl -> Committed(This) ) 

#define IDtcLuRmEnlistmentSink_Forget(This)	\
    ( (This)->lpVtbl -> Forget(This) ) 

#define IDtcLuRmEnlistmentSink_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IDtcLuRmEnlistmentSink_RequestCommit(This)	\
    ( (This)->lpVtbl -> RequestCommit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRmEnlistmentSink_INTERFACE_DEFINED__ */


#ifndef __IDtcLuRmEnlistmentFactory_INTERFACE_DEFINED__
#define __IDtcLuRmEnlistmentFactory_INTERFACE_DEFINED__

/* interface IDtcLuRmEnlistmentFactory */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuRmEnlistmentFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E771-1AEA-11d0-944B-00A0C905416E")
    IDtcLuRmEnlistmentFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ unsigned char *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [in] */ ITransaction *pITransaction,
            /* [in] */ unsigned char *pTransId,
            /* [in] */ DWORD cbTransId,
            /* [in] */ IDtcLuRmEnlistmentSink *pRmEnlistmentSink,
            /* [out][in] */ IDtcLuRmEnlistment **ppRmEnlistment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuRmEnlistmentFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuRmEnlistmentFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuRmEnlistmentFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuRmEnlistmentFactory * This);
        
        DECLSPEC_XFGVIRT(IDtcLuRmEnlistmentFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            IDtcLuRmEnlistmentFactory * This,
            /* [in] */ unsigned char *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [in] */ ITransaction *pITransaction,
            /* [in] */ unsigned char *pTransId,
            /* [in] */ DWORD cbTransId,
            /* [in] */ IDtcLuRmEnlistmentSink *pRmEnlistmentSink,
            /* [out][in] */ IDtcLuRmEnlistment **ppRmEnlistment);
        
        END_INTERFACE
    } IDtcLuRmEnlistmentFactoryVtbl;

    interface IDtcLuRmEnlistmentFactory
    {
        CONST_VTBL struct IDtcLuRmEnlistmentFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuRmEnlistmentFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuRmEnlistmentFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuRmEnlistmentFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuRmEnlistmentFactory_Create(This,pucLuPair,cbLuPair,pITransaction,pTransId,cbTransId,pRmEnlistmentSink,ppRmEnlistment)	\
    ( (This)->lpVtbl -> Create(This,pucLuPair,cbLuPair,pITransaction,pTransId,cbTransId,pRmEnlistmentSink,ppRmEnlistment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuRmEnlistmentFactory_INTERFACE_DEFINED__ */


#ifndef __IDtcLuSubordinateDtc_INTERFACE_DEFINED__
#define __IDtcLuSubordinateDtc_INTERFACE_DEFINED__

/* interface IDtcLuSubordinateDtc */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuSubordinateDtc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E773-1AEA-11d0-944B-00A0C905416E")
    IDtcLuSubordinateDtc : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Unplug( 
            /* [in] */ BOOL fConversationLost) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackedOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Committed( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Forget( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Prepare( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestCommit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuSubordinateDtcVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcLuSubordinateDtc * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, Unplug)
        HRESULT ( STDMETHODCALLTYPE *Unplug )( 
            __RPC__in IDtcLuSubordinateDtc * This,
            /* [in] */ BOOL fConversationLost);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, BackedOut)
        HRESULT ( STDMETHODCALLTYPE *BackedOut )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, BackOut)
        HRESULT ( STDMETHODCALLTYPE *BackOut )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, Committed)
        HRESULT ( STDMETHODCALLTYPE *Committed )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, Forget)
        HRESULT ( STDMETHODCALLTYPE *Forget )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtc, RequestCommit)
        HRESULT ( STDMETHODCALLTYPE *RequestCommit )( 
            __RPC__in IDtcLuSubordinateDtc * This);
        
        END_INTERFACE
    } IDtcLuSubordinateDtcVtbl;

    interface IDtcLuSubordinateDtc
    {
        CONST_VTBL struct IDtcLuSubordinateDtcVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuSubordinateDtc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuSubordinateDtc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuSubordinateDtc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuSubordinateDtc_Unplug(This,fConversationLost)	\
    ( (This)->lpVtbl -> Unplug(This,fConversationLost) ) 

#define IDtcLuSubordinateDtc_BackedOut(This)	\
    ( (This)->lpVtbl -> BackedOut(This) ) 

#define IDtcLuSubordinateDtc_BackOut(This)	\
    ( (This)->lpVtbl -> BackOut(This) ) 

#define IDtcLuSubordinateDtc_Committed(This)	\
    ( (This)->lpVtbl -> Committed(This) ) 

#define IDtcLuSubordinateDtc_Forget(This)	\
    ( (This)->lpVtbl -> Forget(This) ) 

#define IDtcLuSubordinateDtc_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IDtcLuSubordinateDtc_RequestCommit(This)	\
    ( (This)->lpVtbl -> RequestCommit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuSubordinateDtc_INTERFACE_DEFINED__ */


#ifndef __IDtcLuSubordinateDtcSink_INTERFACE_DEFINED__
#define __IDtcLuSubordinateDtcSink_INTERFACE_DEFINED__

/* interface IDtcLuSubordinateDtcSink */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuSubordinateDtcSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E774-1AEA-11d0-944B-00A0C905416E")
    IDtcLuSubordinateDtcSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AckUnplug( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TmDown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SessionLost( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackedOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackOut( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Committed( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Forget( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestCommit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuSubordinateDtcSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuSubordinateDtcSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, AckUnplug)
        HRESULT ( STDMETHODCALLTYPE *AckUnplug )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, TmDown)
        HRESULT ( STDMETHODCALLTYPE *TmDown )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, SessionLost)
        HRESULT ( STDMETHODCALLTYPE *SessionLost )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, BackedOut)
        HRESULT ( STDMETHODCALLTYPE *BackedOut )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, BackOut)
        HRESULT ( STDMETHODCALLTYPE *BackOut )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, Committed)
        HRESULT ( STDMETHODCALLTYPE *Committed )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, Forget)
        HRESULT ( STDMETHODCALLTYPE *Forget )( 
            IDtcLuSubordinateDtcSink * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcSink, RequestCommit)
        HRESULT ( STDMETHODCALLTYPE *RequestCommit )( 
            IDtcLuSubordinateDtcSink * This);
        
        END_INTERFACE
    } IDtcLuSubordinateDtcSinkVtbl;

    interface IDtcLuSubordinateDtcSink
    {
        CONST_VTBL struct IDtcLuSubordinateDtcSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuSubordinateDtcSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuSubordinateDtcSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuSubordinateDtcSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuSubordinateDtcSink_AckUnplug(This)	\
    ( (This)->lpVtbl -> AckUnplug(This) ) 

#define IDtcLuSubordinateDtcSink_TmDown(This)	\
    ( (This)->lpVtbl -> TmDown(This) ) 

#define IDtcLuSubordinateDtcSink_SessionLost(This)	\
    ( (This)->lpVtbl -> SessionLost(This) ) 

#define IDtcLuSubordinateDtcSink_BackedOut(This)	\
    ( (This)->lpVtbl -> BackedOut(This) ) 

#define IDtcLuSubordinateDtcSink_BackOut(This)	\
    ( (This)->lpVtbl -> BackOut(This) ) 

#define IDtcLuSubordinateDtcSink_Committed(This)	\
    ( (This)->lpVtbl -> Committed(This) ) 

#define IDtcLuSubordinateDtcSink_Forget(This)	\
    ( (This)->lpVtbl -> Forget(This) ) 

#define IDtcLuSubordinateDtcSink_RequestCommit(This)	\
    ( (This)->lpVtbl -> RequestCommit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuSubordinateDtcSink_INTERFACE_DEFINED__ */


#ifndef __IDtcLuSubordinateDtcFactory_INTERFACE_DEFINED__
#define __IDtcLuSubordinateDtcFactory_INTERFACE_DEFINED__

/* interface IDtcLuSubordinateDtcFactory */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDtcLuSubordinateDtcFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4131E775-1AEA-11d0-944B-00A0C905416E")
    IDtcLuSubordinateDtcFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ unsigned char *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [in] */ IUnknown *punkTransactionOuter,
            /* [in] */ ISOLEVEL isoLevel,
            /* [in] */ ULONG isoFlags,
            /* [in] */ ITransactionOptions *pOptions,
            /* [out] */ ITransaction **ppTransaction,
            /* [in] */ unsigned char *pTransId,
            /* [in] */ DWORD cbTransId,
            /* [in] */ IDtcLuSubordinateDtcSink *pSubordinateDtcSink,
            /* [out][in] */ IDtcLuSubordinateDtc **ppSubordinateDtc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcLuSubordinateDtcFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcLuSubordinateDtcFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcLuSubordinateDtcFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcLuSubordinateDtcFactory * This);
        
        DECLSPEC_XFGVIRT(IDtcLuSubordinateDtcFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            IDtcLuSubordinateDtcFactory * This,
            /* [in] */ unsigned char *pucLuPair,
            /* [in] */ DWORD cbLuPair,
            /* [in] */ IUnknown *punkTransactionOuter,
            /* [in] */ ISOLEVEL isoLevel,
            /* [in] */ ULONG isoFlags,
            /* [in] */ ITransactionOptions *pOptions,
            /* [out] */ ITransaction **ppTransaction,
            /* [in] */ unsigned char *pTransId,
            /* [in] */ DWORD cbTransId,
            /* [in] */ IDtcLuSubordinateDtcSink *pSubordinateDtcSink,
            /* [out][in] */ IDtcLuSubordinateDtc **ppSubordinateDtc);
        
        END_INTERFACE
    } IDtcLuSubordinateDtcFactoryVtbl;

    interface IDtcLuSubordinateDtcFactory
    {
        CONST_VTBL struct IDtcLuSubordinateDtcFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcLuSubordinateDtcFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcLuSubordinateDtcFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcLuSubordinateDtcFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcLuSubordinateDtcFactory_Create(This,pucLuPair,cbLuPair,punkTransactionOuter,isoLevel,isoFlags,pOptions,ppTransaction,pTransId,cbTransId,pSubordinateDtcSink,ppSubordinateDtc)	\
    ( (This)->lpVtbl -> Create(This,pucLuPair,cbLuPair,punkTransactionOuter,isoLevel,isoFlags,pOptions,ppTransaction,pTransId,cbTransId,pSubordinateDtcSink,ppSubordinateDtc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcLuSubordinateDtcFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_txdtc_0000_0039 */
/* [local] */ 



#if _MSC_VER < 1100 || !defined(__cplusplus)

DEFINE_GUID(IID_IXATransLookup, 0xF3B1F131, 0xEEDA, 0x11ce, 0xAE, 0xD4, 0x00, 0xAA, 0x00, 0x51, 0xE2, 0xC4);
DEFINE_GUID(IID_IXATransLookup2,0xbf193c85, 0xd1a, 0x4290, 0xb8, 0x8f, 0xd2, 0xcb, 0x88, 0x73, 0xd1, 0xe7);
DEFINE_GUID(IID_IResourceManagerSink, 0x0D563181, 0xDEFB, 0x11ce, 0xAE, 0xD1, 0x00, 0xAA, 0x00, 0x51, 0xE2, 0xC4);
DEFINE_GUID(IID_IResourceManager, 0x13741d21, 0x87eb, 0x11ce, 0x80, 0x81, 0x00, 0x80, 0xc7, 0x58, 0x52, 0x7e);
DEFINE_GUID(IID_IResourceManagerRejoinable, 0x6f6de620, 0xb5df, 0x4f3e, 0x9c, 0xfa, 0xc8, 0xae, 0xbd, 0x05, 0x17, 0x2b);
DEFINE_GUID(IID_IResourceManager2, 0xd136c69a, 0xf749, 0x11d1, 0x8f, 0x47, 0x0, 0xc0, 0x4f, 0x8e, 0xe5, 0x7d);
DEFINE_GUID(IID_ILastResourceManager, 0x4d964ad4, 0x5b33, 0x11d3, 0x8a, 0x91, 0x00, 0xc0, 0x4f, 0x79, 0xeb, 0x6d);
DEFINE_GUID(IID_IXAConfig, 0xC8A6E3A1, 0x9A8C, 0x11cf, 0xA3, 0x08, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IRMHelper, 0xE793F6D1, 0xF53D, 0x11cf, 0xA6, 0x0D, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IXAObtainRMInfo, 0xE793F6D2, 0xF53D, 0x11cf, 0xA6, 0x0D, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IXAResourceManager, 0x4131E751, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IXAResourceManagerFactory, 0x4131E750, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IXATransaction, 0x4131E752, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IResourceManagerFactory, 0x13741d20, 0x87eb, 0x11ce, 0x80, 0x81, 0x00, 0x80, 0xc7, 0x58, 0x52, 0x7e);
DEFINE_GUID(IID_IResourceManagerFactory2, 0x6b369c21, 0xfbd2, 0x11d1, 0x8f, 0x47, 0x0, 0xc0, 0x4f, 0x8e, 0xe5, 0x7d);
DEFINE_GUID(IID_IPrepareInfo, 0x80c7bfd0, 0x87ee, 0x11ce, 0x80, 0x81, 0x00, 0x80, 0xc7, 0x58, 0x52, 0x7e);
DEFINE_GUID(IID_IPrepareInfo2, 0x5FAB2547, 0x9779, 0x11d1, 0xB8, 0x86, 0x00, 0xC0, 0x4F, 0xB9, 0x61, 0x8A);
DEFINE_GUID(IID_IGetDispenser, 0xc23cc370, 0x87ef, 0x11ce, 0x80, 0x81, 0x00, 0x80, 0xc7, 0x58, 0x52, 0x7e);
DEFINE_GUID(IID_ITransactionVoterNotifyAsync2, 0x5433376b, 0x414d, 0x11d3, 0xb2, 0x6, 0x0, 0xc0, 0x4f, 0xc2, 0xf3, 0xef);
DEFINE_GUID(IID_ITransactionVoterBallotAsync2, 0x5433376c, 0x414d, 0x11d3, 0xb2, 0x6, 0x0, 0xc0, 0x4f, 0xc2, 0xf3, 0xef);
DEFINE_GUID(IID_ITransactionVoterFactory2, 0x5433376a, 0x414d, 0x11d3, 0xb2, 0x6, 0x0, 0xc0, 0x4f, 0xc2, 0xf3, 0xef);
DEFINE_GUID(IID_ITransactionPhase0EnlistmentAsync, 0x82DC88E1, 0xA954, 0x11d1, 0x8F, 0x88, 0x00, 0x60, 0x08, 0x95, 0xE7, 0xD5);
DEFINE_GUID(IID_ITransactionPhase0NotifyAsync, 0xEF081809, 0x0C76, 0x11d2, 0x87, 0xA6, 0x00, 0xC0, 0x4F, 0x99, 0x0F, 0x34);
DEFINE_GUID(IID_ITransactionPhase0Factory, 0x82DC88E0, 0xA954, 0x11d1, 0x8F, 0x88, 0x00, 0x60, 0x08, 0x95, 0xE7, 0xD5);
DEFINE_GUID(IID_ITransactionTransmitter, 0x59313E01, 0xB36C, 0x11cf, 0xA5, 0x39, 0x00, 0xAA, 0x00, 0x68, 0x87, 0xC3);
DEFINE_GUID(IID_ITransactionTransmitterFactory, 0x59313E00, 0xB36C, 0x11cf, 0xA5, 0x39, 0x00, 0xAA, 0x00, 0x68, 0x87, 0xC3);
DEFINE_GUID(IID_ITransactionReceiver, 0x59313E03, 0xB36C, 0x11cf, 0xA5, 0x39, 0x00, 0xAA, 0x00, 0x68, 0x87, 0xC3);
DEFINE_GUID(IID_ITransactionReceiverFactory, 0x59313E02, 0xB36C, 0x11cf, 0xA5, 0x39, 0x00, 0xAA, 0x00, 0x68, 0x87, 0xC3);

DEFINE_GUID(IID_IDtcLuConfigure, 0x4131E760, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRecovery, 0xac2b8ad2, 0xd6f0, 0x11d0, 0xb3, 0x86, 0x0, 0xa0, 0xc9, 0x8, 0x33, 0x65);
DEFINE_GUID(IID_IDtcLuRecoveryFactory, 0x4131E762, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRecoveryInitiatedByDtcTransWork, 0x4131E765, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRecoveryInitiatedByDtcStatusWork, 0x4131E766, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRecoveryInitiatedByDtc, 0x4131E764, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRecoveryInitiatedByLuWork, 0xac2b8ad1, 0xd6f0, 0x11d0, 0xb3, 0x86, 0x0, 0xa0, 0xc9, 0x8, 0x33, 0x65);
DEFINE_GUID(IID_IDtcLuRecoveryInitiatedByLu, 0x4131E768, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRmEnlistment, 0x4131E769, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRmEnlistmentSink, 0x4131E770, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuRmEnlistmentFactory, 0x4131E771, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuSubordinateDtc, 0x4131E773, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuSubordinateDtcSink, 0x4131E774, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);
DEFINE_GUID(IID_IDtcLuSubordinateDtcFactory, 0x4131E775, 0x1AEA, 0x11d0, 0x94, 0x4B, 0x00, 0xA0, 0xC9, 0x05, 0x41, 0x6E);

#else

#define  IID_IXATransLookup                          __uuidof(IXATransLookup)
#define  IID_IXATransLookup2                         __uuidof(IXATransLookup2)
#define  IID_IResourceManagerSink                    __uuidof(IResourceManagerSink)
#define  IID_IResourceManager                        __uuidof(IResourceManager)
#define  IID_IResourceManagerRejoinable              __uuidof(IResourceManagerRejoinable)
#define  IID_IResourceManager2                       __uuidof(IResourceManager2)
#define  IID_ILastResourceManager                    __uuidof(ILastResourceManager)
#define  IID_IXAConfig                               __uuidof(IXAConfig)
#define  IID_IRMHelper                               __uuidof(IRMHelper)
#define  IID_IXAObtainRMInfo                         __uuidof(IXAObtainRMInfo)
#define  IID_IXAResourceManager                      __uuidof(IXAResourceManager)
#define  IID_IXAResourceManagerFactory               __uuidof(IXAResourceManagerFactory)
#define  IID_IXATransaction                          __uuidof(IXATransaction)
#define  IID_IResourceManagerFactory                 __uuidof(IResourceManagerFactory)
#define  IID_IResourceManagerFactory2                __uuidof(IResourceManagerFactory2)
#define  IID_IPrepareInfo                            __uuidof(IPrepareInfo)
#define  IID_IPrepareInfo2                           __uuidof(IPrepareInfo2)
#define  IID_IGetDispenser                           __uuidof(IGetDispenser)
#define  IID_ITransactionVoterNotifyAsync2           __uuidof(ITransactionVoterNotifyAsync2)
#define  IID_ITransactionVoterBallotAsync2           __uuidof(ITransactionVoterBallotAsync2)
#define  IID_ITransactionVoterFactory2               __uuidof(ITransactionVoterFactory2)
#define  IID_ITransactionPhase0EnlistmentAsync       __uuidof(ITransactionPhase0EnlistmentAsync)
#define  IID_ITransactionPhase0NotifyAsync           __uuidof(ITransactionPhase0NotifyAsync)
#define  IID_ITransactionPhase0Factory               __uuidof(ITransactionPhase0Factory)
#define  IID_ITransactionTransmitter                 __uuidof(ITransactionTransmitter)
#define  IID_ITransactionTransmitterFactory          __uuidof(ITransactionTransmitterFactory)
#define  IID_ITransactionReceiver                    __uuidof(ITransactionReceiver)
#define  IID_ITransactionReceiverFactory             __uuidof(ITransactionReceiverFactory)

#define  IID_IDtcLuConfigure                         __uuidof(IDtcLuConfigure)
#define  IID_IDtcLuRecovery                          __uuidof(IDtcLuRecovery)
#define  IID_IDtcLuRecoveryFactory                   __uuidof(IDtcLuRecoveryFactory)
#define  IID_IDtcLuRecoveryInitiatedByDtcTransWork   __uuidof(IDtcLuRecoveryInitiatedByDtcTransWork)
#define  IID_IDtcLuRecoveryInitiatedByDtcStatusWork  __uuidof(IDtcLuRecoveryInitiatedByDtcStatusWork)
#define  IID_IDtcLuRecoveryInitiatedByDtc            __uuidof(IDtcLuRecoveryInitiatedByDtc)
#define  IID_IDtcLuRecoveryInitiatedByLuWork         __uuidof(IDtcLuRecoveryInitiatedByLuWork)
#define  IID_IDtcLuRecoveryInitiatedByLu             __uuidof(IDtcLuRecoveryInitiatedByLu)
#define  IID_IDtcLuRmEnlistment                      __uuidof(IDtcLuRmEnlistment)
#define  IID_IDtcLuRmEnlistmentSink                  __uuidof(IDtcLuRmEnlistmentSink)
#define  IID_IDtcLuRmEnlistmentFactory               __uuidof(IDtcLuRmEnlistmentFactory)
#define  IID_IDtcLuSubordinateDtc                    __uuidof(IDtcLuSubordinateDtc)
#define  IID_IDtcLuSubordinateDtcSink                __uuidof(IDtcLuSubordinateDtcSink)
#define  IID_IDtcLuSubordinateDtcFactory             __uuidof(IDtcLuSubordinateDtcFactory)

#endif
#pragma deprecated (IXAConfig)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txdtc_0000_0039_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



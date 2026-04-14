

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

#ifndef __radiomgr_h__
#define __radiomgr_h__

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

#ifndef __IMediaRadioManager_FWD_DEFINED__
#define __IMediaRadioManager_FWD_DEFINED__
typedef interface IMediaRadioManager IMediaRadioManager;

#endif 	/* __IMediaRadioManager_FWD_DEFINED__ */


#ifndef __IRadioInstanceCollection_FWD_DEFINED__
#define __IRadioInstanceCollection_FWD_DEFINED__
typedef interface IRadioInstanceCollection IRadioInstanceCollection;

#endif 	/* __IRadioInstanceCollection_FWD_DEFINED__ */


#ifndef __IRadioInstance_FWD_DEFINED__
#define __IRadioInstance_FWD_DEFINED__
typedef interface IRadioInstance IRadioInstance;

#endif 	/* __IRadioInstance_FWD_DEFINED__ */


#ifndef __IMediaRadioManagerNotifySink_FWD_DEFINED__
#define __IMediaRadioManagerNotifySink_FWD_DEFINED__
typedef interface IMediaRadioManagerNotifySink IMediaRadioManagerNotifySink;

#endif 	/* __IMediaRadioManagerNotifySink_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_radiomgr_0000_0000 */
/* [local] */ 

//
// Copyright (C) Microsoft Corporation. All rights reserved.
//
#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN8)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




typedef 
enum _DEVICE_RADIO_STATE
    {
        DRS_RADIO_ON	= 0,
        DRS_SW_RADIO_OFF	= 1,
        DRS_HW_RADIO_OFF	= 2,
        DRS_SW_HW_RADIO_OFF	= 3,
        DRS_HW_RADIO_ON_UNCONTROLLABLE	= 4,
        DRS_RADIO_INVALID	= 5,
        DRS_HW_RADIO_OFF_UNCONTROLLABLE	= 6,
        DRS_RADIO_MAX	= DRS_HW_RADIO_OFF_UNCONTROLLABLE
    } 	DEVICE_RADIO_STATE;

typedef 
enum _SYSTEM_RADIO_STATE
    {
        SRS_RADIO_ENABLED	= 0,
        SRS_RADIO_DISABLED	= 1
    } 	SYSTEM_RADIO_STATE;



extern RPC_IF_HANDLE __MIDL_itf_radiomgr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_radiomgr_0000_0000_v0_0_s_ifspec;

#ifndef __IMediaRadioManager_INTERFACE_DEFINED__
#define __IMediaRadioManager_INTERFACE_DEFINED__

/* interface IMediaRadioManager */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaRadioManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6CFDCAB5-FC47-42A5-9241-074B58830E73")
    IMediaRadioManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRadioInstances( 
            /* [out] */ IRadioInstanceCollection **ppCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSystemRadioStateChange( 
            /* [in] */ SYSTEM_RADIO_STATE sysRadioState,
            /* [in] */ UINT32 uTimeoutSec) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaRadioManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaRadioManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaRadioManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaRadioManager * This);
        
        DECLSPEC_XFGVIRT(IMediaRadioManager, GetRadioInstances)
        HRESULT ( STDMETHODCALLTYPE *GetRadioInstances )( 
            IMediaRadioManager * This,
            /* [out] */ IRadioInstanceCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IMediaRadioManager, OnSystemRadioStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnSystemRadioStateChange )( 
            IMediaRadioManager * This,
            /* [in] */ SYSTEM_RADIO_STATE sysRadioState,
            /* [in] */ UINT32 uTimeoutSec);
        
        END_INTERFACE
    } IMediaRadioManagerVtbl;

    interface IMediaRadioManager
    {
        CONST_VTBL struct IMediaRadioManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaRadioManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaRadioManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaRadioManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaRadioManager_GetRadioInstances(This,ppCollection)	\
    ( (This)->lpVtbl -> GetRadioInstances(This,ppCollection) ) 

#define IMediaRadioManager_OnSystemRadioStateChange(This,sysRadioState,uTimeoutSec)	\
    ( (This)->lpVtbl -> OnSystemRadioStateChange(This,sysRadioState,uTimeoutSec) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaRadioManager_INTERFACE_DEFINED__ */


#ifndef __IRadioInstanceCollection_INTERFACE_DEFINED__
#define __IRadioInstanceCollection_INTERFACE_DEFINED__

/* interface IRadioInstanceCollection */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IRadioInstanceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E5791FAE-5665-4E0C-95BE-5FDE31644185")
    IRadioInstanceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ UINT32 *pcInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 uIndex,
            /* [out] */ IRadioInstance **ppRadioInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRadioInstanceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRadioInstanceCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRadioInstanceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRadioInstanceCollection * This);
        
        DECLSPEC_XFGVIRT(IRadioInstanceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IRadioInstanceCollection * This,
            /* [out] */ UINT32 *pcInstance);
        
        DECLSPEC_XFGVIRT(IRadioInstanceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            IRadioInstanceCollection * This,
            /* [in] */ UINT32 uIndex,
            /* [out] */ IRadioInstance **ppRadioInstance);
        
        END_INTERFACE
    } IRadioInstanceCollectionVtbl;

    interface IRadioInstanceCollection
    {
        CONST_VTBL struct IRadioInstanceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRadioInstanceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRadioInstanceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRadioInstanceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRadioInstanceCollection_GetCount(This,pcInstance)	\
    ( (This)->lpVtbl -> GetCount(This,pcInstance) ) 

#define IRadioInstanceCollection_GetAt(This,uIndex,ppRadioInstance)	\
    ( (This)->lpVtbl -> GetAt(This,uIndex,ppRadioInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRadioInstanceCollection_INTERFACE_DEFINED__ */


#ifndef __IRadioInstance_INTERFACE_DEFINED__
#define __IRadioInstance_INTERFACE_DEFINED__

/* interface IRadioInstance */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IRadioInstance;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70AA1C9E-F2B4-4C61-86D3-6B9FB75FD1A2")
    IRadioInstance : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRadioManagerSignature( 
            /* [out] */ GUID *pguidSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceSignature( 
            /* [string][out] */ BSTR *pbstrId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFriendlyName( 
            /* [in] */ LCID lcid,
            /* [string][out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRadioState( 
            /* [out] */ DEVICE_RADIO_STATE *pRadioState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRadioState( 
            /* [in] */ DEVICE_RADIO_STATE radioState,
            /* [in] */ UINT32 uTimeoutSec) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsMultiComm( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsAssociatingDevice( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRadioInstanceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRadioInstance * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRadioInstance * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRadioInstance * This);
        
        DECLSPEC_XFGVIRT(IRadioInstance, GetRadioManagerSignature)
        HRESULT ( STDMETHODCALLTYPE *GetRadioManagerSignature )( 
            IRadioInstance * This,
            /* [out] */ GUID *pguidSignature);
        
        DECLSPEC_XFGVIRT(IRadioInstance, GetInstanceSignature)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceSignature )( 
            IRadioInstance * This,
            /* [string][out] */ BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IRadioInstance, GetFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *GetFriendlyName )( 
            IRadioInstance * This,
            /* [in] */ LCID lcid,
            /* [string][out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRadioInstance, GetRadioState)
        HRESULT ( STDMETHODCALLTYPE *GetRadioState )( 
            IRadioInstance * This,
            /* [out] */ DEVICE_RADIO_STATE *pRadioState);
        
        DECLSPEC_XFGVIRT(IRadioInstance, SetRadioState)
        HRESULT ( STDMETHODCALLTYPE *SetRadioState )( 
            IRadioInstance * This,
            /* [in] */ DEVICE_RADIO_STATE radioState,
            /* [in] */ UINT32 uTimeoutSec);
        
        DECLSPEC_XFGVIRT(IRadioInstance, IsMultiComm)
        BOOL ( STDMETHODCALLTYPE *IsMultiComm )( 
            IRadioInstance * This);
        
        DECLSPEC_XFGVIRT(IRadioInstance, IsAssociatingDevice)
        BOOL ( STDMETHODCALLTYPE *IsAssociatingDevice )( 
            IRadioInstance * This);
        
        END_INTERFACE
    } IRadioInstanceVtbl;

    interface IRadioInstance
    {
        CONST_VTBL struct IRadioInstanceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRadioInstance_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRadioInstance_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRadioInstance_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRadioInstance_GetRadioManagerSignature(This,pguidSignature)	\
    ( (This)->lpVtbl -> GetRadioManagerSignature(This,pguidSignature) ) 

#define IRadioInstance_GetInstanceSignature(This,pbstrId)	\
    ( (This)->lpVtbl -> GetInstanceSignature(This,pbstrId) ) 

#define IRadioInstance_GetFriendlyName(This,lcid,pbstrName)	\
    ( (This)->lpVtbl -> GetFriendlyName(This,lcid,pbstrName) ) 

#define IRadioInstance_GetRadioState(This,pRadioState)	\
    ( (This)->lpVtbl -> GetRadioState(This,pRadioState) ) 

#define IRadioInstance_SetRadioState(This,radioState,uTimeoutSec)	\
    ( (This)->lpVtbl -> SetRadioState(This,radioState,uTimeoutSec) ) 

#define IRadioInstance_IsMultiComm(This)	\
    ( (This)->lpVtbl -> IsMultiComm(This) ) 

#define IRadioInstance_IsAssociatingDevice(This)	\
    ( (This)->lpVtbl -> IsAssociatingDevice(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRadioInstance_INTERFACE_DEFINED__ */


#ifndef __IMediaRadioManagerNotifySink_INTERFACE_DEFINED__
#define __IMediaRadioManagerNotifySink_INTERFACE_DEFINED__

/* interface IMediaRadioManagerNotifySink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaRadioManagerNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89D81F5F-C147-49ED-A11C-77B20C31E7C9")
    IMediaRadioManagerNotifySink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnInstanceAdd( 
            /* [in] */ IRadioInstance *pRadioInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInstanceRemove( 
            /* [string][in] */ BSTR bstrRadioInstanceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInstanceRadioChange( 
            /* [string][in] */ BSTR bstrRadioInstanceId,
            /* [in] */ DEVICE_RADIO_STATE radioState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaRadioManagerNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaRadioManagerNotifySink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaRadioManagerNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaRadioManagerNotifySink * This);
        
        DECLSPEC_XFGVIRT(IMediaRadioManagerNotifySink, OnInstanceAdd)
        HRESULT ( STDMETHODCALLTYPE *OnInstanceAdd )( 
            IMediaRadioManagerNotifySink * This,
            /* [in] */ IRadioInstance *pRadioInstance);
        
        DECLSPEC_XFGVIRT(IMediaRadioManagerNotifySink, OnInstanceRemove)
        HRESULT ( STDMETHODCALLTYPE *OnInstanceRemove )( 
            IMediaRadioManagerNotifySink * This,
            /* [string][in] */ BSTR bstrRadioInstanceId);
        
        DECLSPEC_XFGVIRT(IMediaRadioManagerNotifySink, OnInstanceRadioChange)
        HRESULT ( STDMETHODCALLTYPE *OnInstanceRadioChange )( 
            IMediaRadioManagerNotifySink * This,
            /* [string][in] */ BSTR bstrRadioInstanceId,
            /* [in] */ DEVICE_RADIO_STATE radioState);
        
        END_INTERFACE
    } IMediaRadioManagerNotifySinkVtbl;

    interface IMediaRadioManagerNotifySink
    {
        CONST_VTBL struct IMediaRadioManagerNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaRadioManagerNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaRadioManagerNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaRadioManagerNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaRadioManagerNotifySink_OnInstanceAdd(This,pRadioInstance)	\
    ( (This)->lpVtbl -> OnInstanceAdd(This,pRadioInstance) ) 

#define IMediaRadioManagerNotifySink_OnInstanceRemove(This,bstrRadioInstanceId)	\
    ( (This)->lpVtbl -> OnInstanceRemove(This,bstrRadioInstanceId) ) 

#define IMediaRadioManagerNotifySink_OnInstanceRadioChange(This,bstrRadioInstanceId,radioState)	\
    ( (This)->lpVtbl -> OnInstanceRadioChange(This,bstrRadioInstanceId,radioState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaRadioManagerNotifySink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_radiomgr_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_radiomgr_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_radiomgr_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



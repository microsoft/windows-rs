

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

#ifndef __oletx2xa_h__
#define __oletx2xa_h__

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

#ifndef __IDtcToXaMapper_FWD_DEFINED__
#define __IDtcToXaMapper_FWD_DEFINED__
typedef interface IDtcToXaMapper IDtcToXaMapper;

#endif 	/* __IDtcToXaMapper_FWD_DEFINED__ */


#ifndef __IDtcToXaHelperFactory_FWD_DEFINED__
#define __IDtcToXaHelperFactory_FWD_DEFINED__
typedef interface IDtcToXaHelperFactory IDtcToXaHelperFactory;

#endif 	/* __IDtcToXaHelperFactory_FWD_DEFINED__ */


#ifndef __IDtcToXaHelper_FWD_DEFINED__
#define __IDtcToXaHelper_FWD_DEFINED__
typedef interface IDtcToXaHelper IDtcToXaHelper;

#endif 	/* __IDtcToXaHelper_FWD_DEFINED__ */


#ifndef __IDtcToXaHelperSinglePipe_FWD_DEFINED__
#define __IDtcToXaHelperSinglePipe_FWD_DEFINED__
typedef interface IDtcToXaHelperSinglePipe IDtcToXaHelperSinglePipe;

#endif 	/* __IDtcToXaHelperSinglePipe_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "transact.h"
#include "txcoord.h"
#include "xa.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_oletx2xa_0000_0000 */
/* [local] */ 

//-------------------------------------------------------------------------
//
//  Microsoft Distributed Transaction Coordinator
//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.
//
//  File: xamapper.h (generated from xamapper.idl)
//
//  Contents: Interfaces and types to map OleTx transaction
//            to XA transaction
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






extern RPC_IF_HANDLE __MIDL_itf_oletx2xa_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oletx2xa_0000_0000_v0_0_s_ifspec;

#ifndef __XaMapperTypes_INTERFACE_DEFINED__
#define __XaMapperTypes_INTERFACE_DEFINED__

/* interface XaMapperTypes */
/* [unique][local] */ 

// Typedefs
typedef DWORD XA_SWITCH_FLAGS;

// Defines
#define XA_SWITCH_F_DTC      0x00000001
#define XA_FMTID_DTC         0x00445443
#define XA_FMTID_DTC_VER1    0x01445443
// Constants
const XID XID_NULL = {-1,0,0,'\0'};


extern RPC_IF_HANDLE XaMapperTypes_v0_0_c_ifspec;
extern RPC_IF_HANDLE XaMapperTypes_v0_0_s_ifspec;
#endif /* __XaMapperTypes_INTERFACE_DEFINED__ */

#ifndef __XaMapperAPIs_INTERFACE_DEFINED__
#define __XaMapperAPIs_INTERFACE_DEFINED__

/* interface XaMapperAPIs */
/* [unique][local] */ 

HRESULT __cdecl GetXaSwitch( 
    /* [in] */ XA_SWITCH_FLAGS XaSwitchFlags,
    /* [out] */ xa_switch_t **ppXaSwitch);



extern RPC_IF_HANDLE XaMapperAPIs_v0_0_c_ifspec;
extern RPC_IF_HANDLE XaMapperAPIs_v0_0_s_ifspec;
#endif /* __XaMapperAPIs_INTERFACE_DEFINED__ */

#ifndef __IDtcToXaMapper_INTERFACE_DEFINED__
#define __IDtcToXaMapper_INTERFACE_DEFINED__

/* interface IDtcToXaMapper */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDtcToXaMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64FFABE0-7CE9-11d0-8CE6-00C04FDC877E")
    IDtcToXaMapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestNewResourceManager( 
            /* [in] */ __RPC__in char *pszDSN,
            /* [in] */ __RPC__in char *pszClientDllName,
            /* [out][in] */ __RPC__inout DWORD *pdwRMCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TranslateTridToXid( 
            /* [in] */ __RPC__in DWORD *pdwITransaction,
            /* [in] */ DWORD dwRMCookie,
            /* [out][in] */ __RPC__inout XID *pXid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnlistResourceManager( 
            /* [in] */ DWORD dwRMCookie,
            /* [in] */ __RPC__in DWORD *pdwITransaction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseResourceManager( 
            /* [in] */ DWORD dwRMCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcToXaMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcToXaMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcToXaMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcToXaMapper * This);
        
        DECLSPEC_XFGVIRT(IDtcToXaMapper, RequestNewResourceManager)
        HRESULT ( STDMETHODCALLTYPE *RequestNewResourceManager )( 
            __RPC__in IDtcToXaMapper * This,
            /* [in] */ __RPC__in char *pszDSN,
            /* [in] */ __RPC__in char *pszClientDllName,
            /* [out][in] */ __RPC__inout DWORD *pdwRMCookie);
        
        DECLSPEC_XFGVIRT(IDtcToXaMapper, TranslateTridToXid)
        HRESULT ( STDMETHODCALLTYPE *TranslateTridToXid )( 
            __RPC__in IDtcToXaMapper * This,
            /* [in] */ __RPC__in DWORD *pdwITransaction,
            /* [in] */ DWORD dwRMCookie,
            /* [out][in] */ __RPC__inout XID *pXid);
        
        DECLSPEC_XFGVIRT(IDtcToXaMapper, EnlistResourceManager)
        HRESULT ( STDMETHODCALLTYPE *EnlistResourceManager )( 
            __RPC__in IDtcToXaMapper * This,
            /* [in] */ DWORD dwRMCookie,
            /* [in] */ __RPC__in DWORD *pdwITransaction);
        
        DECLSPEC_XFGVIRT(IDtcToXaMapper, ReleaseResourceManager)
        HRESULT ( STDMETHODCALLTYPE *ReleaseResourceManager )( 
            __RPC__in IDtcToXaMapper * This,
            /* [in] */ DWORD dwRMCookie);
        
        END_INTERFACE
    } IDtcToXaMapperVtbl;

    interface IDtcToXaMapper
    {
        CONST_VTBL struct IDtcToXaMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcToXaMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcToXaMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcToXaMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcToXaMapper_RequestNewResourceManager(This,pszDSN,pszClientDllName,pdwRMCookie)	\
    ( (This)->lpVtbl -> RequestNewResourceManager(This,pszDSN,pszClientDllName,pdwRMCookie) ) 

#define IDtcToXaMapper_TranslateTridToXid(This,pdwITransaction,dwRMCookie,pXid)	\
    ( (This)->lpVtbl -> TranslateTridToXid(This,pdwITransaction,dwRMCookie,pXid) ) 

#define IDtcToXaMapper_EnlistResourceManager(This,dwRMCookie,pdwITransaction)	\
    ( (This)->lpVtbl -> EnlistResourceManager(This,dwRMCookie,pdwITransaction) ) 

#define IDtcToXaMapper_ReleaseResourceManager(This,dwRMCookie)	\
    ( (This)->lpVtbl -> ReleaseResourceManager(This,dwRMCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcToXaMapper_INTERFACE_DEFINED__ */


#ifndef __IDtcToXaHelperFactory_INTERFACE_DEFINED__
#define __IDtcToXaHelperFactory_INTERFACE_DEFINED__

/* interface IDtcToXaHelperFactory */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDtcToXaHelperFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9861610-304A-11d1-9813-00A0C905416E")
    IDtcToXaHelperFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in char *pszDSN,
            /* [in] */ __RPC__in char *pszClientDllName,
            /* [out] */ __RPC__out GUID *pguidRm,
            /* [out] */ __RPC__deref_out_opt IDtcToXaHelper **ppXaHelper) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcToXaHelperFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcToXaHelperFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcToXaHelperFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcToXaHelperFactory * This);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelperFactory, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IDtcToXaHelperFactory * This,
            /* [in] */ __RPC__in char *pszDSN,
            /* [in] */ __RPC__in char *pszClientDllName,
            /* [out] */ __RPC__out GUID *pguidRm,
            /* [out] */ __RPC__deref_out_opt IDtcToXaHelper **ppXaHelper);
        
        END_INTERFACE
    } IDtcToXaHelperFactoryVtbl;

    interface IDtcToXaHelperFactory
    {
        CONST_VTBL struct IDtcToXaHelperFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcToXaHelperFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcToXaHelperFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcToXaHelperFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcToXaHelperFactory_Create(This,pszDSN,pszClientDllName,pguidRm,ppXaHelper)	\
    ( (This)->lpVtbl -> Create(This,pszDSN,pszClientDllName,pguidRm,ppXaHelper) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcToXaHelperFactory_INTERFACE_DEFINED__ */


#ifndef __IDtcToXaHelper_INTERFACE_DEFINED__
#define __IDtcToXaHelper_INTERFACE_DEFINED__

/* interface IDtcToXaHelper */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDtcToXaHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9861611-304A-11d1-9813-00A0C905416E")
    IDtcToXaHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Close( 
            /* [in] */ BOOL i_fDoRecovery) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TranslateTridToXid( 
            /* [in] */ __RPC__in_opt ITransaction *pITransaction,
            /* [in] */ __RPC__in GUID *pguidBqual,
            /* [out] */ __RPC__out XID *pXid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcToXaHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDtcToXaHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDtcToXaHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDtcToXaHelper * This);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelper, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IDtcToXaHelper * This,
            /* [in] */ BOOL i_fDoRecovery);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelper, TranslateTridToXid)
        HRESULT ( STDMETHODCALLTYPE *TranslateTridToXid )( 
            __RPC__in IDtcToXaHelper * This,
            /* [in] */ __RPC__in_opt ITransaction *pITransaction,
            /* [in] */ __RPC__in GUID *pguidBqual,
            /* [out] */ __RPC__out XID *pXid);
        
        END_INTERFACE
    } IDtcToXaHelperVtbl;

    interface IDtcToXaHelper
    {
        CONST_VTBL struct IDtcToXaHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcToXaHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcToXaHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcToXaHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcToXaHelper_Close(This,i_fDoRecovery)	\
    ( (This)->lpVtbl -> Close(This,i_fDoRecovery) ) 

#define IDtcToXaHelper_TranslateTridToXid(This,pITransaction,pguidBqual,pXid)	\
    ( (This)->lpVtbl -> TranslateTridToXid(This,pITransaction,pguidBqual,pXid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcToXaHelper_INTERFACE_DEFINED__ */


#ifndef __IDtcToXaHelperSinglePipe_INTERFACE_DEFINED__
#define __IDtcToXaHelperSinglePipe_INTERFACE_DEFINED__

/* interface IDtcToXaHelperSinglePipe */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDtcToXaHelperSinglePipe;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("47ED4971-53B3-11d1-BBB9-00C04FD658F6")
    IDtcToXaHelperSinglePipe : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE XARMCreate( 
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszDSN,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszClientDll,
            /* [out][in] */ DWORD *pdwRMCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertTridToXID( 
            /* [in] */ DWORD *pdwITrans,
            /* [in] */ DWORD dwRMCookie,
            /* [out][in] */ XID *pxid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnlistWithRM( 
            /* [in] */ DWORD dwRMCookie,
            /* [in] */ ITransaction *i_pITransaction,
            /* [in] */ ITransactionResourceAsync *i_pITransRes,
            /* [out] */ ITransactionEnlistmentAsync **o_ppITransEnslitment) = 0;
        
        virtual void STDMETHODCALLTYPE ReleaseRMCookie( 
            /* [in] */ DWORD i_dwRMCookie,
            /* [in] */ BOOL i_fNormal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDtcToXaHelperSinglePipeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDtcToXaHelperSinglePipe * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDtcToXaHelperSinglePipe * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDtcToXaHelperSinglePipe * This);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelperSinglePipe, XARMCreate)
        HRESULT ( STDMETHODCALLTYPE *XARMCreate )( 
            IDtcToXaHelperSinglePipe * This,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszDSN,
            /* [annotation][string][in] */ 
            _Null_terminated_  char *pszClientDll,
            /* [out][in] */ DWORD *pdwRMCookie);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelperSinglePipe, ConvertTridToXID)
        HRESULT ( STDMETHODCALLTYPE *ConvertTridToXID )( 
            IDtcToXaHelperSinglePipe * This,
            /* [in] */ DWORD *pdwITrans,
            /* [in] */ DWORD dwRMCookie,
            /* [out][in] */ XID *pxid);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelperSinglePipe, EnlistWithRM)
        HRESULT ( STDMETHODCALLTYPE *EnlistWithRM )( 
            IDtcToXaHelperSinglePipe * This,
            /* [in] */ DWORD dwRMCookie,
            /* [in] */ ITransaction *i_pITransaction,
            /* [in] */ ITransactionResourceAsync *i_pITransRes,
            /* [out] */ ITransactionEnlistmentAsync **o_ppITransEnslitment);
        
        DECLSPEC_XFGVIRT(IDtcToXaHelperSinglePipe, ReleaseRMCookie)
        void ( STDMETHODCALLTYPE *ReleaseRMCookie )( 
            IDtcToXaHelperSinglePipe * This,
            /* [in] */ DWORD i_dwRMCookie,
            /* [in] */ BOOL i_fNormal);
        
        END_INTERFACE
    } IDtcToXaHelperSinglePipeVtbl;

    interface IDtcToXaHelperSinglePipe
    {
        CONST_VTBL struct IDtcToXaHelperSinglePipeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDtcToXaHelperSinglePipe_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDtcToXaHelperSinglePipe_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDtcToXaHelperSinglePipe_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDtcToXaHelperSinglePipe_XARMCreate(This,pszDSN,pszClientDll,pdwRMCookie)	\
    ( (This)->lpVtbl -> XARMCreate(This,pszDSN,pszClientDll,pdwRMCookie) ) 

#define IDtcToXaHelperSinglePipe_ConvertTridToXID(This,pdwITrans,dwRMCookie,pxid)	\
    ( (This)->lpVtbl -> ConvertTridToXID(This,pdwITrans,dwRMCookie,pxid) ) 

#define IDtcToXaHelperSinglePipe_EnlistWithRM(This,dwRMCookie,i_pITransaction,i_pITransRes,o_ppITransEnslitment)	\
    ( (This)->lpVtbl -> EnlistWithRM(This,dwRMCookie,i_pITransaction,i_pITransRes,o_ppITransEnslitment) ) 

#define IDtcToXaHelperSinglePipe_ReleaseRMCookie(This,i_dwRMCookie,i_fNormal)	\
    ( (This)->lpVtbl -> ReleaseRMCookie(This,i_dwRMCookie,i_fNormal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDtcToXaHelperSinglePipe_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oletx2xa_0000_0006 */
/* [local] */ 



#if _MSC_VER < 1100 || !defined(__cplusplus)

DEFINE_GUID(IID_IDtcToXaMapper, 0x64FFABE0, 0x7CE9, 0x11d0, 0x8C, 0xE6, 0x00, 0xC0, 0x4F, 0xDC, 0x87, 0x7E);
DEFINE_GUID(IID_IDtcToXaHelperFactory, 0xadefc46a, 0xcb1d, 0x11d0, 0xb1, 0x35, 0x00, 0xc0, 0x4f, 0xc2, 0xf3, 0xef);
DEFINE_GUID(IID_IDtcToXaHelper, 0xadefc46b, 0xcb1d, 0x11d0, 0xb1, 0x35, 0x00, 0xc0, 0x4f, 0xc2, 0xf3, 0xef);
DEFINE_GUID(IID_IDtcToXaHelperSinglePipe,        0x47ED4971, 0x53B3, 0x11d1, 0xBB, 0xB9, 0x00, 0xC0, 0x4F, 0xD6, 0x58, 0xF6);

#else

#define  IID_IDtcToXaMapper                          __uuidof(IDtcToXaMapper)
#define  IID_IDtcToXaHelperFactory                   __uuidof(IDtcToXaHelperFactory)
#define  IID_IDtcToXaHelper                          __uuidof(IDtcToXaHelper)
#define  IID_IDtcToXaHelperSinglePipe                __uuidof(IDtcToXaHelperSinglePipe)

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_oletx2xa_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oletx2xa_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



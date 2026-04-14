//
//    Copyright (C) Microsoft.  All rights reserved.
//
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 6.00.0363 */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif // __RPCNDR_H_VERSION__


#ifndef __iiisext_h__
#define __iiisext_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

/* Forward Declarations */ 

#ifndef __IISApplicationPool_FWD_DEFINED__
#define __IISApplicationPool_FWD_DEFINED__
typedef interface IISApplicationPool IISApplicationPool;
#endif 	/* __IISApplicationPool_FWD_DEFINED__ */


#ifndef __IISApplicationPools_FWD_DEFINED__
#define __IISApplicationPools_FWD_DEFINED__
typedef interface IISApplicationPools IISApplicationPools;
#endif 	/* __IISApplicationPools_FWD_DEFINED__ */


#ifndef __IISWebService_FWD_DEFINED__
#define __IISWebService_FWD_DEFINED__
typedef interface IISWebService IISWebService;
#endif 	/* __IISWebService_FWD_DEFINED__ */


#ifndef __IISDsCrMap_FWD_DEFINED__
#define __IISDsCrMap_FWD_DEFINED__
typedef interface IISDsCrMap IISDsCrMap;
#endif 	/* __IISDsCrMap_FWD_DEFINED__ */


#ifndef __IISApp_FWD_DEFINED__
#define __IISApp_FWD_DEFINED__
typedef interface IISApp IISApp;
#endif 	/* __IISApp_FWD_DEFINED__ */


#ifndef __IISApp2_FWD_DEFINED__
#define __IISApp2_FWD_DEFINED__
typedef interface IISApp2 IISApp2;
#endif 	/* __IISApp2_FWD_DEFINED__ */


#ifndef __IISApp3_FWD_DEFINED__
#define __IISApp3_FWD_DEFINED__
typedef interface IISApp3 IISApp3;
#endif 	/* __IISApp3_FWD_DEFINED__ */


#ifndef __IISComputer_FWD_DEFINED__
#define __IISComputer_FWD_DEFINED__
typedef interface IISComputer IISComputer;
#endif 	/* __IISComputer_FWD_DEFINED__ */


#ifndef __IISComputer2_FWD_DEFINED__
#define __IISComputer2_FWD_DEFINED__
typedef interface IISComputer2 IISComputer2;
#endif 	/* __IISComputer2_FWD_DEFINED__ */


#ifndef __IISExtComputer_FWD_DEFINED__
#define __IISExtComputer_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtComputer IISExtComputer;
#else
typedef struct IISExtComputer IISExtComputer;
#endif /* __cplusplus */

#endif 	/* __IISExtComputer_FWD_DEFINED__ */


#ifndef __IISExtApp_FWD_DEFINED__
#define __IISExtApp_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtApp IISExtApp;
#else
typedef struct IISExtApp IISExtApp;
#endif /* __cplusplus */

#endif 	/* __IISExtApp_FWD_DEFINED__ */


#ifndef __IISExtServer_FWD_DEFINED__
#define __IISExtServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtServer IISExtServer;
#else
typedef struct IISExtServer IISExtServer;
#endif /* __cplusplus */

#endif 	/* __IISExtServer_FWD_DEFINED__ */


#ifndef __IISExtDsCrMap_FWD_DEFINED__
#define __IISExtDsCrMap_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtDsCrMap IISExtDsCrMap;
#else
typedef struct IISExtDsCrMap IISExtDsCrMap;
#endif /* __cplusplus */

#endif 	/* __IISExtDsCrMap_FWD_DEFINED__ */


#ifndef __IISExtApplicationPool_FWD_DEFINED__
#define __IISExtApplicationPool_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtApplicationPool IISExtApplicationPool;
#else
typedef struct IISExtApplicationPool IISExtApplicationPool;
#endif /* __cplusplus */

#endif 	/* __IISExtApplicationPool_FWD_DEFINED__ */


#ifndef __IISExtApplicationPools_FWD_DEFINED__
#define __IISExtApplicationPools_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtApplicationPools IISExtApplicationPools;
#else
typedef struct IISExtApplicationPools IISExtApplicationPools;
#endif /* __cplusplus */

#endif 	/* __IISExtApplicationPools_FWD_DEFINED__ */


#ifndef __IISExtWebService_FWD_DEFINED__
#define __IISExtWebService_FWD_DEFINED__

#ifdef __cplusplus
typedef class IISExtWebService IISExtWebService;
#else
typedef struct IISExtWebService IISExtWebService;
#endif /* __cplusplus */

#endif 	/* __IISExtWebService_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 

void * __RPC_USER MIDL_user_allocate(size_t);
void __RPC_USER MIDL_user_free( void * ); 


#ifndef __IISExt_LIBRARY_DEFINED__
#define __IISExt_LIBRARY_DEFINED__

/* library IISExt */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_IISExt;

#ifndef __IISApplicationPool_INTERFACE_DEFINED__
#define __IISApplicationPool_INTERFACE_DEFINED__

/* interface IISApplicationPool */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISApplicationPool;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0B3CB1E1-829A-4c06-8B09-F56DA1894C88")
    IISApplicationPool : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Recycle( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumAppsInPool( 
            /* [retval][out] */ VARIANT *bstrBuffer) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISApplicationPoolVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISApplicationPool * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISApplicationPool * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISApplicationPool * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISApplicationPool * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISApplicationPool * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISApplicationPool * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISApplicationPool * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Recycle )( 
            IISApplicationPool * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumAppsInPool )( 
            IISApplicationPool * This,
            /* [retval][out] */ VARIANT *bstrBuffer);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            IISApplicationPool * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IISApplicationPool * This);
        
        END_INTERFACE
    } IISApplicationPoolVtbl;

    interface IISApplicationPool
    {
        CONST_VTBL struct IISApplicationPoolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISApplicationPool_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISApplicationPool_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISApplicationPool_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISApplicationPool_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISApplicationPool_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISApplicationPool_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISApplicationPool_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISApplicationPool_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISApplicationPool_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISApplicationPool_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISApplicationPool_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISApplicationPool_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISApplicationPool_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISApplicationPool_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISApplicationPool_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISApplicationPool_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISApplicationPool_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISApplicationPool_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISApplicationPool_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISApplicationPool_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISApplicationPool_Recycle(This)	\
    (This)->lpVtbl -> Recycle(This)

#define IISApplicationPool_EnumAppsInPool(This,bstrBuffer)	\
    (This)->lpVtbl -> EnumAppsInPool(This,bstrBuffer)

#define IISApplicationPool_Start(This)	\
    (This)->lpVtbl -> Start(This)

#define IISApplicationPool_Stop(This)	\
    (This)->lpVtbl -> Stop(This)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISApplicationPool_Recycle_Proxy( 
    IISApplicationPool * This);


void __RPC_STUB IISApplicationPool_Recycle_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApplicationPool_EnumAppsInPool_Proxy( 
    IISApplicationPool * This,
    /* [retval][out] */ VARIANT *bstrBuffer);


void __RPC_STUB IISApplicationPool_EnumAppsInPool_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApplicationPool_Start_Proxy( 
    IISApplicationPool * This);


void __RPC_STUB IISApplicationPool_Start_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApplicationPool_Stop_Proxy( 
    IISApplicationPool * This);


void __RPC_STUB IISApplicationPool_Stop_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISApplicationPool_INTERFACE_DEFINED__ */


#ifndef __IISApplicationPools_INTERFACE_DEFINED__
#define __IISApplicationPools_INTERFACE_DEFINED__

/* interface IISApplicationPools */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISApplicationPools;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("587F123F-49B4-49dd-939E-F4547AA3FA75")
    IISApplicationPools : public IADs
    {
    public:
    };
    
#else 	/* C style interface */

    typedef struct IISApplicationPoolsVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISApplicationPools * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISApplicationPools * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISApplicationPools * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISApplicationPools * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISApplicationPools * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISApplicationPools * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISApplicationPools * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        END_INTERFACE
    } IISApplicationPoolsVtbl;

    interface IISApplicationPools
    {
        CONST_VTBL struct IISApplicationPoolsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISApplicationPools_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISApplicationPools_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISApplicationPools_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISApplicationPools_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISApplicationPools_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISApplicationPools_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISApplicationPools_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISApplicationPools_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISApplicationPools_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISApplicationPools_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISApplicationPools_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISApplicationPools_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISApplicationPools_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISApplicationPools_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISApplicationPools_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISApplicationPools_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISApplicationPools_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISApplicationPools_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISApplicationPools_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISApplicationPools_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISApplicationPools_INTERFACE_DEFINED__ */


#ifndef __IISWebService_INTERFACE_DEFINED__
#define __IISWebService_INTERFACE_DEFINED__

/* interface IISWebService */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISWebService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE46D40C-1B38-4a02-898D-358E74DFC9D2")
    IISWebService : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCurrentMode( 
            /* [retval][out] */ VARIANT *pvServerMode) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateNewSite( 
            /* [in] */ BSTR bstrServerComment,
            /* [in] */ VARIANT *pvServerBindings,
            /* [in] */ BSTR bstrRootVDirPath,
            /* [defaultvalue][in] */ VARIANT vServerID,
            /* [retval][out] */ VARIANT *pvActualID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnableApplication( 
            /* [in] */ BSTR bstrAppName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RemoveApplication( 
            /* [in] */ BSTR bstrAppName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ListApplications( 
            /* [retval][out] */ VARIANT *bstrBuffer) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddDependency( 
            /* [in] */ BSTR bstrApplication,
            /* [in] */ BSTR bstrGroupID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RemoveDependency( 
            /* [in] */ BSTR bstrApplication,
            /* [in] */ BSTR bstrGroupID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnableWebServiceExtension( 
            /* [in] */ BSTR bstrExtension) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DisableWebServiceExtension( 
            /* [in] */ BSTR bstrExtension) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ListWebServiceExtensions( 
            /* [retval][out] */ VARIANT *bstrBuffer) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnableExtensionFile( 
            /* [in] */ BSTR bstrExtensionFile) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DisableExtensionFile( 
            /* [in] */ BSTR bstrExtensionFile) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddExtensionFile( 
            /* [in] */ BSTR bstrExtensionFile,
            /* [in] */ VARIANT bAccess,
            /* [in] */ BSTR bstrGroupID,
            /* [in] */ VARIANT bCanDelete,
            /* [in] */ BSTR bstrDescription) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteExtensionFileRecord( 
            /* [in] */ BSTR bstrExtensionFile) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ListExtensionFiles( 
            /* [retval][out] */ VARIANT *bstrBuffer) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE QueryGroupIDStatus( 
            /* [in] */ BSTR bstrGroupID,
            /* [retval][out] */ VARIANT *bstrBuffer) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISWebServiceVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISWebService * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISWebService * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISWebService * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISWebService * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISWebService * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISWebService * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISWebService * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISWebService * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISWebService * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISWebService * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISWebService * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISWebService * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISWebService * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISWebService * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISWebService * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentMode )( 
            IISWebService * This,
            /* [retval][out] */ VARIANT *pvServerMode);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateNewSite )( 
            IISWebService * This,
            /* [in] */ BSTR bstrServerComment,
            /* [in] */ VARIANT *pvServerBindings,
            /* [in] */ BSTR bstrRootVDirPath,
            /* [defaultvalue][in] */ VARIANT vServerID,
            /* [retval][out] */ VARIANT *pvActualID);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableApplication )( 
            IISWebService * This,
            /* [in] */ BSTR bstrAppName);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RemoveApplication )( 
            IISWebService * This,
            /* [in] */ BSTR bstrAppName);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListApplications )( 
            IISWebService * This,
            /* [retval][out] */ VARIANT *bstrBuffer);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDependency )( 
            IISWebService * This,
            /* [in] */ BSTR bstrApplication,
            /* [in] */ BSTR bstrGroupID);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RemoveDependency )( 
            IISWebService * This,
            /* [in] */ BSTR bstrApplication,
            /* [in] */ BSTR bstrGroupID);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableWebServiceExtension )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtension);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DisableWebServiceExtension )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtension);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListWebServiceExtensions )( 
            IISWebService * This,
            /* [retval][out] */ VARIANT *bstrBuffer);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableExtensionFile )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtensionFile);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DisableExtensionFile )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtensionFile);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddExtensionFile )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtensionFile,
            /* [in] */ VARIANT bAccess,
            /* [in] */ BSTR bstrGroupID,
            /* [in] */ VARIANT bCanDelete,
            /* [in] */ BSTR bstrDescription);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteExtensionFileRecord )( 
            IISWebService * This,
            /* [in] */ BSTR bstrExtensionFile);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListExtensionFiles )( 
            IISWebService * This,
            /* [retval][out] */ VARIANT *bstrBuffer);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *QueryGroupIDStatus )( 
            IISWebService * This,
            /* [in] */ BSTR bstrGroupID,
            /* [retval][out] */ VARIANT *bstrBuffer);
        
        END_INTERFACE
    } IISWebServiceVtbl;

    interface IISWebService
    {
        CONST_VTBL struct IISWebServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISWebService_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISWebService_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISWebService_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISWebService_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISWebService_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISWebService_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISWebService_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISWebService_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISWebService_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISWebService_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISWebService_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISWebService_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISWebService_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISWebService_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISWebService_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISWebService_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISWebService_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISWebService_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISWebService_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISWebService_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISWebService_GetCurrentMode(This,pvServerMode)	\
    (This)->lpVtbl -> GetCurrentMode(This,pvServerMode)

#define IISWebService_CreateNewSite(This,bstrServerComment,pvServerBindings,bstrRootVDirPath,vServerID,pvActualID)	\
    (This)->lpVtbl -> CreateNewSite(This,bstrServerComment,pvServerBindings,bstrRootVDirPath,vServerID,pvActualID)

#define IISWebService_EnableApplication(This,bstrAppName)	\
    (This)->lpVtbl -> EnableApplication(This,bstrAppName)

#define IISWebService_RemoveApplication(This,bstrAppName)	\
    (This)->lpVtbl -> RemoveApplication(This,bstrAppName)

#define IISWebService_ListApplications(This,bstrBuffer)	\
    (This)->lpVtbl -> ListApplications(This,bstrBuffer)

#define IISWebService_AddDependency(This,bstrApplication,bstrGroupID)	\
    (This)->lpVtbl -> AddDependency(This,bstrApplication,bstrGroupID)

#define IISWebService_RemoveDependency(This,bstrApplication,bstrGroupID)	\
    (This)->lpVtbl -> RemoveDependency(This,bstrApplication,bstrGroupID)

#define IISWebService_EnableWebServiceExtension(This,bstrExtension)	\
    (This)->lpVtbl -> EnableWebServiceExtension(This,bstrExtension)

#define IISWebService_DisableWebServiceExtension(This,bstrExtension)	\
    (This)->lpVtbl -> DisableWebServiceExtension(This,bstrExtension)

#define IISWebService_ListWebServiceExtensions(This,bstrBuffer)	\
    (This)->lpVtbl -> ListWebServiceExtensions(This,bstrBuffer)

#define IISWebService_EnableExtensionFile(This,bstrExtensionFile)	\
    (This)->lpVtbl -> EnableExtensionFile(This,bstrExtensionFile)

#define IISWebService_DisableExtensionFile(This,bstrExtensionFile)	\
    (This)->lpVtbl -> DisableExtensionFile(This,bstrExtensionFile)

#define IISWebService_AddExtensionFile(This,bstrExtensionFile,bAccess,bstrGroupID,bCanDelete,bstrDescription)	\
    (This)->lpVtbl -> AddExtensionFile(This,bstrExtensionFile,bAccess,bstrGroupID,bCanDelete,bstrDescription)

#define IISWebService_DeleteExtensionFileRecord(This,bstrExtensionFile)	\
    (This)->lpVtbl -> DeleteExtensionFileRecord(This,bstrExtensionFile)

#define IISWebService_ListExtensionFiles(This,bstrBuffer)	\
    (This)->lpVtbl -> ListExtensionFiles(This,bstrBuffer)

#define IISWebService_QueryGroupIDStatus(This,bstrGroupID,bstrBuffer)	\
    (This)->lpVtbl -> QueryGroupIDStatus(This,bstrGroupID,bstrBuffer)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_GetCurrentMode_Proxy( 
    IISWebService * This,
    /* [retval][out] */ VARIANT *pvServerMode);


void __RPC_STUB IISWebService_GetCurrentMode_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_CreateNewSite_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrServerComment,
    /* [in] */ VARIANT *pvServerBindings,
    /* [in] */ BSTR bstrRootVDirPath,
    /* [defaultvalue][in] */ VARIANT vServerID,
    /* [retval][out] */ VARIANT *pvActualID);


void __RPC_STUB IISWebService_CreateNewSite_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_EnableApplication_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrAppName);


void __RPC_STUB IISWebService_EnableApplication_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_RemoveApplication_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrAppName);


void __RPC_STUB IISWebService_RemoveApplication_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_ListApplications_Proxy( 
    IISWebService * This,
    /* [retval][out] */ VARIANT *bstrBuffer);


void __RPC_STUB IISWebService_ListApplications_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_AddDependency_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrApplication,
    /* [in] */ BSTR bstrGroupID);


void __RPC_STUB IISWebService_AddDependency_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_RemoveDependency_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrApplication,
    /* [in] */ BSTR bstrGroupID);


void __RPC_STUB IISWebService_RemoveDependency_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_EnableWebServiceExtension_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtension);


void __RPC_STUB IISWebService_EnableWebServiceExtension_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_DisableWebServiceExtension_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtension);


void __RPC_STUB IISWebService_DisableWebServiceExtension_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_ListWebServiceExtensions_Proxy( 
    IISWebService * This,
    /* [retval][out] */ VARIANT *bstrBuffer);


void __RPC_STUB IISWebService_ListWebServiceExtensions_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_EnableExtensionFile_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtensionFile);


void __RPC_STUB IISWebService_EnableExtensionFile_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_DisableExtensionFile_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtensionFile);


void __RPC_STUB IISWebService_DisableExtensionFile_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_AddExtensionFile_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtensionFile,
    /* [in] */ VARIANT bAccess,
    /* [in] */ BSTR bstrGroupID,
    /* [in] */ VARIANT bCanDelete,
    /* [in] */ BSTR bstrDescription);


void __RPC_STUB IISWebService_AddExtensionFile_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_DeleteExtensionFileRecord_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrExtensionFile);


void __RPC_STUB IISWebService_DeleteExtensionFileRecord_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_ListExtensionFiles_Proxy( 
    IISWebService * This,
    /* [retval][out] */ VARIANT *bstrBuffer);


void __RPC_STUB IISWebService_ListExtensionFiles_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISWebService_QueryGroupIDStatus_Proxy( 
    IISWebService * This,
    /* [in] */ BSTR bstrGroupID,
    /* [retval][out] */ VARIANT *bstrBuffer);


void __RPC_STUB IISWebService_QueryGroupIDStatus_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISWebService_INTERFACE_DEFINED__ */


#ifndef __IISDsCrMap_INTERFACE_DEFINED__
#define __IISDsCrMap_INTERFACE_DEFINED__

/* interface IISDsCrMap */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISDsCrMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("edcd6a60-b053-11d0-a62f-00a0c922e752")
    IISDsCrMap : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateMapping( 
            /* [in] */ VARIANT vCert,
            /* [in] */ BSTR bstrNtAcct,
            /* [in] */ BSTR bstrNtPwd,
            /* [in] */ BSTR bstrName,
            /* [in] */ LONG lEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetMapping( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [out] */ VARIANT *pvCert,
            /* [out] */ VARIANT *pbstrNtAcct,
            /* [out] */ VARIANT *pbstrNtPwd,
            /* [out] */ VARIANT *pbstrName,
            /* [out] */ VARIANT *plEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteMapping( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetEnabled( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ LONG lEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetName( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetPwd( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrPwd) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetAcct( 
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrAcct) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISDsCrMapVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISDsCrMap * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISDsCrMap * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISDsCrMap * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISDsCrMap * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISDsCrMap * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISDsCrMap * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISDsCrMap * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateMapping )( 
            IISDsCrMap * This,
            /* [in] */ VARIANT vCert,
            /* [in] */ BSTR bstrNtAcct,
            /* [in] */ BSTR bstrNtPwd,
            /* [in] */ BSTR bstrName,
            /* [in] */ LONG lEnabled);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetMapping )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [out] */ VARIANT *pvCert,
            /* [out] */ VARIANT *pbstrNtAcct,
            /* [out] */ VARIANT *pbstrNtPwd,
            /* [out] */ VARIANT *pbstrName,
            /* [out] */ VARIANT *plEnabled);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteMapping )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetEnabled )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ LONG lEnabled);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetName )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrName);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPwd )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrPwd);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetAcct )( 
            IISDsCrMap * This,
            /* [in] */ LONG lMethod,
            /* [in] */ VARIANT vKey,
            /* [in] */ BSTR bstrAcct);
        
        END_INTERFACE
    } IISDsCrMapVtbl;

    interface IISDsCrMap
    {
        CONST_VTBL struct IISDsCrMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDsCrMap_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISDsCrMap_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISDsCrMap_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISDsCrMap_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISDsCrMap_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISDsCrMap_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISDsCrMap_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISDsCrMap_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISDsCrMap_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISDsCrMap_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISDsCrMap_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISDsCrMap_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISDsCrMap_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISDsCrMap_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISDsCrMap_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISDsCrMap_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISDsCrMap_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISDsCrMap_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISDsCrMap_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISDsCrMap_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISDsCrMap_CreateMapping(This,vCert,bstrNtAcct,bstrNtPwd,bstrName,lEnabled)	\
    (This)->lpVtbl -> CreateMapping(This,vCert,bstrNtAcct,bstrNtPwd,bstrName,lEnabled)

#define IISDsCrMap_GetMapping(This,lMethod,vKey,pvCert,pbstrNtAcct,pbstrNtPwd,pbstrName,plEnabled)	\
    (This)->lpVtbl -> GetMapping(This,lMethod,vKey,pvCert,pbstrNtAcct,pbstrNtPwd,pbstrName,plEnabled)

#define IISDsCrMap_DeleteMapping(This,lMethod,vKey)	\
    (This)->lpVtbl -> DeleteMapping(This,lMethod,vKey)

#define IISDsCrMap_SetEnabled(This,lMethod,vKey,lEnabled)	\
    (This)->lpVtbl -> SetEnabled(This,lMethod,vKey,lEnabled)

#define IISDsCrMap_SetName(This,lMethod,vKey,bstrName)	\
    (This)->lpVtbl -> SetName(This,lMethod,vKey,bstrName)

#define IISDsCrMap_SetPwd(This,lMethod,vKey,bstrPwd)	\
    (This)->lpVtbl -> SetPwd(This,lMethod,vKey,bstrPwd)

#define IISDsCrMap_SetAcct(This,lMethod,vKey,bstrAcct)	\
    (This)->lpVtbl -> SetAcct(This,lMethod,vKey,bstrAcct)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_CreateMapping_Proxy( 
    IISDsCrMap * This,
    /* [in] */ VARIANT vCert,
    /* [in] */ BSTR bstrNtAcct,
    /* [in] */ BSTR bstrNtPwd,
    /* [in] */ BSTR bstrName,
    /* [in] */ LONG lEnabled);


void __RPC_STUB IISDsCrMap_CreateMapping_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_GetMapping_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey,
    /* [out] */ VARIANT *pvCert,
    /* [out] */ VARIANT *pbstrNtAcct,
    /* [out] */ VARIANT *pbstrNtPwd,
    /* [out] */ VARIANT *pbstrName,
    /* [out] */ VARIANT *plEnabled);


void __RPC_STUB IISDsCrMap_GetMapping_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_DeleteMapping_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey);


void __RPC_STUB IISDsCrMap_DeleteMapping_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_SetEnabled_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey,
    /* [in] */ LONG lEnabled);


void __RPC_STUB IISDsCrMap_SetEnabled_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_SetName_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey,
    /* [in] */ BSTR bstrName);


void __RPC_STUB IISDsCrMap_SetName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_SetPwd_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey,
    /* [in] */ BSTR bstrPwd);


void __RPC_STUB IISDsCrMap_SetPwd_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISDsCrMap_SetAcct_Proxy( 
    IISDsCrMap * This,
    /* [in] */ LONG lMethod,
    /* [in] */ VARIANT vKey,
    /* [in] */ BSTR bstrAcct);


void __RPC_STUB IISDsCrMap_SetAcct_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISDsCrMap_INTERFACE_DEFINED__ */


#ifndef __IISApp_INTERFACE_DEFINED__
#define __IISApp_INTERFACE_DEFINED__

/* interface IISApp */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46FBBB80-0192-11d1-9C39-00A0C922E703")
    IISApp : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppCreate( 
            /* [in] */ VARIANT_BOOL bSetInProcFlag) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppDelete( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppDeleteRecursive( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppUnLoad( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppUnLoadRecursive( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppDisable( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppDisableRecursive( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppEnable( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppEnableRecursive( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppGetStatus( 
            /* [out] */ DWORD *pdwStatus) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AspAppRestart( void) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISAppVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISApp * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISApp * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISApp * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISApp * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISApp * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISApp * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISApp * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISApp * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISApp * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISApp * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISApp * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISApp * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISApp * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISApp * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISApp * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate )( 
            IISApp * This,
            /* [in] */ VARIANT_BOOL bSetInProcFlag);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDelete )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDeleteRecursive )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoad )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoadRecursive )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisable )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisableRecursive )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnable )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnableRecursive )( 
            IISApp * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus )( 
            IISApp * This,
            /* [out] */ DWORD *pdwStatus);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AspAppRestart )( 
            IISApp * This);
        
        END_INTERFACE
    } IISAppVtbl;

    interface IISApp
    {
        CONST_VTBL struct IISAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISApp_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISApp_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISApp_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISApp_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISApp_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISApp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISApp_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISApp_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISApp_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISApp_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISApp_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISApp_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISApp_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISApp_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISApp_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISApp_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISApp_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISApp_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISApp_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISApp_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISApp_AppCreate(This,bSetInProcFlag)	\
    (This)->lpVtbl -> AppCreate(This,bSetInProcFlag)

#define IISApp_AppDelete(This)	\
    (This)->lpVtbl -> AppDelete(This)

#define IISApp_AppDeleteRecursive(This)	\
    (This)->lpVtbl -> AppDeleteRecursive(This)

#define IISApp_AppUnLoad(This)	\
    (This)->lpVtbl -> AppUnLoad(This)

#define IISApp_AppUnLoadRecursive(This)	\
    (This)->lpVtbl -> AppUnLoadRecursive(This)

#define IISApp_AppDisable(This)	\
    (This)->lpVtbl -> AppDisable(This)

#define IISApp_AppDisableRecursive(This)	\
    (This)->lpVtbl -> AppDisableRecursive(This)

#define IISApp_AppEnable(This)	\
    (This)->lpVtbl -> AppEnable(This)

#define IISApp_AppEnableRecursive(This)	\
    (This)->lpVtbl -> AppEnableRecursive(This)

#define IISApp_AppGetStatus(This,pdwStatus)	\
    (This)->lpVtbl -> AppGetStatus(This,pdwStatus)

#define IISApp_AspAppRestart(This)	\
    (This)->lpVtbl -> AspAppRestart(This)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppCreate_Proxy( 
    IISApp * This,
    /* [in] */ VARIANT_BOOL bSetInProcFlag);


void __RPC_STUB IISApp_AppCreate_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppDelete_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppDelete_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppDeleteRecursive_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppDeleteRecursive_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppUnLoad_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppUnLoad_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppUnLoadRecursive_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppUnLoadRecursive_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppDisable_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppDisable_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppDisableRecursive_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppDisableRecursive_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppEnable_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppEnable_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppEnableRecursive_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AppEnableRecursive_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AppGetStatus_Proxy( 
    IISApp * This,
    /* [out] */ DWORD *pdwStatus);


void __RPC_STUB IISApp_AppGetStatus_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp_AspAppRestart_Proxy( 
    IISApp * This);


void __RPC_STUB IISApp_AspAppRestart_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISApp_INTERFACE_DEFINED__ */


#ifndef __IISApp2_INTERFACE_DEFINED__
#define __IISApp2_INTERFACE_DEFINED__

/* interface IISApp2 */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISApp2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("603DCBEA-7350-11d2-A7BE-0000F8085B95")
    IISApp2 : public IISApp
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppCreate2( 
            /* [in] */ LONG lAppMode) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppGetStatus2( 
            /* [retval][out] */ LONG *lpStatus) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISApp2Vtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISApp2 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISApp2 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISApp2 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISApp2 * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISApp2 * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISApp2 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISApp2 * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISApp2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISApp2 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISApp2 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISApp2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISApp2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISApp2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISApp2 * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISApp2 * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate )( 
            IISApp2 * This,
            /* [in] */ VARIANT_BOOL bSetInProcFlag);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDelete )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDeleteRecursive )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoad )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoadRecursive )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisable )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisableRecursive )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnable )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnableRecursive )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus )( 
            IISApp2 * This,
            /* [out] */ DWORD *pdwStatus);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AspAppRestart )( 
            IISApp2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate2 )( 
            IISApp2 * This,
            /* [in] */ LONG lAppMode);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus2 )( 
            IISApp2 * This,
            /* [retval][out] */ LONG *lpStatus);
        
        END_INTERFACE
    } IISApp2Vtbl;

    interface IISApp2
    {
        CONST_VTBL struct IISApp2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISApp2_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISApp2_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISApp2_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISApp2_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISApp2_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISApp2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISApp2_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISApp2_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISApp2_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISApp2_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISApp2_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISApp2_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISApp2_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISApp2_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISApp2_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISApp2_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISApp2_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISApp2_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISApp2_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISApp2_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISApp2_AppCreate(This,bSetInProcFlag)	\
    (This)->lpVtbl -> AppCreate(This,bSetInProcFlag)

#define IISApp2_AppDelete(This)	\
    (This)->lpVtbl -> AppDelete(This)

#define IISApp2_AppDeleteRecursive(This)	\
    (This)->lpVtbl -> AppDeleteRecursive(This)

#define IISApp2_AppUnLoad(This)	\
    (This)->lpVtbl -> AppUnLoad(This)

#define IISApp2_AppUnLoadRecursive(This)	\
    (This)->lpVtbl -> AppUnLoadRecursive(This)

#define IISApp2_AppDisable(This)	\
    (This)->lpVtbl -> AppDisable(This)

#define IISApp2_AppDisableRecursive(This)	\
    (This)->lpVtbl -> AppDisableRecursive(This)

#define IISApp2_AppEnable(This)	\
    (This)->lpVtbl -> AppEnable(This)

#define IISApp2_AppEnableRecursive(This)	\
    (This)->lpVtbl -> AppEnableRecursive(This)

#define IISApp2_AppGetStatus(This,pdwStatus)	\
    (This)->lpVtbl -> AppGetStatus(This,pdwStatus)

#define IISApp2_AspAppRestart(This)	\
    (This)->lpVtbl -> AspAppRestart(This)


#define IISApp2_AppCreate2(This,lAppMode)	\
    (This)->lpVtbl -> AppCreate2(This,lAppMode)

#define IISApp2_AppGetStatus2(This,lpStatus)	\
    (This)->lpVtbl -> AppGetStatus2(This,lpStatus)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISApp2_AppCreate2_Proxy( 
    IISApp2 * This,
    /* [in] */ LONG lAppMode);


void __RPC_STUB IISApp2_AppCreate2_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISApp2_AppGetStatus2_Proxy( 
    IISApp2 * This,
    /* [retval][out] */ LONG *lpStatus);


void __RPC_STUB IISApp2_AppGetStatus2_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISApp2_INTERFACE_DEFINED__ */


#ifndef __IISApp3_INTERFACE_DEFINED__
#define __IISApp3_INTERFACE_DEFINED__

/* interface IISApp3 */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISApp3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2812B639-8FAC-4510-96C5-71DDBD1F54FC")
    IISApp3 : public IISApp2
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AppCreate3( 
            /* [in] */ LONG lAppMode,
            /* [in] */ VARIANT bstrAppPooI,
            /* [in] */ VARIANT bCreatePool) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISApp3Vtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISApp3 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISApp3 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISApp3 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISApp3 * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISApp3 * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISApp3 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISApp3 * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISApp3 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISApp3 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISApp3 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISApp3 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISApp3 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISApp3 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISApp3 * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISApp3 * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate )( 
            IISApp3 * This,
            /* [in] */ VARIANT_BOOL bSetInProcFlag);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDelete )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDeleteRecursive )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoad )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoadRecursive )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisable )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppDisableRecursive )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnable )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppEnableRecursive )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus )( 
            IISApp3 * This,
            /* [out] */ DWORD *pdwStatus);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AspAppRestart )( 
            IISApp3 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate2 )( 
            IISApp3 * This,
            /* [in] */ LONG lAppMode);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus2 )( 
            IISApp3 * This,
            /* [retval][out] */ LONG *lpStatus);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AppCreate3 )( 
            IISApp3 * This,
            /* [in] */ LONG lAppMode,
            /* [in] */ VARIANT bstrAppPooI,
            /* [in] */ VARIANT bCreatePool);
        
        END_INTERFACE
    } IISApp3Vtbl;

    interface IISApp3
    {
        CONST_VTBL struct IISApp3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISApp3_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISApp3_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISApp3_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISApp3_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISApp3_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISApp3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISApp3_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISApp3_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISApp3_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISApp3_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISApp3_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISApp3_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISApp3_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISApp3_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISApp3_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISApp3_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISApp3_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISApp3_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISApp3_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISApp3_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISApp3_AppCreate(This,bSetInProcFlag)	\
    (This)->lpVtbl -> AppCreate(This,bSetInProcFlag)

#define IISApp3_AppDelete(This)	\
    (This)->lpVtbl -> AppDelete(This)

#define IISApp3_AppDeleteRecursive(This)	\
    (This)->lpVtbl -> AppDeleteRecursive(This)

#define IISApp3_AppUnLoad(This)	\
    (This)->lpVtbl -> AppUnLoad(This)

#define IISApp3_AppUnLoadRecursive(This)	\
    (This)->lpVtbl -> AppUnLoadRecursive(This)

#define IISApp3_AppDisable(This)	\
    (This)->lpVtbl -> AppDisable(This)

#define IISApp3_AppDisableRecursive(This)	\
    (This)->lpVtbl -> AppDisableRecursive(This)

#define IISApp3_AppEnable(This)	\
    (This)->lpVtbl -> AppEnable(This)

#define IISApp3_AppEnableRecursive(This)	\
    (This)->lpVtbl -> AppEnableRecursive(This)

#define IISApp3_AppGetStatus(This,pdwStatus)	\
    (This)->lpVtbl -> AppGetStatus(This,pdwStatus)

#define IISApp3_AspAppRestart(This)	\
    (This)->lpVtbl -> AspAppRestart(This)


#define IISApp3_AppCreate2(This,lAppMode)	\
    (This)->lpVtbl -> AppCreate2(This,lAppMode)

#define IISApp3_AppGetStatus2(This,lpStatus)	\
    (This)->lpVtbl -> AppGetStatus2(This,lpStatus)


#define IISApp3_AppCreate3(This,lAppMode,bstrAppPooI,bCreatePool)	\
    (This)->lpVtbl -> AppCreate3(This,lAppMode,bstrAppPooI,bCreatePool)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISApp3_AppCreate3_Proxy( 
    IISApp3 * This,
    /* [in] */ LONG lAppMode,
    /* [in] */ VARIANT bstrAppPooI,
    /* [in] */ VARIANT bCreatePool);


void __RPC_STUB IISApp3_AppCreate3_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISApp3_INTERFACE_DEFINED__ */


#ifndef __IISComputer_INTERFACE_DEFINED__
#define __IISComputer_INTERFACE_DEFINED__

/* interface IISComputer */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISComputer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF87A2E0-078B-11d1-9C3D-00A0C922E703")
    IISComputer : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Backup( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Restore( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnumBackups( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lIndex,
            /* [out] */ VARIANT *pvVersion,
            /* [out] */ VARIANT *pvLocations,
            /* [out] */ VARIANT *pvDate) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteBackup( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISComputerVtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISComputer * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISComputer * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISComputer * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISComputer * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISComputer * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISComputer * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISComputer * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISComputer * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISComputer * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISComputer * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISComputer * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISComputer * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISComputer * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISComputer * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISComputer * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            IISComputer * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Restore )( 
            IISComputer * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumBackups )( 
            IISComputer * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lIndex,
            /* [out] */ VARIANT *pvVersion,
            /* [out] */ VARIANT *pvLocations,
            /* [out] */ VARIANT *pvDate);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteBackup )( 
            IISComputer * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion);
        
        END_INTERFACE
    } IISComputerVtbl;

    interface IISComputer
    {
        CONST_VTBL struct IISComputerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISComputer_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISComputer_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISComputer_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISComputer_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISComputer_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISComputer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISComputer_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISComputer_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISComputer_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISComputer_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISComputer_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISComputer_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISComputer_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISComputer_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISComputer_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISComputer_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISComputer_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISComputer_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISComputer_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISComputer_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISComputer_Backup(This,bstrLocation,lVersion,lFlags)	\
    (This)->lpVtbl -> Backup(This,bstrLocation,lVersion,lFlags)

#define IISComputer_Restore(This,bstrLocation,lVersion,lFlags)	\
    (This)->lpVtbl -> Restore(This,bstrLocation,lVersion,lFlags)

#define IISComputer_EnumBackups(This,bstrLocation,lIndex,pvVersion,pvLocations,pvDate)	\
    (This)->lpVtbl -> EnumBackups(This,bstrLocation,lIndex,pvVersion,pvLocations,pvDate)

#define IISComputer_DeleteBackup(This,bstrLocation,lVersion)	\
    (This)->lpVtbl -> DeleteBackup(This,bstrLocation,lVersion)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer_Backup_Proxy( 
    IISComputer * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lVersion,
    /* [in] */ LONG lFlags);


void __RPC_STUB IISComputer_Backup_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer_Restore_Proxy( 
    IISComputer * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lVersion,
    /* [in] */ LONG lFlags);


void __RPC_STUB IISComputer_Restore_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer_EnumBackups_Proxy( 
    IISComputer * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lIndex,
    /* [out] */ VARIANT *pvVersion,
    /* [out] */ VARIANT *pvLocations,
    /* [out] */ VARIANT *pvDate);


void __RPC_STUB IISComputer_EnumBackups_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer_DeleteBackup_Proxy( 
    IISComputer * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lVersion);


void __RPC_STUB IISComputer_DeleteBackup_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISComputer_INTERFACE_DEFINED__ */


#ifndef __IISComputer2_INTERFACE_DEFINED__
#define __IISComputer2_INTERFACE_DEFINED__

/* interface IISComputer2 */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IISComputer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63d89839-5762-4a68-b1b9-3507ea76cbbf")
    IISComputer2 : public IISComputer
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BackupWithPassword( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrPassword) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RestoreWithPassword( 
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrPassword) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Export( 
            /* [in] */ BSTR bstrPassword,
            /* [in] */ BSTR bstrFilename,
            /* [in] */ BSTR bstrSourcePath,
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Import( 
            /* [in] */ BSTR bstrPassword,
            /* [in] */ BSTR bstrFilename,
            /* [in] */ BSTR bstrSourcePath,
            /* [in] */ BSTR bstrDestPath,
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SaveData( void) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IISComputer2Vtbl
    {
        BEGIN_INTERFACE
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [out][idldescattr] */ void **ppvObj,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *AddRef )( 
            IISComputer2 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Release )( 
            IISComputer2 * This,
            /* [retval][out] */ unsigned long *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IISComputer2 * This,
            /* [out][idldescattr] */ UINT *pctinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ UINT itinfo,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ void **pptinfo,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ signed char **rgszNames,
            /* [in][idldescattr] */ UINT cNames,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [out][idldescattr] */ signed long *rgdispid,
            /* [retval][out] */ void *retval);
        
        /* [id][restricted][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ signed long dispidMember,
            /* [in][idldescattr] */ GUID *riid,
            /* [in][idldescattr] */ unsigned long lcid,
            /* [in][idldescattr] */ unsigned short wFlags,
            /* [in][idldescattr] */ DISPPARAMS *pdispparams,
            /* [out][idldescattr] */ VARIANT *pvarResult,
            /* [out][idldescattr] */ EXCEPINFO *pexcepinfo,
            /* [out][idldescattr] */ UINT *puArgErr,
            /* [retval][out] */ void *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][propget][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            IISComputer2 * This,
            /* [retval][out] */ BSTR *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IISComputer2 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            IISComputer2 * This,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [retval][out] */ VARIANT *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ signed long lnControlCode,
            /* [in][idldescattr] */ BSTR bstrName,
            /* [in][idldescattr] */ VARIANT vProp,
            /* [retval][out] */ void *retval);
        
        /* [id][funcdescattr] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            IISComputer2 * This,
            /* [in][idldescattr] */ VARIANT vProperties,
            /* [in][idldescattr] */ signed long lnReserved,
            /* [retval][out] */ void *retval);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Restore )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnumBackups )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lIndex,
            /* [out] */ VARIANT *pvVersion,
            /* [out] */ VARIANT *pvLocations,
            /* [out] */ VARIANT *pvDate);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteBackup )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BackupWithPassword )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrPassword);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RestoreWithPassword )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrLocation,
            /* [in] */ LONG lVersion,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrPassword);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Export )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrPassword,
            /* [in] */ BSTR bstrFilename,
            /* [in] */ BSTR bstrSourcePath,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            IISComputer2 * This,
            /* [in] */ BSTR bstrPassword,
            /* [in] */ BSTR bstrFilename,
            /* [in] */ BSTR bstrSourcePath,
            /* [in] */ BSTR bstrDestPath,
            /* [in] */ LONG lFlags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SaveData )( 
            IISComputer2 * This);
        
        END_INTERFACE
    } IISComputer2Vtbl;

    interface IISComputer2
    {
        CONST_VTBL struct IISComputer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISComputer2_QueryInterface(This,riid,ppvObj,retval)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObj,retval)

#define IISComputer2_AddRef(This,retval)	\
    (This)->lpVtbl -> AddRef(This,retval)

#define IISComputer2_Release(This,retval)	\
    (This)->lpVtbl -> Release(This,retval)

#define IISComputer2_GetTypeInfoCount(This,pctinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo,retval)

#define IISComputer2_GetTypeInfo(This,itinfo,lcid,pptinfo,retval)	\
    (This)->lpVtbl -> GetTypeInfo(This,itinfo,lcid,pptinfo,retval)

#define IISComputer2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid,retval)

#define IISComputer2_Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)	\
    (This)->lpVtbl -> Invoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr,retval)

#define IISComputer2_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define IISComputer2_get_Class(This,retval)	\
    (This)->lpVtbl -> get_Class(This,retval)

#define IISComputer2_get_GUID(This,retval)	\
    (This)->lpVtbl -> get_GUID(This,retval)

#define IISComputer2_get_ADsPath(This,retval)	\
    (This)->lpVtbl -> get_ADsPath(This,retval)

#define IISComputer2_get_Parent(This,retval)	\
    (This)->lpVtbl -> get_Parent(This,retval)

#define IISComputer2_get_Schema(This,retval)	\
    (This)->lpVtbl -> get_Schema(This,retval)

#define IISComputer2_GetInfo(This,retval)	\
    (This)->lpVtbl -> GetInfo(This,retval)

#define IISComputer2_SetInfo(This,retval)	\
    (This)->lpVtbl -> SetInfo(This,retval)

#define IISComputer2_Get(This,bstrName,retval)	\
    (This)->lpVtbl -> Get(This,bstrName,retval)

#define IISComputer2_Put(This,bstrName,vProp,retval)	\
    (This)->lpVtbl -> Put(This,bstrName,vProp,retval)

#define IISComputer2_GetEx(This,bstrName,retval)	\
    (This)->lpVtbl -> GetEx(This,bstrName,retval)

#define IISComputer2_PutEx(This,lnControlCode,bstrName,vProp,retval)	\
    (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp,retval)

#define IISComputer2_GetInfoEx(This,vProperties,lnReserved,retval)	\
    (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved,retval)


#define IISComputer2_Backup(This,bstrLocation,lVersion,lFlags)	\
    (This)->lpVtbl -> Backup(This,bstrLocation,lVersion,lFlags)

#define IISComputer2_Restore(This,bstrLocation,lVersion,lFlags)	\
    (This)->lpVtbl -> Restore(This,bstrLocation,lVersion,lFlags)

#define IISComputer2_EnumBackups(This,bstrLocation,lIndex,pvVersion,pvLocations,pvDate)	\
    (This)->lpVtbl -> EnumBackups(This,bstrLocation,lIndex,pvVersion,pvLocations,pvDate)

#define IISComputer2_DeleteBackup(This,bstrLocation,lVersion)	\
    (This)->lpVtbl -> DeleteBackup(This,bstrLocation,lVersion)


#define IISComputer2_BackupWithPassword(This,bstrLocation,lVersion,lFlags,bstrPassword)	\
    (This)->lpVtbl -> BackupWithPassword(This,bstrLocation,lVersion,lFlags,bstrPassword)

#define IISComputer2_RestoreWithPassword(This,bstrLocation,lVersion,lFlags,bstrPassword)	\
    (This)->lpVtbl -> RestoreWithPassword(This,bstrLocation,lVersion,lFlags,bstrPassword)

#define IISComputer2_Export(This,bstrPassword,bstrFilename,bstrSourcePath,lFlags)	\
    (This)->lpVtbl -> Export(This,bstrPassword,bstrFilename,bstrSourcePath,lFlags)

#define IISComputer2_Import(This,bstrPassword,bstrFilename,bstrSourcePath,bstrDestPath,lFlags)	\
    (This)->lpVtbl -> Import(This,bstrPassword,bstrFilename,bstrSourcePath,bstrDestPath,lFlags)

#define IISComputer2_SaveData(This)	\
    (This)->lpVtbl -> SaveData(This)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer2_BackupWithPassword_Proxy( 
    IISComputer2 * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lVersion,
    /* [in] */ LONG lFlags,
    /* [in] */ BSTR bstrPassword);


void __RPC_STUB IISComputer2_BackupWithPassword_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer2_RestoreWithPassword_Proxy( 
    IISComputer2 * This,
    /* [in] */ BSTR bstrLocation,
    /* [in] */ LONG lVersion,
    /* [in] */ LONG lFlags,
    /* [in] */ BSTR bstrPassword);


void __RPC_STUB IISComputer2_RestoreWithPassword_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer2_Export_Proxy( 
    IISComputer2 * This,
    /* [in] */ BSTR bstrPassword,
    /* [in] */ BSTR bstrFilename,
    /* [in] */ BSTR bstrSourcePath,
    /* [in] */ LONG lFlags);


void __RPC_STUB IISComputer2_Export_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer2_Import_Proxy( 
    IISComputer2 * This,
    /* [in] */ BSTR bstrPassword,
    /* [in] */ BSTR bstrFilename,
    /* [in] */ BSTR bstrSourcePath,
    /* [in] */ BSTR bstrDestPath,
    /* [in] */ LONG lFlags);


void __RPC_STUB IISComputer2_Import_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [id] */ HRESULT STDMETHODCALLTYPE IISComputer2_SaveData_Proxy( 
    IISComputer2 * This);


void __RPC_STUB IISComputer2_SaveData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IISComputer2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_IISExtComputer;

#ifdef __cplusplus

class DECLSPEC_UUID("91ef9258-afec-11d1-9868-00a0c922e703")
IISExtComputer;
#endif

EXTERN_C const CLSID CLSID_IISExtApp;

#ifdef __cplusplus

class DECLSPEC_UUID("b4f34438-afec-11d1-9868-00a0c922e703")
IISExtApp;
#endif

EXTERN_C const CLSID CLSID_IISExtServer;

#ifdef __cplusplus

class DECLSPEC_UUID("c3b32488-afec-11d1-9868-00a0c922e703")
IISExtServer;
#endif

EXTERN_C const CLSID CLSID_IISExtDsCrMap;

#ifdef __cplusplus

class DECLSPEC_UUID("bc36cde8-afeb-11d1-9868-00a0c922e703")
IISExtDsCrMap;
#endif

EXTERN_C const CLSID CLSID_IISExtApplicationPool;

#ifdef __cplusplus

class DECLSPEC_UUID("E99F9D0C-FB39-402b-9EEB-AA185237BD34")
IISExtApplicationPool;
#endif

EXTERN_C const CLSID CLSID_IISExtApplicationPools;

#ifdef __cplusplus

class DECLSPEC_UUID("95863074-A389-406a-A2D7-D98BFC95B905")
IISExtApplicationPools;
#endif

EXTERN_C const CLSID CLSID_IISExtWebService;

#ifdef __cplusplus

class DECLSPEC_UUID("40B8F873-B30E-475d-BEC5-4D0EBB0DBAF3")
IISExtWebService;
#endif
#endif /* __IISExt_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

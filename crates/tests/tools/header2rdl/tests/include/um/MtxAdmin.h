/* this ALWAYS GENERATED file contains the definitions for the interfaces */


/* File created by MIDL compiler version 3.01.75 */
/* at Fri Nov 14 05:07:21 1997
 */
//@@MIDL_FILE_HEADING(  )
#include "rpc.h"
#include "rpcndr.h"
#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __mtxadmin_h__
#define __mtxadmin_h__

#ifdef __cplusplus
extern "C"{
#endif 

/* Forward Declarations */ 

#ifndef __ICatalog_FWD_DEFINED__
#define __ICatalog_FWD_DEFINED__
typedef interface ICatalog ICatalog;
#endif 	/* __ICatalog_FWD_DEFINED__ */


#ifndef __ICatalogObject_FWD_DEFINED__
#define __ICatalogObject_FWD_DEFINED__
typedef interface ICatalogObject ICatalogObject;
#endif 	/* __ICatalogObject_FWD_DEFINED__ */


#ifndef __ICatalogCollection_FWD_DEFINED__
#define __ICatalogCollection_FWD_DEFINED__
typedef interface ICatalogCollection ICatalogCollection;
#endif 	/* __ICatalogCollection_FWD_DEFINED__ */


#ifndef __IComponentUtil_FWD_DEFINED__
#define __IComponentUtil_FWD_DEFINED__
typedef interface IComponentUtil IComponentUtil;
#endif 	/* __IComponentUtil_FWD_DEFINED__ */


#ifndef __IPackageUtil_FWD_DEFINED__
#define __IPackageUtil_FWD_DEFINED__
typedef interface IPackageUtil IPackageUtil;
#endif 	/* __IPackageUtil_FWD_DEFINED__ */


#ifndef __IRemoteComponentUtil_FWD_DEFINED__
#define __IRemoteComponentUtil_FWD_DEFINED__
typedef interface IRemoteComponentUtil IRemoteComponentUtil;
#endif 	/* __IRemoteComponentUtil_FWD_DEFINED__ */


#ifndef __IRoleAssociationUtil_FWD_DEFINED__
#define __IRoleAssociationUtil_FWD_DEFINED__
typedef interface IRoleAssociationUtil IRoleAssociationUtil;
#endif 	/* __IRoleAssociationUtil_FWD_DEFINED__ */


#ifndef __Catalog_FWD_DEFINED__
#define __Catalog_FWD_DEFINED__

#ifdef __cplusplus
typedef class Catalog Catalog;
#else
typedef struct Catalog Catalog;
#endif /* __cplusplus */

#endif 	/* __Catalog_FWD_DEFINED__ */


#ifndef __CatalogObject_FWD_DEFINED__
#define __CatalogObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class CatalogObject CatalogObject;
#else
typedef struct CatalogObject CatalogObject;
#endif /* __cplusplus */

#endif 	/* __CatalogObject_FWD_DEFINED__ */


#ifndef __CatalogCollection_FWD_DEFINED__
#define __CatalogCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class CatalogCollection CatalogCollection;
#else
typedef struct CatalogCollection CatalogCollection;
#endif /* __cplusplus */

#endif 	/* __CatalogCollection_FWD_DEFINED__ */


#ifndef __ComponentUtil_FWD_DEFINED__
#define __ComponentUtil_FWD_DEFINED__

#ifdef __cplusplus
typedef class ComponentUtil ComponentUtil;
#else
typedef struct ComponentUtil ComponentUtil;
#endif /* __cplusplus */

#endif 	/* __ComponentUtil_FWD_DEFINED__ */


#ifndef __PackageUtil_FWD_DEFINED__
#define __PackageUtil_FWD_DEFINED__

#ifdef __cplusplus
typedef class PackageUtil PackageUtil;
#else
typedef struct PackageUtil PackageUtil;
#endif /* __cplusplus */

#endif 	/* __PackageUtil_FWD_DEFINED__ */


#ifndef __RemoteComponentUtil_FWD_DEFINED__
#define __RemoteComponentUtil_FWD_DEFINED__

#ifdef __cplusplus
typedef class RemoteComponentUtil RemoteComponentUtil;
#else
typedef struct RemoteComponentUtil RemoteComponentUtil;
#endif /* __cplusplus */

#endif 	/* __RemoteComponentUtil_FWD_DEFINED__ */


#ifndef __RoleAssociationUtil_FWD_DEFINED__
#define __RoleAssociationUtil_FWD_DEFINED__

#ifdef __cplusplus
typedef class RoleAssociationUtil RoleAssociationUtil;
#else
typedef struct RoleAssociationUtil RoleAssociationUtil;
#endif /* __cplusplus */

#endif 	/* __RoleAssociationUtil_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

/****************************************
 * Generated header for interface: __MIDL_itf_mtxadmin_0000
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [local] */ 


// -----------------------------------------------------------------------	
// mtxadmin.h  -- Microsoft Transaction Server Programming Interfaces				
//																			
// This file provides the prototypes for the APIs and COM interfaces			
// used by Microsoft Transaction Server applications.													
//																			
// Microsoft Transaction Server SDK												
// Copyright (c) 1997 Microsoft Corporation.  All Rights Reserved.		
// -----------------------------------------------------------------------	
#include <objbase.h>
#ifndef DECLSPEC_UUID
#if _MSC_VER >= 1100
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_mtxadmin_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mtxadmin_0000_v0_0_s_ifspec;

#ifndef __ICatalog_INTERFACE_DEFINED__
#define __ICatalog_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: ICatalog
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_ICatalog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22870-8a19-11d0-81b6-00a0c9231c29")
    ICatalog : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollection( 
            /* [in] */ BSTR bstrCollName,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ BSTR bstrConnectString,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct ICatalogVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            ICatalog __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            ICatalog __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            ICatalog __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetCollection )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ BSTR bstrCollName,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Connect )( 
            ICatalog __RPC_FAR * This,
            /* [in] */ BSTR bstrConnectString,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_MajorVersion )( 
            ICatalog __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_MinorVersion )( 
            ICatalog __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        END_INTERFACE
    } ICatalogVtbl;

    interface ICatalog
    {
        CONST_VTBL struct ICatalogVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalog_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define ICatalog_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define ICatalog_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define ICatalog_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define ICatalog_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define ICatalog_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define ICatalog_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define ICatalog_GetCollection(This,bstrCollName,ppCatalogCollection)	\
    (This)->lpVtbl -> GetCollection(This,bstrCollName,ppCatalogCollection)

#define ICatalog_Connect(This,bstrConnectString,ppCatalogCollection)	\
    (This)->lpVtbl -> Connect(This,bstrConnectString,ppCatalogCollection)

#define ICatalog_get_MajorVersion(This,retval)	\
    (This)->lpVtbl -> get_MajorVersion(This,retval)

#define ICatalog_get_MinorVersion(This,retval)	\
    (This)->lpVtbl -> get_MinorVersion(This,retval)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalog_GetCollection_Proxy( 
    ICatalog __RPC_FAR * This,
    /* [in] */ BSTR bstrCollName,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);


void __RPC_STUB ICatalog_GetCollection_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalog_Connect_Proxy( 
    ICatalog __RPC_FAR * This,
    /* [in] */ BSTR bstrConnectString,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);


void __RPC_STUB ICatalog_Connect_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalog_get_MajorVersion_Proxy( 
    ICatalog __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalog_get_MajorVersion_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalog_get_MinorVersion_Proxy( 
    ICatalog __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalog_get_MinorVersion_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICatalog_INTERFACE_DEFINED__ */


#ifndef __ICatalogObject_INTERFACE_DEFINED__
#define __ICatalogObject_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: ICatalogObject
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_ICatalogObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22871-8a19-11d0-81b6-00a0c9231c29")
    ICatalogObject : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ BSTR bstrPropName,
            /* [in] */ VARIANT val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [retval][out] */ VARIANT __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ VARIANT __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPropertyReadOnly( 
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Valid( 
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPropertyWriteOnly( 
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct ICatalogObjectVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            ICatalogObject __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            ICatalogObject __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            ICatalogObject __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Value )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT __RPC_FAR *retval);
        
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *put_Value )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ BSTR bstrPropName,
            /* [in] */ VARIANT val);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Key )( 
            ICatalogObject __RPC_FAR * This,
            /* [retval][out] */ VARIANT __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Name )( 
            ICatalogObject __RPC_FAR * This,
            /* [retval][out] */ VARIANT __RPC_FAR *retval);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *IsPropertyReadOnly )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Valid )( 
            ICatalogObject __RPC_FAR * This,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *IsPropertyWriteOnly )( 
            ICatalogObject __RPC_FAR * This,
            /* [in] */ BSTR bstrPropName,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);
        
        END_INTERFACE
    } ICatalogObjectVtbl;

    interface ICatalogObject
    {
        CONST_VTBL struct ICatalogObjectVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalogObject_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define ICatalogObject_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define ICatalogObject_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define ICatalogObject_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define ICatalogObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define ICatalogObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define ICatalogObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define ICatalogObject_get_Value(This,bstrPropName,retval)	\
    (This)->lpVtbl -> get_Value(This,bstrPropName,retval)

#define ICatalogObject_put_Value(This,bstrPropName,val)	\
    (This)->lpVtbl -> put_Value(This,bstrPropName,val)

#define ICatalogObject_get_Key(This,retval)	\
    (This)->lpVtbl -> get_Key(This,retval)

#define ICatalogObject_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define ICatalogObject_IsPropertyReadOnly(This,bstrPropName,retval)	\
    (This)->lpVtbl -> IsPropertyReadOnly(This,bstrPropName,retval)

#define ICatalogObject_get_Valid(This,retval)	\
    (This)->lpVtbl -> get_Valid(This,retval)

#define ICatalogObject_IsPropertyWriteOnly(This,bstrPropName,retval)	\
    (This)->lpVtbl -> IsPropertyWriteOnly(This,bstrPropName,retval)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogObject_get_Value_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [in] */ BSTR bstrPropName,
    /* [retval][out] */ VARIANT __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_get_Value_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE ICatalogObject_put_Value_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [in] */ BSTR bstrPropName,
    /* [in] */ VARIANT val);


void __RPC_STUB ICatalogObject_put_Value_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogObject_get_Key_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [retval][out] */ VARIANT __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_get_Key_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogObject_get_Name_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [retval][out] */ VARIANT __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_get_Name_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogObject_IsPropertyReadOnly_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [in] */ BSTR bstrPropName,
    /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_IsPropertyReadOnly_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogObject_get_Valid_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_get_Valid_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogObject_IsPropertyWriteOnly_Proxy( 
    ICatalogObject __RPC_FAR * This,
    /* [in] */ BSTR bstrPropName,
    /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);


void __RPC_STUB ICatalogObject_IsPropertyWriteOnly_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICatalogObject_INTERFACE_DEFINED__ */


#ifndef __ICatalogCollection_INTERFACE_DEFINED__
#define __ICatalogCollection_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: ICatalogCollection
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_ICatalogCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22872-8a19-11d0-81b6-00a0c9231c29")
    ICatalogCollection : public IDispatch
    {
    public:
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *ppEnumVariant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lIndex) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Populate( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveChanges( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollection( 
            /* [in] */ BSTR bstrCollName,
            /* [in] */ VARIANT varObjectKey,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ VARIANT __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddEnabled( 
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemoveEnabled( 
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetUtilInterface( 
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppUtil) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataStoreMajorVersion( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataStoreMinorVersion( 
            /* [retval][out] */ long __RPC_FAR *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PopulateByKey( 
            /* [in] */ SAFEARRAY __RPC_FAR * aKeys) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PopulateByQuery( 
            /* [in] */ BSTR bstrQueryString,
            /* [in] */ long lQueryType) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct ICatalogCollectionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            ICatalogCollection __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            ICatalogCollection __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            ICatalogCollection __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get__NewEnum )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *ppEnumVariant);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Item )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject);
        
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Count )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Remove )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ long lIndex);
        
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Add )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Populate )( 
            ICatalogCollection __RPC_FAR * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *SaveChanges )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetCollection )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ BSTR bstrCollName,
            /* [in] */ VARIANT varObjectKey,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Name )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ VARIANT __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_AddEnabled )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_RemoveEnabled )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetUtilInterface )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppUtil);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DataStoreMajorVersion )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DataStoreMinorVersion )( 
            ICatalogCollection __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *retval);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *PopulateByKey )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ SAFEARRAY __RPC_FAR * aKeys);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *PopulateByQuery )( 
            ICatalogCollection __RPC_FAR * This,
            /* [in] */ BSTR bstrQueryString,
            /* [in] */ long lQueryType);
        
        END_INTERFACE
    } ICatalogCollectionVtbl;

    interface ICatalogCollection
    {
        CONST_VTBL struct ICatalogCollectionVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalogCollection_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define ICatalogCollection_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define ICatalogCollection_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define ICatalogCollection_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define ICatalogCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define ICatalogCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define ICatalogCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define ICatalogCollection_get__NewEnum(This,ppEnumVariant)	\
    (This)->lpVtbl -> get__NewEnum(This,ppEnumVariant)

#define ICatalogCollection_get_Item(This,lIndex,ppCatalogObject)	\
    (This)->lpVtbl -> get_Item(This,lIndex,ppCatalogObject)

#define ICatalogCollection_get_Count(This,retval)	\
    (This)->lpVtbl -> get_Count(This,retval)

#define ICatalogCollection_Remove(This,lIndex)	\
    (This)->lpVtbl -> Remove(This,lIndex)

#define ICatalogCollection_Add(This,ppCatalogObject)	\
    (This)->lpVtbl -> Add(This,ppCatalogObject)

#define ICatalogCollection_Populate(This)	\
    (This)->lpVtbl -> Populate(This)

#define ICatalogCollection_SaveChanges(This,retval)	\
    (This)->lpVtbl -> SaveChanges(This,retval)

#define ICatalogCollection_GetCollection(This,bstrCollName,varObjectKey,ppCatalogCollection)	\
    (This)->lpVtbl -> GetCollection(This,bstrCollName,varObjectKey,ppCatalogCollection)

#define ICatalogCollection_get_Name(This,retval)	\
    (This)->lpVtbl -> get_Name(This,retval)

#define ICatalogCollection_get_AddEnabled(This,retval)	\
    (This)->lpVtbl -> get_AddEnabled(This,retval)

#define ICatalogCollection_get_RemoveEnabled(This,retval)	\
    (This)->lpVtbl -> get_RemoveEnabled(This,retval)

#define ICatalogCollection_GetUtilInterface(This,ppUtil)	\
    (This)->lpVtbl -> GetUtilInterface(This,ppUtil)

#define ICatalogCollection_get_DataStoreMajorVersion(This,retval)	\
    (This)->lpVtbl -> get_DataStoreMajorVersion(This,retval)

#define ICatalogCollection_get_DataStoreMinorVersion(This,retval)	\
    (This)->lpVtbl -> get_DataStoreMinorVersion(This,retval)

#define ICatalogCollection_PopulateByKey(This,aKeys)	\
    (This)->lpVtbl -> PopulateByKey(This,aKeys)

#define ICatalogCollection_PopulateByQuery(This,bstrQueryString,lQueryType)	\
    (This)->lpVtbl -> PopulateByQuery(This,bstrQueryString,lQueryType)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get__NewEnum_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *ppEnumVariant);


void __RPC_STUB ICatalogCollection_get__NewEnum_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_Item_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [in] */ long lIndex,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject);


void __RPC_STUB ICatalogCollection_get_Item_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_Count_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_Count_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_Remove_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [in] */ long lIndex);


void __RPC_STUB ICatalogCollection_Remove_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_Add_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogObject);


void __RPC_STUB ICatalogCollection_Add_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_Populate_Proxy( 
    ICatalogCollection __RPC_FAR * This);


void __RPC_STUB ICatalogCollection_Populate_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_SaveChanges_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_SaveChanges_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_GetCollection_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [in] */ BSTR bstrCollName,
    /* [in] */ VARIANT varObjectKey,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppCatalogCollection);


void __RPC_STUB ICatalogCollection_GetCollection_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_Name_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ VARIANT __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_Name_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_AddEnabled_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_AddEnabled_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_RemoveEnabled_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ VARIANT_BOOL __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_RemoveEnabled_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_GetUtilInterface_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ IDispatch __RPC_FAR *__RPC_FAR *ppUtil);


void __RPC_STUB ICatalogCollection_GetUtilInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_DataStoreMajorVersion_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_DataStoreMajorVersion_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_get_DataStoreMinorVersion_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *retval);


void __RPC_STUB ICatalogCollection_get_DataStoreMinorVersion_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_PopulateByKey_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [in] */ SAFEARRAY __RPC_FAR * aKeys);


void __RPC_STUB ICatalogCollection_PopulateByKey_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ICatalogCollection_PopulateByQuery_Proxy( 
    ICatalogCollection __RPC_FAR * This,
    /* [in] */ BSTR bstrQueryString,
    /* [in] */ long lQueryType);


void __RPC_STUB ICatalogCollection_PopulateByQuery_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICatalogCollection_INTERFACE_DEFINED__ */


#ifndef __IComponentUtil_INTERFACE_DEFINED__
#define __IComponentUtil_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: IComponentUtil
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_IComponentUtil;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22873-8a19-11d0-81b6-00a0c9231c29")
    IComponentUtil : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallComponent( 
            /* [in] */ BSTR bstrDLLFile,
            /* [in] */ BSTR bstrTypelibFile,
            /* [in] */ BSTR bstrProxyStubDLLFile) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportComponent( 
            /* [in] */ BSTR bstrCLSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportComponentByName( 
            /* [in] */ BSTR bstrProgID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCLSIDs( 
            /* [in] */ BSTR bstrDLLFile,
            /* [in] */ BSTR bstrTypelibFile,
            /* [out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *aCLSIDs) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IComponentUtilVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IComponentUtil __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IComponentUtil __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IComponentUtil __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *InstallComponent )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrDLLFile,
            /* [in] */ BSTR bstrTypelibFile,
            /* [in] */ BSTR bstrProxyStubDLLFile);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ImportComponent )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrCLSID);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ImportComponentByName )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrProgID);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetCLSIDs )( 
            IComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrDLLFile,
            /* [in] */ BSTR bstrTypelibFile,
            /* [out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *aCLSIDs);
        
        END_INTERFACE
    } IComponentUtilVtbl;

    interface IComponentUtil
    {
        CONST_VTBL struct IComponentUtilVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentUtil_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IComponentUtil_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IComponentUtil_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IComponentUtil_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IComponentUtil_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IComponentUtil_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IComponentUtil_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IComponentUtil_InstallComponent(This,bstrDLLFile,bstrTypelibFile,bstrProxyStubDLLFile)	\
    (This)->lpVtbl -> InstallComponent(This,bstrDLLFile,bstrTypelibFile,bstrProxyStubDLLFile)

#define IComponentUtil_ImportComponent(This,bstrCLSID)	\
    (This)->lpVtbl -> ImportComponent(This,bstrCLSID)

#define IComponentUtil_ImportComponentByName(This,bstrProgID)	\
    (This)->lpVtbl -> ImportComponentByName(This,bstrProgID)

#define IComponentUtil_GetCLSIDs(This,bstrDLLFile,bstrTypelibFile,aCLSIDs)	\
    (This)->lpVtbl -> GetCLSIDs(This,bstrDLLFile,bstrTypelibFile,aCLSIDs)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IComponentUtil_InstallComponent_Proxy( 
    IComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrDLLFile,
    /* [in] */ BSTR bstrTypelibFile,
    /* [in] */ BSTR bstrProxyStubDLLFile);


void __RPC_STUB IComponentUtil_InstallComponent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IComponentUtil_ImportComponent_Proxy( 
    IComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrCLSID);


void __RPC_STUB IComponentUtil_ImportComponent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IComponentUtil_ImportComponentByName_Proxy( 
    IComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrProgID);


void __RPC_STUB IComponentUtil_ImportComponentByName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IComponentUtil_GetCLSIDs_Proxy( 
    IComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrDLLFile,
    /* [in] */ BSTR bstrTypelibFile,
    /* [out] */ SAFEARRAY __RPC_FAR * __RPC_FAR *aCLSIDs);


void __RPC_STUB IComponentUtil_GetCLSIDs_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IComponentUtil_INTERFACE_DEFINED__ */


#ifndef __IPackageUtil_INTERFACE_DEFINED__
#define __IPackageUtil_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: IPackageUtil
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_IPackageUtil;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22874-8a19-11d0-81b6-00a0c9231c29")
    IPackageUtil : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallPackage( 
            /* [in] */ BSTR bstrPackageFile,
            /* [in] */ BSTR bstrInstallPath,
            /* [in] */ long lOptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportPackage( 
            /* [in] */ BSTR bstrPackageID,
            /* [in] */ BSTR bstrPackageFile,
            /* [in] */ long lOptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ShutdownPackage( 
            /* [in] */ BSTR bstrPackageID) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IPackageUtilVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IPackageUtil __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IPackageUtil __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IPackageUtil __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *InstallPackage )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrPackageFile,
            /* [in] */ BSTR bstrInstallPath,
            /* [in] */ long lOptions);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExportPackage )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrPackageID,
            /* [in] */ BSTR bstrPackageFile,
            /* [in] */ long lOptions);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ShutdownPackage )( 
            IPackageUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrPackageID);
        
        END_INTERFACE
    } IPackageUtilVtbl;

    interface IPackageUtil
    {
        CONST_VTBL struct IPackageUtilVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPackageUtil_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IPackageUtil_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IPackageUtil_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IPackageUtil_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IPackageUtil_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IPackageUtil_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IPackageUtil_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IPackageUtil_InstallPackage(This,bstrPackageFile,bstrInstallPath,lOptions)	\
    (This)->lpVtbl -> InstallPackage(This,bstrPackageFile,bstrInstallPath,lOptions)

#define IPackageUtil_ExportPackage(This,bstrPackageID,bstrPackageFile,lOptions)	\
    (This)->lpVtbl -> ExportPackage(This,bstrPackageID,bstrPackageFile,lOptions)

#define IPackageUtil_ShutdownPackage(This,bstrPackageID)	\
    (This)->lpVtbl -> ShutdownPackage(This,bstrPackageID)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IPackageUtil_InstallPackage_Proxy( 
    IPackageUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrPackageFile,
    /* [in] */ BSTR bstrInstallPath,
    /* [in] */ long lOptions);


void __RPC_STUB IPackageUtil_InstallPackage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IPackageUtil_ExportPackage_Proxy( 
    IPackageUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrPackageID,
    /* [in] */ BSTR bstrPackageFile,
    /* [in] */ long lOptions);


void __RPC_STUB IPackageUtil_ExportPackage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IPackageUtil_ShutdownPackage_Proxy( 
    IPackageUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrPackageID);


void __RPC_STUB IPackageUtil_ShutdownPackage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IPackageUtil_INTERFACE_DEFINED__ */


#ifndef __IRemoteComponentUtil_INTERFACE_DEFINED__
#define __IRemoteComponentUtil_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: IRemoteComponentUtil
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_IRemoteComponentUtil;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22875-8a19-11d0-81b6-00a0c9231c29")
    IRemoteComponentUtil : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallRemoteComponent( 
            /* [in] */ BSTR bstrServer,
            /* [in] */ BSTR bstrPackageID,
            /* [in] */ BSTR bstrCLSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallRemoteComponentByName( 
            /* [in] */ BSTR bstrServer,
            /* [in] */ BSTR bstrPackageName,
            /* [in] */ BSTR bstrProgID) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IRemoteComponentUtilVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IRemoteComponentUtil __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IRemoteComponentUtil __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *InstallRemoteComponent )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrServer,
            /* [in] */ BSTR bstrPackageID,
            /* [in] */ BSTR bstrCLSID);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *InstallRemoteComponentByName )( 
            IRemoteComponentUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrServer,
            /* [in] */ BSTR bstrPackageName,
            /* [in] */ BSTR bstrProgID);
        
        END_INTERFACE
    } IRemoteComponentUtilVtbl;

    interface IRemoteComponentUtil
    {
        CONST_VTBL struct IRemoteComponentUtilVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteComponentUtil_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IRemoteComponentUtil_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IRemoteComponentUtil_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IRemoteComponentUtil_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IRemoteComponentUtil_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IRemoteComponentUtil_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IRemoteComponentUtil_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IRemoteComponentUtil_InstallRemoteComponent(This,bstrServer,bstrPackageID,bstrCLSID)	\
    (This)->lpVtbl -> InstallRemoteComponent(This,bstrServer,bstrPackageID,bstrCLSID)

#define IRemoteComponentUtil_InstallRemoteComponentByName(This,bstrServer,bstrPackageName,bstrProgID)	\
    (This)->lpVtbl -> InstallRemoteComponentByName(This,bstrServer,bstrPackageName,bstrProgID)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IRemoteComponentUtil_InstallRemoteComponent_Proxy( 
    IRemoteComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrServer,
    /* [in] */ BSTR bstrPackageID,
    /* [in] */ BSTR bstrCLSID);


void __RPC_STUB IRemoteComponentUtil_InstallRemoteComponent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IRemoteComponentUtil_InstallRemoteComponentByName_Proxy( 
    IRemoteComponentUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrServer,
    /* [in] */ BSTR bstrPackageName,
    /* [in] */ BSTR bstrProgID);


void __RPC_STUB IRemoteComponentUtil_InstallRemoteComponentByName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IRemoteComponentUtil_INTERFACE_DEFINED__ */


#ifndef __IRoleAssociationUtil_INTERFACE_DEFINED__
#define __IRoleAssociationUtil_INTERFACE_DEFINED__

/****************************************
 * Generated header for interface: IRoleAssociationUtil
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [unique][helpstring][dual][uuid][object] */ 



EXTERN_C const IID IID_IRoleAssociationUtil;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("6eb22876-8a19-11d0-81b6-00a0c9231c29")
    IRoleAssociationUtil : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssociateRole( 
            /* [in] */ BSTR bstrRoleID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssociateRoleByName( 
            /* [in] */ BSTR bstrRoleName) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IRoleAssociationUtilVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IRoleAssociationUtil __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IRoleAssociationUtil __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *AssociateRole )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrRoleID);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *AssociateRoleByName )( 
            IRoleAssociationUtil __RPC_FAR * This,
            /* [in] */ BSTR bstrRoleName);
        
        END_INTERFACE
    } IRoleAssociationUtilVtbl;

    interface IRoleAssociationUtil
    {
        CONST_VTBL struct IRoleAssociationUtilVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRoleAssociationUtil_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IRoleAssociationUtil_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IRoleAssociationUtil_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IRoleAssociationUtil_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IRoleAssociationUtil_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IRoleAssociationUtil_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IRoleAssociationUtil_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IRoleAssociationUtil_AssociateRole(This,bstrRoleID)	\
    (This)->lpVtbl -> AssociateRole(This,bstrRoleID)

#define IRoleAssociationUtil_AssociateRoleByName(This,bstrRoleName)	\
    (This)->lpVtbl -> AssociateRoleByName(This,bstrRoleName)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IRoleAssociationUtil_AssociateRole_Proxy( 
    IRoleAssociationUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrRoleID);


void __RPC_STUB IRoleAssociationUtil_AssociateRole_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IRoleAssociationUtil_AssociateRoleByName_Proxy( 
    IRoleAssociationUtil __RPC_FAR * This,
    /* [in] */ BSTR bstrRoleName);


void __RPC_STUB IRoleAssociationUtil_AssociateRoleByName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IRoleAssociationUtil_INTERFACE_DEFINED__ */



#ifndef __MTSAdmin_LIBRARY_DEFINED__
#define __MTSAdmin_LIBRARY_DEFINED__

/****************************************
 * Generated header for library: MTSAdmin
 * at Fri Nov 14 05:07:21 1997
 * using MIDL 3.01.75
 ****************************************/
/* [helpstring][version][uuid] */ 


typedef /* [public][helpstring] */ 
enum __MIDL___MIDL_itf_mtxadmin_0107_0001
    {	mtsInstallUsers	= 1
    }	MTSPackageInstallOptions;

typedef /* [public][helpstring] */ 
enum __MIDL___MIDL_itf_mtxadmin_0107_0002
    {	mtsExportUsers	= 1
    }	MTSPackageExportOptions;

typedef /* [public][helpstring] */ 
enum __MIDL___MIDL_itf_mtxadmin_0107_0003
    {	mtsErrObjectErrors	= 0x80110401,
	mtsErrObjectInvalid	= 0x80110402,
	mtsErrKeyMissing	= 0x80110403,
	mtsErrAlreadyInstalled	= 0x80110404,
	mtsErrDownloadFailed	= 0x80110405,
	mtsErrPDFWriteFail	= 0x80110407,
	mtsErrPDFReadFail	= 0x80110408,
	mtsErrPDFVersion	= 0x80110409,
	mtsErrCoReqCompInstalled	= 0x80110410,
	mtsErrBadPath	= 0x8011040a,
	mtsErrPackageExists	= 0x8011040b,
	mtsErrRoleExists	= 0x8011040c,
	mtsErrCantCopyFile	= 0x8011040d,
	mtsErrNoTypeLib	= 0x8011040e,
	mtsErrNoUser	= 0x8011040f,
	mtsErrInvalidUserids	= 0x80110410,
	mtsErrNoRegistryCLSID	= 0x80110411,
	mtsErrBadRegistryProgID	= 0x80110412,
	mtsErrAuthenticationLevel	= 0x80110413,
	mtsErrUserPasswdNotValid	= 0x80110414,
	mtsErrNoRegistryRead	= 0x80110415,
	mtsErrNoRegistryWrite	= 0x80110416,
	mtsErrNoRegistryRepair	= 0x80110417,
	mtsErrCLSIDOrIIDMismatch	= 0x80110418,
	mtsErrRemoteInterface	= 0x80110419,
	mtsErrDllRegisterServer	= 0x8011041a,
	mtsErrNoServerShare	= 0x8011041b,
	mtsErrNoAccessToUNC	= 0x8011041c,
	mtsErrDllLoadFailed	= 0x8011041d,
	mtsErrBadRegistryLibID	= 0x8011041e,
	mtsErrPackDirNotFound	= 0x8011041f,
	mtsErrTreatAs	= 0x80110420,
	mtsErrBadForward	= 0x80110421,
	mtsErrBadIID	= 0x80110422,
	mtsErrRegistrarFailed	= 0x80110423,
	mtsErrCompFileDoesNotExist	= 0x80110424,
	mtsErrCompFileLoadDLLFail	= 0x80110425,
	mtsErrCompFileGetClassObj	= 0x80110426,
	mtsErrCompFileClassNotAvail	= 0x80110427,
	mtsErrCompFileBadTLB	= 0x80110428,
	mtsErrCompFileNotInstallable	= 0x80110429,
	mtsErrNotChangeable	= 0x8011042a,
	mtsErrNotDeletable	= 0x8011042b,
	mtsErrSession	= 0x8011042c,
	mtsErrCompFileNoRegistrar	= 0x80110434
    }	MTSAdminErrorCodes;

#define E_MTS_OBJECTERRORS			 	mtsErrObjectErrors				
#define E_MTS_OBJECTINVALID				mtsErrObjectInvalid				
#define E_MTS_KEYMISSING				mtsErrKeyMissing				
#define E_MTS_ALREADYINSTALLED			mtsErrAlreadyInstalled			
#define E_MTS_DOWNLOADFAILED			mtsErrDownloadFailed			
#define E_MTS_PDFWRITEFAIL				mtsErrPDFWriteFail				
#define E_MTS_PDFREADFAIL				mtsErrPDFReadFail				
#define E_MTS_PDFVERSION				mtsErrPDFVersion				
#define E_MTS_COREQCOMPINSTALLED		mtsErrCoReqCompInstalled		
#define E_MTS_BADPATH					mtsErrBadPath					
#define E_MTS_PACKAGEEXISTS				mtsErrPackageExists				
#define E_MTS_ROLEEXISTS				mtsErrRoleExists				
#define E_MTS_CANTCOPYFILE				mtsErrCantCopyFile				
#define E_MTS_NOTYPELIB					mtsErrNoTypeLib					
#define E_MTS_NOUSER					mtsErrNoUser					
#define E_MTS_INVALIDUSERIDS			mtsErrInvalidUserids			
#define E_MTS_NOREGISTRYCLSID			mtsErrNoRegistryCLSID			
#define E_MTS_BADREGISTRYPROGID			mtsErrBadRegistryProgID			
#define E_MTS_AUTHENTICATIONLEVEL		mtsErrAuthenticationLevel		
#define E_MTS_USERPASSWDNOTVALID		mtsErrUserPasswdNotValid		
#define E_MTS_NOREGISTRYREAD			mtsErrNoRegistryRead			
#define E_MTS_NOREGISTRYWRITE			mtsErrNoRegistryWrite			
#define E_MTS_NOREGISTRYREPAIR			mtsErrNoRegistryRepair			
#define E_MTS_CLSIDORIIDMISMATCH		mtsErrCLSIDOrIIDMismatch		
#define E_MTS_REMOTEINTERFACE			mtsErrRemoteInterface			
#define E_MTS_DLLREGISTERSERVER			mtsErrDllRegisterServer			
#define E_MTS_NOSERVERSHARE				mtsErrNoServerShare				
#define E_MTS_NOACCESSTOUNC				mtsErrNoAccessToUNC				
#define E_MTS_DLLLOADFAILED				mtsErrDllLoadFailed				
#define E_MTS_BADREGISTRYLIBID			mtsErrBadRegistryLibID			
#define E_MTS_PACKDIRNOTFOUND			mtsErrPackDirNotFound			
#define E_MTS_TREATAS					mtsErrTreatAs					
#define E_MTS_BADFORWARD				mtsErrBadForward				
#define E_MTS_BADIID					mtsErrBadIID					
#define E_MTS_REGISTRARFAILED			mtsErrRegistrarFailed			
#define E_MTS_COMPFILE_DOESNOTEXIST		mtsErrCompFileDoesNotExist		
#define E_MTS_COMPFILE_LOADDLLFAIL		mtsErrCompFileLoadDLLFail		
#define E_MTS_COMPFILE_GETCLASSOBJ		mtsErrCompFileGetClassObj		
#define E_MTS_COMPFILE_CLASSNOTAVAIL	mtsErrCompFileClassNotAvail		
#define E_MTS_COMPFILE_BADTLB			mtsErrCompFileBadTLB			
#define E_MTS_COMPFILE_NOTINSTALLABLE	mtsErrCompFileNotInstallable	
#define E_MTS_NOTCHANGEABLE				mtsErrNotChangeable				
#define E_MTS_NOTDELETEABLE				mtsErrNotDeleteable				
#define E_MTS_SESSION					mtsErrSession					
#define E_MTS_COMPFILE_NOREGISTRAR		mtsErrCompFileNoRegistrar		

EXTERN_C const IID LIBID_MTSAdmin;

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_Catalog;

class DECLSPEC_UUID("6eb22881-8a19-11d0-81b6-00a0c9231c29")
Catalog;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_CatalogObject;

class DECLSPEC_UUID("6eb22882-8a19-11d0-81b6-00a0c9231c29")
CatalogObject;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_CatalogCollection;

class DECLSPEC_UUID("6eb22883-8a19-11d0-81b6-00a0c9231c29")
CatalogCollection;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_ComponentUtil;

class DECLSPEC_UUID("6eb22884-8a19-11d0-81b6-00a0c9231c29")
ComponentUtil;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_PackageUtil;

class DECLSPEC_UUID("6eb22885-8a19-11d0-81b6-00a0c9231c29")
PackageUtil;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_RemoteComponentUtil;

class DECLSPEC_UUID("6eb22886-8a19-11d0-81b6-00a0c9231c29")
RemoteComponentUtil;
#endif

#ifdef __cplusplus
EXTERN_C const CLSID CLSID_RoleAssociationUtil;

class DECLSPEC_UUID("6eb22887-8a19-11d0-81b6-00a0c9231c29")
RoleAssociationUtil;
#endif
#endif /* __MTSAdmin_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long __RPC_FAR *, unsigned long            , BSTR __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  BSTR_UserMarshal(  unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, BSTR __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  BSTR_UserUnmarshal(unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, BSTR __RPC_FAR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long __RPC_FAR *, BSTR __RPC_FAR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     unsigned long __RPC_FAR *, unsigned long            , LPSAFEARRAY __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  LPSAFEARRAY_UserMarshal(  unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, LPSAFEARRAY __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  LPSAFEARRAY_UserUnmarshal(unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, LPSAFEARRAY __RPC_FAR * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     unsigned long __RPC_FAR *, LPSAFEARRAY __RPC_FAR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     unsigned long __RPC_FAR *, unsigned long            , VARIANT __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  VARIANT_UserMarshal(  unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, VARIANT __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  VARIANT_UserUnmarshal(unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, VARIANT __RPC_FAR * ); 
void                      __RPC_USER  VARIANT_UserFree(     unsigned long __RPC_FAR *, VARIANT __RPC_FAR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif

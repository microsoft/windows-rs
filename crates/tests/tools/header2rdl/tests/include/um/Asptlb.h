

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


#ifndef __asptlb_h__
#define __asptlb_h__

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

#ifndef __IStringList_FWD_DEFINED__
#define __IStringList_FWD_DEFINED__
typedef interface IStringList IStringList;

#endif 	/* __IStringList_FWD_DEFINED__ */


#ifndef __IRequestDictionary_FWD_DEFINED__
#define __IRequestDictionary_FWD_DEFINED__
typedef interface IRequestDictionary IRequestDictionary;

#endif 	/* __IRequestDictionary_FWD_DEFINED__ */


#ifndef __IRequest_FWD_DEFINED__
#define __IRequest_FWD_DEFINED__
typedef interface IRequest IRequest;

#endif 	/* __IRequest_FWD_DEFINED__ */


#ifndef __Request_FWD_DEFINED__
#define __Request_FWD_DEFINED__

#ifdef __cplusplus
typedef class Request Request;
#else
typedef struct Request Request;
#endif /* __cplusplus */

#endif 	/* __Request_FWD_DEFINED__ */


#ifndef __IReadCookie_FWD_DEFINED__
#define __IReadCookie_FWD_DEFINED__
typedef interface IReadCookie IReadCookie;

#endif 	/* __IReadCookie_FWD_DEFINED__ */


#ifndef __IWriteCookie_FWD_DEFINED__
#define __IWriteCookie_FWD_DEFINED__
typedef interface IWriteCookie IWriteCookie;

#endif 	/* __IWriteCookie_FWD_DEFINED__ */


#ifndef __IResponse_FWD_DEFINED__
#define __IResponse_FWD_DEFINED__
typedef interface IResponse IResponse;

#endif 	/* __IResponse_FWD_DEFINED__ */


#ifndef __Response_FWD_DEFINED__
#define __Response_FWD_DEFINED__

#ifdef __cplusplus
typedef class Response Response;
#else
typedef struct Response Response;
#endif /* __cplusplus */

#endif 	/* __Response_FWD_DEFINED__ */


#ifndef __IVariantDictionary_FWD_DEFINED__
#define __IVariantDictionary_FWD_DEFINED__
typedef interface IVariantDictionary IVariantDictionary;

#endif 	/* __IVariantDictionary_FWD_DEFINED__ */


#ifndef __ISessionObject_FWD_DEFINED__
#define __ISessionObject_FWD_DEFINED__
typedef interface ISessionObject ISessionObject;

#endif 	/* __ISessionObject_FWD_DEFINED__ */


#ifndef __Session_FWD_DEFINED__
#define __Session_FWD_DEFINED__

#ifdef __cplusplus
typedef class Session Session;
#else
typedef struct Session Session;
#endif /* __cplusplus */

#endif 	/* __Session_FWD_DEFINED__ */


#ifndef __IApplicationObject_FWD_DEFINED__
#define __IApplicationObject_FWD_DEFINED__
typedef interface IApplicationObject IApplicationObject;

#endif 	/* __IApplicationObject_FWD_DEFINED__ */


#ifndef __Application_FWD_DEFINED__
#define __Application_FWD_DEFINED__

#ifdef __cplusplus
typedef class Application Application;
#else
typedef struct Application Application;
#endif /* __cplusplus */

#endif 	/* __Application_FWD_DEFINED__ */


#ifndef __IASPError_FWD_DEFINED__
#define __IASPError_FWD_DEFINED__
typedef interface IASPError IASPError;

#endif 	/* __IASPError_FWD_DEFINED__ */


#ifndef __IServer_FWD_DEFINED__
#define __IServer_FWD_DEFINED__
typedef interface IServer IServer;

#endif 	/* __IServer_FWD_DEFINED__ */


#ifndef __Server_FWD_DEFINED__
#define __Server_FWD_DEFINED__

#ifdef __cplusplus
typedef class Server Server;
#else
typedef struct Server Server;
#endif /* __cplusplus */

#endif 	/* __Server_FWD_DEFINED__ */


#ifndef __IScriptingContext_FWD_DEFINED__
#define __IScriptingContext_FWD_DEFINED__
typedef interface IScriptingContext IScriptingContext;

#endif 	/* __IScriptingContext_FWD_DEFINED__ */


#ifndef __ScriptingContext_FWD_DEFINED__
#define __ScriptingContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class ScriptingContext ScriptingContext;
#else
typedef struct ScriptingContext ScriptingContext;
#endif /* __cplusplus */

#endif 	/* __ScriptingContext_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __ASPTypeLibrary_LIBRARY_DEFINED__
#define __ASPTypeLibrary_LIBRARY_DEFINED__

/* library ASPTypeLibrary */
/* [version][lcid][helpstring][uuid] */ 


DEFINE_GUID(LIBID_ASPTypeLibrary,0xD97A6DA0,0xA85C,0x11cf,0x83,0xAE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#ifndef __IStringList_INTERFACE_DEFINED__
#define __IStringList_INTERFACE_DEFINED__

/* interface IStringList */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IStringList,0xD97A6DA0,0xA85D,0x11cf,0x83,0xAE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A85D-11cf-83AE-00A0C90C2BD8")
    IStringList : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in][optional] */ VARIANT i,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *cStrRet) = 0;
        
        virtual /* [restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStringListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStringList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStringList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStringList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IStringList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IStringList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IStringList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IStringList * This,
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
        
        DECLSPEC_XFGVIRT(IStringList, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IStringList * This,
            /* [in][optional] */ VARIANT i,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn);
        
        DECLSPEC_XFGVIRT(IStringList, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IStringList * This,
            /* [retval][out] */ __RPC__out int *cStrRet);
        
        DECLSPEC_XFGVIRT(IStringList, get__NewEnum)
        /* [restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IStringList * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn);
        
        END_INTERFACE
    } IStringListVtbl;

    interface IStringList
    {
        CONST_VTBL struct IStringListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStringList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStringList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStringList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStringList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IStringList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IStringList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IStringList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IStringList_get_Item(This,i,pVariantReturn)	\
    ( (This)->lpVtbl -> get_Item(This,i,pVariantReturn) ) 

#define IStringList_get_Count(This,cStrRet)	\
    ( (This)->lpVtbl -> get_Count(This,cStrRet) ) 

#define IStringList_get__NewEnum(This,ppEnumReturn)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumReturn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStringList_INTERFACE_DEFINED__ */


#ifndef __IRequestDictionary_INTERFACE_DEFINED__
#define __IRequestDictionary_INTERFACE_DEFINED__

/* interface IRequestDictionary */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IRequestDictionary,0xD97A6DA0,0xA85F,0x11df,0x83,0xAE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A85F-11df-83AE-00A0C90C2BD8")
    IRequestDictionary : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in][optional] */ VARIANT Var,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn) = 0;
        
        virtual /* [restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *cStrRet) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRequestDictionaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRequestDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRequestDictionary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRequestDictionary * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRequestDictionary * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRequestDictionary * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRequestDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRequestDictionary * This,
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
        
        DECLSPEC_XFGVIRT(IRequestDictionary, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IRequestDictionary * This,
            /* [in][optional] */ VARIANT Var,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn);
        
        DECLSPEC_XFGVIRT(IRequestDictionary, get__NewEnum)
        /* [restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IRequestDictionary * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn);
        
        DECLSPEC_XFGVIRT(IRequestDictionary, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IRequestDictionary * This,
            /* [retval][out] */ __RPC__out int *cStrRet);
        
        DECLSPEC_XFGVIRT(IRequestDictionary, get_Key)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IRequestDictionary * This,
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        END_INTERFACE
    } IRequestDictionaryVtbl;

    interface IRequestDictionary
    {
        CONST_VTBL struct IRequestDictionaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRequestDictionary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRequestDictionary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRequestDictionary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRequestDictionary_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRequestDictionary_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRequestDictionary_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRequestDictionary_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRequestDictionary_get_Item(This,Var,pVariantReturn)	\
    ( (This)->lpVtbl -> get_Item(This,Var,pVariantReturn) ) 

#define IRequestDictionary_get__NewEnum(This,ppEnumReturn)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumReturn) ) 

#define IRequestDictionary_get_Count(This,cStrRet)	\
    ( (This)->lpVtbl -> get_Count(This,cStrRet) ) 

#define IRequestDictionary_get_Key(This,VarKey,pvar)	\
    ( (This)->lpVtbl -> get_Key(This,VarKey,pvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRequestDictionary_INTERFACE_DEFINED__ */


#ifndef __IRequest_INTERFACE_DEFINED__
#define __IRequest_INTERFACE_DEFINED__

/* interface IRequest */
/* [object][hidden][dual][oleautomation][uuid] */ 


DEFINE_GUID(IID_IRequest,0xD97A6DA0,0xA861,0x11cf,0x93,0xAE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A861-11cf-93AE-00A0C90C2BD8")
    IRequest : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR bstrVar,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObjReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_QueryString( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Form( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ServerVariables( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientCertificate( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Cookies( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_TotalBytes( 
            /* [retval][out] */ __RPC__out long *pcbTotal) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BinaryRead( 
            /* [out][in] */ __RPC__inout VARIANT *pvarCountToRead,
            /* [retval][out] */ __RPC__out VARIANT *pvarReturn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRequest * This,
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
        
        DECLSPEC_XFGVIRT(IRequest, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IRequest * This,
            /* [in] */ __RPC__in BSTR bstrVar,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObjReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_QueryString)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QueryString )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_Form)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Form )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_Body)
        /* [hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_ServerVariables)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerVariables )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_ClientCertificate)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientCertificate )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_Cookies)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookies )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppDictReturn);
        
        DECLSPEC_XFGVIRT(IRequest, get_TotalBytes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalBytes )( 
            __RPC__in IRequest * This,
            /* [retval][out] */ __RPC__out long *pcbTotal);
        
        DECLSPEC_XFGVIRT(IRequest, BinaryRead)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BinaryRead )( 
            __RPC__in IRequest * This,
            /* [out][in] */ __RPC__inout VARIANT *pvarCountToRead,
            /* [retval][out] */ __RPC__out VARIANT *pvarReturn);
        
        END_INTERFACE
    } IRequestVtbl;

    interface IRequest
    {
        CONST_VTBL struct IRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRequest_get_Item(This,bstrVar,ppObjReturn)	\
    ( (This)->lpVtbl -> get_Item(This,bstrVar,ppObjReturn) ) 

#define IRequest_get_QueryString(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_QueryString(This,ppDictReturn) ) 

#define IRequest_get_Form(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_Form(This,ppDictReturn) ) 

#define IRequest_get_Body(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_Body(This,ppDictReturn) ) 

#define IRequest_get_ServerVariables(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_ServerVariables(This,ppDictReturn) ) 

#define IRequest_get_ClientCertificate(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_ClientCertificate(This,ppDictReturn) ) 

#define IRequest_get_Cookies(This,ppDictReturn)	\
    ( (This)->lpVtbl -> get_Cookies(This,ppDictReturn) ) 

#define IRequest_get_TotalBytes(This,pcbTotal)	\
    ( (This)->lpVtbl -> get_TotalBytes(This,pcbTotal) ) 

#define IRequest_BinaryRead(This,pvarCountToRead,pvarReturn)	\
    ( (This)->lpVtbl -> BinaryRead(This,pvarCountToRead,pvarReturn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRequest_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_Request,0x920c25d0,0x25d9,0x11d0,0xa5,0x5f,0x00,0xa0,0xc9,0x0c,0x20,0x91);

#ifdef __cplusplus

class DECLSPEC_UUID("920c25d0-25d9-11d0-a55f-00a0c90c2091")
Request;
#endif

#ifndef __IReadCookie_INTERFACE_DEFINED__
#define __IReadCookie_INTERFACE_DEFINED__

/* interface IReadCookie */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IReadCookie,0x71EAF260,0x0CE0,0x11D0,0xA5,0x3E,0x00,0xA0,0xC9,0x0C,0x20,0x91);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71EAF260-0CE0-11D0-A53E-00A0C90C2091")
    IReadCookie : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in][optional] */ VARIANT Var,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_HasKeys( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfHasKeys) = 0;
        
        virtual /* [restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *cStrRet) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReadCookieVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IReadCookie * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IReadCookie * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IReadCookie * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IReadCookie * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IReadCookie * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IReadCookie * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IReadCookie * This,
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
        
        DECLSPEC_XFGVIRT(IReadCookie, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IReadCookie * This,
            /* [in][optional] */ VARIANT Var,
            /* [retval][out] */ __RPC__out VARIANT *pVariantReturn);
        
        DECLSPEC_XFGVIRT(IReadCookie, get_HasKeys)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasKeys )( 
            __RPC__in IReadCookie * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfHasKeys);
        
        DECLSPEC_XFGVIRT(IReadCookie, get__NewEnum)
        /* [restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IReadCookie * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn);
        
        DECLSPEC_XFGVIRT(IReadCookie, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IReadCookie * This,
            /* [retval][out] */ __RPC__out int *cStrRet);
        
        DECLSPEC_XFGVIRT(IReadCookie, get_Key)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IReadCookie * This,
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        END_INTERFACE
    } IReadCookieVtbl;

    interface IReadCookie
    {
        CONST_VTBL struct IReadCookieVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReadCookie_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReadCookie_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReadCookie_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReadCookie_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IReadCookie_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IReadCookie_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IReadCookie_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IReadCookie_get_Item(This,Var,pVariantReturn)	\
    ( (This)->lpVtbl -> get_Item(This,Var,pVariantReturn) ) 

#define IReadCookie_get_HasKeys(This,pfHasKeys)	\
    ( (This)->lpVtbl -> get_HasKeys(This,pfHasKeys) ) 

#define IReadCookie_get__NewEnum(This,ppEnumReturn)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumReturn) ) 

#define IReadCookie_get_Count(This,cStrRet)	\
    ( (This)->lpVtbl -> get_Count(This,cStrRet) ) 

#define IReadCookie_get_Key(This,VarKey,pvar)	\
    ( (This)->lpVtbl -> get_Key(This,VarKey,pvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReadCookie_INTERFACE_DEFINED__ */


#ifndef __IWriteCookie_INTERFACE_DEFINED__
#define __IWriteCookie_INTERFACE_DEFINED__

/* interface IWriteCookie */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IWriteCookie,0xD97A6DA0,0xA862,0x11cf,0x84,0xAE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A862-11cf-84AE-00A0C90C2BD8")
    IWriteCookie : public IDispatch
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Item( 
            /* [in][optional] */ VARIANT key,
            /* [in] */ __RPC__in BSTR bstrValue) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Expires( 
            /* [in] */ DATE dtExpires) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Domain( 
            /* [in] */ __RPC__in BSTR bstrDomain) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ __RPC__in BSTR bstrPath) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Secure( 
            /* [in] */ VARIANT_BOOL fSecure) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_HasKeys( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfHasKeys) = 0;
        
        virtual /* [restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWriteCookieVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWriteCookie * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWriteCookie * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWriteCookie * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWriteCookie * This,
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
        
        DECLSPEC_XFGVIRT(IWriteCookie, put_Item)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Item )( 
            __RPC__in IWriteCookie * This,
            /* [in][optional] */ VARIANT key,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IWriteCookie, put_Expires)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Expires )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ DATE dtExpires);
        
        DECLSPEC_XFGVIRT(IWriteCookie, put_Domain)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Domain )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ __RPC__in BSTR bstrDomain);
        
        DECLSPEC_XFGVIRT(IWriteCookie, put_Path)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ __RPC__in BSTR bstrPath);
        
        DECLSPEC_XFGVIRT(IWriteCookie, put_Secure)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Secure )( 
            __RPC__in IWriteCookie * This,
            /* [in] */ VARIANT_BOOL fSecure);
        
        DECLSPEC_XFGVIRT(IWriteCookie, get_HasKeys)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasKeys )( 
            __RPC__in IWriteCookie * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfHasKeys);
        
        DECLSPEC_XFGVIRT(IWriteCookie, get__NewEnum)
        /* [restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IWriteCookie * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn);
        
        END_INTERFACE
    } IWriteCookieVtbl;

    interface IWriteCookie
    {
        CONST_VTBL struct IWriteCookieVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWriteCookie_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWriteCookie_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWriteCookie_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWriteCookie_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWriteCookie_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWriteCookie_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWriteCookie_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWriteCookie_put_Item(This,key,bstrValue)	\
    ( (This)->lpVtbl -> put_Item(This,key,bstrValue) ) 

#define IWriteCookie_put_Expires(This,dtExpires)	\
    ( (This)->lpVtbl -> put_Expires(This,dtExpires) ) 

#define IWriteCookie_put_Domain(This,bstrDomain)	\
    ( (This)->lpVtbl -> put_Domain(This,bstrDomain) ) 

#define IWriteCookie_put_Path(This,bstrPath)	\
    ( (This)->lpVtbl -> put_Path(This,bstrPath) ) 

#define IWriteCookie_put_Secure(This,fSecure)	\
    ( (This)->lpVtbl -> put_Secure(This,fSecure) ) 

#define IWriteCookie_get_HasKeys(This,pfHasKeys)	\
    ( (This)->lpVtbl -> get_HasKeys(This,pfHasKeys) ) 

#define IWriteCookie_get__NewEnum(This,ppEnumReturn)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumReturn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWriteCookie_INTERFACE_DEFINED__ */


#ifndef __IResponse_INTERFACE_DEFINED__
#define __IResponse_INTERFACE_DEFINED__

/* interface IResponse */
/* [object][hidden][dual][oleautomation][uuid] */ 


DEFINE_GUID(IID_IResponse,0xD97A6DA0,0xA864,0x11cf,0x83,0xBE,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A864-11cf-83BE-00A0C90C2BD8")
    IResponse : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Buffer( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *fIsBuffering) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Buffer( 
            /* [in] */ VARIANT_BOOL fIsBuffering) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ContentType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContentTypeRet) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ContentType( 
            /* [in] */ __RPC__in BSTR bstrContentType) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Expires( 
            /* [retval][out] */ __RPC__out VARIANT *pvarExpiresMinutesRet) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Expires( 
            /* [in] */ long lExpiresMinutes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ExpiresAbsolute( 
            /* [retval][out] */ __RPC__out VARIANT *pvarExpiresRet) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExpiresAbsolute( 
            /* [in] */ DATE dtExpires) = 0;
        
        virtual /* [propget][helpstring] */ HRESULT STDMETHODCALLTYPE get_Cookies( 
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppCookies) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusRet) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Status( 
            /* [in] */ __RPC__in BSTR bstrStatus) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrHeaderValue,
            /* [in] */ __RPC__in BSTR bstrHeaderName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddHeader( 
            /* [in] */ __RPC__in BSTR bstrHeaderName,
            /* [in] */ __RPC__in BSTR bstrHeaderValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppendToLog( 
            /* [in] */ __RPC__in BSTR bstrLogEntry) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BinaryWrite( 
            /* [in] */ VARIANT varInput) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE End( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Redirect( 
            /* [in] */ __RPC__in BSTR bstrURL) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ VARIANT varText) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE WriteBlock( 
            /* [in] */ short iBlockNumber) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsClientConnected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsClientConnected) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CharSet( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCharSetRet) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CharSet( 
            /* [in] */ __RPC__in BSTR bstrCharSet) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Pics( 
            /* [in] */ __RPC__in BSTR bstrHeaderValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CacheControl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCacheControl) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CacheControl( 
            /* [in] */ __RPC__in BSTR bstrCacheControl) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CodePage( 
            /* [retval][out] */ __RPC__out long *plvar) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CodePage( 
            /* [in] */ long lvar) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_LCID( 
            /* [retval][out] */ __RPC__out long *plvar) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LCID( 
            /* [in] */ long lvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResponseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResponse * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResponse * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IResponse * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IResponse * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IResponse * This,
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
        
        DECLSPEC_XFGVIRT(IResponse, get_Buffer)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buffer )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *fIsBuffering);
        
        DECLSPEC_XFGVIRT(IResponse, put_Buffer)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Buffer )( 
            __RPC__in IResponse * This,
            /* [in] */ VARIANT_BOOL fIsBuffering);
        
        DECLSPEC_XFGVIRT(IResponse, get_ContentType)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentType )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContentTypeRet);
        
        DECLSPEC_XFGVIRT(IResponse, put_ContentType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContentType )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrContentType);
        
        DECLSPEC_XFGVIRT(IResponse, get_Expires)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Expires )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarExpiresMinutesRet);
        
        DECLSPEC_XFGVIRT(IResponse, put_Expires)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Expires )( 
            __RPC__in IResponse * This,
            /* [in] */ long lExpiresMinutes);
        
        DECLSPEC_XFGVIRT(IResponse, get_ExpiresAbsolute)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpiresAbsolute )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarExpiresRet);
        
        DECLSPEC_XFGVIRT(IResponse, put_ExpiresAbsolute)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExpiresAbsolute )( 
            __RPC__in IResponse * This,
            /* [in] */ DATE dtExpires);
        
        DECLSPEC_XFGVIRT(IResponse, get_Cookies)
        /* [propget][helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_Cookies )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequestDictionary **ppCookies);
        
        DECLSPEC_XFGVIRT(IResponse, get_Status)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusRet);
        
        DECLSPEC_XFGVIRT(IResponse, put_Status)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Status )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrStatus);
        
        DECLSPEC_XFGVIRT(IResponse, Add)
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrHeaderValue,
            /* [in] */ __RPC__in BSTR bstrHeaderName);
        
        DECLSPEC_XFGVIRT(IResponse, AddHeader)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddHeader )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrHeaderName,
            /* [in] */ __RPC__in BSTR bstrHeaderValue);
        
        DECLSPEC_XFGVIRT(IResponse, AppendToLog)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppendToLog )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrLogEntry);
        
        DECLSPEC_XFGVIRT(IResponse, BinaryWrite)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BinaryWrite )( 
            __RPC__in IResponse * This,
            /* [in] */ VARIANT varInput);
        
        DECLSPEC_XFGVIRT(IResponse, Clear)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IResponse * This);
        
        DECLSPEC_XFGVIRT(IResponse, End)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IResponse * This);
        
        DECLSPEC_XFGVIRT(IResponse, Flush)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in IResponse * This);
        
        DECLSPEC_XFGVIRT(IResponse, Redirect)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Redirect )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrURL);
        
        DECLSPEC_XFGVIRT(IResponse, Write)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IResponse * This,
            /* [in] */ VARIANT varText);
        
        DECLSPEC_XFGVIRT(IResponse, WriteBlock)
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *WriteBlock )( 
            __RPC__in IResponse * This,
            /* [in] */ short iBlockNumber);
        
        DECLSPEC_XFGVIRT(IResponse, IsClientConnected)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsClientConnected )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsClientConnected);
        
        DECLSPEC_XFGVIRT(IResponse, get_CharSet)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CharSet )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCharSetRet);
        
        DECLSPEC_XFGVIRT(IResponse, put_CharSet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CharSet )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrCharSet);
        
        DECLSPEC_XFGVIRT(IResponse, Pics)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Pics )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrHeaderValue);
        
        DECLSPEC_XFGVIRT(IResponse, get_CacheControl)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CacheControl )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCacheControl);
        
        DECLSPEC_XFGVIRT(IResponse, put_CacheControl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CacheControl )( 
            __RPC__in IResponse * This,
            /* [in] */ __RPC__in BSTR bstrCacheControl);
        
        DECLSPEC_XFGVIRT(IResponse, get_CodePage)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CodePage )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out long *plvar);
        
        DECLSPEC_XFGVIRT(IResponse, put_CodePage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CodePage )( 
            __RPC__in IResponse * This,
            /* [in] */ long lvar);
        
        DECLSPEC_XFGVIRT(IResponse, get_LCID)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LCID )( 
            __RPC__in IResponse * This,
            /* [retval][out] */ __RPC__out long *plvar);
        
        DECLSPEC_XFGVIRT(IResponse, put_LCID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LCID )( 
            __RPC__in IResponse * This,
            /* [in] */ long lvar);
        
        END_INTERFACE
    } IResponseVtbl;

    interface IResponse
    {
        CONST_VTBL struct IResponseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResponse_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResponse_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResponse_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResponse_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IResponse_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IResponse_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IResponse_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IResponse_get_Buffer(This,fIsBuffering)	\
    ( (This)->lpVtbl -> get_Buffer(This,fIsBuffering) ) 

#define IResponse_put_Buffer(This,fIsBuffering)	\
    ( (This)->lpVtbl -> put_Buffer(This,fIsBuffering) ) 

#define IResponse_get_ContentType(This,pbstrContentTypeRet)	\
    ( (This)->lpVtbl -> get_ContentType(This,pbstrContentTypeRet) ) 

#define IResponse_put_ContentType(This,bstrContentType)	\
    ( (This)->lpVtbl -> put_ContentType(This,bstrContentType) ) 

#define IResponse_get_Expires(This,pvarExpiresMinutesRet)	\
    ( (This)->lpVtbl -> get_Expires(This,pvarExpiresMinutesRet) ) 

#define IResponse_put_Expires(This,lExpiresMinutes)	\
    ( (This)->lpVtbl -> put_Expires(This,lExpiresMinutes) ) 

#define IResponse_get_ExpiresAbsolute(This,pvarExpiresRet)	\
    ( (This)->lpVtbl -> get_ExpiresAbsolute(This,pvarExpiresRet) ) 

#define IResponse_put_ExpiresAbsolute(This,dtExpires)	\
    ( (This)->lpVtbl -> put_ExpiresAbsolute(This,dtExpires) ) 

#define IResponse_get_Cookies(This,ppCookies)	\
    ( (This)->lpVtbl -> get_Cookies(This,ppCookies) ) 

#define IResponse_get_Status(This,pbstrStatusRet)	\
    ( (This)->lpVtbl -> get_Status(This,pbstrStatusRet) ) 

#define IResponse_put_Status(This,bstrStatus)	\
    ( (This)->lpVtbl -> put_Status(This,bstrStatus) ) 

#define IResponse_Add(This,bstrHeaderValue,bstrHeaderName)	\
    ( (This)->lpVtbl -> Add(This,bstrHeaderValue,bstrHeaderName) ) 

#define IResponse_AddHeader(This,bstrHeaderName,bstrHeaderValue)	\
    ( (This)->lpVtbl -> AddHeader(This,bstrHeaderName,bstrHeaderValue) ) 

#define IResponse_AppendToLog(This,bstrLogEntry)	\
    ( (This)->lpVtbl -> AppendToLog(This,bstrLogEntry) ) 

#define IResponse_BinaryWrite(This,varInput)	\
    ( (This)->lpVtbl -> BinaryWrite(This,varInput) ) 

#define IResponse_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IResponse_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define IResponse_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IResponse_Redirect(This,bstrURL)	\
    ( (This)->lpVtbl -> Redirect(This,bstrURL) ) 

#define IResponse_Write(This,varText)	\
    ( (This)->lpVtbl -> Write(This,varText) ) 

#define IResponse_WriteBlock(This,iBlockNumber)	\
    ( (This)->lpVtbl -> WriteBlock(This,iBlockNumber) ) 

#define IResponse_IsClientConnected(This,pfIsClientConnected)	\
    ( (This)->lpVtbl -> IsClientConnected(This,pfIsClientConnected) ) 

#define IResponse_get_CharSet(This,pbstrCharSetRet)	\
    ( (This)->lpVtbl -> get_CharSet(This,pbstrCharSetRet) ) 

#define IResponse_put_CharSet(This,bstrCharSet)	\
    ( (This)->lpVtbl -> put_CharSet(This,bstrCharSet) ) 

#define IResponse_Pics(This,bstrHeaderValue)	\
    ( (This)->lpVtbl -> Pics(This,bstrHeaderValue) ) 

#define IResponse_get_CacheControl(This,pbstrCacheControl)	\
    ( (This)->lpVtbl -> get_CacheControl(This,pbstrCacheControl) ) 

#define IResponse_put_CacheControl(This,bstrCacheControl)	\
    ( (This)->lpVtbl -> put_CacheControl(This,bstrCacheControl) ) 

#define IResponse_get_CodePage(This,plvar)	\
    ( (This)->lpVtbl -> get_CodePage(This,plvar) ) 

#define IResponse_put_CodePage(This,lvar)	\
    ( (This)->lpVtbl -> put_CodePage(This,lvar) ) 

#define IResponse_get_LCID(This,plvar)	\
    ( (This)->lpVtbl -> get_LCID(This,plvar) ) 

#define IResponse_put_LCID(This,lvar)	\
    ( (This)->lpVtbl -> put_LCID(This,lvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResponse_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_Response,0x46E19BA0,0x25DD,0x11D0,0xA5,0x5F,0x00,0xA0,0xC9,0x0C,0x20,0x91);

#ifdef __cplusplus

class DECLSPEC_UUID("46E19BA0-25DD-11D0-A55F-00A0C90C2091")
Response;
#endif

#ifndef __IVariantDictionary_INTERFACE_DEFINED__
#define __IVariantDictionary_INTERFACE_DEFINED__

/* interface IVariantDictionary */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IVariantDictionary,0x4a7deb90,0xb069,0x11d0,0xb3,0x73,0x00,0xa0,0xc9,0x0c,0x2b,0xd8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4a7deb90-b069-11d0-b373-00a0c90c2bd8")
    IVariantDictionary : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Item( 
            /* [in] */ VARIANT VarKey,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Item( 
            /* [in] */ VARIANT VarKey,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *cStrRet) = 0;
        
        virtual /* [restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT VarKey) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVariantDictionaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVariantDictionary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVariantDictionary * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IVariantDictionary * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVariantDictionary * This,
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
        
        DECLSPEC_XFGVIRT(IVariantDictionary, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, put_Item)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Item )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ VARIANT VarKey,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, putref_Item)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Item )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ VARIANT VarKey,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, get_Key)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ VARIANT VarKey,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IVariantDictionary * This,
            /* [retval][out] */ __RPC__out int *cStrRet);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, get__NewEnum)
        /* [restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IVariantDictionary * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumReturn);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, Remove)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IVariantDictionary * This,
            /* [in] */ VARIANT VarKey);
        
        DECLSPEC_XFGVIRT(IVariantDictionary, RemoveAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            __RPC__in IVariantDictionary * This);
        
        END_INTERFACE
    } IVariantDictionaryVtbl;

    interface IVariantDictionary
    {
        CONST_VTBL struct IVariantDictionaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVariantDictionary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVariantDictionary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVariantDictionary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVariantDictionary_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVariantDictionary_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVariantDictionary_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVariantDictionary_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVariantDictionary_get_Item(This,VarKey,pvar)	\
    ( (This)->lpVtbl -> get_Item(This,VarKey,pvar) ) 

#define IVariantDictionary_put_Item(This,VarKey,var)	\
    ( (This)->lpVtbl -> put_Item(This,VarKey,var) ) 

#define IVariantDictionary_putref_Item(This,VarKey,var)	\
    ( (This)->lpVtbl -> putref_Item(This,VarKey,var) ) 

#define IVariantDictionary_get_Key(This,VarKey,pvar)	\
    ( (This)->lpVtbl -> get_Key(This,VarKey,pvar) ) 

#define IVariantDictionary_get_Count(This,cStrRet)	\
    ( (This)->lpVtbl -> get_Count(This,cStrRet) ) 

#define IVariantDictionary_get__NewEnum(This,ppEnumReturn)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumReturn) ) 

#define IVariantDictionary_Remove(This,VarKey)	\
    ( (This)->lpVtbl -> Remove(This,VarKey) ) 

#define IVariantDictionary_RemoveAll(This)	\
    ( (This)->lpVtbl -> RemoveAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVariantDictionary_INTERFACE_DEFINED__ */


#ifndef __ISessionObject_INTERFACE_DEFINED__
#define __ISessionObject_INTERFACE_DEFINED__

/* interface ISessionObject */
/* [object][hidden][oleautomation][dual][uuid] */ 


DEFINE_GUID(IID_ISessionObject,0xD97A6DA0,0xA865,0x11cf,0x83,0xAF,0x00,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A865-11cf-83AF-00A0C90C2BD8")
    ISessionObject : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SessionID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRet) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Timeout( 
            /* [retval][out] */ __RPC__out long *plvar) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Timeout( 
            /* [in] */ long lvar) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Abandon( void) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CodePage( 
            /* [retval][out] */ __RPC__out long *plvar) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CodePage( 
            /* [in] */ long lvar) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_LCID( 
            /* [retval][out] */ __RPC__out long *plvar) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LCID( 
            /* [in] */ long lvar) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_StaticObjects( 
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppTaggedObjects) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Contents( 
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISessionObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISessionObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISessionObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISessionObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISessionObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISessionObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISessionObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISessionObject * This,
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
        
        DECLSPEC_XFGVIRT(ISessionObject, get_SessionID)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionID )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRet);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_Value)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ISessionObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ISessionObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(ISessionObject, putref_Value)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Value )( 
            __RPC__in ISessionObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_Timeout)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timeout )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__out long *plvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, put_Timeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Timeout )( 
            __RPC__in ISessionObject * This,
            /* [in] */ long lvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, Abandon)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Abandon )( 
            __RPC__in ISessionObject * This);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_CodePage)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CodePage )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__out long *plvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, put_CodePage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CodePage )( 
            __RPC__in ISessionObject * This,
            /* [in] */ long lvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_LCID)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LCID )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__out long *plvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, put_LCID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LCID )( 
            __RPC__in ISessionObject * This,
            /* [in] */ long lvar);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_StaticObjects)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StaticObjects )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppTaggedObjects);
        
        DECLSPEC_XFGVIRT(ISessionObject, get_Contents)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Contents )( 
            __RPC__in ISessionObject * This,
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties);
        
        END_INTERFACE
    } ISessionObjectVtbl;

    interface ISessionObject
    {
        CONST_VTBL struct ISessionObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISessionObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISessionObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISessionObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISessionObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISessionObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISessionObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISessionObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISessionObject_get_SessionID(This,pbstrRet)	\
    ( (This)->lpVtbl -> get_SessionID(This,pbstrRet) ) 

#define ISessionObject_get_Value(This,bstrValue,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,bstrValue,pvar) ) 

#define ISessionObject_put_Value(This,bstrValue,var)	\
    ( (This)->lpVtbl -> put_Value(This,bstrValue,var) ) 

#define ISessionObject_putref_Value(This,bstrValue,var)	\
    ( (This)->lpVtbl -> putref_Value(This,bstrValue,var) ) 

#define ISessionObject_get_Timeout(This,plvar)	\
    ( (This)->lpVtbl -> get_Timeout(This,plvar) ) 

#define ISessionObject_put_Timeout(This,lvar)	\
    ( (This)->lpVtbl -> put_Timeout(This,lvar) ) 

#define ISessionObject_Abandon(This)	\
    ( (This)->lpVtbl -> Abandon(This) ) 

#define ISessionObject_get_CodePage(This,plvar)	\
    ( (This)->lpVtbl -> get_CodePage(This,plvar) ) 

#define ISessionObject_put_CodePage(This,lvar)	\
    ( (This)->lpVtbl -> put_CodePage(This,lvar) ) 

#define ISessionObject_get_LCID(This,plvar)	\
    ( (This)->lpVtbl -> get_LCID(This,plvar) ) 

#define ISessionObject_put_LCID(This,lvar)	\
    ( (This)->lpVtbl -> put_LCID(This,lvar) ) 

#define ISessionObject_get_StaticObjects(This,ppTaggedObjects)	\
    ( (This)->lpVtbl -> get_StaticObjects(This,ppTaggedObjects) ) 

#define ISessionObject_get_Contents(This,ppProperties)	\
    ( (This)->lpVtbl -> get_Contents(This,ppProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISessionObject_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_Session,0x509F8F20,0x25DE,0x11D0,0xA5,0x5F,0x00,0xA0,0xC9,0x0C,0x20,0x91);

#ifdef __cplusplus

class DECLSPEC_UUID("509F8F20-25DE-11D0-A55F-00A0C90C2091")
Session;
#endif

#ifndef __IApplicationObject_INTERFACE_DEFINED__
#define __IApplicationObject_INTERFACE_DEFINED__

/* interface IApplicationObject */
/* [object][hidden][dual][oleautomation][uuid] */ 


DEFINE_GUID(IID_IApplicationObject,0xD97A6DA0,0xA866,0x11cf,0x83,0xAE,0x10,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A866-11cf-83AE-10A0C90C2BD8")
    IApplicationObject : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Value( 
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Lock( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnLock( void) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_StaticObjects( 
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Contents( 
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApplicationObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApplicationObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApplicationObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IApplicationObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IApplicationObject * This,
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
        
        DECLSPEC_XFGVIRT(IApplicationObject, get_Value)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IApplicationObject, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IApplicationObject, putref_Value)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Value )( 
            __RPC__in IApplicationObject * This,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IApplicationObject, Lock)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Lock )( 
            __RPC__in IApplicationObject * This);
        
        DECLSPEC_XFGVIRT(IApplicationObject, UnLock)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnLock )( 
            __RPC__in IApplicationObject * This);
        
        DECLSPEC_XFGVIRT(IApplicationObject, get_StaticObjects)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StaticObjects )( 
            __RPC__in IApplicationObject * This,
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties);
        
        DECLSPEC_XFGVIRT(IApplicationObject, get_Contents)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Contents )( 
            __RPC__in IApplicationObject * This,
            /* [retval][out] */ __RPC__deref_out_opt IVariantDictionary **ppProperties);
        
        END_INTERFACE
    } IApplicationObjectVtbl;

    interface IApplicationObject
    {
        CONST_VTBL struct IApplicationObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApplicationObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApplicationObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApplicationObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApplicationObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IApplicationObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IApplicationObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IApplicationObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IApplicationObject_get_Value(This,bstrValue,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,bstrValue,pvar) ) 

#define IApplicationObject_put_Value(This,bstrValue,var)	\
    ( (This)->lpVtbl -> put_Value(This,bstrValue,var) ) 

#define IApplicationObject_putref_Value(This,bstrValue,var)	\
    ( (This)->lpVtbl -> putref_Value(This,bstrValue,var) ) 

#define IApplicationObject_Lock(This)	\
    ( (This)->lpVtbl -> Lock(This) ) 

#define IApplicationObject_UnLock(This)	\
    ( (This)->lpVtbl -> UnLock(This) ) 

#define IApplicationObject_get_StaticObjects(This,ppProperties)	\
    ( (This)->lpVtbl -> get_StaticObjects(This,ppProperties) ) 

#define IApplicationObject_get_Contents(This,ppProperties)	\
    ( (This)->lpVtbl -> get_Contents(This,ppProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApplicationObject_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_Application,0x7C3BAF00,0x25DE,0x11D0,0xA5,0x5F,0x00,0xA0,0xC9,0x0C,0x20,0x91);

#ifdef __cplusplus

class DECLSPEC_UUID("7C3BAF00-25DE-11D0-A55F-00A0C90C2091")
Application;
#endif

#ifndef __IASPError_INTERFACE_DEFINED__
#define __IASPError_INTERFACE_DEFINED__

/* interface IASPError */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IASPError,0xF5A6893E,0xA0F5,0x11d1,0x8C,0x4B,0x00,0xC0,0x4F,0xC3,0x24,0xA4);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5A6893E-A0F5-11d1-8C4B-00C04FC324A4")
    IASPError : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ASPCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrASPCode) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Number( 
            /* [retval][out] */ __RPC__out long *plNumber) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Category( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSource) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_File( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Line( 
            /* [retval][out] */ __RPC__out long *plLineNumber) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ASPDescription( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Column( 
            /* [retval][out] */ __RPC__out long *plColumn) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLineText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IASPErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IASPError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IASPError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IASPError * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IASPError * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IASPError * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IASPError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IASPError * This,
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
        
        DECLSPEC_XFGVIRT(IASPError, get_ASPCode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ASPCode )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrASPCode);
        
        DECLSPEC_XFGVIRT(IASPError, get_Number)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Number )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__out long *plNumber);
        
        DECLSPEC_XFGVIRT(IASPError, get_Category)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSource);
        
        DECLSPEC_XFGVIRT(IASPError, get_File)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_File )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName);
        
        DECLSPEC_XFGVIRT(IASPError, get_Line)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Line )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__out long *plLineNumber);
        
        DECLSPEC_XFGVIRT(IASPError, get_Description)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IASPError, get_ASPDescription)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ASPDescription )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IASPError, get_Column)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Column )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__out long *plColumn);
        
        DECLSPEC_XFGVIRT(IASPError, get_Source)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in IASPError * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLineText);
        
        END_INTERFACE
    } IASPErrorVtbl;

    interface IASPError
    {
        CONST_VTBL struct IASPErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IASPError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IASPError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IASPError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IASPError_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IASPError_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IASPError_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IASPError_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IASPError_get_ASPCode(This,pbstrASPCode)	\
    ( (This)->lpVtbl -> get_ASPCode(This,pbstrASPCode) ) 

#define IASPError_get_Number(This,plNumber)	\
    ( (This)->lpVtbl -> get_Number(This,plNumber) ) 

#define IASPError_get_Category(This,pbstrSource)	\
    ( (This)->lpVtbl -> get_Category(This,pbstrSource) ) 

#define IASPError_get_File(This,pbstrFileName)	\
    ( (This)->lpVtbl -> get_File(This,pbstrFileName) ) 

#define IASPError_get_Line(This,plLineNumber)	\
    ( (This)->lpVtbl -> get_Line(This,plLineNumber) ) 

#define IASPError_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IASPError_get_ASPDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_ASPDescription(This,pbstrDescription) ) 

#define IASPError_get_Column(This,plColumn)	\
    ( (This)->lpVtbl -> get_Column(This,plColumn) ) 

#define IASPError_get_Source(This,pbstrLineText)	\
    ( (This)->lpVtbl -> get_Source(This,pbstrLineText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IASPError_INTERFACE_DEFINED__ */


#ifndef __IServer_INTERFACE_DEFINED__
#define __IServer_INTERFACE_DEFINED__

/* interface IServer */
/* [object][hidden][dual][oleautomation][uuid] */ 


DEFINE_GUID(IID_IServer,0xD97A6DA0,0xA867,0x11cf,0x83,0xAE,0x01,0xA0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A867-11cf-83AE-01A0C90C2BD8")
    IServer : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ScriptTimeout( 
            /* [retval][out] */ __RPC__out long *plTimeoutSeconds) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ScriptTimeout( 
            /* [in] */ long lTimeoutSeconds) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateObject( 
            /* [in] */ __RPC__in BSTR bstrProgID,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDispObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HTMLEncode( 
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MapPath( 
            /* [in] */ __RPC__in BSTR bstrLogicalPath,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPhysicalPath) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE URLEncode( 
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded) = 0;
        
        virtual /* [hidden] */ HRESULT STDMETHODCALLTYPE URLPathEncode( 
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Execute( 
            /* [in] */ __RPC__in BSTR bstrLogicalPath) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Transfer( 
            /* [in] */ __RPC__in BSTR bstrLogicalPath) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLastError( 
            /* [retval][out] */ __RPC__deref_out_opt IASPError **ppASPErrorObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IServer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IServer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IServer * This,
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
        
        DECLSPEC_XFGVIRT(IServer, get_ScriptTimeout)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScriptTimeout )( 
            __RPC__in IServer * This,
            /* [retval][out] */ __RPC__out long *plTimeoutSeconds);
        
        DECLSPEC_XFGVIRT(IServer, put_ScriptTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScriptTimeout )( 
            __RPC__in IServer * This,
            /* [in] */ long lTimeoutSeconds);
        
        DECLSPEC_XFGVIRT(IServer, CreateObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateObject )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrProgID,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDispObject);
        
        DECLSPEC_XFGVIRT(IServer, HTMLEncode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HTMLEncode )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded);
        
        DECLSPEC_XFGVIRT(IServer, MapPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MapPath )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrLogicalPath,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPhysicalPath);
        
        DECLSPEC_XFGVIRT(IServer, URLEncode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *URLEncode )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded);
        
        DECLSPEC_XFGVIRT(IServer, URLPathEncode)
        /* [hidden] */ HRESULT ( STDMETHODCALLTYPE *URLPathEncode )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrIn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEncoded);
        
        DECLSPEC_XFGVIRT(IServer, Execute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrLogicalPath);
        
        DECLSPEC_XFGVIRT(IServer, Transfer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Transfer )( 
            __RPC__in IServer * This,
            /* [in] */ __RPC__in BSTR bstrLogicalPath);
        
        DECLSPEC_XFGVIRT(IServer, GetLastError)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLastError )( 
            __RPC__in IServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IASPError **ppASPErrorObject);
        
        END_INTERFACE
    } IServerVtbl;

    interface IServer
    {
        CONST_VTBL struct IServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IServer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IServer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IServer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IServer_get_ScriptTimeout(This,plTimeoutSeconds)	\
    ( (This)->lpVtbl -> get_ScriptTimeout(This,plTimeoutSeconds) ) 

#define IServer_put_ScriptTimeout(This,lTimeoutSeconds)	\
    ( (This)->lpVtbl -> put_ScriptTimeout(This,lTimeoutSeconds) ) 

#define IServer_CreateObject(This,bstrProgID,ppDispObject)	\
    ( (This)->lpVtbl -> CreateObject(This,bstrProgID,ppDispObject) ) 

#define IServer_HTMLEncode(This,bstrIn,pbstrEncoded)	\
    ( (This)->lpVtbl -> HTMLEncode(This,bstrIn,pbstrEncoded) ) 

#define IServer_MapPath(This,bstrLogicalPath,pbstrPhysicalPath)	\
    ( (This)->lpVtbl -> MapPath(This,bstrLogicalPath,pbstrPhysicalPath) ) 

#define IServer_URLEncode(This,bstrIn,pbstrEncoded)	\
    ( (This)->lpVtbl -> URLEncode(This,bstrIn,pbstrEncoded) ) 

#define IServer_URLPathEncode(This,bstrIn,pbstrEncoded)	\
    ( (This)->lpVtbl -> URLPathEncode(This,bstrIn,pbstrEncoded) ) 

#define IServer_Execute(This,bstrLogicalPath)	\
    ( (This)->lpVtbl -> Execute(This,bstrLogicalPath) ) 

#define IServer_Transfer(This,bstrLogicalPath)	\
    ( (This)->lpVtbl -> Transfer(This,bstrLogicalPath) ) 

#define IServer_GetLastError(This,ppASPErrorObject)	\
    ( (This)->lpVtbl -> GetLastError(This,ppASPErrorObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServer_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_Server,0xA506D160,0x25E0,0x11D0,0xA5,0x5F,0x00,0xA0,0xC9,0x0C,0x20,0x91);

#ifdef __cplusplus

class DECLSPEC_UUID("A506D160-25E0-11D0-A55F-00A0C90C2091")
Server;
#endif

#ifndef __IScriptingContext_INTERFACE_DEFINED__
#define __IScriptingContext_INTERFACE_DEFINED__

/* interface IScriptingContext */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


DEFINE_GUID(IID_IScriptingContext,0xD97A6DA0,0xA868,0x11cf,0x83,0xAE,0x00,0xB0,0xC9,0x0C,0x2B,0xD8);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D97A6DA0-A868-11cf-83AE-00B0C90C2BD8")
    IScriptingContext : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Request( 
            /* [retval][out] */ __RPC__deref_out_opt IRequest **ppRequest) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Response( 
            /* [retval][out] */ __RPC__deref_out_opt IResponse **ppResponse) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Server( 
            /* [retval][out] */ __RPC__deref_out_opt IServer **ppServer) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt ISessionObject **ppSession) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IApplicationObject **ppApplication) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptingContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptingContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptingContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptingContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IScriptingContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IScriptingContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IScriptingContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IScriptingContext * This,
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
        
        DECLSPEC_XFGVIRT(IScriptingContext, get_Request)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Request )( 
            __RPC__in IScriptingContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IRequest **ppRequest);
        
        DECLSPEC_XFGVIRT(IScriptingContext, get_Response)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Response )( 
            __RPC__in IScriptingContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IResponse **ppResponse);
        
        DECLSPEC_XFGVIRT(IScriptingContext, get_Server)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Server )( 
            __RPC__in IScriptingContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IServer **ppServer);
        
        DECLSPEC_XFGVIRT(IScriptingContext, get_Session)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IScriptingContext * This,
            /* [retval][out] */ __RPC__deref_out_opt ISessionObject **ppSession);
        
        DECLSPEC_XFGVIRT(IScriptingContext, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IScriptingContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IApplicationObject **ppApplication);
        
        END_INTERFACE
    } IScriptingContextVtbl;

    interface IScriptingContext
    {
        CONST_VTBL struct IScriptingContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptingContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptingContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptingContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptingContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IScriptingContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IScriptingContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IScriptingContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IScriptingContext_get_Request(This,ppRequest)	\
    ( (This)->lpVtbl -> get_Request(This,ppRequest) ) 

#define IScriptingContext_get_Response(This,ppResponse)	\
    ( (This)->lpVtbl -> get_Response(This,ppResponse) ) 

#define IScriptingContext_get_Server(This,ppServer)	\
    ( (This)->lpVtbl -> get_Server(This,ppServer) ) 

#define IScriptingContext_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IScriptingContext_get_Application(This,ppApplication)	\
    ( (This)->lpVtbl -> get_Application(This,ppApplication) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptingContext_INTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_ScriptingContext,0xD97A6DA0,0xA868,0x11cf,0x83,0xAE,0x11,0xB0,0xC9,0x0C,0x2B,0xD8);

#ifdef __cplusplus

class DECLSPEC_UUID("D97A6DA0-A868-11cf-83AE-11B0C90C2BD8")
ScriptingContext;
#endif
#endif /* __ASPTypeLibrary_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



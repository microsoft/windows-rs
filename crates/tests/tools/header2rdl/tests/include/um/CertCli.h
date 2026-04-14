

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __certcli_h__
#define __certcli_h__

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

#ifndef __ICertGetConfig_FWD_DEFINED__
#define __ICertGetConfig_FWD_DEFINED__
typedef interface ICertGetConfig ICertGetConfig;

#endif 	/* __ICertGetConfig_FWD_DEFINED__ */


#ifndef __ICertConfig_FWD_DEFINED__
#define __ICertConfig_FWD_DEFINED__
typedef interface ICertConfig ICertConfig;

#endif 	/* __ICertConfig_FWD_DEFINED__ */


#ifndef __ICertConfig2_FWD_DEFINED__
#define __ICertConfig2_FWD_DEFINED__
typedef interface ICertConfig2 ICertConfig2;

#endif 	/* __ICertConfig2_FWD_DEFINED__ */


#ifndef __ICertRequest_FWD_DEFINED__
#define __ICertRequest_FWD_DEFINED__
typedef interface ICertRequest ICertRequest;

#endif 	/* __ICertRequest_FWD_DEFINED__ */


#ifndef __ICertRequest2_FWD_DEFINED__
#define __ICertRequest2_FWD_DEFINED__
typedef interface ICertRequest2 ICertRequest2;

#endif 	/* __ICertRequest2_FWD_DEFINED__ */


#ifndef __ICertRequest3_FWD_DEFINED__
#define __ICertRequest3_FWD_DEFINED__
typedef interface ICertRequest3 ICertRequest3;

#endif 	/* __ICertRequest3_FWD_DEFINED__ */


#ifndef __CCertGetConfig_FWD_DEFINED__
#define __CCertGetConfig_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertGetConfig CCertGetConfig;
#else
typedef struct CCertGetConfig CCertGetConfig;
#endif /* __cplusplus */

#endif 	/* __CCertGetConfig_FWD_DEFINED__ */


#ifndef __CCertConfig_FWD_DEFINED__
#define __CCertConfig_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertConfig CCertConfig;
#else
typedef struct CCertConfig CCertConfig;
#endif /* __cplusplus */

#endif 	/* __CCertConfig_FWD_DEFINED__ */


#ifndef __CCertRequest_FWD_DEFINED__
#define __CCertRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertRequest CCertRequest;
#else
typedef struct CCertRequest CCertRequest;
#endif /* __cplusplus */

#endif 	/* __CCertRequest_FWD_DEFINED__ */


#ifndef __CCertServerPolicy_FWD_DEFINED__
#define __CCertServerPolicy_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertServerPolicy CCertServerPolicy;
#else
typedef struct CCertServerPolicy CCertServerPolicy;
#endif /* __cplusplus */

#endif 	/* __CCertServerPolicy_FWD_DEFINED__ */


#ifndef __CCertServerExit_FWD_DEFINED__
#define __CCertServerExit_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertServerExit CCertServerExit;
#else
typedef struct CCertServerExit CCertServerExit;
#endif /* __cplusplus */

#endif 	/* __CCertServerExit_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "certif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certcli_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0000_v0_0_s_ifspec;

#ifndef __ICertGetConfig_INTERFACE_DEFINED__
#define __ICertGetConfig_INTERFACE_DEFINED__

/* interface ICertGetConfig */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertGetConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7ea09c0-ce17-11d0-8833-00a0c903b83c")
    ICertGetConfig : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetConfig( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertGetConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertGetConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertGetConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertGetConfig * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertGetConfig * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertGetConfig * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertGetConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertGetConfig * This,
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
        
        DECLSPEC_XFGVIRT(ICertGetConfig, GetConfig)
        HRESULT ( STDMETHODCALLTYPE *GetConfig )( 
            __RPC__in ICertGetConfig * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut);
        
        END_INTERFACE
    } ICertGetConfigVtbl;

    interface ICertGetConfig
    {
        CONST_VTBL struct ICertGetConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertGetConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertGetConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertGetConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertGetConfig_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertGetConfig_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertGetConfig_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertGetConfig_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertGetConfig_GetConfig(This,Flags,pstrOut)	\
    ( (This)->lpVtbl -> GetConfig(This,Flags,pstrOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertGetConfig_INTERFACE_DEFINED__ */


#ifndef __ICertConfig_INTERFACE_DEFINED__
#define __ICertConfig_INTERFACE_DEFINED__

/* interface ICertConfig */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("372fce34-4324-11d0-8810-00a0c903b83c")
    ICertConfig : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__out LONG *pIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetField( 
            /* [in] */ __RPC__in const BSTR strFieldName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfig( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertConfig * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertConfig * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertConfig * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertConfig * This,
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
        
        DECLSPEC_XFGVIRT(ICertConfig, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertConfig * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertConfig, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in ICertConfig * This,
            /* [retval][out] */ __RPC__out LONG *pIndex);
        
        DECLSPEC_XFGVIRT(ICertConfig, GetField)
        HRESULT ( STDMETHODCALLTYPE *GetField )( 
            __RPC__in ICertConfig * This,
            /* [in] */ __RPC__in const BSTR strFieldName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(ICertConfig, GetConfig)
        HRESULT ( STDMETHODCALLTYPE *GetConfig )( 
            __RPC__in ICertConfig * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut);
        
        END_INTERFACE
    } ICertConfigVtbl;

    interface ICertConfig
    {
        CONST_VTBL struct ICertConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertConfig_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertConfig_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertConfig_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertConfig_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertConfig_Reset(This,Index,pCount)	\
    ( (This)->lpVtbl -> Reset(This,Index,pCount) ) 

#define ICertConfig_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define ICertConfig_GetField(This,strFieldName,pstrOut)	\
    ( (This)->lpVtbl -> GetField(This,strFieldName,pstrOut) ) 

#define ICertConfig_GetConfig(This,Flags,pstrOut)	\
    ( (This)->lpVtbl -> GetConfig(This,Flags,pstrOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertConfig_INTERFACE_DEFINED__ */


#ifndef __ICertConfig2_INTERFACE_DEFINED__
#define __ICertConfig2_INTERFACE_DEFINED__

/* interface ICertConfig2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertConfig2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a18edde-7e78-4163-8ded-78e2c9cee924")
    ICertConfig2 : public ICertConfig
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSharedFolder( 
            /* [in] */ __RPC__in const BSTR strSharedFolder) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertConfig2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertConfig2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertConfig2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertConfig2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertConfig2 * This,
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
        
        DECLSPEC_XFGVIRT(ICertConfig, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertConfig, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in ICertConfig2 * This,
            /* [retval][out] */ __RPC__out LONG *pIndex);
        
        DECLSPEC_XFGVIRT(ICertConfig, GetField)
        HRESULT ( STDMETHODCALLTYPE *GetField )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ __RPC__in const BSTR strFieldName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(ICertConfig, GetConfig)
        HRESULT ( STDMETHODCALLTYPE *GetConfig )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(ICertConfig2, SetSharedFolder)
        HRESULT ( STDMETHODCALLTYPE *SetSharedFolder )( 
            __RPC__in ICertConfig2 * This,
            /* [in] */ __RPC__in const BSTR strSharedFolder);
        
        END_INTERFACE
    } ICertConfig2Vtbl;

    interface ICertConfig2
    {
        CONST_VTBL struct ICertConfig2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertConfig2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertConfig2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertConfig2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertConfig2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertConfig2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertConfig2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertConfig2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertConfig2_Reset(This,Index,pCount)	\
    ( (This)->lpVtbl -> Reset(This,Index,pCount) ) 

#define ICertConfig2_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define ICertConfig2_GetField(This,strFieldName,pstrOut)	\
    ( (This)->lpVtbl -> GetField(This,strFieldName,pstrOut) ) 

#define ICertConfig2_GetConfig(This,Flags,pstrOut)	\
    ( (This)->lpVtbl -> GetConfig(This,Flags,pstrOut) ) 


#define ICertConfig2_SetSharedFolder(This,strSharedFolder)	\
    ( (This)->lpVtbl -> SetSharedFolder(This,strSharedFolder) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertConfig2_INTERFACE_DEFINED__ */


#ifndef __ICertRequest_INTERFACE_DEFINED__
#define __ICertRequest_INTERFACE_DEFINED__

/* interface ICertRequest */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("014e4840-5523-11d0-8812-00a0c903b83c")
    ICertRequest : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strRequest,
            /* [in] */ __RPC__in const BSTR strAttributes,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RetrievePending( 
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastStatus( 
            /* [retval][out] */ __RPC__out LONG *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestId( 
            /* [retval][out] */ __RPC__out LONG *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDispositionMessage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDispositionMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCACertificate( 
            /* [in] */ LONG fExchangeCertificate,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificate( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertRequest * This,
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
        
        DECLSPEC_XFGVIRT(ICertRequest, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in ICertRequest * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strRequest,
            /* [in] */ __RPC__in const BSTR strAttributes,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, RetrievePending)
        HRESULT ( STDMETHODCALLTYPE *RetrievePending )( 
            __RPC__in ICertRequest * This,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetLastStatus)
        HRESULT ( STDMETHODCALLTYPE *GetLastStatus )( 
            __RPC__in ICertRequest * This,
            /* [retval][out] */ __RPC__out LONG *pStatus);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetRequestId)
        HRESULT ( STDMETHODCALLTYPE *GetRequestId )( 
            __RPC__in ICertRequest * This,
            /* [retval][out] */ __RPC__out LONG *pRequestId);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetDispositionMessage)
        HRESULT ( STDMETHODCALLTYPE *GetDispositionMessage )( 
            __RPC__in ICertRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCACertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCACertificate )( 
            __RPC__in ICertRequest * This,
            /* [in] */ LONG fExchangeCertificate,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCertificate )( 
            __RPC__in ICertRequest * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        END_INTERFACE
    } ICertRequestVtbl;

    interface ICertRequest
    {
        CONST_VTBL struct ICertRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertRequest_Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition) ) 

#define ICertRequest_RetrievePending(This,RequestId,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> RetrievePending(This,RequestId,strConfig,pDisposition) ) 

#define ICertRequest_GetLastStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetLastStatus(This,pStatus) ) 

#define ICertRequest_GetRequestId(This,pRequestId)	\
    ( (This)->lpVtbl -> GetRequestId(This,pRequestId) ) 

#define ICertRequest_GetDispositionMessage(This,pstrDispositionMessage)	\
    ( (This)->lpVtbl -> GetDispositionMessage(This,pstrDispositionMessage) ) 

#define ICertRequest_GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate) ) 

#define ICertRequest_GetCertificate(This,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCertificate(This,Flags,pstrCertificate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertRequest_INTERFACE_DEFINED__ */


#ifndef __ICertRequest2_INTERFACE_DEFINED__
#define __ICertRequest2_INTERFACE_DEFINED__

/* interface ICertRequest2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertRequest2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a4772988-4a85-4fa9-824e-b5cf5c16405a")
    ICertRequest2 : public ICertRequest
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIssuedCertificate( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorMessageText( 
            /* [in] */ LONG hrMessage,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrErrorMessageText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAProperty( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPropertyFlags( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__out LONG *pPropFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPropertyDisplayName( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullResponseProperty( 
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertRequest2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertRequest2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertRequest2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertRequest2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertRequest2 * This,
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
        
        DECLSPEC_XFGVIRT(ICertRequest, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strRequest,
            /* [in] */ __RPC__in const BSTR strAttributes,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, RetrievePending)
        HRESULT ( STDMETHODCALLTYPE *RetrievePending )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetLastStatus)
        HRESULT ( STDMETHODCALLTYPE *GetLastStatus )( 
            __RPC__in ICertRequest2 * This,
            /* [retval][out] */ __RPC__out LONG *pStatus);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetRequestId)
        HRESULT ( STDMETHODCALLTYPE *GetRequestId )( 
            __RPC__in ICertRequest2 * This,
            /* [retval][out] */ __RPC__out LONG *pRequestId);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetDispositionMessage)
        HRESULT ( STDMETHODCALLTYPE *GetDispositionMessage )( 
            __RPC__in ICertRequest2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCACertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCACertificate )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG fExchangeCertificate,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCertificate )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetIssuedCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetIssuedCertificate )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetErrorMessageText)
        HRESULT ( STDMETHODCALLTYPE *GetErrorMessageText )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG hrMessage,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrErrorMessageText);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCAProperty )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAPropertyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyFlags )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__out LONG *pPropFlags);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAPropertyDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyDisplayName )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDisplayName);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetFullResponseProperty)
        HRESULT ( STDMETHODCALLTYPE *GetFullResponseProperty )( 
            __RPC__in ICertRequest2 * This,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        END_INTERFACE
    } ICertRequest2Vtbl;

    interface ICertRequest2
    {
        CONST_VTBL struct ICertRequest2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertRequest2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertRequest2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertRequest2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertRequest2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertRequest2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertRequest2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertRequest2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertRequest2_Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition) ) 

#define ICertRequest2_RetrievePending(This,RequestId,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> RetrievePending(This,RequestId,strConfig,pDisposition) ) 

#define ICertRequest2_GetLastStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetLastStatus(This,pStatus) ) 

#define ICertRequest2_GetRequestId(This,pRequestId)	\
    ( (This)->lpVtbl -> GetRequestId(This,pRequestId) ) 

#define ICertRequest2_GetDispositionMessage(This,pstrDispositionMessage)	\
    ( (This)->lpVtbl -> GetDispositionMessage(This,pstrDispositionMessage) ) 

#define ICertRequest2_GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate) ) 

#define ICertRequest2_GetCertificate(This,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCertificate(This,Flags,pstrCertificate) ) 


#define ICertRequest2_GetIssuedCertificate(This,strConfig,RequestId,strSerialNumber,pDisposition)	\
    ( (This)->lpVtbl -> GetIssuedCertificate(This,strConfig,RequestId,strSerialNumber,pDisposition) ) 

#define ICertRequest2_GetErrorMessageText(This,hrMessage,Flags,pstrErrorMessageText)	\
    ( (This)->lpVtbl -> GetErrorMessageText(This,hrMessage,Flags,pstrErrorMessageText) ) 

#define ICertRequest2_GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue) ) 

#define ICertRequest2_GetCAPropertyFlags(This,strConfig,PropId,pPropFlags)	\
    ( (This)->lpVtbl -> GetCAPropertyFlags(This,strConfig,PropId,pPropFlags) ) 

#define ICertRequest2_GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName)	\
    ( (This)->lpVtbl -> GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName) ) 

#define ICertRequest2_GetFullResponseProperty(This,PropId,PropIndex,PropType,Flags,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetFullResponseProperty(This,PropId,PropIndex,PropType,Flags,pvarPropertyValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertRequest2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certcli_0000_0005 */
/* [local] */ 

typedef 
enum X509EnrollmentAuthFlags
    {
        X509AuthNone	= 0,
        X509AuthAnonymous	= 1,
        X509AuthKerberos	= 2,
        X509AuthUsername	= 4,
        X509AuthCertificate	= 8
    } 	X509EnrollmentAuthFlags;



extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0005_v0_0_s_ifspec;

#ifndef __ICertRequest3_INTERFACE_DEFINED__
#define __ICertRequest3_INTERFACE_DEFINED__

/* interface ICertRequest3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertRequest3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AFC8F92B-33A2-4861-BF36-2933B7CD67B3")
    ICertRequest3 : public ICertRequest2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCredential( 
            /* [in] */ LONG hWnd,
            /* [in] */ X509EnrollmentAuthFlags AuthType,
            /* [in] */ __RPC__in BSTR strCredential,
            /* [in] */ __RPC__in BSTR strPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestIdString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIssuedCertificate2( 
            /* [in] */ __RPC__in BSTR strConfig,
            /* [in] */ __RPC__in BSTR strRequestId,
            /* [in] */ __RPC__in BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRefreshPolicy( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertRequest3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertRequest3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertRequest3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertRequest3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertRequest3 * This,
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
        
        DECLSPEC_XFGVIRT(ICertRequest, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strRequest,
            /* [in] */ __RPC__in const BSTR strAttributes,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, RetrievePending)
        HRESULT ( STDMETHODCALLTYPE *RetrievePending )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetLastStatus)
        HRESULT ( STDMETHODCALLTYPE *GetLastStatus )( 
            __RPC__in ICertRequest3 * This,
            /* [retval][out] */ __RPC__out LONG *pStatus);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetRequestId)
        HRESULT ( STDMETHODCALLTYPE *GetRequestId )( 
            __RPC__in ICertRequest3 * This,
            /* [retval][out] */ __RPC__out LONG *pRequestId);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetDispositionMessage)
        HRESULT ( STDMETHODCALLTYPE *GetDispositionMessage )( 
            __RPC__in ICertRequest3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCACertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCACertificate )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG fExchangeCertificate,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        DECLSPEC_XFGVIRT(ICertRequest, GetCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCertificate )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCertificate);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetIssuedCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetIssuedCertificate )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetErrorMessageText)
        HRESULT ( STDMETHODCALLTYPE *GetErrorMessageText )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG hrMessage,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrErrorMessageText);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCAProperty )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAPropertyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyFlags )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__out LONG *pPropFlags);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetCAPropertyDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyDisplayName )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDisplayName);
        
        DECLSPEC_XFGVIRT(ICertRequest2, GetFullResponseProperty)
        HRESULT ( STDMETHODCALLTYPE *GetFullResponseProperty )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertRequest3, SetCredential)
        HRESULT ( STDMETHODCALLTYPE *SetCredential )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ LONG hWnd,
            /* [in] */ X509EnrollmentAuthFlags AuthType,
            /* [in] */ __RPC__in BSTR strCredential,
            /* [in] */ __RPC__in BSTR strPassword);
        
        DECLSPEC_XFGVIRT(ICertRequest3, GetRequestIdString)
        HRESULT ( STDMETHODCALLTYPE *GetRequestIdString )( 
            __RPC__in ICertRequest3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrRequestId);
        
        DECLSPEC_XFGVIRT(ICertRequest3, GetIssuedCertificate2)
        HRESULT ( STDMETHODCALLTYPE *GetIssuedCertificate2 )( 
            __RPC__in ICertRequest3 * This,
            /* [in] */ __RPC__in BSTR strConfig,
            /* [in] */ __RPC__in BSTR strRequestId,
            /* [in] */ __RPC__in BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertRequest3, GetRefreshPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetRefreshPolicy )( 
            __RPC__in ICertRequest3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pValue);
        
        END_INTERFACE
    } ICertRequest3Vtbl;

    interface ICertRequest3
    {
        CONST_VTBL struct ICertRequest3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertRequest3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertRequest3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertRequest3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertRequest3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertRequest3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertRequest3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertRequest3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertRequest3_Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> Submit(This,Flags,strRequest,strAttributes,strConfig,pDisposition) ) 

#define ICertRequest3_RetrievePending(This,RequestId,strConfig,pDisposition)	\
    ( (This)->lpVtbl -> RetrievePending(This,RequestId,strConfig,pDisposition) ) 

#define ICertRequest3_GetLastStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetLastStatus(This,pStatus) ) 

#define ICertRequest3_GetRequestId(This,pRequestId)	\
    ( (This)->lpVtbl -> GetRequestId(This,pRequestId) ) 

#define ICertRequest3_GetDispositionMessage(This,pstrDispositionMessage)	\
    ( (This)->lpVtbl -> GetDispositionMessage(This,pstrDispositionMessage) ) 

#define ICertRequest3_GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCACertificate(This,fExchangeCertificate,strConfig,Flags,pstrCertificate) ) 

#define ICertRequest3_GetCertificate(This,Flags,pstrCertificate)	\
    ( (This)->lpVtbl -> GetCertificate(This,Flags,pstrCertificate) ) 


#define ICertRequest3_GetIssuedCertificate(This,strConfig,RequestId,strSerialNumber,pDisposition)	\
    ( (This)->lpVtbl -> GetIssuedCertificate(This,strConfig,RequestId,strSerialNumber,pDisposition) ) 

#define ICertRequest3_GetErrorMessageText(This,hrMessage,Flags,pstrErrorMessageText)	\
    ( (This)->lpVtbl -> GetErrorMessageText(This,hrMessage,Flags,pstrErrorMessageText) ) 

#define ICertRequest3_GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue) ) 

#define ICertRequest3_GetCAPropertyFlags(This,strConfig,PropId,pPropFlags)	\
    ( (This)->lpVtbl -> GetCAPropertyFlags(This,strConfig,PropId,pPropFlags) ) 

#define ICertRequest3_GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName)	\
    ( (This)->lpVtbl -> GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName) ) 

#define ICertRequest3_GetFullResponseProperty(This,PropId,PropIndex,PropType,Flags,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetFullResponseProperty(This,PropId,PropIndex,PropType,Flags,pvarPropertyValue) ) 


#define ICertRequest3_SetCredential(This,hWnd,AuthType,strCredential,strPassword)	\
    ( (This)->lpVtbl -> SetCredential(This,hWnd,AuthType,strCredential,strPassword) ) 

#define ICertRequest3_GetRequestIdString(This,pstrRequestId)	\
    ( (This)->lpVtbl -> GetRequestIdString(This,pstrRequestId) ) 

#define ICertRequest3_GetIssuedCertificate2(This,strConfig,strRequestId,strSerialNumber,pDisposition)	\
    ( (This)->lpVtbl -> GetIssuedCertificate2(This,strConfig,strRequestId,strSerialNumber,pDisposition) ) 

#define ICertRequest3_GetRefreshPolicy(This,pValue)	\
    ( (This)->lpVtbl -> GetRefreshPolicy(This,pValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertRequest3_INTERFACE_DEFINED__ */



#ifndef __CERTCLILib_LIBRARY_DEFINED__
#define __CERTCLILib_LIBRARY_DEFINED__

/* library CERTCLILib */
/* [helpstring][version][uuid] */ 

#define wszCONFIG_COMMONNAME 		L"CommonName"
#define wszCONFIG_ORGUNIT 		L"OrgUnit"
#define wszCONFIG_ORGANIZATION 		L"Organization"
#define wszCONFIG_LOCALITY 		L"Locality"
#define wszCONFIG_STATE			L"State"
#define wszCONFIG_COUNTRY		L"Country"
#define wszCONFIG_CONFIG		L"Config"
#define wszCONFIG_EXCHANGECERTIFICATE	L"ExchangeCertificate"
#define wszCONFIG_SIGNATURECERTIFICATE	L"SignatureCertificate"
#define wszCONFIG_DESCRIPTION		L"Description"
#define wszCONFIG_COMMENT		L"Comment" // obsolete: use Description
#define wszCONFIG_SERVER 		L"Server"
#define wszCONFIG_AUTHORITY 		L"Authority"
#define wszCONFIG_SANITIZEDNAME		L"SanitizedName"
#define wszCONFIG_SHORTNAME		L"ShortName"
#define wszCONFIG_SANITIZEDSHORTNAME	L"SanitizedShortName"
#define wszCONFIG_FLAGS			L"Flags"
#define wszCONFIG_WEBENROLLMENTSERVERS	L"WebEnrollmentServers"
#define	CAIF_DSENTRY	( 0x1 )

#define	CAIF_SHAREDFOLDERENTRY	( 0x2 )

#define	CAIF_REGISTRY	( 0x4 )

#define	CAIF_LOCAL	( 0x8 )

#define	CAIF_REGISTRYPARENT	( 0x10 )

#define	CR_IN_BASE64HEADER	( 0 )

#define	CR_IN_BASE64	( 0x1 )

#define	CR_IN_BINARY	( 0x2 )

#define	CR_IN_ENCODEANY	( 0xff )

#define	CR_IN_ENCODEMASK	( 0xff )

#define	CR_IN_FORMATANY	( 0 )

#define	CR_IN_PKCS10	( 0x100 )

#define	CR_IN_KEYGEN	( 0x200 )

#define	CR_IN_PKCS7	( 0x300 )

#define	CR_IN_CMC	( 0x400 )

#define	CR_IN_CHALLENGERESPONSE	( 0x500 )

#define	CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST	( 0x600 )

#define	CR_IN_FORMATMASK	( 0xff00 )

#define	CR_IN_SCEP	( 0x10000 )

#define	CR_IN_RPC	( 0x20000 )

#define	CR_IN_HTTP	( 0x30000 )

#define	CR_IN_FULLRESPONSE	( 0x40000 )

#define	CR_IN_CRLS	( 0x80000 )

#define	CR_IN_MACHINE	( 0x100000 )

#define	CR_IN_ROBO	( 0x200000 )

#define	CR_IN_CLIENTIDNONE	( 0x400000 )

#define	CR_IN_CONNECTONLY	( 0x800000 )

#define	CR_IN_RETURNCHALLENGE	( 0x1000000 )

#define	CR_IN_SCEPPOST	( 0x2000000 )

#define	CR_IN_CERTIFICATETRANSPARENCY	( 0x4000000 )

#define	CR_IN_PRESIGN	( 0x8000000 )

#define	CR_IN_CLIENTFLAGSMASK	( ( ( ( ( ( CR_IN_ENCODEMASK | CR_IN_RPC )  | CR_IN_MACHINE )  | CR_IN_CLIENTIDNONE )  | CR_IN_CONNECTONLY )  | CR_IN_RETURNCHALLENGE )  )

#define	CC_DEFAULTCONFIG	( 0 )

#define	CC_UIPICKCONFIG	( 0x1 )

#define	CC_FIRSTCONFIG	( 0x2 )

#define	CC_LOCALCONFIG	( 0x3 )

#define	CC_LOCALACTIVECONFIG	( 0x4 )

#define	CC_UIPICKCONFIGSKIPLOCALCA	( 0x5 )

#define	CR_DISP_INCOMPLETE	( 0 )

#define	CR_DISP_ERROR	( 0x1 )

#define	CR_DISP_DENIED	( 0x2 )

#define	CR_DISP_ISSUED	( 0x3 )

#define	CR_DISP_ISSUED_OUT_OF_BAND	( 0x4 )

#define	CR_DISP_UNDER_SUBMISSION	( 0x5 )

#define	CR_DISP_REVOKED	( 0x6 )

#define	CR_OUT_BASE64HEADER	( 0 )

#define	CR_OUT_BASE64	( 0x1 )

#define	CR_OUT_BINARY	( 0x2 )

#define	CR_OUT_BASE64REQUESTHEADER	( 0x3 )

#define	CR_OUT_HEX	( 0x4 )

#define	CR_OUT_HEXASCII	( 0x5 )

#define	CR_OUT_BASE64X509CRLHEADER	( 0x9 )

#define	CR_OUT_HEXADDR	( 0xa )

#define	CR_OUT_HEXASCIIADDR	( 0xb )

#define	CR_OUT_HEXRAW	( 0xc )

#define	CR_OUT_ENCODEMASK	( 0xff )

#define	CR_OUT_CHAIN	( 0x100 )

#define	CR_OUT_CRLS	( 0x200 )

#define	CR_OUT_NOCRLF	( 0x40000000 )

#define	CR_OUT_NOCR	( 0x80000000 )

#define	CR_GEMT_DEFAULT	( 0 )

#define	CR_GEMT_HRESULT_STRING	( 0x1 )

#define	CR_GEMT_HTTP_ERROR	( 0x2 )

#define CR_PROP_NONE               0  // Invalid
#define CR_PROP_FILEVERSION        1  // String
#define CR_PROP_PRODUCTVERSION     2  // String
#define CR_PROP_EXITCOUNT          3  // Long

// CR_PROP_EXITCOUNT Elements:
#define CR_PROP_EXITDESCRIPTION    4  // String, Indexed

#define CR_PROP_POLICYDESCRIPTION  5  // String
#define CR_PROP_CANAME             6  // String
#define CR_PROP_SANITIZEDCANAME    7  // String
#define CR_PROP_SHAREDFOLDER       8  // String
#define CR_PROP_PARENTCA           9  // String
#define CR_PROP_CATYPE            10  // Long
#define CR_PROP_CASIGCERTCOUNT    11  // Long

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CASIGCERT         12  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CASIGCERTCHAIN    13  // Binary, Indexed

#define CR_PROP_CAXCHGCERTCOUNT   14  // Long

// CR_PROP_CAXCHGCERTCOUNT Elements:
#define CR_PROP_CAXCHGCERT        15  // Binary, Indexed

// CR_PROP_CAXCHGCERTCOUNT Elements:
#define CR_PROP_CAXCHGCERTCHAIN   16  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if CR_PROP_CRLSTATE[i] == CA_DISP_VALID
// May also be available if CR_PROP_CRLSTATE[i] == CA_DISP_INVALID
#define CR_PROP_BASECRL           17  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if Delta CRLs enabled && CR_PROP_CRLSTATE[i] == CA_DISP_VALID
// May also be available if CR_PROP_CRLSTATE[i] == CA_DISP_INVALID
#define CR_PROP_DELTACRL          18  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CACERTSTATE       19  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CRLSTATE          20  // Long, Indexed

#define CR_PROP_CAPROPIDMAX       21  // Long
#define CR_PROP_DNSNAME           22  // String
#define CR_PROP_ROLESEPARATIONENABLED 23 // Long
#define CR_PROP_KRACERTUSEDCOUNT  24  // Long
#define CR_PROP_KRACERTCOUNT      25  // Long

// CR_PROP_KRACERTCOUNT Elements:
#define CR_PROP_KRACERT           26  // Binary, Indexed

// CR_PROP_KRACERTCOUNT Elements:
#define CR_PROP_KRACERTSTATE      27  // Long, Indexed

#define CR_PROP_ADVANCEDSERVER    28  // Long
#define CR_PROP_TEMPLATES         29  // String

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if CR_PROP_CRLSTATE[i] == CA_DISP_VALID
#define CR_PROP_BASECRLPUBLISHSTATUS 30  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if Delta CRLs enabled && CR_PROP_CRLSTATE[i] == CA_DISP_VALID
#define CR_PROP_DELTACRLPUBLISHSTATUS 31  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CASIGCERTCRLCHAIN 32  // Binary, Indexed

// CR_PROP_CAXCHGCERTCOUNT Elements:
#define CR_PROP_CAXCHGCERTCRLCHAIN 33 // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CACERTSTATUSCODE  34  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CAFORWARDCROSSCERT 35  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CABACKWARDCROSSCERT 36  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CAFORWARDCROSSCERTSTATE 37  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CABACKWARDCROSSCERTSTATE 38  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
#define CR_PROP_CACERTVERSION       39  // Long, Indexed
#define CR_PROP_SANITIZEDCASHORTNAME 40  // String

// CR_PROP_CERTCDPURLS Elements:
#define CR_PROP_CERTCDPURLS 41  // String, Indexed

// CR_PROP_CERTAIAURLS Elements:
#define CR_PROP_CERTAIAURLS 42  // String, Indexed

// CR_PROP_CERTAIAOCSPURLS Elements:
#define CR_PROP_CERTAIAOCSPURLS 43  // String, Indexed

// CR_PROP_LOCALENAME Elements:
#define CR_PROP_LOCALENAME 44  // String

// CR_PROP_SUBJECTTEMPLATE_OIDS Elements:
#define CR_PROP_SUBJECTTEMPLATE_OIDS 45  // String
#define CR_PROP_CRLPARTITIONCOUNT    46  // Long
// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if CR_PROP_CRLSTATE[i] == CA_DISP_VALID
// May also be available if CR_PROP_CRLSTATE[i] == CA_DISP_INVALID
#define CR_PROP_PARTITIONED_BASECRL          47  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if Delta CRLs enabled && CR_PROP_CRLSTATE[i] == CA_DISP_VALID
// May also be available if CR_PROP_CRLSTATE[i] == CA_DISP_INVALID
#define CR_PROP_PARTITIONED_DELTACRL         48  // Binary, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if CR_PROP_CRLSTATE[i] == CA_DISP_VALID
#define CR_PROP_PARTITIONED_BASECRLPUBLISHSTATUS 49  // Long, Indexed

// CR_PROP_CASIGCERTCOUNT Elements:
// Fetch only if Delta CRLs enabled && CR_PROP_CRLSTATE[i] == CA_DISP_VALID
#define CR_PROP_PARTITIONED_DELTACRLPUBLISHSTATUS 50  // Long, Indexed
#define CR_PROP_SCEPSERVERCERTS        1000  // Binary
#define CR_PROP_SCEPSERVERCAPABILITIES 1001  // String
#define CR_PROP_SCEPSERVERCERTSCHAIN   1002  // Binary
#define CR_PROP_SCEPMIN CR_PROP_SCEPSERVERCERTS
#define CR_PROP_SCEPMAX CR_PROP_SCEPSERVERCERTSCHAIN


#define FR_PROP_NONE                    0  // Invalid
#define FR_PROP_FULLRESPONSE            1  // Binary
#define FR_PROP_STATUSINFOCOUNT         2  // Long

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_BODYPARTSTRING          3  // String, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_STATUS                  4  // Long, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_STATUSSTRING            5  // String, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_OTHERINFOCHOICE         6  // Long, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_FAILINFO                7  // Long, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_PENDINFOTOKEN           8  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_PENDINFOTIME            9  // Date, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ISSUEDCERTIFICATEHASH  10  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ISSUEDCERTIFICATE      11  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ISSUEDCERTIFICATECHAIN 12  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ISSUEDCERTIFICATECRLCHAIN 13  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ENCRYPTEDKEYHASH	  14  // Binary, Indexed

#define FR_PROP_FULLRESPONSENOPKCS7	  15  // Binary

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_CAEXCHANGECERTIFICATEHASH	  16  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_CAEXCHANGECERTIFICATE	  17  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_CAEXCHANGECERTIFICATECHAIN	  18  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN  19  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ATTESTATIONCHALLENGE		  20  // Binary, Indexed

// FR_PROP_STATUSINFOCOUNT Elements:
#define FR_PROP_ATTESTATIONPROVIDERNAME	  21  // Binary, Indexed

// FR_PROP_CLAIMCHALLENGE:
#define FR_PROP_CLAIMCHALLENGE		  22  // Long

EXTERN_C const IID LIBID_CERTCLILib;

EXTERN_C const CLSID CLSID_CCertGetConfig;

#ifdef __cplusplus

class DECLSPEC_UUID("c6cc49b0-ce17-11d0-8833-00a0c903b83c")
CCertGetConfig;
#endif

EXTERN_C const CLSID CLSID_CCertConfig;

#ifdef __cplusplus

class DECLSPEC_UUID("372fce38-4324-11d0-8810-00a0c903b83c")
CCertConfig;
#endif

EXTERN_C const CLSID CLSID_CCertRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("98aff3f0-5524-11d0-8812-00a0c903b83c")
CCertRequest;
#endif

EXTERN_C const CLSID CLSID_CCertServerPolicy;

#ifdef __cplusplus

class DECLSPEC_UUID("aa000926-ffbe-11cf-8800-00a0c903b83c")
CCertServerPolicy;
#endif

EXTERN_C const CLSID CLSID_CCertServerExit;

#ifdef __cplusplus

class DECLSPEC_UUID("4c4a5e40-732c-11d0-8816-00a0c903b83c")
CCertServerExit;
#endif
#endif /* __CERTCLILib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_certcli_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certcli_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



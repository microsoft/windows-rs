

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

#ifndef __xenroll_h__
#define __xenroll_h__

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

#ifndef __ICEnroll_FWD_DEFINED__
#define __ICEnroll_FWD_DEFINED__
typedef interface ICEnroll ICEnroll;

#endif 	/* __ICEnroll_FWD_DEFINED__ */


#ifndef __ICEnroll2_FWD_DEFINED__
#define __ICEnroll2_FWD_DEFINED__
typedef interface ICEnroll2 ICEnroll2;

#endif 	/* __ICEnroll2_FWD_DEFINED__ */


#ifndef __ICEnroll3_FWD_DEFINED__
#define __ICEnroll3_FWD_DEFINED__
typedef interface ICEnroll3 ICEnroll3;

#endif 	/* __ICEnroll3_FWD_DEFINED__ */


#ifndef __ICEnroll4_FWD_DEFINED__
#define __ICEnroll4_FWD_DEFINED__
typedef interface ICEnroll4 ICEnroll4;

#endif 	/* __ICEnroll4_FWD_DEFINED__ */


#ifndef __IEnroll_FWD_DEFINED__
#define __IEnroll_FWD_DEFINED__
typedef interface IEnroll IEnroll;

#endif 	/* __IEnroll_FWD_DEFINED__ */


#ifndef __IEnroll2_FWD_DEFINED__
#define __IEnroll2_FWD_DEFINED__
typedef interface IEnroll2 IEnroll2;

#endif 	/* __IEnroll2_FWD_DEFINED__ */


#ifndef __IEnroll4_FWD_DEFINED__
#define __IEnroll4_FWD_DEFINED__
typedef interface IEnroll4 IEnroll4;

#endif 	/* __IEnroll4_FWD_DEFINED__ */


#ifndef __CEnroll2_FWD_DEFINED__
#define __CEnroll2_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEnroll2 CEnroll2;
#else
typedef struct CEnroll2 CEnroll2;
#endif /* __cplusplus */

#endif 	/* __CEnroll2_FWD_DEFINED__ */


#ifndef __CEnroll_FWD_DEFINED__
#define __CEnroll_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEnroll CEnroll;
#else
typedef struct CEnroll CEnroll;
#endif /* __cplusplus */

#endif 	/* __CEnroll_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wincrypt.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xenroll_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_xenroll_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xenroll_0000_0000_v0_0_s_ifspec;

#ifndef __ICEnroll_INTERFACE_DEFINED__
#define __ICEnroll_INTERFACE_DEFINED__

/* interface ICEnroll */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICEnroll;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43F8F288-7A20-11D0-8F06-00C04FC295E1")
    ICEnroll : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE createFilePKCS10( 
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [in] */ __RPC__in BSTR wszPKCS10FileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptFilePKCS7( 
            /* [in] */ __RPC__in BSTR wszPKCS7FileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createPKCS10( 
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPKCS10) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptPKCS7( 
            /* [in] */ __RPC__in BSTR PKCS7) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getCertFromPKCS7( 
            /* [in] */ __RPC__in BSTR wszPKCS7,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumProviders( 
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumContainers( 
            /* [in] */ LONG dwIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE freeRequestInfo( 
            /* [in] */ __RPC__in BSTR PKCS7OrPKCS10) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreName( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreType( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreName( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreType( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreName( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreType( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreName( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreType( 
            /* [in] */ __RPC__in BSTR bstrType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ContainerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContainer) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ContainerName( 
            /* [in] */ __RPC__in BSTR bstrContainer) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvider) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderName( 
            /* [in] */ __RPC__in BSTR bstrProvider) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderType( 
            /* [retval][out] */ __RPC__out LONG *pdwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderType( 
            /* [in] */ LONG dwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_KeySpec( 
            /* [retval][out] */ __RPC__out LONG *pdw) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_KeySpec( 
            /* [in] */ LONG dw) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UseExistingKeySet( 
            /* [retval][out] */ __RPC__out BOOL *fUseExistingKeys) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_UseExistingKeySet( 
            /* [in] */ BOOL fUseExistingKeys) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GenKeyFlags( 
            /* [retval][out] */ __RPC__out LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GenKeyFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DeleteRequestCert( 
            /* [retval][out] */ __RPC__out BOOL *fDelete) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DeleteRequestCert( 
            /* [in] */ BOOL fDelete) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WriteCertToCSP( 
            /* [retval][out] */ __RPC__out BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WriteCertToCSP( 
            /* [in] */ BOOL fBool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SPCFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SPCFileName( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PVKFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PVKFileName( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICEnrollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICEnroll * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICEnroll * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICEnroll * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICEnroll * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICEnroll * This,
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
        
        DECLSPEC_XFGVIRT(ICEnroll, createFilePKCS10)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10 )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [in] */ __RPC__in BSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptFilePKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7 )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, createPKCS10)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10 )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptPKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7 )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll, getCertFromPKCS7)
        HRESULT ( STDMETHODCALLTYPE *getCertFromPKCS7 )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR wszPKCS7,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumProviders)
        HRESULT ( STDMETHODCALLTYPE *enumProviders )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumContainers)
        HRESULT ( STDMETHODCALLTYPE *enumContainers )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, freeRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfo )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR PKCS7OrPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreType )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreType )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreType )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreType )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreType )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreType )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreType )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreType )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ContainerName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ContainerName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdw);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            __RPC__in ICEnroll * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            __RPC__in ICEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            __RPC__in ICEnroll * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            __RPC__in ICEnroll * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_SPCFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_SPCFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_PVKFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileName )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_PVKFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileName )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_HashAlgorithm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in ICEnroll * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_HashAlgorithm)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in ICEnroll * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        END_INTERFACE
    } ICEnrollVtbl;

    interface ICEnroll
    {
        CONST_VTBL struct ICEnrollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICEnroll_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICEnroll_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICEnroll_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICEnroll_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICEnroll_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICEnroll_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICEnroll_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICEnroll_createFilePKCS10(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10(This,DNName,Usage,wszPKCS10FileName) ) 

#define ICEnroll_acceptFilePKCS7(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7(This,wszPKCS7FileName) ) 

#define ICEnroll_createPKCS10(This,DNName,Usage,pPKCS10)	\
    ( (This)->lpVtbl -> createPKCS10(This,DNName,Usage,pPKCS10) ) 

#define ICEnroll_acceptPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7(This,PKCS7) ) 

#define ICEnroll_getCertFromPKCS7(This,wszPKCS7,pbstrCert)	\
    ( (This)->lpVtbl -> getCertFromPKCS7(This,wszPKCS7,pbstrCert) ) 

#define ICEnroll_enumProviders(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProviders(This,dwIndex,dwFlags,pbstrProvName) ) 

#define ICEnroll_enumContainers(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainers(This,dwIndex,pbstr) ) 

#define ICEnroll_freeRequestInfo(This,PKCS7OrPKCS10)	\
    ( (This)->lpVtbl -> freeRequestInfo(This,PKCS7OrPKCS10) ) 

#define ICEnroll_get_MyStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_MyStoreName(This,pbstrName) ) 

#define ICEnroll_put_MyStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_MyStoreName(This,bstrName) ) 

#define ICEnroll_get_MyStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_MyStoreType(This,pbstrType) ) 

#define ICEnroll_put_MyStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_MyStoreType(This,bstrType) ) 

#define ICEnroll_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define ICEnroll_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define ICEnroll_get_CAStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_CAStoreName(This,pbstrName) ) 

#define ICEnroll_put_CAStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_CAStoreName(This,bstrName) ) 

#define ICEnroll_get_CAStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_CAStoreType(This,pbstrType) ) 

#define ICEnroll_put_CAStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_CAStoreType(This,bstrType) ) 

#define ICEnroll_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define ICEnroll_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define ICEnroll_get_RootStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RootStoreName(This,pbstrName) ) 

#define ICEnroll_put_RootStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RootStoreName(This,bstrName) ) 

#define ICEnroll_get_RootStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RootStoreType(This,pbstrType) ) 

#define ICEnroll_put_RootStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RootStoreType(This,bstrType) ) 

#define ICEnroll_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define ICEnroll_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define ICEnroll_get_RequestStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RequestStoreName(This,pbstrName) ) 

#define ICEnroll_put_RequestStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RequestStoreName(This,bstrName) ) 

#define ICEnroll_get_RequestStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RequestStoreType(This,pbstrType) ) 

#define ICEnroll_put_RequestStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RequestStoreType(This,bstrType) ) 

#define ICEnroll_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define ICEnroll_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define ICEnroll_get_ContainerName(This,pbstrContainer)	\
    ( (This)->lpVtbl -> get_ContainerName(This,pbstrContainer) ) 

#define ICEnroll_put_ContainerName(This,bstrContainer)	\
    ( (This)->lpVtbl -> put_ContainerName(This,bstrContainer) ) 

#define ICEnroll_get_ProviderName(This,pbstrProvider)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrProvider) ) 

#define ICEnroll_put_ProviderName(This,bstrProvider)	\
    ( (This)->lpVtbl -> put_ProviderName(This,bstrProvider) ) 

#define ICEnroll_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define ICEnroll_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define ICEnroll_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define ICEnroll_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define ICEnroll_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define ICEnroll_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define ICEnroll_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define ICEnroll_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define ICEnroll_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define ICEnroll_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define ICEnroll_get_SPCFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_SPCFileName(This,pbstr) ) 

#define ICEnroll_put_SPCFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_SPCFileName(This,bstr) ) 

#define ICEnroll_get_PVKFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_PVKFileName(This,pbstr) ) 

#define ICEnroll_put_PVKFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_PVKFileName(This,bstr) ) 

#define ICEnroll_get_HashAlgorithm(This,pbstr)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pbstr) ) 

#define ICEnroll_put_HashAlgorithm(This,bstr)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,bstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICEnroll_INTERFACE_DEFINED__ */


#ifndef __ICEnroll2_INTERFACE_DEFINED__
#define __ICEnroll2_INTERFACE_DEFINED__

/* interface ICEnroll2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICEnroll2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("704ca730-c90b-11d1-9bec-00c04fc295e1")
    ICEnroll2 : public ICEnroll
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE addCertTypeToRequest( 
            /* [in] */ __RPC__in BSTR CertType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addNameValuePairToSignature( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in BSTR Value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WriteCertToUserDS( 
            /* [retval][out] */ __RPC__out BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WriteCertToUserDS( 
            /* [in] */ BOOL fBool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EnableT61DNEncoding( 
            /* [retval][out] */ __RPC__out BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EnableT61DNEncoding( 
            /* [in] */ BOOL fBool) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICEnroll2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICEnroll2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICEnroll2 * This,
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
        
        DECLSPEC_XFGVIRT(ICEnroll, createFilePKCS10)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10 )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [in] */ __RPC__in BSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptFilePKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7 )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, createPKCS10)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10 )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptPKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7 )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll, getCertFromPKCS7)
        HRESULT ( STDMETHODCALLTYPE *getCertFromPKCS7 )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumProviders)
        HRESULT ( STDMETHODCALLTYPE *enumProviders )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumContainers)
        HRESULT ( STDMETHODCALLTYPE *enumContainers )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, freeRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfo )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR PKCS7OrPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreType )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ContainerName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ContainerName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdw);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_SPCFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_SPCFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_PVKFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileName )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_PVKFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileName )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_HashAlgorithm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_HashAlgorithm)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addCertTypeToRequest)
        HRESULT ( STDMETHODCALLTYPE *addCertTypeToRequest )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR CertType);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addNameValuePairToSignature)
        HRESULT ( STDMETHODCALLTYPE *addNameValuePairToSignature )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in BSTR Value);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            __RPC__in ICEnroll2 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            __RPC__in ICEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        END_INTERFACE
    } ICEnroll2Vtbl;

    interface ICEnroll2
    {
        CONST_VTBL struct ICEnroll2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICEnroll2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICEnroll2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICEnroll2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICEnroll2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICEnroll2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICEnroll2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICEnroll2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICEnroll2_createFilePKCS10(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10(This,DNName,Usage,wszPKCS10FileName) ) 

#define ICEnroll2_acceptFilePKCS7(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7(This,wszPKCS7FileName) ) 

#define ICEnroll2_createPKCS10(This,DNName,Usage,pPKCS10)	\
    ( (This)->lpVtbl -> createPKCS10(This,DNName,Usage,pPKCS10) ) 

#define ICEnroll2_acceptPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7(This,PKCS7) ) 

#define ICEnroll2_getCertFromPKCS7(This,wszPKCS7,pbstrCert)	\
    ( (This)->lpVtbl -> getCertFromPKCS7(This,wszPKCS7,pbstrCert) ) 

#define ICEnroll2_enumProviders(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProviders(This,dwIndex,dwFlags,pbstrProvName) ) 

#define ICEnroll2_enumContainers(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainers(This,dwIndex,pbstr) ) 

#define ICEnroll2_freeRequestInfo(This,PKCS7OrPKCS10)	\
    ( (This)->lpVtbl -> freeRequestInfo(This,PKCS7OrPKCS10) ) 

#define ICEnroll2_get_MyStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_MyStoreName(This,pbstrName) ) 

#define ICEnroll2_put_MyStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_MyStoreName(This,bstrName) ) 

#define ICEnroll2_get_MyStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_MyStoreType(This,pbstrType) ) 

#define ICEnroll2_put_MyStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_MyStoreType(This,bstrType) ) 

#define ICEnroll2_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define ICEnroll2_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define ICEnroll2_get_CAStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_CAStoreName(This,pbstrName) ) 

#define ICEnroll2_put_CAStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_CAStoreName(This,bstrName) ) 

#define ICEnroll2_get_CAStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_CAStoreType(This,pbstrType) ) 

#define ICEnroll2_put_CAStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_CAStoreType(This,bstrType) ) 

#define ICEnroll2_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define ICEnroll2_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define ICEnroll2_get_RootStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RootStoreName(This,pbstrName) ) 

#define ICEnroll2_put_RootStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RootStoreName(This,bstrName) ) 

#define ICEnroll2_get_RootStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RootStoreType(This,pbstrType) ) 

#define ICEnroll2_put_RootStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RootStoreType(This,bstrType) ) 

#define ICEnroll2_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define ICEnroll2_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define ICEnroll2_get_RequestStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RequestStoreName(This,pbstrName) ) 

#define ICEnroll2_put_RequestStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RequestStoreName(This,bstrName) ) 

#define ICEnroll2_get_RequestStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RequestStoreType(This,pbstrType) ) 

#define ICEnroll2_put_RequestStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RequestStoreType(This,bstrType) ) 

#define ICEnroll2_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define ICEnroll2_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define ICEnroll2_get_ContainerName(This,pbstrContainer)	\
    ( (This)->lpVtbl -> get_ContainerName(This,pbstrContainer) ) 

#define ICEnroll2_put_ContainerName(This,bstrContainer)	\
    ( (This)->lpVtbl -> put_ContainerName(This,bstrContainer) ) 

#define ICEnroll2_get_ProviderName(This,pbstrProvider)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrProvider) ) 

#define ICEnroll2_put_ProviderName(This,bstrProvider)	\
    ( (This)->lpVtbl -> put_ProviderName(This,bstrProvider) ) 

#define ICEnroll2_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define ICEnroll2_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define ICEnroll2_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define ICEnroll2_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define ICEnroll2_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define ICEnroll2_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define ICEnroll2_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll2_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll2_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define ICEnroll2_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define ICEnroll2_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll2_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll2_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define ICEnroll2_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define ICEnroll2_get_SPCFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_SPCFileName(This,pbstr) ) 

#define ICEnroll2_put_SPCFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_SPCFileName(This,bstr) ) 

#define ICEnroll2_get_PVKFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_PVKFileName(This,pbstr) ) 

#define ICEnroll2_put_PVKFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_PVKFileName(This,bstr) ) 

#define ICEnroll2_get_HashAlgorithm(This,pbstr)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pbstr) ) 

#define ICEnroll2_put_HashAlgorithm(This,bstr)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,bstr) ) 


#define ICEnroll2_addCertTypeToRequest(This,CertType)	\
    ( (This)->lpVtbl -> addCertTypeToRequest(This,CertType) ) 

#define ICEnroll2_addNameValuePairToSignature(This,Name,Value)	\
    ( (This)->lpVtbl -> addNameValuePairToSignature(This,Name,Value) ) 

#define ICEnroll2_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll2_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll2_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define ICEnroll2_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICEnroll2_INTERFACE_DEFINED__ */


#ifndef __ICEnroll3_INTERFACE_DEFINED__
#define __ICEnroll3_INTERFACE_DEFINED__

/* interface ICEnroll3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICEnroll3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c28c2d95-b7de-11d2-a421-00c04f79fe8e")
    ICEnroll3 : public ICEnroll2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InstallPKCS7( 
            /* [in] */ __RPC__in BSTR PKCS7) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedKeySpec( 
            /* [retval][out] */ __RPC__out LONG *pdwKeySpec) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLen( 
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [retval][out] */ __RPC__out LONG *pdwKeySize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumAlgs( 
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [retval][out] */ __RPC__out LONG *pdwAlgID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlgName( 
            /* [in] */ LONG algID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ReuseHardwareKeyIfUnableToGenNew( 
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ReuseHardwareKeyIfUnableToGenNew( 
            /* [retval][out] */ __RPC__out BOOL *fReuseHardwareKeyIfUnableToGenNew) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgID( 
            /* [in] */ LONG hashAlgID) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgID( 
            /* [retval][out] */ __RPC__out LONG *hashAlgID) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LimitExchangeKeyToEncipherment( 
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LimitExchangeKeyToEncipherment( 
            /* [retval][out] */ __RPC__out BOOL *fLimitExchangeKeyToEncipherment) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EnableSMIMECapabilities( 
            /* [in] */ BOOL fEnableSMIMECapabilities) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EnableSMIMECapabilities( 
            /* [retval][out] */ __RPC__out BOOL *fEnableSMIMECapabilities) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICEnroll3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICEnroll3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICEnroll3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICEnroll3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICEnroll3 * This,
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
        
        DECLSPEC_XFGVIRT(ICEnroll, createFilePKCS10)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [in] */ __RPC__in BSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptFilePKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, createPKCS10)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptPKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll, getCertFromPKCS7)
        HRESULT ( STDMETHODCALLTYPE *getCertFromPKCS7 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumProviders)
        HRESULT ( STDMETHODCALLTYPE *enumProviders )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumContainers)
        HRESULT ( STDMETHODCALLTYPE *enumContainers )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, freeRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfo )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR PKCS7OrPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreType )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ContainerName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ContainerName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdw);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_SPCFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_SPCFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_PVKFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileName )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_PVKFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_HashAlgorithm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_HashAlgorithm)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addCertTypeToRequest)
        HRESULT ( STDMETHODCALLTYPE *addCertTypeToRequest )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR CertType);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addNameValuePairToSignature)
        HRESULT ( STDMETHODCALLTYPE *addNameValuePairToSignature )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in BSTR Value);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll3, InstallPKCS7)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7 )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll3, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICEnroll3 * This);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetSupportedKeySpec)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedKeySpec )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *pdwKeySpec);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetKeyLen)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLen )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [retval][out] */ __RPC__out LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(ICEnroll3, EnumAlgs)
        HRESULT ( STDMETHODCALLTYPE *EnumAlgs )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [retval][out] */ __RPC__out LONG *pdwAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetAlgName)
        HRESULT ( STDMETHODCALLTYPE *GetAlgName )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG algID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_ReuseHardwareKeyIfUnableToGenNew)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReuseHardwareKeyIfUnableToGenNew )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_ReuseHardwareKeyIfUnableToGenNew)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReuseHardwareKeyIfUnableToGenNew )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_HashAlgID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgID )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ LONG hashAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_HashAlgID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgID )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out LONG *hashAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_LimitExchangeKeyToEncipherment)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LimitExchangeKeyToEncipherment )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_LimitExchangeKeyToEncipherment)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LimitExchangeKeyToEncipherment )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_EnableSMIMECapabilities)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableSMIMECapabilities )( 
            __RPC__in ICEnroll3 * This,
            /* [in] */ BOOL fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_EnableSMIMECapabilities)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableSMIMECapabilities )( 
            __RPC__in ICEnroll3 * This,
            /* [retval][out] */ __RPC__out BOOL *fEnableSMIMECapabilities);
        
        END_INTERFACE
    } ICEnroll3Vtbl;

    interface ICEnroll3
    {
        CONST_VTBL struct ICEnroll3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICEnroll3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICEnroll3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICEnroll3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICEnroll3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICEnroll3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICEnroll3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICEnroll3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICEnroll3_createFilePKCS10(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10(This,DNName,Usage,wszPKCS10FileName) ) 

#define ICEnroll3_acceptFilePKCS7(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7(This,wszPKCS7FileName) ) 

#define ICEnroll3_createPKCS10(This,DNName,Usage,pPKCS10)	\
    ( (This)->lpVtbl -> createPKCS10(This,DNName,Usage,pPKCS10) ) 

#define ICEnroll3_acceptPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7(This,PKCS7) ) 

#define ICEnroll3_getCertFromPKCS7(This,wszPKCS7,pbstrCert)	\
    ( (This)->lpVtbl -> getCertFromPKCS7(This,wszPKCS7,pbstrCert) ) 

#define ICEnroll3_enumProviders(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProviders(This,dwIndex,dwFlags,pbstrProvName) ) 

#define ICEnroll3_enumContainers(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainers(This,dwIndex,pbstr) ) 

#define ICEnroll3_freeRequestInfo(This,PKCS7OrPKCS10)	\
    ( (This)->lpVtbl -> freeRequestInfo(This,PKCS7OrPKCS10) ) 

#define ICEnroll3_get_MyStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_MyStoreName(This,pbstrName) ) 

#define ICEnroll3_put_MyStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_MyStoreName(This,bstrName) ) 

#define ICEnroll3_get_MyStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_MyStoreType(This,pbstrType) ) 

#define ICEnroll3_put_MyStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_MyStoreType(This,bstrType) ) 

#define ICEnroll3_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define ICEnroll3_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define ICEnroll3_get_CAStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_CAStoreName(This,pbstrName) ) 

#define ICEnroll3_put_CAStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_CAStoreName(This,bstrName) ) 

#define ICEnroll3_get_CAStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_CAStoreType(This,pbstrType) ) 

#define ICEnroll3_put_CAStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_CAStoreType(This,bstrType) ) 

#define ICEnroll3_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define ICEnroll3_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define ICEnroll3_get_RootStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RootStoreName(This,pbstrName) ) 

#define ICEnroll3_put_RootStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RootStoreName(This,bstrName) ) 

#define ICEnroll3_get_RootStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RootStoreType(This,pbstrType) ) 

#define ICEnroll3_put_RootStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RootStoreType(This,bstrType) ) 

#define ICEnroll3_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define ICEnroll3_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define ICEnroll3_get_RequestStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RequestStoreName(This,pbstrName) ) 

#define ICEnroll3_put_RequestStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RequestStoreName(This,bstrName) ) 

#define ICEnroll3_get_RequestStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RequestStoreType(This,pbstrType) ) 

#define ICEnroll3_put_RequestStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RequestStoreType(This,bstrType) ) 

#define ICEnroll3_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define ICEnroll3_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define ICEnroll3_get_ContainerName(This,pbstrContainer)	\
    ( (This)->lpVtbl -> get_ContainerName(This,pbstrContainer) ) 

#define ICEnroll3_put_ContainerName(This,bstrContainer)	\
    ( (This)->lpVtbl -> put_ContainerName(This,bstrContainer) ) 

#define ICEnroll3_get_ProviderName(This,pbstrProvider)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrProvider) ) 

#define ICEnroll3_put_ProviderName(This,bstrProvider)	\
    ( (This)->lpVtbl -> put_ProviderName(This,bstrProvider) ) 

#define ICEnroll3_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define ICEnroll3_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define ICEnroll3_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define ICEnroll3_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define ICEnroll3_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define ICEnroll3_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define ICEnroll3_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll3_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll3_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define ICEnroll3_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define ICEnroll3_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll3_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll3_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define ICEnroll3_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define ICEnroll3_get_SPCFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_SPCFileName(This,pbstr) ) 

#define ICEnroll3_put_SPCFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_SPCFileName(This,bstr) ) 

#define ICEnroll3_get_PVKFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_PVKFileName(This,pbstr) ) 

#define ICEnroll3_put_PVKFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_PVKFileName(This,bstr) ) 

#define ICEnroll3_get_HashAlgorithm(This,pbstr)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pbstr) ) 

#define ICEnroll3_put_HashAlgorithm(This,bstr)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,bstr) ) 


#define ICEnroll3_addCertTypeToRequest(This,CertType)	\
    ( (This)->lpVtbl -> addCertTypeToRequest(This,CertType) ) 

#define ICEnroll3_addNameValuePairToSignature(This,Name,Value)	\
    ( (This)->lpVtbl -> addNameValuePairToSignature(This,Name,Value) ) 

#define ICEnroll3_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll3_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll3_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define ICEnroll3_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 


#define ICEnroll3_InstallPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> InstallPKCS7(This,PKCS7) ) 

#define ICEnroll3_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ICEnroll3_GetSupportedKeySpec(This,pdwKeySpec)	\
    ( (This)->lpVtbl -> GetSupportedKeySpec(This,pdwKeySpec) ) 

#define ICEnroll3_GetKeyLen(This,fMin,fExchange,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLen(This,fMin,fExchange,pdwKeySize) ) 

#define ICEnroll3_EnumAlgs(This,dwIndex,algClass,pdwAlgID)	\
    ( (This)->lpVtbl -> EnumAlgs(This,dwIndex,algClass,pdwAlgID) ) 

#define ICEnroll3_GetAlgName(This,algID,pbstr)	\
    ( (This)->lpVtbl -> GetAlgName(This,algID,pbstr) ) 

#define ICEnroll3_put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define ICEnroll3_get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define ICEnroll3_put_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> put_HashAlgID(This,hashAlgID) ) 

#define ICEnroll3_get_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> get_HashAlgID(This,hashAlgID) ) 

#define ICEnroll3_put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define ICEnroll3_get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define ICEnroll3_put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#define ICEnroll3_get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICEnroll3_INTERFACE_DEFINED__ */


#ifndef __ICEnroll4_INTERFACE_DEFINED__
#define __ICEnroll4_INTERFACE_DEFINED__

/* interface ICEnroll4 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICEnroll4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c1f1188a-2eb5-4a80-841b-7e729a356d90")
    ICEnroll4 : public ICEnroll3
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PrivateKeyArchiveCertificate( 
            /* [in] */ __RPC__in BSTR bstrCert) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PrivateKeyArchiveCertificate( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ThumbPrint( 
            /* [in] */ __RPC__in BSTR bstrThumbPrint) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ThumbPrint( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrThumbPrint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE binaryToString( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strBinary,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncoded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE stringToBinary( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strEncoded,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addExtensionToRequest( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addAttributeToRequest( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addNameValuePairToRequest( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resetExtensions( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resetAttributes( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createRequest( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strDNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createFileRequest( 
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strDNName,
            /* [in] */ __RPC__in BSTR strUsage,
            /* [in] */ __RPC__in BSTR strRequestFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptResponse( 
            /* [in] */ __RPC__in BSTR strResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptFileResponse( 
            /* [in] */ __RPC__in BSTR strResponseFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getCertFromResponse( 
            /* [in] */ __RPC__in BSTR strResponse,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getCertFromFileResponse( 
            /* [in] */ __RPC__in BSTR strResponseFileName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createPFX( 
            /* [in] */ __RPC__in BSTR strPassword,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrPFX) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createFilePFX( 
            /* [in] */ __RPC__in BSTR strPassword,
            /* [in] */ __RPC__in BSTR strPFXFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setPendingRequestInfo( 
            /* [in] */ LONG lRequestID,
            /* [in] */ __RPC__in BSTR strCADNS,
            /* [in] */ __RPC__in BSTR strCAName,
            /* [in] */ __RPC__in BSTR strFriendlyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumPendingRequest( 
            /* [in] */ LONG lIndex,
            /* [in] */ LONG lDesiredProperty,
            /* [retval][out] */ __RPC__out VARIANT *pvarProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE removePendingRequest( 
            /* [in] */ __RPC__in BSTR strThumbprint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLenEx( 
            /* [in] */ LONG lSizeSpec,
            /* [in] */ LONG lKeySpec,
            /* [retval][out] */ __RPC__out LONG *pdwKeySize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallPKCS7Ex( 
            /* [in] */ __RPC__in BSTR PKCS7,
            /* [retval][out] */ __RPC__out LONG *plCertInstalled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addCertTypeToRequestEx( 
            /* [in] */ LONG lType,
            /* [in] */ __RPC__in BSTR bstrOIDOrName,
            /* [in] */ LONG lMajorVersion,
            /* [in] */ BOOL fMinorVersion,
            /* [in] */ LONG lMinorVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getProviderType( 
            /* [in] */ __RPC__in BSTR strProvName,
            /* [retval][out] */ __RPC__out LONG *plProvType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SignerCertificate( 
            /* [in] */ __RPC__in BSTR bstrCert) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClientId( 
            /* [in] */ LONG lClientId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClientId( 
            /* [retval][out] */ __RPC__out LONG *plClientId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addBlobPropertyToCertificate( 
            /* [in] */ LONG lPropertyId,
            /* [in] */ LONG lReserved,
            /* [in] */ __RPC__in BSTR bstrProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resetBlobProperties( void) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IncludeSubjectKeyID( 
            /* [in] */ BOOL fInclude) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IncludeSubjectKeyID( 
            /* [retval][out] */ __RPC__out BOOL *pfInclude) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICEnroll4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICEnroll4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICEnroll4 * This,
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
        
        DECLSPEC_XFGVIRT(ICEnroll, createFilePKCS10)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [in] */ __RPC__in BSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptFilePKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(ICEnroll, createPKCS10)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR DNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, acceptPKCS7)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll, getCertFromPKCS7)
        HRESULT ( STDMETHODCALLTYPE *getCertFromPKCS7 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR wszPKCS7,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumProviders)
        HRESULT ( STDMETHODCALLTYPE *enumProviders )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(ICEnroll, enumContainers)
        HRESULT ( STDMETHODCALLTYPE *enumContainers )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, freeRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfo )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR PKCS7OrPKCS10);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ContainerName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ContainerName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrContainer);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrProvider);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdw);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_SPCFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_SPCFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_PVKFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileName )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_PVKFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, get_HashAlgorithm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll, put_HashAlgorithm)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addCertTypeToRequest)
        HRESULT ( STDMETHODCALLTYPE *addCertTypeToRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR CertType);
        
        DECLSPEC_XFGVIRT(ICEnroll2, addNameValuePairToSignature)
        HRESULT ( STDMETHODCALLTYPE *addNameValuePairToSignature )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in BSTR Value);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll2, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(ICEnroll3, InstallPKCS7)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7 )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR PKCS7);
        
        DECLSPEC_XFGVIRT(ICEnroll3, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetSupportedKeySpec)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedKeySpec )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *pdwKeySpec);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetKeyLen)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLen )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [retval][out] */ __RPC__out LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(ICEnroll3, EnumAlgs)
        HRESULT ( STDMETHODCALLTYPE *EnumAlgs )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [retval][out] */ __RPC__out LONG *pdwAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, GetAlgName)
        HRESULT ( STDMETHODCALLTYPE *GetAlgName )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG algID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_ReuseHardwareKeyIfUnableToGenNew)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReuseHardwareKeyIfUnableToGenNew )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_ReuseHardwareKeyIfUnableToGenNew)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReuseHardwareKeyIfUnableToGenNew )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_HashAlgID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgID )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG hashAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_HashAlgID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgID )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *hashAlgID);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_LimitExchangeKeyToEncipherment)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LimitExchangeKeyToEncipherment )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_LimitExchangeKeyToEncipherment)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LimitExchangeKeyToEncipherment )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(ICEnroll3, put_EnableSMIMECapabilities)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableSMIMECapabilities )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(ICEnroll3, get_EnableSMIMECapabilities)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableSMIMECapabilities )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(ICEnroll4, put_PrivateKeyArchiveCertificate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrivateKeyArchiveCertificate )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll4, get_PrivateKeyArchiveCertificate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateKeyArchiveCertificate )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll4, put_ThumbPrint)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ThumbPrint )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrThumbPrint);
        
        DECLSPEC_XFGVIRT(ICEnroll4, get_ThumbPrint)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ThumbPrint )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrThumbPrint);
        
        DECLSPEC_XFGVIRT(ICEnroll4, binaryToString)
        HRESULT ( STDMETHODCALLTYPE *binaryToString )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strBinary,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncoded);
        
        DECLSPEC_XFGVIRT(ICEnroll4, stringToBinary)
        HRESULT ( STDMETHODCALLTYPE *stringToBinary )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strEncoded,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICEnroll4, addExtensionToRequest)
        HRESULT ( STDMETHODCALLTYPE *addExtensionToRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue);
        
        DECLSPEC_XFGVIRT(ICEnroll4, addAttributeToRequest)
        HRESULT ( STDMETHODCALLTYPE *addAttributeToRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue);
        
        DECLSPEC_XFGVIRT(ICEnroll4, addNameValuePairToRequest)
        HRESULT ( STDMETHODCALLTYPE *addNameValuePairToRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strName,
            /* [in] */ __RPC__in BSTR strValue);
        
        DECLSPEC_XFGVIRT(ICEnroll4, resetExtensions)
        HRESULT ( STDMETHODCALLTYPE *resetExtensions )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(ICEnroll4, resetAttributes)
        HRESULT ( STDMETHODCALLTYPE *resetAttributes )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(ICEnroll4, createRequest)
        HRESULT ( STDMETHODCALLTYPE *createRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strDNName,
            /* [in] */ __RPC__in BSTR Usage,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrRequest);
        
        DECLSPEC_XFGVIRT(ICEnroll4, createFileRequest)
        HRESULT ( STDMETHODCALLTYPE *createFileRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in BSTR strDNName,
            /* [in] */ __RPC__in BSTR strUsage,
            /* [in] */ __RPC__in BSTR strRequestFileName);
        
        DECLSPEC_XFGVIRT(ICEnroll4, acceptResponse)
        HRESULT ( STDMETHODCALLTYPE *acceptResponse )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strResponse);
        
        DECLSPEC_XFGVIRT(ICEnroll4, acceptFileResponse)
        HRESULT ( STDMETHODCALLTYPE *acceptFileResponse )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strResponseFileName);
        
        DECLSPEC_XFGVIRT(ICEnroll4, getCertFromResponse)
        HRESULT ( STDMETHODCALLTYPE *getCertFromResponse )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strResponse,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll4, getCertFromFileResponse)
        HRESULT ( STDMETHODCALLTYPE *getCertFromFileResponse )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strResponseFileName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll4, createPFX)
        HRESULT ( STDMETHODCALLTYPE *createPFX )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strPassword,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrPFX);
        
        DECLSPEC_XFGVIRT(ICEnroll4, createFilePFX)
        HRESULT ( STDMETHODCALLTYPE *createFilePFX )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strPassword,
            /* [in] */ __RPC__in BSTR strPFXFileName);
        
        DECLSPEC_XFGVIRT(ICEnroll4, setPendingRequestInfo)
        HRESULT ( STDMETHODCALLTYPE *setPendingRequestInfo )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lRequestID,
            /* [in] */ __RPC__in BSTR strCADNS,
            /* [in] */ __RPC__in BSTR strCAName,
            /* [in] */ __RPC__in BSTR strFriendlyName);
        
        DECLSPEC_XFGVIRT(ICEnroll4, enumPendingRequest)
        HRESULT ( STDMETHODCALLTYPE *enumPendingRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lIndex,
            /* [in] */ LONG lDesiredProperty,
            /* [retval][out] */ __RPC__out VARIANT *pvarProperty);
        
        DECLSPEC_XFGVIRT(ICEnroll4, removePendingRequest)
        HRESULT ( STDMETHODCALLTYPE *removePendingRequest )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strThumbprint);
        
        DECLSPEC_XFGVIRT(ICEnroll4, GetKeyLenEx)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLenEx )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lSizeSpec,
            /* [in] */ LONG lKeySpec,
            /* [retval][out] */ __RPC__out LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(ICEnroll4, InstallPKCS7Ex)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7Ex )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR PKCS7,
            /* [retval][out] */ __RPC__out LONG *plCertInstalled);
        
        DECLSPEC_XFGVIRT(ICEnroll4, addCertTypeToRequestEx)
        HRESULT ( STDMETHODCALLTYPE *addCertTypeToRequestEx )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lType,
            /* [in] */ __RPC__in BSTR bstrOIDOrName,
            /* [in] */ LONG lMajorVersion,
            /* [in] */ BOOL fMinorVersion,
            /* [in] */ LONG lMinorVersion);
        
        DECLSPEC_XFGVIRT(ICEnroll4, getProviderType)
        HRESULT ( STDMETHODCALLTYPE *getProviderType )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR strProvName,
            /* [retval][out] */ __RPC__out LONG *plProvType);
        
        DECLSPEC_XFGVIRT(ICEnroll4, put_SignerCertificate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignerCertificate )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ __RPC__in BSTR bstrCert);
        
        DECLSPEC_XFGVIRT(ICEnroll4, put_ClientId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientId )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lClientId);
        
        DECLSPEC_XFGVIRT(ICEnroll4, get_ClientId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientId )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out LONG *plClientId);
        
        DECLSPEC_XFGVIRT(ICEnroll4, addBlobPropertyToCertificate)
        HRESULT ( STDMETHODCALLTYPE *addBlobPropertyToCertificate )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ LONG lPropertyId,
            /* [in] */ LONG lReserved,
            /* [in] */ __RPC__in BSTR bstrProperty);
        
        DECLSPEC_XFGVIRT(ICEnroll4, resetBlobProperties)
        HRESULT ( STDMETHODCALLTYPE *resetBlobProperties )( 
            __RPC__in ICEnroll4 * This);
        
        DECLSPEC_XFGVIRT(ICEnroll4, put_IncludeSubjectKeyID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncludeSubjectKeyID )( 
            __RPC__in ICEnroll4 * This,
            /* [in] */ BOOL fInclude);
        
        DECLSPEC_XFGVIRT(ICEnroll4, get_IncludeSubjectKeyID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncludeSubjectKeyID )( 
            __RPC__in ICEnroll4 * This,
            /* [retval][out] */ __RPC__out BOOL *pfInclude);
        
        END_INTERFACE
    } ICEnroll4Vtbl;

    interface ICEnroll4
    {
        CONST_VTBL struct ICEnroll4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICEnroll4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICEnroll4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICEnroll4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICEnroll4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICEnroll4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICEnroll4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICEnroll4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICEnroll4_createFilePKCS10(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10(This,DNName,Usage,wszPKCS10FileName) ) 

#define ICEnroll4_acceptFilePKCS7(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7(This,wszPKCS7FileName) ) 

#define ICEnroll4_createPKCS10(This,DNName,Usage,pPKCS10)	\
    ( (This)->lpVtbl -> createPKCS10(This,DNName,Usage,pPKCS10) ) 

#define ICEnroll4_acceptPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7(This,PKCS7) ) 

#define ICEnroll4_getCertFromPKCS7(This,wszPKCS7,pbstrCert)	\
    ( (This)->lpVtbl -> getCertFromPKCS7(This,wszPKCS7,pbstrCert) ) 

#define ICEnroll4_enumProviders(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProviders(This,dwIndex,dwFlags,pbstrProvName) ) 

#define ICEnroll4_enumContainers(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainers(This,dwIndex,pbstr) ) 

#define ICEnroll4_freeRequestInfo(This,PKCS7OrPKCS10)	\
    ( (This)->lpVtbl -> freeRequestInfo(This,PKCS7OrPKCS10) ) 

#define ICEnroll4_get_MyStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_MyStoreName(This,pbstrName) ) 

#define ICEnroll4_put_MyStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_MyStoreName(This,bstrName) ) 

#define ICEnroll4_get_MyStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_MyStoreType(This,pbstrType) ) 

#define ICEnroll4_put_MyStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_MyStoreType(This,bstrType) ) 

#define ICEnroll4_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define ICEnroll4_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define ICEnroll4_get_CAStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_CAStoreName(This,pbstrName) ) 

#define ICEnroll4_put_CAStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_CAStoreName(This,bstrName) ) 

#define ICEnroll4_get_CAStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_CAStoreType(This,pbstrType) ) 

#define ICEnroll4_put_CAStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_CAStoreType(This,bstrType) ) 

#define ICEnroll4_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define ICEnroll4_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define ICEnroll4_get_RootStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RootStoreName(This,pbstrName) ) 

#define ICEnroll4_put_RootStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RootStoreName(This,bstrName) ) 

#define ICEnroll4_get_RootStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RootStoreType(This,pbstrType) ) 

#define ICEnroll4_put_RootStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RootStoreType(This,bstrType) ) 

#define ICEnroll4_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define ICEnroll4_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define ICEnroll4_get_RequestStoreName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_RequestStoreName(This,pbstrName) ) 

#define ICEnroll4_put_RequestStoreName(This,bstrName)	\
    ( (This)->lpVtbl -> put_RequestStoreName(This,bstrName) ) 

#define ICEnroll4_get_RequestStoreType(This,pbstrType)	\
    ( (This)->lpVtbl -> get_RequestStoreType(This,pbstrType) ) 

#define ICEnroll4_put_RequestStoreType(This,bstrType)	\
    ( (This)->lpVtbl -> put_RequestStoreType(This,bstrType) ) 

#define ICEnroll4_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define ICEnroll4_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define ICEnroll4_get_ContainerName(This,pbstrContainer)	\
    ( (This)->lpVtbl -> get_ContainerName(This,pbstrContainer) ) 

#define ICEnroll4_put_ContainerName(This,bstrContainer)	\
    ( (This)->lpVtbl -> put_ContainerName(This,bstrContainer) ) 

#define ICEnroll4_get_ProviderName(This,pbstrProvider)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrProvider) ) 

#define ICEnroll4_put_ProviderName(This,bstrProvider)	\
    ( (This)->lpVtbl -> put_ProviderName(This,bstrProvider) ) 

#define ICEnroll4_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define ICEnroll4_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define ICEnroll4_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define ICEnroll4_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define ICEnroll4_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define ICEnroll4_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define ICEnroll4_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll4_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define ICEnroll4_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define ICEnroll4_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define ICEnroll4_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll4_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define ICEnroll4_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define ICEnroll4_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define ICEnroll4_get_SPCFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_SPCFileName(This,pbstr) ) 

#define ICEnroll4_put_SPCFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_SPCFileName(This,bstr) ) 

#define ICEnroll4_get_PVKFileName(This,pbstr)	\
    ( (This)->lpVtbl -> get_PVKFileName(This,pbstr) ) 

#define ICEnroll4_put_PVKFileName(This,bstr)	\
    ( (This)->lpVtbl -> put_PVKFileName(This,bstr) ) 

#define ICEnroll4_get_HashAlgorithm(This,pbstr)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pbstr) ) 

#define ICEnroll4_put_HashAlgorithm(This,bstr)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,bstr) ) 


#define ICEnroll4_addCertTypeToRequest(This,CertType)	\
    ( (This)->lpVtbl -> addCertTypeToRequest(This,CertType) ) 

#define ICEnroll4_addNameValuePairToSignature(This,Name,Value)	\
    ( (This)->lpVtbl -> addNameValuePairToSignature(This,Name,Value) ) 

#define ICEnroll4_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll4_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define ICEnroll4_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define ICEnroll4_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 


#define ICEnroll4_InstallPKCS7(This,PKCS7)	\
    ( (This)->lpVtbl -> InstallPKCS7(This,PKCS7) ) 

#define ICEnroll4_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ICEnroll4_GetSupportedKeySpec(This,pdwKeySpec)	\
    ( (This)->lpVtbl -> GetSupportedKeySpec(This,pdwKeySpec) ) 

#define ICEnroll4_GetKeyLen(This,fMin,fExchange,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLen(This,fMin,fExchange,pdwKeySize) ) 

#define ICEnroll4_EnumAlgs(This,dwIndex,algClass,pdwAlgID)	\
    ( (This)->lpVtbl -> EnumAlgs(This,dwIndex,algClass,pdwAlgID) ) 

#define ICEnroll4_GetAlgName(This,algID,pbstr)	\
    ( (This)->lpVtbl -> GetAlgName(This,algID,pbstr) ) 

#define ICEnroll4_put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define ICEnroll4_get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define ICEnroll4_put_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> put_HashAlgID(This,hashAlgID) ) 

#define ICEnroll4_get_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> get_HashAlgID(This,hashAlgID) ) 

#define ICEnroll4_put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define ICEnroll4_get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define ICEnroll4_put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#define ICEnroll4_get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 


#define ICEnroll4_put_PrivateKeyArchiveCertificate(This,bstrCert)	\
    ( (This)->lpVtbl -> put_PrivateKeyArchiveCertificate(This,bstrCert) ) 

#define ICEnroll4_get_PrivateKeyArchiveCertificate(This,pbstrCert)	\
    ( (This)->lpVtbl -> get_PrivateKeyArchiveCertificate(This,pbstrCert) ) 

#define ICEnroll4_put_ThumbPrint(This,bstrThumbPrint)	\
    ( (This)->lpVtbl -> put_ThumbPrint(This,bstrThumbPrint) ) 

#define ICEnroll4_get_ThumbPrint(This,pbstrThumbPrint)	\
    ( (This)->lpVtbl -> get_ThumbPrint(This,pbstrThumbPrint) ) 

#define ICEnroll4_binaryToString(This,Flags,strBinary,pstrEncoded)	\
    ( (This)->lpVtbl -> binaryToString(This,Flags,strBinary,pstrEncoded) ) 

#define ICEnroll4_stringToBinary(This,Flags,strEncoded,pstrBinary)	\
    ( (This)->lpVtbl -> stringToBinary(This,Flags,strEncoded,pstrBinary) ) 

#define ICEnroll4_addExtensionToRequest(This,Flags,strName,strValue)	\
    ( (This)->lpVtbl -> addExtensionToRequest(This,Flags,strName,strValue) ) 

#define ICEnroll4_addAttributeToRequest(This,Flags,strName,strValue)	\
    ( (This)->lpVtbl -> addAttributeToRequest(This,Flags,strName,strValue) ) 

#define ICEnroll4_addNameValuePairToRequest(This,Flags,strName,strValue)	\
    ( (This)->lpVtbl -> addNameValuePairToRequest(This,Flags,strName,strValue) ) 

#define ICEnroll4_resetExtensions(This)	\
    ( (This)->lpVtbl -> resetExtensions(This) ) 

#define ICEnroll4_resetAttributes(This)	\
    ( (This)->lpVtbl -> resetAttributes(This) ) 

#define ICEnroll4_createRequest(This,Flags,strDNName,Usage,pstrRequest)	\
    ( (This)->lpVtbl -> createRequest(This,Flags,strDNName,Usage,pstrRequest) ) 

#define ICEnroll4_createFileRequest(This,Flags,strDNName,strUsage,strRequestFileName)	\
    ( (This)->lpVtbl -> createFileRequest(This,Flags,strDNName,strUsage,strRequestFileName) ) 

#define ICEnroll4_acceptResponse(This,strResponse)	\
    ( (This)->lpVtbl -> acceptResponse(This,strResponse) ) 

#define ICEnroll4_acceptFileResponse(This,strResponseFileName)	\
    ( (This)->lpVtbl -> acceptFileResponse(This,strResponseFileName) ) 

#define ICEnroll4_getCertFromResponse(This,strResponse,pstrCert)	\
    ( (This)->lpVtbl -> getCertFromResponse(This,strResponse,pstrCert) ) 

#define ICEnroll4_getCertFromFileResponse(This,strResponseFileName,pstrCert)	\
    ( (This)->lpVtbl -> getCertFromFileResponse(This,strResponseFileName,pstrCert) ) 

#define ICEnroll4_createPFX(This,strPassword,pstrPFX)	\
    ( (This)->lpVtbl -> createPFX(This,strPassword,pstrPFX) ) 

#define ICEnroll4_createFilePFX(This,strPassword,strPFXFileName)	\
    ( (This)->lpVtbl -> createFilePFX(This,strPassword,strPFXFileName) ) 

#define ICEnroll4_setPendingRequestInfo(This,lRequestID,strCADNS,strCAName,strFriendlyName)	\
    ( (This)->lpVtbl -> setPendingRequestInfo(This,lRequestID,strCADNS,strCAName,strFriendlyName) ) 

#define ICEnroll4_enumPendingRequest(This,lIndex,lDesiredProperty,pvarProperty)	\
    ( (This)->lpVtbl -> enumPendingRequest(This,lIndex,lDesiredProperty,pvarProperty) ) 

#define ICEnroll4_removePendingRequest(This,strThumbprint)	\
    ( (This)->lpVtbl -> removePendingRequest(This,strThumbprint) ) 

#define ICEnroll4_GetKeyLenEx(This,lSizeSpec,lKeySpec,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLenEx(This,lSizeSpec,lKeySpec,pdwKeySize) ) 

#define ICEnroll4_InstallPKCS7Ex(This,PKCS7,plCertInstalled)	\
    ( (This)->lpVtbl -> InstallPKCS7Ex(This,PKCS7,plCertInstalled) ) 

#define ICEnroll4_addCertTypeToRequestEx(This,lType,bstrOIDOrName,lMajorVersion,fMinorVersion,lMinorVersion)	\
    ( (This)->lpVtbl -> addCertTypeToRequestEx(This,lType,bstrOIDOrName,lMajorVersion,fMinorVersion,lMinorVersion) ) 

#define ICEnroll4_getProviderType(This,strProvName,plProvType)	\
    ( (This)->lpVtbl -> getProviderType(This,strProvName,plProvType) ) 

#define ICEnroll4_put_SignerCertificate(This,bstrCert)	\
    ( (This)->lpVtbl -> put_SignerCertificate(This,bstrCert) ) 

#define ICEnroll4_put_ClientId(This,lClientId)	\
    ( (This)->lpVtbl -> put_ClientId(This,lClientId) ) 

#define ICEnroll4_get_ClientId(This,plClientId)	\
    ( (This)->lpVtbl -> get_ClientId(This,plClientId) ) 

#define ICEnroll4_addBlobPropertyToCertificate(This,lPropertyId,lReserved,bstrProperty)	\
    ( (This)->lpVtbl -> addBlobPropertyToCertificate(This,lPropertyId,lReserved,bstrProperty) ) 

#define ICEnroll4_resetBlobProperties(This)	\
    ( (This)->lpVtbl -> resetBlobProperties(This) ) 

#define ICEnroll4_put_IncludeSubjectKeyID(This,fInclude)	\
    ( (This)->lpVtbl -> put_IncludeSubjectKeyID(This,fInclude) ) 

#define ICEnroll4_get_IncludeSubjectKeyID(This,pfInclude)	\
    ( (This)->lpVtbl -> get_IncludeSubjectKeyID(This,pfInclude) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICEnroll4_INTERFACE_DEFINED__ */


#ifndef __IEnroll_INTERFACE_DEFINED__
#define __IEnroll_INTERFACE_DEFINED__

/* interface IEnroll */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnroll;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("acaa7838-4585-11d1-ab57-00c04fc295e1")
    IEnroll : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE createFilePKCS10WStr( 
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [in] */ LPCWSTR wszPKCS10FileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptFilePKCS7WStr( 
            /* [in] */ LPCWSTR wszPKCS7FileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createPKCS10WStr( 
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs10Blob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptPKCS7Blob( 
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7) = 0;
        
        virtual PCCERT_CONTEXT STDMETHODCALLTYPE getCertContextFromPKCS7( 
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7) = 0;
        
        virtual HCERTSTORE STDMETHODCALLTYPE getMyStore( void) = 0;
        
        virtual HCERTSTORE STDMETHODCALLTYPE getCAStore( void) = 0;
        
        virtual HCERTSTORE STDMETHODCALLTYPE getROOTHStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumProvidersWStr( 
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [out] */ LPWSTR *pbstrProvName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumContainersWStr( 
            /* [in] */ LONG dwIndex,
            /* [out] */ LPWSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE freeRequestInfoBlob( 
            /* [in] */ CRYPT_DATA_BLOB pkcs7OrPkcs10) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreNameWStr( 
            /* [out] */ LPWSTR *szwName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreNameWStr( 
            /* [in] */ LPWSTR szwName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreTypeWStr( 
            /* [out] */ LPWSTR *szwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreTypeWStr( 
            /* [in] */ LPWSTR szwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MyStoreFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MyStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreNameWStr( 
            /* [out] */ LPWSTR *szwName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreNameWStr( 
            /* [in] */ LPWSTR szwName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreTypeWStr( 
            /* [out] */ LPWSTR *szwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreTypeWStr( 
            /* [in] */ LPWSTR szwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CAStoreFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CAStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreNameWStr( 
            /* [out] */ LPWSTR *szwName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreNameWStr( 
            /* [in] */ LPWSTR szwName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreTypeWStr( 
            /* [out] */ LPWSTR *szwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreTypeWStr( 
            /* [in] */ LPWSTR szwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootStoreFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreNameWStr( 
            /* [out] */ LPWSTR *szwName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreNameWStr( 
            /* [in] */ LPWSTR szwName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreTypeWStr( 
            /* [out] */ LPWSTR *szwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreTypeWStr( 
            /* [in] */ LPWSTR szwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RequestStoreFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RequestStoreFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ContainerNameWStr( 
            /* [out] */ LPWSTR *szwContainer) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ContainerNameWStr( 
            /* [in] */ LPWSTR szwContainer) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderNameWStr( 
            /* [out] */ LPWSTR *szwProvider) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderNameWStr( 
            /* [in] */ LPWSTR szwProvider) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderType( 
            /* [out] */ LONG *pdwType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderType( 
            /* [in] */ LONG dwType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_KeySpec( 
            /* [out] */ LONG *pdw) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_KeySpec( 
            /* [in] */ LONG dw) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProviderFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UseExistingKeySet( 
            /* [out] */ BOOL *fUseExistingKeys) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_UseExistingKeySet( 
            /* [in] */ BOOL fUseExistingKeys) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GenKeyFlags( 
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GenKeyFlags( 
            /* [in] */ LONG dwFlags) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DeleteRequestCert( 
            /* [out] */ BOOL *fDelete) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DeleteRequestCert( 
            /* [in] */ BOOL fDelete) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WriteCertToUserDS( 
            /* [out] */ BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WriteCertToUserDS( 
            /* [in] */ BOOL fBool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EnableT61DNEncoding( 
            /* [out] */ BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EnableT61DNEncoding( 
            /* [in] */ BOOL fBool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WriteCertToCSP( 
            /* [out] */ BOOL *fBool) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_WriteCertToCSP( 
            /* [in] */ BOOL fBool) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SPCFileNameWStr( 
            /* [out] */ LPWSTR *szw) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SPCFileNameWStr( 
            /* [in] */ LPWSTR szw) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PVKFileNameWStr( 
            /* [out] */ LPWSTR *szw) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PVKFileNameWStr( 
            /* [in] */ LPWSTR szw) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithmWStr( 
            /* [out] */ LPWSTR *szw) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithmWStr( 
            /* [in] */ LPWSTR szw) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RenewalCertificate( 
            /* [out] */ PCCERT_CONTEXT *ppCertContext) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RenewalCertificate( 
            /* [in] */ PCCERT_CONTEXT pCertContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddCertTypeToRequestWStr( 
            /* [in] */ LPWSTR szw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddNameValuePairToSignatureWStr( 
            /* [in] */ LPWSTR Name,
            /* [in] */ LPWSTR Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddExtensionsToRequest( 
            /* [in] */ PCERT_EXTENSIONS pCertExtensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddAuthenticatedAttributesToPKCS7Request( 
            /* [in] */ PCRYPT_ATTRIBUTES pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePKCS7RequestFromRequest( 
            /* [in] */ PCRYPT_DATA_BLOB pRequest,
            /* [in] */ PCCERT_CONTEXT pSigningCertContext,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs7Blob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnrollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnroll * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnroll * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnroll * This);
        
        DECLSPEC_XFGVIRT(IEnroll, createFilePKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10WStr )( 
            IEnroll * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [in] */ LPCWSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptFilePKCS7WStr)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7WStr )( 
            IEnroll * This,
            /* [in] */ LPCWSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, createPKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10WStr )( 
            IEnroll * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs10Blob);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptPKCS7Blob)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7Blob )( 
            IEnroll * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getCertContextFromPKCS7)
        PCCERT_CONTEXT ( STDMETHODCALLTYPE *getCertContextFromPKCS7 )( 
            IEnroll * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getMyStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getMyStore )( 
            IEnroll * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getCAStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getCAStore )( 
            IEnroll * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getROOTHStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getROOTHStore )( 
            IEnroll * This);
        
        DECLSPEC_XFGVIRT(IEnroll, enumProvidersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumProvidersWStr )( 
            IEnroll * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [out] */ LPWSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(IEnroll, enumContainersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumContainersWStr )( 
            IEnroll * This,
            /* [in] */ LONG dwIndex,
            /* [out] */ LPWSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IEnroll, freeRequestInfoBlob)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfoBlob )( 
            IEnroll * This,
            /* [in] */ CRYPT_DATA_BLOB pkcs7OrPkcs10);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreTypeWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreTypeWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreTypeWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreTypeWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreTypeWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreTypeWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreTypeWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreTypeWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ContainerNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ContainerNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            IEnroll * This,
            /* [out] */ LONG *pdwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            IEnroll * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            IEnroll * This,
            /* [out] */ LONG *pdw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            IEnroll * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            IEnroll * This,
            /* [out] */ BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            IEnroll * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            IEnroll * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            IEnroll * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            IEnroll * This,
            /* [out] */ BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            IEnroll * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            IEnroll * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            IEnroll * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            IEnroll * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            IEnroll * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            IEnroll * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            IEnroll * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_SPCFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_SPCFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_PVKFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileNameWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_PVKFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileNameWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_HashAlgorithmWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithmWStr )( 
            IEnroll * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_HashAlgorithmWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithmWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RenewalCertificate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RenewalCertificate )( 
            IEnroll * This,
            /* [out] */ PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RenewalCertificate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RenewalCertificate )( 
            IEnroll * This,
            /* [in] */ PCCERT_CONTEXT pCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, AddCertTypeToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *AddCertTypeToRequestWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, AddNameValuePairToSignatureWStr)
        HRESULT ( STDMETHODCALLTYPE *AddNameValuePairToSignatureWStr )( 
            IEnroll * This,
            /* [in] */ LPWSTR Name,
            /* [in] */ LPWSTR Value);
        
        DECLSPEC_XFGVIRT(IEnroll, AddExtensionsToRequest)
        HRESULT ( STDMETHODCALLTYPE *AddExtensionsToRequest )( 
            IEnroll * This,
            /* [in] */ PCERT_EXTENSIONS pCertExtensions);
        
        DECLSPEC_XFGVIRT(IEnroll, AddAuthenticatedAttributesToPKCS7Request)
        HRESULT ( STDMETHODCALLTYPE *AddAuthenticatedAttributesToPKCS7Request )( 
            IEnroll * This,
            /* [in] */ PCRYPT_ATTRIBUTES pAttributes);
        
        DECLSPEC_XFGVIRT(IEnroll, CreatePKCS7RequestFromRequest)
        HRESULT ( STDMETHODCALLTYPE *CreatePKCS7RequestFromRequest )( 
            IEnroll * This,
            /* [in] */ PCRYPT_DATA_BLOB pRequest,
            /* [in] */ PCCERT_CONTEXT pSigningCertContext,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs7Blob);
        
        END_INTERFACE
    } IEnrollVtbl;

    interface IEnroll
    {
        CONST_VTBL struct IEnrollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnroll_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnroll_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnroll_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnroll_createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName) ) 

#define IEnroll_acceptFilePKCS7WStr(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7WStr(This,wszPKCS7FileName) ) 

#define IEnroll_createPKCS10WStr(This,DNName,Usage,pPkcs10Blob)	\
    ( (This)->lpVtbl -> createPKCS10WStr(This,DNName,Usage,pPkcs10Blob) ) 

#define IEnroll_acceptPKCS7Blob(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7Blob(This,pBlobPKCS7) ) 

#define IEnroll_getCertContextFromPKCS7(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> getCertContextFromPKCS7(This,pBlobPKCS7) ) 

#define IEnroll_getMyStore(This)	\
    ( (This)->lpVtbl -> getMyStore(This) ) 

#define IEnroll_getCAStore(This)	\
    ( (This)->lpVtbl -> getCAStore(This) ) 

#define IEnroll_getROOTHStore(This)	\
    ( (This)->lpVtbl -> getROOTHStore(This) ) 

#define IEnroll_enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName) ) 

#define IEnroll_enumContainersWStr(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainersWStr(This,dwIndex,pbstr) ) 

#define IEnroll_freeRequestInfoBlob(This,pkcs7OrPkcs10)	\
    ( (This)->lpVtbl -> freeRequestInfoBlob(This,pkcs7OrPkcs10) ) 

#define IEnroll_get_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_MyStoreNameWStr(This,szwName) ) 

#define IEnroll_put_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_MyStoreNameWStr(This,szwName) ) 

#define IEnroll_get_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll_put_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define IEnroll_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define IEnroll_get_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_CAStoreNameWStr(This,szwName) ) 

#define IEnroll_put_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_CAStoreNameWStr(This,szwName) ) 

#define IEnroll_get_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll_put_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define IEnroll_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define IEnroll_get_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RootStoreNameWStr(This,szwName) ) 

#define IEnroll_put_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RootStoreNameWStr(This,szwName) ) 

#define IEnroll_get_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll_put_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define IEnroll_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define IEnroll_get_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll_put_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll_get_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll_put_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define IEnroll_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define IEnroll_get_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> get_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll_put_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> put_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll_get_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> get_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll_put_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> put_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define IEnroll_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define IEnroll_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define IEnroll_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define IEnroll_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define IEnroll_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define IEnroll_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define IEnroll_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define IEnroll_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define IEnroll_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define IEnroll_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define IEnroll_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define IEnroll_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define IEnroll_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define IEnroll_get_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_SPCFileNameWStr(This,szw) ) 

#define IEnroll_put_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_SPCFileNameWStr(This,szw) ) 

#define IEnroll_get_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_PVKFileNameWStr(This,szw) ) 

#define IEnroll_put_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_PVKFileNameWStr(This,szw) ) 

#define IEnroll_get_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> get_HashAlgorithmWStr(This,szw) ) 

#define IEnroll_put_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> put_HashAlgorithmWStr(This,szw) ) 

#define IEnroll_get_RenewalCertificate(This,ppCertContext)	\
    ( (This)->lpVtbl -> get_RenewalCertificate(This,ppCertContext) ) 

#define IEnroll_put_RenewalCertificate(This,pCertContext)	\
    ( (This)->lpVtbl -> put_RenewalCertificate(This,pCertContext) ) 

#define IEnroll_AddCertTypeToRequestWStr(This,szw)	\
    ( (This)->lpVtbl -> AddCertTypeToRequestWStr(This,szw) ) 

#define IEnroll_AddNameValuePairToSignatureWStr(This,Name,Value)	\
    ( (This)->lpVtbl -> AddNameValuePairToSignatureWStr(This,Name,Value) ) 

#define IEnroll_AddExtensionsToRequest(This,pCertExtensions)	\
    ( (This)->lpVtbl -> AddExtensionsToRequest(This,pCertExtensions) ) 

#define IEnroll_AddAuthenticatedAttributesToPKCS7Request(This,pAttributes)	\
    ( (This)->lpVtbl -> AddAuthenticatedAttributesToPKCS7Request(This,pAttributes) ) 

#define IEnroll_CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob)	\
    ( (This)->lpVtbl -> CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnroll_INTERFACE_DEFINED__ */


#ifndef __IEnroll2_INTERFACE_DEFINED__
#define __IEnroll2_INTERFACE_DEFINED__

/* interface IEnroll2 */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnroll2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c080e199-b7df-11d2-a421-00c04f79fe8e")
    IEnroll2 : public IEnroll
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InstallPKCS7Blob( 
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedKeySpec( 
            /* [out] */ LONG *pdwKeySpec) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLen( 
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [out] */ LONG *pdwKeySize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumAlgs( 
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [out] */ LONG *pdwAlgID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlgNameWStr( 
            /* [in] */ LONG algID,
            /* [out] */ LPWSTR *ppwsz) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ReuseHardwareKeyIfUnableToGenNew( 
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ReuseHardwareKeyIfUnableToGenNew( 
            /* [out] */ BOOL *fReuseHardwareKeyIfUnableToGenNew) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgID( 
            /* [in] */ LONG hashAlgID) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgID( 
            /* [retval][out] */ LONG *hashAlgID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHStoreMy( 
            /* [in] */ HCERTSTORE hStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHStoreCA( 
            /* [in] */ HCERTSTORE hStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHStoreROOT( 
            /* [in] */ HCERTSTORE hStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHStoreRequest( 
            /* [in] */ HCERTSTORE hStore) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LimitExchangeKeyToEncipherment( 
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LimitExchangeKeyToEncipherment( 
            /* [retval][out] */ BOOL *fLimitExchangeKeyToEncipherment) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EnableSMIMECapabilities( 
            /* [in] */ BOOL fEnableSMIMECapabilities) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EnableSMIMECapabilities( 
            /* [retval][out] */ BOOL *fEnableSMIMECapabilities) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnroll2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnroll2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, createFilePKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10WStr )( 
            IEnroll2 * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [in] */ LPCWSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptFilePKCS7WStr)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7WStr )( 
            IEnroll2 * This,
            /* [in] */ LPCWSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, createPKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10WStr )( 
            IEnroll2 * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs10Blob);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptPKCS7Blob)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7Blob )( 
            IEnroll2 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getCertContextFromPKCS7)
        PCCERT_CONTEXT ( STDMETHODCALLTYPE *getCertContextFromPKCS7 )( 
            IEnroll2 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getMyStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getMyStore )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getCAStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getCAStore )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getROOTHStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getROOTHStore )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, enumProvidersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumProvidersWStr )( 
            IEnroll2 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [out] */ LPWSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(IEnroll, enumContainersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumContainersWStr )( 
            IEnroll2 * This,
            /* [in] */ LONG dwIndex,
            /* [out] */ LPWSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IEnroll, freeRequestInfoBlob)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfoBlob )( 
            IEnroll2 * This,
            /* [in] */ CRYPT_DATA_BLOB pkcs7OrPkcs10);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreTypeWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreTypeWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreTypeWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreTypeWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreTypeWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreTypeWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreTypeWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreTypeWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ContainerNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ContainerNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            IEnroll2 * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            IEnroll2 * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            IEnroll2 * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            IEnroll2 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            IEnroll2 * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            IEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            IEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            IEnroll2 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_SPCFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_SPCFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_PVKFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileNameWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_PVKFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_HashAlgorithmWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithmWStr )( 
            IEnroll2 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_HashAlgorithmWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithmWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RenewalCertificate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RenewalCertificate )( 
            IEnroll2 * This,
            /* [out] */ PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RenewalCertificate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RenewalCertificate )( 
            IEnroll2 * This,
            /* [in] */ PCCERT_CONTEXT pCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, AddCertTypeToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *AddCertTypeToRequestWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, AddNameValuePairToSignatureWStr)
        HRESULT ( STDMETHODCALLTYPE *AddNameValuePairToSignatureWStr )( 
            IEnroll2 * This,
            /* [in] */ LPWSTR Name,
            /* [in] */ LPWSTR Value);
        
        DECLSPEC_XFGVIRT(IEnroll, AddExtensionsToRequest)
        HRESULT ( STDMETHODCALLTYPE *AddExtensionsToRequest )( 
            IEnroll2 * This,
            /* [in] */ PCERT_EXTENSIONS pCertExtensions);
        
        DECLSPEC_XFGVIRT(IEnroll, AddAuthenticatedAttributesToPKCS7Request)
        HRESULT ( STDMETHODCALLTYPE *AddAuthenticatedAttributesToPKCS7Request )( 
            IEnroll2 * This,
            /* [in] */ PCRYPT_ATTRIBUTES pAttributes);
        
        DECLSPEC_XFGVIRT(IEnroll, CreatePKCS7RequestFromRequest)
        HRESULT ( STDMETHODCALLTYPE *CreatePKCS7RequestFromRequest )( 
            IEnroll2 * This,
            /* [in] */ PCRYPT_DATA_BLOB pRequest,
            /* [in] */ PCCERT_CONTEXT pSigningCertContext,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs7Blob);
        
        DECLSPEC_XFGVIRT(IEnroll2, InstallPKCS7Blob)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7Blob )( 
            IEnroll2 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll2, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnroll2 * This);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetSupportedKeySpec)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedKeySpec )( 
            IEnroll2 * This,
            /* [out] */ LONG *pdwKeySpec);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetKeyLen)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLen )( 
            IEnroll2 * This,
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [out] */ LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(IEnroll2, EnumAlgs)
        HRESULT ( STDMETHODCALLTYPE *EnumAlgs )( 
            IEnroll2 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [out] */ LONG *pdwAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetAlgNameWStr)
        HRESULT ( STDMETHODCALLTYPE *GetAlgNameWStr )( 
            IEnroll2 * This,
            /* [in] */ LONG algID,
            /* [out] */ LPWSTR *ppwsz);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_ReuseHardwareKeyIfUnableToGenNew)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReuseHardwareKeyIfUnableToGenNew )( 
            IEnroll2 * This,
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_ReuseHardwareKeyIfUnableToGenNew)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReuseHardwareKeyIfUnableToGenNew )( 
            IEnroll2 * This,
            /* [out] */ BOOL *fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_HashAlgID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgID )( 
            IEnroll2 * This,
            /* [in] */ LONG hashAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_HashAlgID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgID )( 
            IEnroll2 * This,
            /* [retval][out] */ LONG *hashAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreMy)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreMy )( 
            IEnroll2 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreCA)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreCA )( 
            IEnroll2 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreROOT)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreROOT )( 
            IEnroll2 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreRequest)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreRequest )( 
            IEnroll2 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_LimitExchangeKeyToEncipherment)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LimitExchangeKeyToEncipherment )( 
            IEnroll2 * This,
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_LimitExchangeKeyToEncipherment)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LimitExchangeKeyToEncipherment )( 
            IEnroll2 * This,
            /* [retval][out] */ BOOL *fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_EnableSMIMECapabilities)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableSMIMECapabilities )( 
            IEnroll2 * This,
            /* [in] */ BOOL fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_EnableSMIMECapabilities)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableSMIMECapabilities )( 
            IEnroll2 * This,
            /* [retval][out] */ BOOL *fEnableSMIMECapabilities);
        
        END_INTERFACE
    } IEnroll2Vtbl;

    interface IEnroll2
    {
        CONST_VTBL struct IEnroll2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnroll2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnroll2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnroll2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnroll2_createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName) ) 

#define IEnroll2_acceptFilePKCS7WStr(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7WStr(This,wszPKCS7FileName) ) 

#define IEnroll2_createPKCS10WStr(This,DNName,Usage,pPkcs10Blob)	\
    ( (This)->lpVtbl -> createPKCS10WStr(This,DNName,Usage,pPkcs10Blob) ) 

#define IEnroll2_acceptPKCS7Blob(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7Blob(This,pBlobPKCS7) ) 

#define IEnroll2_getCertContextFromPKCS7(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> getCertContextFromPKCS7(This,pBlobPKCS7) ) 

#define IEnroll2_getMyStore(This)	\
    ( (This)->lpVtbl -> getMyStore(This) ) 

#define IEnroll2_getCAStore(This)	\
    ( (This)->lpVtbl -> getCAStore(This) ) 

#define IEnroll2_getROOTHStore(This)	\
    ( (This)->lpVtbl -> getROOTHStore(This) ) 

#define IEnroll2_enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName) ) 

#define IEnroll2_enumContainersWStr(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainersWStr(This,dwIndex,pbstr) ) 

#define IEnroll2_freeRequestInfoBlob(This,pkcs7OrPkcs10)	\
    ( (This)->lpVtbl -> freeRequestInfoBlob(This,pkcs7OrPkcs10) ) 

#define IEnroll2_get_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_MyStoreNameWStr(This,szwName) ) 

#define IEnroll2_put_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_MyStoreNameWStr(This,szwName) ) 

#define IEnroll2_get_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll2_put_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll2_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define IEnroll2_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define IEnroll2_get_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_CAStoreNameWStr(This,szwName) ) 

#define IEnroll2_put_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_CAStoreNameWStr(This,szwName) ) 

#define IEnroll2_get_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll2_put_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll2_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define IEnroll2_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define IEnroll2_get_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RootStoreNameWStr(This,szwName) ) 

#define IEnroll2_put_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RootStoreNameWStr(This,szwName) ) 

#define IEnroll2_get_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll2_put_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll2_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define IEnroll2_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define IEnroll2_get_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll2_put_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll2_get_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll2_put_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll2_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define IEnroll2_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define IEnroll2_get_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> get_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll2_put_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> put_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll2_get_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> get_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll2_put_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> put_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll2_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define IEnroll2_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define IEnroll2_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define IEnroll2_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define IEnroll2_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define IEnroll2_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define IEnroll2_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll2_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll2_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define IEnroll2_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define IEnroll2_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define IEnroll2_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define IEnroll2_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define IEnroll2_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define IEnroll2_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll2_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll2_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define IEnroll2_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define IEnroll2_get_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_SPCFileNameWStr(This,szw) ) 

#define IEnroll2_put_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_SPCFileNameWStr(This,szw) ) 

#define IEnroll2_get_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_PVKFileNameWStr(This,szw) ) 

#define IEnroll2_put_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_PVKFileNameWStr(This,szw) ) 

#define IEnroll2_get_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> get_HashAlgorithmWStr(This,szw) ) 

#define IEnroll2_put_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> put_HashAlgorithmWStr(This,szw) ) 

#define IEnroll2_get_RenewalCertificate(This,ppCertContext)	\
    ( (This)->lpVtbl -> get_RenewalCertificate(This,ppCertContext) ) 

#define IEnroll2_put_RenewalCertificate(This,pCertContext)	\
    ( (This)->lpVtbl -> put_RenewalCertificate(This,pCertContext) ) 

#define IEnroll2_AddCertTypeToRequestWStr(This,szw)	\
    ( (This)->lpVtbl -> AddCertTypeToRequestWStr(This,szw) ) 

#define IEnroll2_AddNameValuePairToSignatureWStr(This,Name,Value)	\
    ( (This)->lpVtbl -> AddNameValuePairToSignatureWStr(This,Name,Value) ) 

#define IEnroll2_AddExtensionsToRequest(This,pCertExtensions)	\
    ( (This)->lpVtbl -> AddExtensionsToRequest(This,pCertExtensions) ) 

#define IEnroll2_AddAuthenticatedAttributesToPKCS7Request(This,pAttributes)	\
    ( (This)->lpVtbl -> AddAuthenticatedAttributesToPKCS7Request(This,pAttributes) ) 

#define IEnroll2_CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob)	\
    ( (This)->lpVtbl -> CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob) ) 


#define IEnroll2_InstallPKCS7Blob(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> InstallPKCS7Blob(This,pBlobPKCS7) ) 

#define IEnroll2_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnroll2_GetSupportedKeySpec(This,pdwKeySpec)	\
    ( (This)->lpVtbl -> GetSupportedKeySpec(This,pdwKeySpec) ) 

#define IEnroll2_GetKeyLen(This,fMin,fExchange,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLen(This,fMin,fExchange,pdwKeySize) ) 

#define IEnroll2_EnumAlgs(This,dwIndex,algClass,pdwAlgID)	\
    ( (This)->lpVtbl -> EnumAlgs(This,dwIndex,algClass,pdwAlgID) ) 

#define IEnroll2_GetAlgNameWStr(This,algID,ppwsz)	\
    ( (This)->lpVtbl -> GetAlgNameWStr(This,algID,ppwsz) ) 

#define IEnroll2_put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define IEnroll2_get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define IEnroll2_put_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> put_HashAlgID(This,hashAlgID) ) 

#define IEnroll2_get_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> get_HashAlgID(This,hashAlgID) ) 

#define IEnroll2_SetHStoreMy(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreMy(This,hStore) ) 

#define IEnroll2_SetHStoreCA(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreCA(This,hStore) ) 

#define IEnroll2_SetHStoreROOT(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreROOT(This,hStore) ) 

#define IEnroll2_SetHStoreRequest(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreRequest(This,hStore) ) 

#define IEnroll2_put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define IEnroll2_get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define IEnroll2_put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#define IEnroll2_get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnroll2_INTERFACE_DEFINED__ */


#ifndef __IEnroll4_INTERFACE_DEFINED__
#define __IEnroll4_INTERFACE_DEFINED__

/* interface IEnroll4 */
/* [unique][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IEnroll4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f8053fe5-78f4-448f-a0db-41d61b73446b")
    IEnroll4 : public IEnroll2
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ThumbPrintWStr( 
            /* [in] */ CRYPT_DATA_BLOB thumbPrintBlob) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ThumbPrintWStr( 
            /* [retval][out] */ PCRYPT_DATA_BLOB thumbPrintBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateKeyArchiveCertificate( 
            /* [in] */ PCCERT_CONTEXT pPrivateKeyArchiveCert) = 0;
        
        virtual PCCERT_CONTEXT STDMETHODCALLTYPE GetPrivateKeyArchiveCertificate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE binaryBlobToString( 
            /* [in] */ LONG Flags,
            /* [in] */ PCRYPT_DATA_BLOB pblobBinary,
            /* [out] */ LPWSTR *ppwszString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE stringToBinaryBlob( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszString,
            /* [out] */ PCRYPT_DATA_BLOB pblobBinary,
            /* [out] */ LONG *pdwSkip,
            /* [out] */ LONG *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addExtensionToRequestWStr( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ PCRYPT_DATA_BLOB pblobValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addAttributeToRequestWStr( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ PCRYPT_DATA_BLOB pblobValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addNameValuePairToRequestWStr( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ LPCWSTR pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resetExtensions( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resetAttributes( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createRequestWStr( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszDNName,
            /* [in] */ LPCWSTR pwszUsage,
            /* [out] */ PCRYPT_DATA_BLOB pblobRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createFileRequestWStr( 
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszDNName,
            /* [in] */ LPCWSTR pwszUsage,
            /* [in] */ LPCWSTR pwszRequestFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptResponseBlob( 
            /* [in] */ PCRYPT_DATA_BLOB pblobResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE acceptFileResponseWStr( 
            /* [in] */ LPCWSTR pwszResponseFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getCertContextFromResponseBlob( 
            /* [in] */ PCRYPT_DATA_BLOB pblobResponse,
            /* [out] */ PCCERT_CONTEXT *ppCertContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getCertContextFromFileResponseWStr( 
            /* [in] */ LPCWSTR pwszResponseFileName,
            /* [out] */ PCCERT_CONTEXT *ppCertContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createPFXWStr( 
            /* [in] */ LPCWSTR pwszPassword,
            /* [out] */ PCRYPT_DATA_BLOB pblobPFX) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE createFilePFXWStr( 
            /* [in] */ LPCWSTR pwszPassword,
            /* [in] */ LPCWSTR pwszPFXFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setPendingRequestInfoWStr( 
            /* [in] */ LONG lRequestID,
            /* [in] */ LPCWSTR pwszCADNS,
            /* [in] */ LPCWSTR pwszCAName,
            /* [in] */ LPCWSTR pwszFriendlyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE enumPendingRequestWStr( 
            /* [in] */ LONG lIndex,
            /* [in] */ LONG lDesiredProperty,
            /* [out] */ LPVOID ppProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE removePendingRequestWStr( 
            /* [in] */ CRYPT_DATA_BLOB thumbPrintBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLenEx( 
            /* [in] */ LONG lSizeSpec,
            /* [in] */ LONG lKeySpec,
            /* [retval][out] */ LONG *pdwKeySize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallPKCS7BlobEx( 
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7,
            /* [retval][out] */ LONG *plCertInstalled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddCertTypeToRequestWStrEx( 
            /* [in] */ LONG lType,
            /* [in] */ LPCWSTR pwszOIDOrName,
            /* [in] */ LONG lMajorVersion,
            /* [in] */ BOOL fMinorVersion,
            /* [in] */ LONG lMinorVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getProviderTypeWStr( 
            /* [in] */ LPCWSTR pwszProvName,
            /* [retval][out] */ LONG *plProvType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addBlobPropertyToCertificateWStr( 
            /* [in] */ LONG lPropertyId,
            /* [in] */ LONG lReserved,
            /* [in] */ PCRYPT_DATA_BLOB pBlobProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignerCertificate( 
            /* [in] */ PCCERT_CONTEXT pSignerCert) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClientId( 
            /* [in] */ LONG lClientId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClientId( 
            /* [retval][out] */ LONG *plClientId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IncludeSubjectKeyID( 
            /* [in] */ BOOL fInclude) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IncludeSubjectKeyID( 
            /* [retval][out] */ BOOL *pfInclude) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnroll4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnroll4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, createFilePKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createFilePKCS10WStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [in] */ LPCWSTR wszPKCS10FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptFilePKCS7WStr)
        HRESULT ( STDMETHODCALLTYPE *acceptFilePKCS7WStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR wszPKCS7FileName);
        
        DECLSPEC_XFGVIRT(IEnroll, createPKCS10WStr)
        HRESULT ( STDMETHODCALLTYPE *createPKCS10WStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR DNName,
            /* [in] */ LPCWSTR Usage,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs10Blob);
        
        DECLSPEC_XFGVIRT(IEnroll, acceptPKCS7Blob)
        HRESULT ( STDMETHODCALLTYPE *acceptPKCS7Blob )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getCertContextFromPKCS7)
        PCCERT_CONTEXT ( STDMETHODCALLTYPE *getCertContextFromPKCS7 )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll, getMyStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getMyStore )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getCAStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getCAStore )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, getROOTHStore)
        HCERTSTORE ( STDMETHODCALLTYPE *getROOTHStore )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll, enumProvidersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumProvidersWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG dwFlags,
            /* [out] */ LPWSTR *pbstrProvName);
        
        DECLSPEC_XFGVIRT(IEnroll, enumContainersWStr)
        HRESULT ( STDMETHODCALLTYPE *enumContainersWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [out] */ LPWSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IEnroll, freeRequestInfoBlob)
        HRESULT ( STDMETHODCALLTYPE *freeRequestInfoBlob )( 
            IEnroll4 * This,
            /* [in] */ CRYPT_DATA_BLOB pkcs7OrPkcs10);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreTypeWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreTypeWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_MyStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MyStoreFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_MyStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MyStoreFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreTypeWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreTypeWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_CAStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAStoreFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_CAStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAStoreFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreTypeWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreTypeWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RootStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootStoreFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RootStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootStoreFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwName);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreTypeWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreTypeWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreTypeWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreTypeWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RequestStoreFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestStoreFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RequestStoreFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequestStoreFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ContainerNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ContainerNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwContainer);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szwProvider);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderType )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwType);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderType )( 
            IEnroll4 * This,
            /* [in] */ LONG dwType);
        
        DECLSPEC_XFGVIRT(IEnroll, get_KeySpec)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_KeySpec)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_KeySpec )( 
            IEnroll4 * This,
            /* [in] */ LONG dw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_ProviderFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_ProviderFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_UseExistingKeySet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseExistingKeySet )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, put_UseExistingKeySet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseExistingKeySet )( 
            IEnroll4 * This,
            /* [in] */ BOOL fUseExistingKeys);
        
        DECLSPEC_XFGVIRT(IEnroll, get_GenKeyFlags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenKeyFlags )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, put_GenKeyFlags)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenKeyFlags )( 
            IEnroll4 * This,
            /* [in] */ LONG dwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll, get_DeleteRequestCert)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRequestCert )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, put_DeleteRequestCert)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRequestCert )( 
            IEnroll4 * This,
            /* [in] */ BOOL fDelete);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToUserDS)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToUserDS )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToUserDS)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToUserDS )( 
            IEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_EnableT61DNEncoding)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableT61DNEncoding )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_EnableT61DNEncoding)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableT61DNEncoding )( 
            IEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_WriteCertToCSP)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WriteCertToCSP )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, put_WriteCertToCSP)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_WriteCertToCSP )( 
            IEnroll4 * This,
            /* [in] */ BOOL fBool);
        
        DECLSPEC_XFGVIRT(IEnroll, get_SPCFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SPCFileNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_SPCFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SPCFileNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_PVKFileNameWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PVKFileNameWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_PVKFileNameWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PVKFileNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_HashAlgorithmWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithmWStr )( 
            IEnroll4 * This,
            /* [out] */ LPWSTR *szw);
        
        DECLSPEC_XFGVIRT(IEnroll, put_HashAlgorithmWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithmWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, get_RenewalCertificate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RenewalCertificate )( 
            IEnroll4 * This,
            /* [out] */ PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, put_RenewalCertificate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RenewalCertificate )( 
            IEnroll4 * This,
            /* [in] */ PCCERT_CONTEXT pCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll, AddCertTypeToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *AddCertTypeToRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR szw);
        
        DECLSPEC_XFGVIRT(IEnroll, AddNameValuePairToSignatureWStr)
        HRESULT ( STDMETHODCALLTYPE *AddNameValuePairToSignatureWStr )( 
            IEnroll4 * This,
            /* [in] */ LPWSTR Name,
            /* [in] */ LPWSTR Value);
        
        DECLSPEC_XFGVIRT(IEnroll, AddExtensionsToRequest)
        HRESULT ( STDMETHODCALLTYPE *AddExtensionsToRequest )( 
            IEnroll4 * This,
            /* [in] */ PCERT_EXTENSIONS pCertExtensions);
        
        DECLSPEC_XFGVIRT(IEnroll, AddAuthenticatedAttributesToPKCS7Request)
        HRESULT ( STDMETHODCALLTYPE *AddAuthenticatedAttributesToPKCS7Request )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_ATTRIBUTES pAttributes);
        
        DECLSPEC_XFGVIRT(IEnroll, CreatePKCS7RequestFromRequest)
        HRESULT ( STDMETHODCALLTYPE *CreatePKCS7RequestFromRequest )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pRequest,
            /* [in] */ PCCERT_CONTEXT pSigningCertContext,
            /* [out] */ PCRYPT_DATA_BLOB pPkcs7Blob);
        
        DECLSPEC_XFGVIRT(IEnroll2, InstallPKCS7Blob)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7Blob )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7);
        
        DECLSPEC_XFGVIRT(IEnroll2, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetSupportedKeySpec)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedKeySpec )( 
            IEnroll4 * This,
            /* [out] */ LONG *pdwKeySpec);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetKeyLen)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLen )( 
            IEnroll4 * This,
            /* [in] */ BOOL fMin,
            /* [in] */ BOOL fExchange,
            /* [out] */ LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(IEnroll2, EnumAlgs)
        HRESULT ( STDMETHODCALLTYPE *EnumAlgs )( 
            IEnroll4 * This,
            /* [in] */ LONG dwIndex,
            /* [in] */ LONG algClass,
            /* [out] */ LONG *pdwAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, GetAlgNameWStr)
        HRESULT ( STDMETHODCALLTYPE *GetAlgNameWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG algID,
            /* [out] */ LPWSTR *ppwsz);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_ReuseHardwareKeyIfUnableToGenNew)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReuseHardwareKeyIfUnableToGenNew )( 
            IEnroll4 * This,
            /* [in] */ BOOL fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_ReuseHardwareKeyIfUnableToGenNew)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReuseHardwareKeyIfUnableToGenNew )( 
            IEnroll4 * This,
            /* [out] */ BOOL *fReuseHardwareKeyIfUnableToGenNew);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_HashAlgID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgID )( 
            IEnroll4 * This,
            /* [in] */ LONG hashAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_HashAlgID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgID )( 
            IEnroll4 * This,
            /* [retval][out] */ LONG *hashAlgID);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreMy)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreMy )( 
            IEnroll4 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreCA)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreCA )( 
            IEnroll4 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreROOT)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreROOT )( 
            IEnroll4 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, SetHStoreRequest)
        HRESULT ( STDMETHODCALLTYPE *SetHStoreRequest )( 
            IEnroll4 * This,
            /* [in] */ HCERTSTORE hStore);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_LimitExchangeKeyToEncipherment)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LimitExchangeKeyToEncipherment )( 
            IEnroll4 * This,
            /* [in] */ BOOL fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_LimitExchangeKeyToEncipherment)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LimitExchangeKeyToEncipherment )( 
            IEnroll4 * This,
            /* [retval][out] */ BOOL *fLimitExchangeKeyToEncipherment);
        
        DECLSPEC_XFGVIRT(IEnroll2, put_EnableSMIMECapabilities)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableSMIMECapabilities )( 
            IEnroll4 * This,
            /* [in] */ BOOL fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(IEnroll2, get_EnableSMIMECapabilities)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableSMIMECapabilities )( 
            IEnroll4 * This,
            /* [retval][out] */ BOOL *fEnableSMIMECapabilities);
        
        DECLSPEC_XFGVIRT(IEnroll4, put_ThumbPrintWStr)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ThumbPrintWStr )( 
            IEnroll4 * This,
            /* [in] */ CRYPT_DATA_BLOB thumbPrintBlob);
        
        DECLSPEC_XFGVIRT(IEnroll4, get_ThumbPrintWStr)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ThumbPrintWStr )( 
            IEnroll4 * This,
            /* [retval][out] */ PCRYPT_DATA_BLOB thumbPrintBlob);
        
        DECLSPEC_XFGVIRT(IEnroll4, SetPrivateKeyArchiveCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetPrivateKeyArchiveCertificate )( 
            IEnroll4 * This,
            /* [in] */ PCCERT_CONTEXT pPrivateKeyArchiveCert);
        
        DECLSPEC_XFGVIRT(IEnroll4, GetPrivateKeyArchiveCertificate)
        PCCERT_CONTEXT ( STDMETHODCALLTYPE *GetPrivateKeyArchiveCertificate )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll4, binaryBlobToString)
        HRESULT ( STDMETHODCALLTYPE *binaryBlobToString )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ PCRYPT_DATA_BLOB pblobBinary,
            /* [out] */ LPWSTR *ppwszString);
        
        DECLSPEC_XFGVIRT(IEnroll4, stringToBinaryBlob)
        HRESULT ( STDMETHODCALLTYPE *stringToBinaryBlob )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszString,
            /* [out] */ PCRYPT_DATA_BLOB pblobBinary,
            /* [out] */ LONG *pdwSkip,
            /* [out] */ LONG *pdwFlags);
        
        DECLSPEC_XFGVIRT(IEnroll4, addExtensionToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *addExtensionToRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ PCRYPT_DATA_BLOB pblobValue);
        
        DECLSPEC_XFGVIRT(IEnroll4, addAttributeToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *addAttributeToRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ PCRYPT_DATA_BLOB pblobValue);
        
        DECLSPEC_XFGVIRT(IEnroll4, addNameValuePairToRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *addNameValuePairToRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszName,
            /* [in] */ LPCWSTR pwszValue);
        
        DECLSPEC_XFGVIRT(IEnroll4, resetExtensions)
        HRESULT ( STDMETHODCALLTYPE *resetExtensions )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll4, resetAttributes)
        HRESULT ( STDMETHODCALLTYPE *resetAttributes )( 
            IEnroll4 * This);
        
        DECLSPEC_XFGVIRT(IEnroll4, createRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *createRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszDNName,
            /* [in] */ LPCWSTR pwszUsage,
            /* [out] */ PCRYPT_DATA_BLOB pblobRequest);
        
        DECLSPEC_XFGVIRT(IEnroll4, createFileRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *createFileRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG Flags,
            /* [in] */ LPCWSTR pwszDNName,
            /* [in] */ LPCWSTR pwszUsage,
            /* [in] */ LPCWSTR pwszRequestFileName);
        
        DECLSPEC_XFGVIRT(IEnroll4, acceptResponseBlob)
        HRESULT ( STDMETHODCALLTYPE *acceptResponseBlob )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pblobResponse);
        
        DECLSPEC_XFGVIRT(IEnroll4, acceptFileResponseWStr)
        HRESULT ( STDMETHODCALLTYPE *acceptFileResponseWStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR pwszResponseFileName);
        
        DECLSPEC_XFGVIRT(IEnroll4, getCertContextFromResponseBlob)
        HRESULT ( STDMETHODCALLTYPE *getCertContextFromResponseBlob )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pblobResponse,
            /* [out] */ PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll4, getCertContextFromFileResponseWStr)
        HRESULT ( STDMETHODCALLTYPE *getCertContextFromFileResponseWStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR pwszResponseFileName,
            /* [out] */ PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IEnroll4, createPFXWStr)
        HRESULT ( STDMETHODCALLTYPE *createPFXWStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR pwszPassword,
            /* [out] */ PCRYPT_DATA_BLOB pblobPFX);
        
        DECLSPEC_XFGVIRT(IEnroll4, createFilePFXWStr)
        HRESULT ( STDMETHODCALLTYPE *createFilePFXWStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR pwszPassword,
            /* [in] */ LPCWSTR pwszPFXFileName);
        
        DECLSPEC_XFGVIRT(IEnroll4, setPendingRequestInfoWStr)
        HRESULT ( STDMETHODCALLTYPE *setPendingRequestInfoWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG lRequestID,
            /* [in] */ LPCWSTR pwszCADNS,
            /* [in] */ LPCWSTR pwszCAName,
            /* [in] */ LPCWSTR pwszFriendlyName);
        
        DECLSPEC_XFGVIRT(IEnroll4, enumPendingRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *enumPendingRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG lIndex,
            /* [in] */ LONG lDesiredProperty,
            /* [out] */ LPVOID ppProperty);
        
        DECLSPEC_XFGVIRT(IEnroll4, removePendingRequestWStr)
        HRESULT ( STDMETHODCALLTYPE *removePendingRequestWStr )( 
            IEnroll4 * This,
            /* [in] */ CRYPT_DATA_BLOB thumbPrintBlob);
        
        DECLSPEC_XFGVIRT(IEnroll4, GetKeyLenEx)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLenEx )( 
            IEnroll4 * This,
            /* [in] */ LONG lSizeSpec,
            /* [in] */ LONG lKeySpec,
            /* [retval][out] */ LONG *pdwKeySize);
        
        DECLSPEC_XFGVIRT(IEnroll4, InstallPKCS7BlobEx)
        HRESULT ( STDMETHODCALLTYPE *InstallPKCS7BlobEx )( 
            IEnroll4 * This,
            /* [in] */ PCRYPT_DATA_BLOB pBlobPKCS7,
            /* [retval][out] */ LONG *plCertInstalled);
        
        DECLSPEC_XFGVIRT(IEnroll4, AddCertTypeToRequestWStrEx)
        HRESULT ( STDMETHODCALLTYPE *AddCertTypeToRequestWStrEx )( 
            IEnroll4 * This,
            /* [in] */ LONG lType,
            /* [in] */ LPCWSTR pwszOIDOrName,
            /* [in] */ LONG lMajorVersion,
            /* [in] */ BOOL fMinorVersion,
            /* [in] */ LONG lMinorVersion);
        
        DECLSPEC_XFGVIRT(IEnroll4, getProviderTypeWStr)
        HRESULT ( STDMETHODCALLTYPE *getProviderTypeWStr )( 
            IEnroll4 * This,
            /* [in] */ LPCWSTR pwszProvName,
            /* [retval][out] */ LONG *plProvType);
        
        DECLSPEC_XFGVIRT(IEnroll4, addBlobPropertyToCertificateWStr)
        HRESULT ( STDMETHODCALLTYPE *addBlobPropertyToCertificateWStr )( 
            IEnroll4 * This,
            /* [in] */ LONG lPropertyId,
            /* [in] */ LONG lReserved,
            /* [in] */ PCRYPT_DATA_BLOB pBlobProperty);
        
        DECLSPEC_XFGVIRT(IEnroll4, SetSignerCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetSignerCertificate )( 
            IEnroll4 * This,
            /* [in] */ PCCERT_CONTEXT pSignerCert);
        
        DECLSPEC_XFGVIRT(IEnroll4, put_ClientId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientId )( 
            IEnroll4 * This,
            /* [in] */ LONG lClientId);
        
        DECLSPEC_XFGVIRT(IEnroll4, get_ClientId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientId )( 
            IEnroll4 * This,
            /* [retval][out] */ LONG *plClientId);
        
        DECLSPEC_XFGVIRT(IEnroll4, put_IncludeSubjectKeyID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncludeSubjectKeyID )( 
            IEnroll4 * This,
            /* [in] */ BOOL fInclude);
        
        DECLSPEC_XFGVIRT(IEnroll4, get_IncludeSubjectKeyID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncludeSubjectKeyID )( 
            IEnroll4 * This,
            /* [retval][out] */ BOOL *pfInclude);
        
        END_INTERFACE
    } IEnroll4Vtbl;

    interface IEnroll4
    {
        CONST_VTBL struct IEnroll4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnroll4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnroll4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnroll4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnroll4_createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName)	\
    ( (This)->lpVtbl -> createFilePKCS10WStr(This,DNName,Usage,wszPKCS10FileName) ) 

#define IEnroll4_acceptFilePKCS7WStr(This,wszPKCS7FileName)	\
    ( (This)->lpVtbl -> acceptFilePKCS7WStr(This,wszPKCS7FileName) ) 

#define IEnroll4_createPKCS10WStr(This,DNName,Usage,pPkcs10Blob)	\
    ( (This)->lpVtbl -> createPKCS10WStr(This,DNName,Usage,pPkcs10Blob) ) 

#define IEnroll4_acceptPKCS7Blob(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> acceptPKCS7Blob(This,pBlobPKCS7) ) 

#define IEnroll4_getCertContextFromPKCS7(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> getCertContextFromPKCS7(This,pBlobPKCS7) ) 

#define IEnroll4_getMyStore(This)	\
    ( (This)->lpVtbl -> getMyStore(This) ) 

#define IEnroll4_getCAStore(This)	\
    ( (This)->lpVtbl -> getCAStore(This) ) 

#define IEnroll4_getROOTHStore(This)	\
    ( (This)->lpVtbl -> getROOTHStore(This) ) 

#define IEnroll4_enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName)	\
    ( (This)->lpVtbl -> enumProvidersWStr(This,dwIndex,dwFlags,pbstrProvName) ) 

#define IEnroll4_enumContainersWStr(This,dwIndex,pbstr)	\
    ( (This)->lpVtbl -> enumContainersWStr(This,dwIndex,pbstr) ) 

#define IEnroll4_freeRequestInfoBlob(This,pkcs7OrPkcs10)	\
    ( (This)->lpVtbl -> freeRequestInfoBlob(This,pkcs7OrPkcs10) ) 

#define IEnroll4_get_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_MyStoreNameWStr(This,szwName) ) 

#define IEnroll4_put_MyStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_MyStoreNameWStr(This,szwName) ) 

#define IEnroll4_get_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll4_put_MyStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_MyStoreTypeWStr(This,szwType) ) 

#define IEnroll4_get_MyStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_MyStoreFlags(This,pdwFlags) ) 

#define IEnroll4_put_MyStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_MyStoreFlags(This,dwFlags) ) 

#define IEnroll4_get_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_CAStoreNameWStr(This,szwName) ) 

#define IEnroll4_put_CAStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_CAStoreNameWStr(This,szwName) ) 

#define IEnroll4_get_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll4_put_CAStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_CAStoreTypeWStr(This,szwType) ) 

#define IEnroll4_get_CAStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_CAStoreFlags(This,pdwFlags) ) 

#define IEnroll4_put_CAStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_CAStoreFlags(This,dwFlags) ) 

#define IEnroll4_get_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RootStoreNameWStr(This,szwName) ) 

#define IEnroll4_put_RootStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RootStoreNameWStr(This,szwName) ) 

#define IEnroll4_get_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll4_put_RootStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RootStoreTypeWStr(This,szwType) ) 

#define IEnroll4_get_RootStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RootStoreFlags(This,pdwFlags) ) 

#define IEnroll4_put_RootStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RootStoreFlags(This,dwFlags) ) 

#define IEnroll4_get_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> get_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll4_put_RequestStoreNameWStr(This,szwName)	\
    ( (This)->lpVtbl -> put_RequestStoreNameWStr(This,szwName) ) 

#define IEnroll4_get_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> get_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll4_put_RequestStoreTypeWStr(This,szwType)	\
    ( (This)->lpVtbl -> put_RequestStoreTypeWStr(This,szwType) ) 

#define IEnroll4_get_RequestStoreFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_RequestStoreFlags(This,pdwFlags) ) 

#define IEnroll4_put_RequestStoreFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_RequestStoreFlags(This,dwFlags) ) 

#define IEnroll4_get_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> get_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll4_put_ContainerNameWStr(This,szwContainer)	\
    ( (This)->lpVtbl -> put_ContainerNameWStr(This,szwContainer) ) 

#define IEnroll4_get_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> get_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll4_put_ProviderNameWStr(This,szwProvider)	\
    ( (This)->lpVtbl -> put_ProviderNameWStr(This,szwProvider) ) 

#define IEnroll4_get_ProviderType(This,pdwType)	\
    ( (This)->lpVtbl -> get_ProviderType(This,pdwType) ) 

#define IEnroll4_put_ProviderType(This,dwType)	\
    ( (This)->lpVtbl -> put_ProviderType(This,dwType) ) 

#define IEnroll4_get_KeySpec(This,pdw)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pdw) ) 

#define IEnroll4_put_KeySpec(This,dw)	\
    ( (This)->lpVtbl -> put_KeySpec(This,dw) ) 

#define IEnroll4_get_ProviderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_ProviderFlags(This,pdwFlags) ) 

#define IEnroll4_put_ProviderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_ProviderFlags(This,dwFlags) ) 

#define IEnroll4_get_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> get_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll4_put_UseExistingKeySet(This,fUseExistingKeys)	\
    ( (This)->lpVtbl -> put_UseExistingKeySet(This,fUseExistingKeys) ) 

#define IEnroll4_get_GenKeyFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_GenKeyFlags(This,pdwFlags) ) 

#define IEnroll4_put_GenKeyFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_GenKeyFlags(This,dwFlags) ) 

#define IEnroll4_get_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> get_DeleteRequestCert(This,fDelete) ) 

#define IEnroll4_put_DeleteRequestCert(This,fDelete)	\
    ( (This)->lpVtbl -> put_DeleteRequestCert(This,fDelete) ) 

#define IEnroll4_get_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToUserDS(This,fBool) ) 

#define IEnroll4_put_WriteCertToUserDS(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToUserDS(This,fBool) ) 

#define IEnroll4_get_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> get_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll4_put_EnableT61DNEncoding(This,fBool)	\
    ( (This)->lpVtbl -> put_EnableT61DNEncoding(This,fBool) ) 

#define IEnroll4_get_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> get_WriteCertToCSP(This,fBool) ) 

#define IEnroll4_put_WriteCertToCSP(This,fBool)	\
    ( (This)->lpVtbl -> put_WriteCertToCSP(This,fBool) ) 

#define IEnroll4_get_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_SPCFileNameWStr(This,szw) ) 

#define IEnroll4_put_SPCFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_SPCFileNameWStr(This,szw) ) 

#define IEnroll4_get_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> get_PVKFileNameWStr(This,szw) ) 

#define IEnroll4_put_PVKFileNameWStr(This,szw)	\
    ( (This)->lpVtbl -> put_PVKFileNameWStr(This,szw) ) 

#define IEnroll4_get_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> get_HashAlgorithmWStr(This,szw) ) 

#define IEnroll4_put_HashAlgorithmWStr(This,szw)	\
    ( (This)->lpVtbl -> put_HashAlgorithmWStr(This,szw) ) 

#define IEnroll4_get_RenewalCertificate(This,ppCertContext)	\
    ( (This)->lpVtbl -> get_RenewalCertificate(This,ppCertContext) ) 

#define IEnroll4_put_RenewalCertificate(This,pCertContext)	\
    ( (This)->lpVtbl -> put_RenewalCertificate(This,pCertContext) ) 

#define IEnroll4_AddCertTypeToRequestWStr(This,szw)	\
    ( (This)->lpVtbl -> AddCertTypeToRequestWStr(This,szw) ) 

#define IEnroll4_AddNameValuePairToSignatureWStr(This,Name,Value)	\
    ( (This)->lpVtbl -> AddNameValuePairToSignatureWStr(This,Name,Value) ) 

#define IEnroll4_AddExtensionsToRequest(This,pCertExtensions)	\
    ( (This)->lpVtbl -> AddExtensionsToRequest(This,pCertExtensions) ) 

#define IEnroll4_AddAuthenticatedAttributesToPKCS7Request(This,pAttributes)	\
    ( (This)->lpVtbl -> AddAuthenticatedAttributesToPKCS7Request(This,pAttributes) ) 

#define IEnroll4_CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob)	\
    ( (This)->lpVtbl -> CreatePKCS7RequestFromRequest(This,pRequest,pSigningCertContext,pPkcs7Blob) ) 


#define IEnroll4_InstallPKCS7Blob(This,pBlobPKCS7)	\
    ( (This)->lpVtbl -> InstallPKCS7Blob(This,pBlobPKCS7) ) 

#define IEnroll4_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnroll4_GetSupportedKeySpec(This,pdwKeySpec)	\
    ( (This)->lpVtbl -> GetSupportedKeySpec(This,pdwKeySpec) ) 

#define IEnroll4_GetKeyLen(This,fMin,fExchange,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLen(This,fMin,fExchange,pdwKeySize) ) 

#define IEnroll4_EnumAlgs(This,dwIndex,algClass,pdwAlgID)	\
    ( (This)->lpVtbl -> EnumAlgs(This,dwIndex,algClass,pdwAlgID) ) 

#define IEnroll4_GetAlgNameWStr(This,algID,ppwsz)	\
    ( (This)->lpVtbl -> GetAlgNameWStr(This,algID,ppwsz) ) 

#define IEnroll4_put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> put_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define IEnroll4_get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew)	\
    ( (This)->lpVtbl -> get_ReuseHardwareKeyIfUnableToGenNew(This,fReuseHardwareKeyIfUnableToGenNew) ) 

#define IEnroll4_put_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> put_HashAlgID(This,hashAlgID) ) 

#define IEnroll4_get_HashAlgID(This,hashAlgID)	\
    ( (This)->lpVtbl -> get_HashAlgID(This,hashAlgID) ) 

#define IEnroll4_SetHStoreMy(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreMy(This,hStore) ) 

#define IEnroll4_SetHStoreCA(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreCA(This,hStore) ) 

#define IEnroll4_SetHStoreROOT(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreROOT(This,hStore) ) 

#define IEnroll4_SetHStoreRequest(This,hStore)	\
    ( (This)->lpVtbl -> SetHStoreRequest(This,hStore) ) 

#define IEnroll4_put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> put_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define IEnroll4_get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment)	\
    ( (This)->lpVtbl -> get_LimitExchangeKeyToEncipherment(This,fLimitExchangeKeyToEncipherment) ) 

#define IEnroll4_put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> put_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 

#define IEnroll4_get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities)	\
    ( (This)->lpVtbl -> get_EnableSMIMECapabilities(This,fEnableSMIMECapabilities) ) 


#define IEnroll4_put_ThumbPrintWStr(This,thumbPrintBlob)	\
    ( (This)->lpVtbl -> put_ThumbPrintWStr(This,thumbPrintBlob) ) 

#define IEnroll4_get_ThumbPrintWStr(This,thumbPrintBlob)	\
    ( (This)->lpVtbl -> get_ThumbPrintWStr(This,thumbPrintBlob) ) 

#define IEnroll4_SetPrivateKeyArchiveCertificate(This,pPrivateKeyArchiveCert)	\
    ( (This)->lpVtbl -> SetPrivateKeyArchiveCertificate(This,pPrivateKeyArchiveCert) ) 

#define IEnroll4_GetPrivateKeyArchiveCertificate(This)	\
    ( (This)->lpVtbl -> GetPrivateKeyArchiveCertificate(This) ) 

#define IEnroll4_binaryBlobToString(This,Flags,pblobBinary,ppwszString)	\
    ( (This)->lpVtbl -> binaryBlobToString(This,Flags,pblobBinary,ppwszString) ) 

#define IEnroll4_stringToBinaryBlob(This,Flags,pwszString,pblobBinary,pdwSkip,pdwFlags)	\
    ( (This)->lpVtbl -> stringToBinaryBlob(This,Flags,pwszString,pblobBinary,pdwSkip,pdwFlags) ) 

#define IEnroll4_addExtensionToRequestWStr(This,Flags,pwszName,pblobValue)	\
    ( (This)->lpVtbl -> addExtensionToRequestWStr(This,Flags,pwszName,pblobValue) ) 

#define IEnroll4_addAttributeToRequestWStr(This,Flags,pwszName,pblobValue)	\
    ( (This)->lpVtbl -> addAttributeToRequestWStr(This,Flags,pwszName,pblobValue) ) 

#define IEnroll4_addNameValuePairToRequestWStr(This,Flags,pwszName,pwszValue)	\
    ( (This)->lpVtbl -> addNameValuePairToRequestWStr(This,Flags,pwszName,pwszValue) ) 

#define IEnroll4_resetExtensions(This)	\
    ( (This)->lpVtbl -> resetExtensions(This) ) 

#define IEnroll4_resetAttributes(This)	\
    ( (This)->lpVtbl -> resetAttributes(This) ) 

#define IEnroll4_createRequestWStr(This,Flags,pwszDNName,pwszUsage,pblobRequest)	\
    ( (This)->lpVtbl -> createRequestWStr(This,Flags,pwszDNName,pwszUsage,pblobRequest) ) 

#define IEnroll4_createFileRequestWStr(This,Flags,pwszDNName,pwszUsage,pwszRequestFileName)	\
    ( (This)->lpVtbl -> createFileRequestWStr(This,Flags,pwszDNName,pwszUsage,pwszRequestFileName) ) 

#define IEnroll4_acceptResponseBlob(This,pblobResponse)	\
    ( (This)->lpVtbl -> acceptResponseBlob(This,pblobResponse) ) 

#define IEnroll4_acceptFileResponseWStr(This,pwszResponseFileName)	\
    ( (This)->lpVtbl -> acceptFileResponseWStr(This,pwszResponseFileName) ) 

#define IEnroll4_getCertContextFromResponseBlob(This,pblobResponse,ppCertContext)	\
    ( (This)->lpVtbl -> getCertContextFromResponseBlob(This,pblobResponse,ppCertContext) ) 

#define IEnroll4_getCertContextFromFileResponseWStr(This,pwszResponseFileName,ppCertContext)	\
    ( (This)->lpVtbl -> getCertContextFromFileResponseWStr(This,pwszResponseFileName,ppCertContext) ) 

#define IEnroll4_createPFXWStr(This,pwszPassword,pblobPFX)	\
    ( (This)->lpVtbl -> createPFXWStr(This,pwszPassword,pblobPFX) ) 

#define IEnroll4_createFilePFXWStr(This,pwszPassword,pwszPFXFileName)	\
    ( (This)->lpVtbl -> createFilePFXWStr(This,pwszPassword,pwszPFXFileName) ) 

#define IEnroll4_setPendingRequestInfoWStr(This,lRequestID,pwszCADNS,pwszCAName,pwszFriendlyName)	\
    ( (This)->lpVtbl -> setPendingRequestInfoWStr(This,lRequestID,pwszCADNS,pwszCAName,pwszFriendlyName) ) 

#define IEnroll4_enumPendingRequestWStr(This,lIndex,lDesiredProperty,ppProperty)	\
    ( (This)->lpVtbl -> enumPendingRequestWStr(This,lIndex,lDesiredProperty,ppProperty) ) 

#define IEnroll4_removePendingRequestWStr(This,thumbPrintBlob)	\
    ( (This)->lpVtbl -> removePendingRequestWStr(This,thumbPrintBlob) ) 

#define IEnroll4_GetKeyLenEx(This,lSizeSpec,lKeySpec,pdwKeySize)	\
    ( (This)->lpVtbl -> GetKeyLenEx(This,lSizeSpec,lKeySpec,pdwKeySize) ) 

#define IEnroll4_InstallPKCS7BlobEx(This,pBlobPKCS7,plCertInstalled)	\
    ( (This)->lpVtbl -> InstallPKCS7BlobEx(This,pBlobPKCS7,plCertInstalled) ) 

#define IEnroll4_AddCertTypeToRequestWStrEx(This,lType,pwszOIDOrName,lMajorVersion,fMinorVersion,lMinorVersion)	\
    ( (This)->lpVtbl -> AddCertTypeToRequestWStrEx(This,lType,pwszOIDOrName,lMajorVersion,fMinorVersion,lMinorVersion) ) 

#define IEnroll4_getProviderTypeWStr(This,pwszProvName,plProvType)	\
    ( (This)->lpVtbl -> getProviderTypeWStr(This,pwszProvName,plProvType) ) 

#define IEnroll4_addBlobPropertyToCertificateWStr(This,lPropertyId,lReserved,pBlobProperty)	\
    ( (This)->lpVtbl -> addBlobPropertyToCertificateWStr(This,lPropertyId,lReserved,pBlobProperty) ) 

#define IEnroll4_SetSignerCertificate(This,pSignerCert)	\
    ( (This)->lpVtbl -> SetSignerCertificate(This,pSignerCert) ) 

#define IEnroll4_put_ClientId(This,lClientId)	\
    ( (This)->lpVtbl -> put_ClientId(This,lClientId) ) 

#define IEnroll4_get_ClientId(This,plClientId)	\
    ( (This)->lpVtbl -> get_ClientId(This,plClientId) ) 

#define IEnroll4_put_IncludeSubjectKeyID(This,fInclude)	\
    ( (This)->lpVtbl -> put_IncludeSubjectKeyID(This,fInclude) ) 

#define IEnroll4_get_IncludeSubjectKeyID(This,pfInclude)	\
    ( (This)->lpVtbl -> get_IncludeSubjectKeyID(This,pfInclude) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnroll4_INTERFACE_DEFINED__ */



#ifndef __XENROLLLib_LIBRARY_DEFINED__
#define __XENROLLLib_LIBRARY_DEFINED__

/* library XENROLLLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_XENROLLLib;

EXTERN_C const CLSID CLSID_CEnroll2;

#ifdef __cplusplus

class DECLSPEC_UUID("127698e4-e730-4e5c-a2b1-21490a70c8a1")
CEnroll2;
#endif

EXTERN_C const CLSID CLSID_CEnroll;

#ifdef __cplusplus

class DECLSPEC_UUID("43F8F289-7A20-11D0-8F06-00C04FC295E1")
CEnroll;
#endif
#endif /* __XENROLLLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_xenroll_0000_0008 */
/* [local] */ 

EXTERN_C IEnroll * WINAPI PIEnrollGetNoCOM(void);
EXTERN_C IEnroll2 * WINAPI PIEnroll2GetNoCOM(void);
EXTERN_C IEnroll4 * WINAPI PIEnroll4GetNoCOM(void);
#define CRYPT_ENUM_ALL_PROVIDERS  0x1
#define XEPR_ENUM_FIRST        -1
#define XEPR_CADNS           0x01
#define XEPR_CANAME          0x02
#define XEPR_CAFRIENDLYNAME  0x03
#define XEPR_REQUESTID       0x04
#define XEPR_DATE            0x05
#define XEPR_TEMPLATENAME    0x06
#define XEPR_VERSION         0x07
#define XEPR_HASH            0x08
#define XEPR_V1TEMPLATENAME  0x09
#define XEPR_V2TEMPLATEOID   0x10
#define XECR_PKCS10_V2_0     0x1
#define XECR_PKCS7           0x2
#define XECR_CMC             0x3
#define XECR_PKCS10_V1_5     0x4
#define XEKL_KEYSIZE_MIN     0x1
#define XEKL_KEYSIZE_MAX     0x2
#define XEKL_KEYSIZE_INC     0x3
#define XEKL_KEYSIZE_DEFAULT 0x4
#define XEKL_KEYSPEC_KEYX    0x1
#define XEKL_KEYSPEC_SIG     0x2
#define XECT_EXTENSION_V1    0x1
#define XECT_EXTENSION_V2    0x2
#define XECP_STRING_PROPERTY 0x1
#define XECI_DISABLE     0x0
#define XECI_XENROLL     0x1
#define XECI_AUTOENROLL  0x2
#define XECI_REQWIZARD   0x3
#define XECI_CERTREQ     0x4
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xenroll_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xenroll_0000_0008_v0_0_s_ifspec;

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



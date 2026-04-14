

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

#ifndef __casetup_h__
#define __casetup_h__

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

#ifndef __ICertSrvSetupKeyInformation_FWD_DEFINED__
#define __ICertSrvSetupKeyInformation_FWD_DEFINED__
typedef interface ICertSrvSetupKeyInformation ICertSrvSetupKeyInformation;

#endif 	/* __ICertSrvSetupKeyInformation_FWD_DEFINED__ */


#ifndef __ICertSrvSetupKeyInformationCollection_FWD_DEFINED__
#define __ICertSrvSetupKeyInformationCollection_FWD_DEFINED__
typedef interface ICertSrvSetupKeyInformationCollection ICertSrvSetupKeyInformationCollection;

#endif 	/* __ICertSrvSetupKeyInformationCollection_FWD_DEFINED__ */


#ifndef __ICertSrvSetup_FWD_DEFINED__
#define __ICertSrvSetup_FWD_DEFINED__
typedef interface ICertSrvSetup ICertSrvSetup;

#endif 	/* __ICertSrvSetup_FWD_DEFINED__ */


#ifndef __IMSCEPSetup_FWD_DEFINED__
#define __IMSCEPSetup_FWD_DEFINED__
typedef interface IMSCEPSetup IMSCEPSetup;

#endif 	/* __IMSCEPSetup_FWD_DEFINED__ */


#ifndef __ICertificateEnrollmentServerSetup_FWD_DEFINED__
#define __ICertificateEnrollmentServerSetup_FWD_DEFINED__
typedef interface ICertificateEnrollmentServerSetup ICertificateEnrollmentServerSetup;

#endif 	/* __ICertificateEnrollmentServerSetup_FWD_DEFINED__ */


#ifndef __ICertificateEnrollmentPolicyServerSetup_FWD_DEFINED__
#define __ICertificateEnrollmentPolicyServerSetup_FWD_DEFINED__
typedef interface ICertificateEnrollmentPolicyServerSetup ICertificateEnrollmentPolicyServerSetup;

#endif 	/* __ICertificateEnrollmentPolicyServerSetup_FWD_DEFINED__ */


#ifndef __CCertSrvSetupKeyInformation_FWD_DEFINED__
#define __CCertSrvSetupKeyInformation_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertSrvSetupKeyInformation CCertSrvSetupKeyInformation;
#else
typedef struct CCertSrvSetupKeyInformation CCertSrvSetupKeyInformation;
#endif /* __cplusplus */

#endif 	/* __CCertSrvSetupKeyInformation_FWD_DEFINED__ */


#ifndef __CCertSrvSetup_FWD_DEFINED__
#define __CCertSrvSetup_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertSrvSetup CCertSrvSetup;
#else
typedef struct CCertSrvSetup CCertSrvSetup;
#endif /* __cplusplus */

#endif 	/* __CCertSrvSetup_FWD_DEFINED__ */


#ifndef __CMSCEPSetup_FWD_DEFINED__
#define __CMSCEPSetup_FWD_DEFINED__

#ifdef __cplusplus
typedef class CMSCEPSetup CMSCEPSetup;
#else
typedef struct CMSCEPSetup CMSCEPSetup;
#endif /* __cplusplus */

#endif 	/* __CMSCEPSetup_FWD_DEFINED__ */


#ifndef __CCertificateEnrollmentServerSetup_FWD_DEFINED__
#define __CCertificateEnrollmentServerSetup_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertificateEnrollmentServerSetup CCertificateEnrollmentServerSetup;
#else
typedef struct CCertificateEnrollmentServerSetup CCertificateEnrollmentServerSetup;
#endif /* __cplusplus */

#endif 	/* __CCertificateEnrollmentServerSetup_FWD_DEFINED__ */


#ifndef __CCertificateEnrollmentPolicyServerSetup_FWD_DEFINED__
#define __CCertificateEnrollmentPolicyServerSetup_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertificateEnrollmentPolicyServerSetup CCertificateEnrollmentPolicyServerSetup;
#else
typedef struct CCertificateEnrollmentPolicyServerSetup CCertificateEnrollmentPolicyServerSetup;
#endif /* __cplusplus */

#endif 	/* __CCertificateEnrollmentPolicyServerSetup_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_casetup_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0000_v0_0_s_ifspec;

#ifndef __ICertSrvSetupKeyInformation_INTERFACE_DEFINED__
#define __ICertSrvSetupKeyInformation_INTERFACE_DEFINED__

/* interface ICertSrvSetupKeyInformation */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertSrvSetupKeyInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ba73778-36da-4c39-8a85-bcfa7d000793")
    ICertSrvSetupKeyInformation : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProviderName( 
            /* [in] */ __RPC__in const BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Length( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Length( 
            /* [in] */ LONG lVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Existing( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Existing( 
            /* [in] */ VARIANT_BOOL bVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ContainerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ContainerName( 
            /* [in] */ __RPC__in const BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ __RPC__in const BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExistingCACertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ExistingCACertificate( 
            /* [in] */ VARIANT varVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertSrvSetupKeyInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertSrvSetupKeyInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertSrvSetupKeyInformation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertSrvSetupKeyInformation * This,
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
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_ProviderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_ProviderName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderName )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ __RPC__in const BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_Length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Length )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_Length)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Length )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ LONG lVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_Existing)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Existing )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_Existing)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Existing )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ VARIANT_BOOL bVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_ContainerName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainerName )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_ContainerName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ContainerName )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ __RPC__in const BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_HashAlgorithm)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_HashAlgorithm)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ __RPC__in const BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, get_ExistingCACertificate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExistingCACertificate )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformation, put_ExistingCACertificate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExistingCACertificate )( 
            __RPC__in ICertSrvSetupKeyInformation * This,
            /* [in] */ VARIANT varVal);
        
        END_INTERFACE
    } ICertSrvSetupKeyInformationVtbl;

    interface ICertSrvSetupKeyInformation
    {
        CONST_VTBL struct ICertSrvSetupKeyInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertSrvSetupKeyInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertSrvSetupKeyInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertSrvSetupKeyInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertSrvSetupKeyInformation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertSrvSetupKeyInformation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertSrvSetupKeyInformation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertSrvSetupKeyInformation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertSrvSetupKeyInformation_get_ProviderName(This,pVal)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_ProviderName(This,bstrVal)	\
    ( (This)->lpVtbl -> put_ProviderName(This,bstrVal) ) 

#define ICertSrvSetupKeyInformation_get_Length(This,pVal)	\
    ( (This)->lpVtbl -> get_Length(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_Length(This,lVal)	\
    ( (This)->lpVtbl -> put_Length(This,lVal) ) 

#define ICertSrvSetupKeyInformation_get_Existing(This,pVal)	\
    ( (This)->lpVtbl -> get_Existing(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_Existing(This,bVal)	\
    ( (This)->lpVtbl -> put_Existing(This,bVal) ) 

#define ICertSrvSetupKeyInformation_get_ContainerName(This,pVal)	\
    ( (This)->lpVtbl -> get_ContainerName(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_ContainerName(This,bstrVal)	\
    ( (This)->lpVtbl -> put_ContainerName(This,bstrVal) ) 

#define ICertSrvSetupKeyInformation_get_HashAlgorithm(This,pVal)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_HashAlgorithm(This,bstrVal)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,bstrVal) ) 

#define ICertSrvSetupKeyInformation_get_ExistingCACertificate(This,pVal)	\
    ( (This)->lpVtbl -> get_ExistingCACertificate(This,pVal) ) 

#define ICertSrvSetupKeyInformation_put_ExistingCACertificate(This,varVal)	\
    ( (This)->lpVtbl -> put_ExistingCACertificate(This,varVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertSrvSetupKeyInformation_INTERFACE_DEFINED__ */


#ifndef __ICertSrvSetupKeyInformationCollection_INTERFACE_DEFINED__
#define __ICertSrvSetupKeyInformationCollection_INTERFACE_DEFINED__

/* interface ICertSrvSetupKeyInformationCollection */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertSrvSetupKeyInformationCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e65c8b00-e58f-41f9-a9ec-a28d7427c844")
    ICertSrvSetupKeyInformationCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt ICertSrvSetupKeyInformation *pIKeyInformation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertSrvSetupKeyInformationCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertSrvSetupKeyInformationCollection * This,
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
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformationCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformationCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformationCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetupKeyInformationCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ICertSrvSetupKeyInformationCollection * This,
            /* [in] */ __RPC__in_opt ICertSrvSetupKeyInformation *pIKeyInformation);
        
        END_INTERFACE
    } ICertSrvSetupKeyInformationCollectionVtbl;

    interface ICertSrvSetupKeyInformationCollection
    {
        CONST_VTBL struct ICertSrvSetupKeyInformationCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertSrvSetupKeyInformationCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertSrvSetupKeyInformationCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertSrvSetupKeyInformationCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertSrvSetupKeyInformationCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertSrvSetupKeyInformationCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertSrvSetupKeyInformationCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertSrvSetupKeyInformationCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertSrvSetupKeyInformationCollection_get__NewEnum(This,ppVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppVal) ) 

#define ICertSrvSetupKeyInformationCollection_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#define ICertSrvSetupKeyInformationCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define ICertSrvSetupKeyInformationCollection_Add(This,pIKeyInformation)	\
    ( (This)->lpVtbl -> Add(This,pIKeyInformation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertSrvSetupKeyInformationCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_casetup_0000_0002 */
/* [local] */ 

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_casetup_0000_0002_0001
    {
        ENUM_SETUPPROP_INVALID	= -1,
        ENUM_SETUPPROP_CATYPE	= 0,
        ENUM_SETUPPROP_CAKEYINFORMATION	= 1,
        ENUM_SETUPPROP_INTERACTIVE	= 2,
        ENUM_SETUPPROP_CANAME	= 3,
        ENUM_SETUPPROP_CADSSUFFIX	= 4,
        ENUM_SETUPPROP_VALIDITYPERIOD	= 5,
        ENUM_SETUPPROP_VALIDITYPERIODUNIT	= 6,
        ENUM_SETUPPROP_EXPIRATIONDATE	= 7,
        ENUM_SETUPPROP_PRESERVEDATABASE	= 8,
        ENUM_SETUPPROP_DATABASEDIRECTORY	= 9,
        ENUM_SETUPPROP_LOGDIRECTORY	= 10,
        ENUM_SETUPPROP_SHAREDFOLDER	= 11,
        ENUM_SETUPPROP_PARENTCAMACHINE	= 12,
        ENUM_SETUPPROP_PARENTCANAME	= 13,
        ENUM_SETUPPROP_REQUESTFILE	= 14,
        ENUM_SETUPPROP_WEBCAMACHINE	= 15,
        ENUM_SETUPPROP_WEBCANAME	= 16
    } 	CASetupProperty;



extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0002_v0_0_s_ifspec;

#ifndef __ICertSrvSetup_INTERFACE_DEFINED__
#define __ICertSrvSetup_INTERFACE_DEFINED__

/* interface ICertSrvSetup */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertSrvSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b760a1bb-4784-44c0-8f12-555f0780ff25")
    ICertSrvSetup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CAErrorId( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CAErrorString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeDefaults( 
            /* [in] */ VARIANT_BOOL bServer,
            /* [in] */ VARIANT_BOOL bClient) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCASetupProperty( 
            /* [in] */ CASetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCASetupProperty( 
            /* [in] */ CASetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPropertyEditable( 
            /* [in] */ CASetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEditable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedCATypes( 
            /* [retval][out] */ __RPC__out VARIANT *pCATypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProviderNameList( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLengthList( 
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHashAlgorithmList( 
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateKeyContainerList( 
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExistingCACertificates( 
            /* [retval][out] */ __RPC__deref_out_opt ICertSrvSetupKeyInformationCollection **ppVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CAImportPFX( 
            /* [in] */ __RPC__in const BSTR bstrFileName,
            /* [in] */ __RPC__in const BSTR bstrPasswd,
            /* [in] */ VARIANT_BOOL bOverwriteExistingKey,
            /* [retval][out] */ __RPC__deref_out_opt ICertSrvSetupKeyInformation **ppVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCADistinguishedName( 
            /* [in] */ __RPC__in const BSTR bstrCADN,
            /* [in] */ VARIANT_BOOL bIgnoreUnicode,
            /* [in] */ VARIANT_BOOL bOverwriteExistingKey,
            /* [in] */ VARIANT_BOOL bOverwriteExistingCAInDS) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDatabaseInformation( 
            /* [in] */ __RPC__in const BSTR bstrDBDirectory,
            /* [in] */ __RPC__in const BSTR bstrLogDirectory,
            /* [in] */ __RPC__in const BSTR bstrSharedFolder,
            /* [in] */ VARIANT_BOOL bForceOverwrite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParentCAInformation( 
            /* [in] */ __RPC__in const BSTR bstrCAConfiguration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWebCAInformation( 
            /* [in] */ __RPC__in const BSTR bstrCAConfiguration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreUnInstall( 
            /* [in] */ VARIANT_BOOL bClientOnly) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PostUnInstall( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertSrvSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertSrvSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertSrvSetup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertSrvSetup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertSrvSetup * This,
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
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, get_CAErrorId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAErrorId )( 
            __RPC__in ICertSrvSetup * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, get_CAErrorString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAErrorString )( 
            __RPC__in ICertSrvSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, InitializeDefaults)
        HRESULT ( STDMETHODCALLTYPE *InitializeDefaults )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ VARIANT_BOOL bServer,
            /* [in] */ VARIANT_BOOL bClient);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetCASetupProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCASetupProperty )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ CASetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, SetCASetupProperty)
        HRESULT ( STDMETHODCALLTYPE *SetCASetupProperty )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ CASetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, IsPropertyEditable)
        HRESULT ( STDMETHODCALLTYPE *IsPropertyEditable )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ CASetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEditable);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetSupportedCATypes)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedCATypes )( 
            __RPC__in ICertSrvSetup * This,
            /* [retval][out] */ __RPC__out VARIANT *pCATypes);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetProviderNameList)
        HRESULT ( STDMETHODCALLTYPE *GetProviderNameList )( 
            __RPC__in ICertSrvSetup * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetKeyLengthList)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLengthList )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetHashAlgorithmList)
        HRESULT ( STDMETHODCALLTYPE *GetHashAlgorithmList )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetPrivateKeyContainerList)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateKeyContainerList )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, GetExistingCACertificates)
        HRESULT ( STDMETHODCALLTYPE *GetExistingCACertificates )( 
            __RPC__in ICertSrvSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt ICertSrvSetupKeyInformationCollection **ppVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, CAImportPFX)
        HRESULT ( STDMETHODCALLTYPE *CAImportPFX )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrFileName,
            /* [in] */ __RPC__in const BSTR bstrPasswd,
            /* [in] */ VARIANT_BOOL bOverwriteExistingKey,
            /* [retval][out] */ __RPC__deref_out_opt ICertSrvSetupKeyInformation **ppVal);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, SetCADistinguishedName)
        HRESULT ( STDMETHODCALLTYPE *SetCADistinguishedName )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrCADN,
            /* [in] */ VARIANT_BOOL bIgnoreUnicode,
            /* [in] */ VARIANT_BOOL bOverwriteExistingKey,
            /* [in] */ VARIANT_BOOL bOverwriteExistingCAInDS);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, SetDatabaseInformation)
        HRESULT ( STDMETHODCALLTYPE *SetDatabaseInformation )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrDBDirectory,
            /* [in] */ __RPC__in const BSTR bstrLogDirectory,
            /* [in] */ __RPC__in const BSTR bstrSharedFolder,
            /* [in] */ VARIANT_BOOL bForceOverwrite);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, SetParentCAInformation)
        HRESULT ( STDMETHODCALLTYPE *SetParentCAInformation )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrCAConfiguration);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, SetWebCAInformation)
        HRESULT ( STDMETHODCALLTYPE *SetWebCAInformation )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ __RPC__in const BSTR bstrCAConfiguration);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in ICertSrvSetup * This);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, PreUnInstall)
        HRESULT ( STDMETHODCALLTYPE *PreUnInstall )( 
            __RPC__in ICertSrvSetup * This,
            /* [in] */ VARIANT_BOOL bClientOnly);
        
        DECLSPEC_XFGVIRT(ICertSrvSetup, PostUnInstall)
        HRESULT ( STDMETHODCALLTYPE *PostUnInstall )( 
            __RPC__in ICertSrvSetup * This);
        
        END_INTERFACE
    } ICertSrvSetupVtbl;

    interface ICertSrvSetup
    {
        CONST_VTBL struct ICertSrvSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertSrvSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertSrvSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertSrvSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertSrvSetup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertSrvSetup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertSrvSetup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertSrvSetup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertSrvSetup_get_CAErrorId(This,pVal)	\
    ( (This)->lpVtbl -> get_CAErrorId(This,pVal) ) 

#define ICertSrvSetup_get_CAErrorString(This,pVal)	\
    ( (This)->lpVtbl -> get_CAErrorString(This,pVal) ) 

#define ICertSrvSetup_InitializeDefaults(This,bServer,bClient)	\
    ( (This)->lpVtbl -> InitializeDefaults(This,bServer,bClient) ) 

#define ICertSrvSetup_GetCASetupProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> GetCASetupProperty(This,propertyId,pPropertyValue) ) 

#define ICertSrvSetup_SetCASetupProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> SetCASetupProperty(This,propertyId,pPropertyValue) ) 

#define ICertSrvSetup_IsPropertyEditable(This,propertyId,pbEditable)	\
    ( (This)->lpVtbl -> IsPropertyEditable(This,propertyId,pbEditable) ) 

#define ICertSrvSetup_GetSupportedCATypes(This,pCATypes)	\
    ( (This)->lpVtbl -> GetSupportedCATypes(This,pCATypes) ) 

#define ICertSrvSetup_GetProviderNameList(This,pVal)	\
    ( (This)->lpVtbl -> GetProviderNameList(This,pVal) ) 

#define ICertSrvSetup_GetKeyLengthList(This,bstrProviderName,pVal)	\
    ( (This)->lpVtbl -> GetKeyLengthList(This,bstrProviderName,pVal) ) 

#define ICertSrvSetup_GetHashAlgorithmList(This,bstrProviderName,pVal)	\
    ( (This)->lpVtbl -> GetHashAlgorithmList(This,bstrProviderName,pVal) ) 

#define ICertSrvSetup_GetPrivateKeyContainerList(This,bstrProviderName,pVal)	\
    ( (This)->lpVtbl -> GetPrivateKeyContainerList(This,bstrProviderName,pVal) ) 

#define ICertSrvSetup_GetExistingCACertificates(This,ppVal)	\
    ( (This)->lpVtbl -> GetExistingCACertificates(This,ppVal) ) 

#define ICertSrvSetup_CAImportPFX(This,bstrFileName,bstrPasswd,bOverwriteExistingKey,ppVal)	\
    ( (This)->lpVtbl -> CAImportPFX(This,bstrFileName,bstrPasswd,bOverwriteExistingKey,ppVal) ) 

#define ICertSrvSetup_SetCADistinguishedName(This,bstrCADN,bIgnoreUnicode,bOverwriteExistingKey,bOverwriteExistingCAInDS)	\
    ( (This)->lpVtbl -> SetCADistinguishedName(This,bstrCADN,bIgnoreUnicode,bOverwriteExistingKey,bOverwriteExistingCAInDS) ) 

#define ICertSrvSetup_SetDatabaseInformation(This,bstrDBDirectory,bstrLogDirectory,bstrSharedFolder,bForceOverwrite)	\
    ( (This)->lpVtbl -> SetDatabaseInformation(This,bstrDBDirectory,bstrLogDirectory,bstrSharedFolder,bForceOverwrite) ) 

#define ICertSrvSetup_SetParentCAInformation(This,bstrCAConfiguration)	\
    ( (This)->lpVtbl -> SetParentCAInformation(This,bstrCAConfiguration) ) 

#define ICertSrvSetup_SetWebCAInformation(This,bstrCAConfiguration)	\
    ( (This)->lpVtbl -> SetWebCAInformation(This,bstrCAConfiguration) ) 

#define ICertSrvSetup_Install(This)	\
    ( (This)->lpVtbl -> Install(This) ) 

#define ICertSrvSetup_PreUnInstall(This,bClientOnly)	\
    ( (This)->lpVtbl -> PreUnInstall(This,bClientOnly) ) 

#define ICertSrvSetup_PostUnInstall(This)	\
    ( (This)->lpVtbl -> PostUnInstall(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertSrvSetup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_casetup_0000_0003 */
/* [local] */ 

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_casetup_0000_0003_0001
    {
        ENUM_CEPSETUPPROP_USELOCALSYSTEM	= 0,
        ENUM_CEPSETUPPROP_USECHALLENGE	= 1,
        ENUM_CEPSETUPPROP_RANAME_CN	= 2,
        ENUM_CEPSETUPPROP_RANAME_EMAIL	= 3,
        ENUM_CEPSETUPPROP_RANAME_COMPANY	= 4,
        ENUM_CEPSETUPPROP_RANAME_DEPT	= 5,
        ENUM_CEPSETUPPROP_RANAME_CITY	= 6,
        ENUM_CEPSETUPPROP_RANAME_STATE	= 7,
        ENUM_CEPSETUPPROP_RANAME_COUNTRY	= 8,
        ENUM_CEPSETUPPROP_SIGNINGKEYINFORMATION	= 9,
        ENUM_CEPSETUPPROP_EXCHANGEKEYINFORMATION	= 10,
        ENUM_CEPSETUPPROP_CAINFORMATION	= 11,
        ENUM_CEPSETUPPROP_MSCEPURL	= 12,
        ENUM_CEPSETUPPROP_CHALLENGEURL	= 13
    } 	MSCEPSetupProperty;



extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0003_v0_0_s_ifspec;

#ifndef __IMSCEPSetup_INTERFACE_DEFINED__
#define __IMSCEPSetup_INTERFACE_DEFINED__

/* interface IMSCEPSetup */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMSCEPSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4f7761bb-9f3b-4592-9ee0-9a73259c313e")
    IMSCEPSetup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MSCEPErrorId( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MSCEPErrorString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeDefaults( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMSCEPSetupProperty( 
            /* [in] */ MSCEPSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMSCEPSetupProperty( 
            /* [in] */ MSCEPSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccountInformation( 
            /* [in] */ __RPC__in const BSTR bstrUserName,
            /* [in] */ __RPC__in const BSTR bstrPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsMSCEPStoreEmpty( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEmpty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProviderNameList( 
            /* [in] */ VARIANT_BOOL bExchange,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyLengthList( 
            /* [in] */ VARIANT_BOOL bExchange,
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreUnInstall( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PostUnInstall( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSCEPSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSCEPSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSCEPSetup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSCEPSetup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSCEPSetup * This,
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
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, get_MSCEPErrorId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MSCEPErrorId )( 
            __RPC__in IMSCEPSetup * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, get_MSCEPErrorString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MSCEPErrorString )( 
            __RPC__in IMSCEPSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, InitializeDefaults)
        HRESULT ( STDMETHODCALLTYPE *InitializeDefaults )( 
            __RPC__in IMSCEPSetup * This);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, GetMSCEPSetupProperty)
        HRESULT ( STDMETHODCALLTYPE *GetMSCEPSetupProperty )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ MSCEPSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, SetMSCEPSetupProperty)
        HRESULT ( STDMETHODCALLTYPE *SetMSCEPSetupProperty )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ MSCEPSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, SetAccountInformation)
        HRESULT ( STDMETHODCALLTYPE *SetAccountInformation )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ __RPC__in const BSTR bstrUserName,
            /* [in] */ __RPC__in const BSTR bstrPassword);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, IsMSCEPStoreEmpty)
        HRESULT ( STDMETHODCALLTYPE *IsMSCEPStoreEmpty )( 
            __RPC__in IMSCEPSetup * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEmpty);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, GetProviderNameList)
        HRESULT ( STDMETHODCALLTYPE *GetProviderNameList )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ VARIANT_BOOL bExchange,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, GetKeyLengthList)
        HRESULT ( STDMETHODCALLTYPE *GetKeyLengthList )( 
            __RPC__in IMSCEPSetup * This,
            /* [in] */ VARIANT_BOOL bExchange,
            /* [in] */ __RPC__in const BSTR bstrProviderName,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in IMSCEPSetup * This);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, PreUnInstall)
        HRESULT ( STDMETHODCALLTYPE *PreUnInstall )( 
            __RPC__in IMSCEPSetup * This);
        
        DECLSPEC_XFGVIRT(IMSCEPSetup, PostUnInstall)
        HRESULT ( STDMETHODCALLTYPE *PostUnInstall )( 
            __RPC__in IMSCEPSetup * This);
        
        END_INTERFACE
    } IMSCEPSetupVtbl;

    interface IMSCEPSetup
    {
        CONST_VTBL struct IMSCEPSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSCEPSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSCEPSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSCEPSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSCEPSetup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSCEPSetup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSCEPSetup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSCEPSetup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSCEPSetup_get_MSCEPErrorId(This,pVal)	\
    ( (This)->lpVtbl -> get_MSCEPErrorId(This,pVal) ) 

#define IMSCEPSetup_get_MSCEPErrorString(This,pVal)	\
    ( (This)->lpVtbl -> get_MSCEPErrorString(This,pVal) ) 

#define IMSCEPSetup_InitializeDefaults(This)	\
    ( (This)->lpVtbl -> InitializeDefaults(This) ) 

#define IMSCEPSetup_GetMSCEPSetupProperty(This,propertyId,pVal)	\
    ( (This)->lpVtbl -> GetMSCEPSetupProperty(This,propertyId,pVal) ) 

#define IMSCEPSetup_SetMSCEPSetupProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> SetMSCEPSetupProperty(This,propertyId,pPropertyValue) ) 

#define IMSCEPSetup_SetAccountInformation(This,bstrUserName,bstrPassword)	\
    ( (This)->lpVtbl -> SetAccountInformation(This,bstrUserName,bstrPassword) ) 

#define IMSCEPSetup_IsMSCEPStoreEmpty(This,pbEmpty)	\
    ( (This)->lpVtbl -> IsMSCEPStoreEmpty(This,pbEmpty) ) 

#define IMSCEPSetup_GetProviderNameList(This,bExchange,pVal)	\
    ( (This)->lpVtbl -> GetProviderNameList(This,bExchange,pVal) ) 

#define IMSCEPSetup_GetKeyLengthList(This,bExchange,bstrProviderName,pVal)	\
    ( (This)->lpVtbl -> GetKeyLengthList(This,bExchange,bstrProviderName,pVal) ) 

#define IMSCEPSetup_Install(This)	\
    ( (This)->lpVtbl -> Install(This) ) 

#define IMSCEPSetup_PreUnInstall(This)	\
    ( (This)->lpVtbl -> PreUnInstall(This) ) 

#define IMSCEPSetup_PostUnInstall(This)	\
    ( (This)->lpVtbl -> PostUnInstall(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSCEPSetup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_casetup_0000_0004 */
/* [local] */ 

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_casetup_0000_0004_0001
    {
        ENUM_CESSETUPPROP_USE_IISAPPPOOLIDENTITY	= 0,
        ENUM_CESSETUPPROP_CACONFIG	= 1,
        ENUM_CESSETUPPROP_AUTHENTICATION	= 2,
        ENUM_CESSETUPPROP_SSLCERTHASH	= 3,
        ENUM_CESSETUPPROP_URL	= 4,
        ENUM_CESSETUPPROP_RENEWALONLY	= 5,
        ENUM_CESSETUPPROP_ALLOW_KEYBASED_RENEWAL	= 6
    } 	CESSetupProperty;



extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0004_v0_0_s_ifspec;

#ifndef __ICertificateEnrollmentServerSetup_INTERFACE_DEFINED__
#define __ICertificateEnrollmentServerSetup_INTERFACE_DEFINED__

/* interface ICertificateEnrollmentServerSetup */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertificateEnrollmentServerSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70027FDB-9DD9-4921-8944-B35CB31BD2EC")
    ICertificateEnrollmentServerSetup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ErrorString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeInstallDefaults( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ CESSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ CESSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetApplicationPoolCredentials( 
            /* [in] */ __RPC__in const BSTR bstrUsername,
            /* [in] */ __RPC__in const BSTR bstrPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnInstall( 
            /* [optional][in] */ __RPC__in VARIANT *pCAConfig,
            /* [optional][in] */ __RPC__in VARIANT *pAuthentication) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertificateEnrollmentServerSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertificateEnrollmentServerSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertificateEnrollmentServerSetup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertificateEnrollmentServerSetup * This,
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
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, get_ErrorString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorString )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, InitializeInstallDefaults)
        HRESULT ( STDMETHODCALLTYPE *InitializeInstallDefaults )( 
            __RPC__in ICertificateEnrollmentServerSetup * This);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ CESSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ CESSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, SetApplicationPoolCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetApplicationPoolCredentials )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [in] */ __RPC__in const BSTR bstrUsername,
            /* [in] */ __RPC__in const BSTR bstrPassword);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in ICertificateEnrollmentServerSetup * This);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentServerSetup, UnInstall)
        HRESULT ( STDMETHODCALLTYPE *UnInstall )( 
            __RPC__in ICertificateEnrollmentServerSetup * This,
            /* [optional][in] */ __RPC__in VARIANT *pCAConfig,
            /* [optional][in] */ __RPC__in VARIANT *pAuthentication);
        
        END_INTERFACE
    } ICertificateEnrollmentServerSetupVtbl;

    interface ICertificateEnrollmentServerSetup
    {
        CONST_VTBL struct ICertificateEnrollmentServerSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertificateEnrollmentServerSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertificateEnrollmentServerSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertificateEnrollmentServerSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertificateEnrollmentServerSetup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertificateEnrollmentServerSetup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertificateEnrollmentServerSetup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertificateEnrollmentServerSetup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertificateEnrollmentServerSetup_get_ErrorString(This,pVal)	\
    ( (This)->lpVtbl -> get_ErrorString(This,pVal) ) 

#define ICertificateEnrollmentServerSetup_InitializeInstallDefaults(This)	\
    ( (This)->lpVtbl -> InitializeInstallDefaults(This) ) 

#define ICertificateEnrollmentServerSetup_GetProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> GetProperty(This,propertyId,pPropertyValue) ) 

#define ICertificateEnrollmentServerSetup_SetProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> SetProperty(This,propertyId,pPropertyValue) ) 

#define ICertificateEnrollmentServerSetup_SetApplicationPoolCredentials(This,bstrUsername,bstrPassword)	\
    ( (This)->lpVtbl -> SetApplicationPoolCredentials(This,bstrUsername,bstrPassword) ) 

#define ICertificateEnrollmentServerSetup_Install(This)	\
    ( (This)->lpVtbl -> Install(This) ) 

#define ICertificateEnrollmentServerSetup_UnInstall(This,pCAConfig,pAuthentication)	\
    ( (This)->lpVtbl -> UnInstall(This,pCAConfig,pAuthentication) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertificateEnrollmentServerSetup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_casetup_0000_0005 */
/* [local] */ 

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_casetup_0000_0005_0001
    {
        ENUM_CEPSETUPPROP_AUTHENTICATION	= 0,
        ENUM_CEPSETUPPROP_SSLCERTHASH	= 1,
        ENUM_CEPSETUPPROP_URL	= 2,
        ENUM_CEPSETUPPROP_KEYBASED_RENEWAL	= 3
    } 	CEPSetupProperty;



extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0005_v0_0_s_ifspec;

#ifndef __ICertificateEnrollmentPolicyServerSetup_INTERFACE_DEFINED__
#define __ICertificateEnrollmentPolicyServerSetup_INTERFACE_DEFINED__

/* interface ICertificateEnrollmentPolicyServerSetup */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertificateEnrollmentPolicyServerSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("859252CC-238C-4a88-B8FD-A37E7D04E68B")
    ICertificateEnrollmentPolicyServerSetup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ErrorString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeInstallDefaults( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ CEPSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ CEPSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnInstall( 
            /* [optional][in] */ __RPC__in VARIANT *pAuthKeyBasedRenewal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertificateEnrollmentPolicyServerSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertificateEnrollmentPolicyServerSetup * This,
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
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, get_ErrorString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorString )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, InitializeInstallDefaults)
        HRESULT ( STDMETHODCALLTYPE *InitializeInstallDefaults )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [in] */ CEPSetupProperty propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [in] */ CEPSetupProperty propertyId,
            /* [in] */ __RPC__in VARIANT *pPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This);
        
        DECLSPEC_XFGVIRT(ICertificateEnrollmentPolicyServerSetup, UnInstall)
        HRESULT ( STDMETHODCALLTYPE *UnInstall )( 
            __RPC__in ICertificateEnrollmentPolicyServerSetup * This,
            /* [optional][in] */ __RPC__in VARIANT *pAuthKeyBasedRenewal);
        
        END_INTERFACE
    } ICertificateEnrollmentPolicyServerSetupVtbl;

    interface ICertificateEnrollmentPolicyServerSetup
    {
        CONST_VTBL struct ICertificateEnrollmentPolicyServerSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertificateEnrollmentPolicyServerSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertificateEnrollmentPolicyServerSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertificateEnrollmentPolicyServerSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertificateEnrollmentPolicyServerSetup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertificateEnrollmentPolicyServerSetup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertificateEnrollmentPolicyServerSetup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertificateEnrollmentPolicyServerSetup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertificateEnrollmentPolicyServerSetup_get_ErrorString(This,pVal)	\
    ( (This)->lpVtbl -> get_ErrorString(This,pVal) ) 

#define ICertificateEnrollmentPolicyServerSetup_InitializeInstallDefaults(This)	\
    ( (This)->lpVtbl -> InitializeInstallDefaults(This) ) 

#define ICertificateEnrollmentPolicyServerSetup_GetProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> GetProperty(This,propertyId,pPropertyValue) ) 

#define ICertificateEnrollmentPolicyServerSetup_SetProperty(This,propertyId,pPropertyValue)	\
    ( (This)->lpVtbl -> SetProperty(This,propertyId,pPropertyValue) ) 

#define ICertificateEnrollmentPolicyServerSetup_Install(This)	\
    ( (This)->lpVtbl -> Install(This) ) 

#define ICertificateEnrollmentPolicyServerSetup_UnInstall(This,pAuthKeyBasedRenewal)	\
    ( (This)->lpVtbl -> UnInstall(This,pAuthKeyBasedRenewal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertificateEnrollmentPolicyServerSetup_INTERFACE_DEFINED__ */



#ifndef __CertSrvSetupLib_LIBRARY_DEFINED__
#define __CertSrvSetupLib_LIBRARY_DEFINED__

/* library CertSrvSetupLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_CertSrvSetupLib;

EXTERN_C const CLSID CLSID_CCertSrvSetupKeyInformation;

#ifdef __cplusplus

class DECLSPEC_UUID("38373906-5433-4633-b0fb-29b7e78262e1")
CCertSrvSetupKeyInformation;
#endif

EXTERN_C const CLSID CLSID_CCertSrvSetup;

#ifdef __cplusplus

class DECLSPEC_UUID("961f180f-f55c-413d-a9b3-7d2af4d8e42f")
CCertSrvSetup;
#endif

EXTERN_C const CLSID CLSID_CMSCEPSetup;

#ifdef __cplusplus

class DECLSPEC_UUID("aa4f5c02-8e7c-49c4-94fa-67a5cc5eadb4")
CMSCEPSetup;
#endif

EXTERN_C const CLSID CLSID_CCertificateEnrollmentServerSetup;

#ifdef __cplusplus

class DECLSPEC_UUID("9902F3BC-88AF-4cf8-AE62-7140531552B6")
CCertificateEnrollmentServerSetup;
#endif

EXTERN_C const CLSID CLSID_CCertificateEnrollmentPolicyServerSetup;

#ifdef __cplusplus

class DECLSPEC_UUID("AFE2FA32-41B1-459d-A5DE-49ADD8A72182")
CCertificateEnrollmentPolicyServerSetup;
#endif
#endif /* __CertSrvSetupLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_casetup_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_casetup_0000_0007_v0_0_s_ifspec;

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



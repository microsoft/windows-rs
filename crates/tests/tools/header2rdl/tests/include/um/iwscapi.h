

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

#ifndef __iwscapi_h__
#define __iwscapi_h__

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

#ifndef __IWscProduct_FWD_DEFINED__
#define __IWscProduct_FWD_DEFINED__
typedef interface IWscProduct IWscProduct;

#endif 	/* __IWscProduct_FWD_DEFINED__ */


#ifndef __IWscProduct2_FWD_DEFINED__
#define __IWscProduct2_FWD_DEFINED__
typedef interface IWscProduct2 IWscProduct2;

#endif 	/* __IWscProduct2_FWD_DEFINED__ */


#ifndef __IWscProduct3_FWD_DEFINED__
#define __IWscProduct3_FWD_DEFINED__
typedef interface IWscProduct3 IWscProduct3;

#endif 	/* __IWscProduct3_FWD_DEFINED__ */


#ifndef __IWSCProductList_FWD_DEFINED__
#define __IWSCProductList_FWD_DEFINED__
typedef interface IWSCProductList IWSCProductList;

#endif 	/* __IWSCProductList_FWD_DEFINED__ */


#ifndef __IWSCDefaultProduct_FWD_DEFINED__
#define __IWSCDefaultProduct_FWD_DEFINED__
typedef interface IWSCDefaultProduct IWSCDefaultProduct;

#endif 	/* __IWSCDefaultProduct_FWD_DEFINED__ */


#ifndef __WSCProductList_FWD_DEFINED__
#define __WSCProductList_FWD_DEFINED__

#ifdef __cplusplus
typedef class WSCProductList WSCProductList;
#else
typedef struct WSCProductList WSCProductList;
#endif /* __cplusplus */

#endif 	/* __WSCProductList_FWD_DEFINED__ */


#ifndef __WSCDefaultProduct_FWD_DEFINED__
#define __WSCDefaultProduct_FWD_DEFINED__

#ifdef __cplusplus
typedef class WSCDefaultProduct WSCDefaultProduct;
#else
typedef struct WSCDefaultProduct WSCDefaultProduct;
#endif /* __cplusplus */

#endif 	/* __WSCDefaultProduct_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iwscapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef __WSC_SECURITY_PRODUCT_SUBSTATUS__
#define __WSC_SECURITY_PRODUCT_SUBSTATUS__
typedef 
enum WSC_SECURITY_PRODUCT_SUBSTATUS
    {
        WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET	= 0,
        WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION	= 1,
        WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED	= 2,
        WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED	= 3
    } 	WSC_SECURITY_PRODUCT_SUBSTATUS;

#endif
#ifndef __WSC_SECURITY_PRODUCT_STATE__
#define __WSC_SECURITY_PRODUCT_STATE__
typedef 
enum WSC_SECURITY_PRODUCT_STATE
    {
        WSC_SECURITY_PRODUCT_STATE_ON	= 0,
        WSC_SECURITY_PRODUCT_STATE_OFF	= 1,
        WSC_SECURITY_PRODUCT_STATE_SNOOZED	= 2,
        WSC_SECURITY_PRODUCT_STATE_EXPIRED	= 3
    } 	WSC_SECURITY_PRODUCT_STATE;

#endif
#ifndef __SECURITY_PRODUCT_TYPE__
#define __SECURITY_PRODUCT_TYPE__
typedef 
enum _SECURITY_PRODUCT_TYPE
    {
        SECURITY_PRODUCT_TYPE_ANTIVIRUS	= 0,
        SECURITY_PRODUCT_TYPE_FIREWALL	= 1,
        SECURITY_PRODUCT_TYPE_ANTISPYWARE	= 2
    } 	SECURITY_PRODUCT_TYPE;

#endif
#ifndef __WSC_SECURITY_SIGNATURE_STATUS__
#define __WSC_SECURITY_SIGNATURE_STATUS__
typedef 
enum _WSC_SECURITY_SIGNATURE_STATUS
    {
        WSC_SECURITY_PRODUCT_OUT_OF_DATE	= 0,
        WSC_SECURITY_PRODUCT_UP_TO_DATE	= 1
    } 	WSC_SECURITY_SIGNATURE_STATUS;

#endif


extern RPC_IF_HANDLE __MIDL_itf_iwscapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iwscapi_0000_0000_v0_0_s_ifspec;

#ifndef __IWscProduct_INTERFACE_DEFINED__
#define __IWscProduct_INTERFACE_DEFINED__

/* interface IWscProduct */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWscProduct;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8C38232E-3A45-4A27-92B0-1A16A975F669")
    IWscProduct : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductState( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_STATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SignatureStatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_SIGNATURE_STATUS *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemediationPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductStateTimestamp( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProductIsDefault( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWscProductVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWscProduct * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWscProduct * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWscProduct * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWscProduct * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWscProduct * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWscProduct * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWscProduct * This,
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
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductName )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductState )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_SignatureStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignatureStatus )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_SIGNATURE_STATUS *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_RemediationPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemediationPath )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductStateTimestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductStateTimestamp )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductGuid )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductIsDefault)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductIsDefault )( 
            __RPC__in IWscProduct * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        END_INTERFACE
    } IWscProductVtbl;

    interface IWscProduct
    {
        CONST_VTBL struct IWscProductVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWscProduct_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWscProduct_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWscProduct_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWscProduct_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWscProduct_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWscProduct_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWscProduct_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWscProduct_get_ProductName(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductName(This,pVal) ) 

#define IWscProduct_get_ProductState(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductState(This,pVal) ) 

#define IWscProduct_get_SignatureStatus(This,pVal)	\
    ( (This)->lpVtbl -> get_SignatureStatus(This,pVal) ) 

#define IWscProduct_get_RemediationPath(This,pVal)	\
    ( (This)->lpVtbl -> get_RemediationPath(This,pVal) ) 

#define IWscProduct_get_ProductStateTimestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductStateTimestamp(This,pVal) ) 

#define IWscProduct_get_ProductGuid(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductGuid(This,pVal) ) 

#define IWscProduct_get_ProductIsDefault(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductIsDefault(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWscProduct_INTERFACE_DEFINED__ */


#ifndef __IWscProduct2_INTERFACE_DEFINED__
#define __IWscProduct2_INTERFACE_DEFINED__

/* interface IWscProduct2 */
/* [unique][dual][uuid][object] */ 


EXTERN_C const IID IID_IWscProduct2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F896CA54-FE09-4403-86D4-23CB488D81D8")
    IWscProduct2 : public IWscProduct
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AntivirusScanSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AntivirusSettingsSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AntivirusProtectionUpdateSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FirewallDomainProfileSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FirewallPrivateProfileSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FirewallPublicProfileSubstatus( 
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWscProduct2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWscProduct2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWscProduct2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWscProduct2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWscProduct2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWscProduct2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWscProduct2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWscProduct2 * This,
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
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductName )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductState )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_SignatureStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignatureStatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_SIGNATURE_STATUS *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_RemediationPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemediationPath )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductStateTimestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductStateTimestamp )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductGuid )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductIsDefault)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductIsDefault )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusScanSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusScanSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusSettingsSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusSettingsSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusProtectionUpdateSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusProtectionUpdateSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallDomainProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallDomainProfileSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallPrivateProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallPrivateProfileSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallPublicProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallPublicProfileSubstatus )( 
            __RPC__in IWscProduct2 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        END_INTERFACE
    } IWscProduct2Vtbl;

    interface IWscProduct2
    {
        CONST_VTBL struct IWscProduct2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWscProduct2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWscProduct2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWscProduct2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWscProduct2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWscProduct2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWscProduct2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWscProduct2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWscProduct2_get_ProductName(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductName(This,pVal) ) 

#define IWscProduct2_get_ProductState(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductState(This,pVal) ) 

#define IWscProduct2_get_SignatureStatus(This,pVal)	\
    ( (This)->lpVtbl -> get_SignatureStatus(This,pVal) ) 

#define IWscProduct2_get_RemediationPath(This,pVal)	\
    ( (This)->lpVtbl -> get_RemediationPath(This,pVal) ) 

#define IWscProduct2_get_ProductStateTimestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductStateTimestamp(This,pVal) ) 

#define IWscProduct2_get_ProductGuid(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductGuid(This,pVal) ) 

#define IWscProduct2_get_ProductIsDefault(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductIsDefault(This,pVal) ) 


#define IWscProduct2_get_AntivirusScanSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusScanSubstatus(This,peStatus) ) 

#define IWscProduct2_get_AntivirusSettingsSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusSettingsSubstatus(This,peStatus) ) 

#define IWscProduct2_get_AntivirusProtectionUpdateSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusProtectionUpdateSubstatus(This,peStatus) ) 

#define IWscProduct2_get_FirewallDomainProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallDomainProfileSubstatus(This,peStatus) ) 

#define IWscProduct2_get_FirewallPrivateProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallPrivateProfileSubstatus(This,peStatus) ) 

#define IWscProduct2_get_FirewallPublicProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallPublicProfileSubstatus(This,peStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWscProduct2_INTERFACE_DEFINED__ */


#ifndef __IWscProduct3_INTERFACE_DEFINED__
#define __IWscProduct3_INTERFACE_DEFINED__

/* interface IWscProduct3 */
/* [unique][dual][uuid][object] */ 


EXTERN_C const IID IID_IWscProduct3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55536524-D1D1-4726-8C7C-04996A1904E7")
    IWscProduct3 : public IWscProduct2
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AntivirusDaysUntilExpired( 
            /* [retval][out] */ __RPC__out DWORD *pdwDays) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWscProduct3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWscProduct3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWscProduct3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWscProduct3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWscProduct3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWscProduct3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWscProduct3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWscProduct3 * This,
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
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductName )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductState )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_SignatureStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignatureStatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_SIGNATURE_STATUS *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_RemediationPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemediationPath )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductStateTimestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductStateTimestamp )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductGuid )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct, get_ProductIsDefault)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProductIsDefault )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusScanSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusScanSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusSettingsSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusSettingsSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_AntivirusProtectionUpdateSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusProtectionUpdateSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallDomainProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallDomainProfileSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallPrivateProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallPrivateProfileSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct2, get_FirewallPublicProfileSubstatus)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirewallPublicProfileSubstatus )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out WSC_SECURITY_PRODUCT_SUBSTATUS *peStatus);
        
        DECLSPEC_XFGVIRT(IWscProduct3, get_AntivirusDaysUntilExpired)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntivirusDaysUntilExpired )( 
            __RPC__in IWscProduct3 * This,
            /* [retval][out] */ __RPC__out DWORD *pdwDays);
        
        END_INTERFACE
    } IWscProduct3Vtbl;

    interface IWscProduct3
    {
        CONST_VTBL struct IWscProduct3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWscProduct3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWscProduct3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWscProduct3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWscProduct3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWscProduct3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWscProduct3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWscProduct3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWscProduct3_get_ProductName(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductName(This,pVal) ) 

#define IWscProduct3_get_ProductState(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductState(This,pVal) ) 

#define IWscProduct3_get_SignatureStatus(This,pVal)	\
    ( (This)->lpVtbl -> get_SignatureStatus(This,pVal) ) 

#define IWscProduct3_get_RemediationPath(This,pVal)	\
    ( (This)->lpVtbl -> get_RemediationPath(This,pVal) ) 

#define IWscProduct3_get_ProductStateTimestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductStateTimestamp(This,pVal) ) 

#define IWscProduct3_get_ProductGuid(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductGuid(This,pVal) ) 

#define IWscProduct3_get_ProductIsDefault(This,pVal)	\
    ( (This)->lpVtbl -> get_ProductIsDefault(This,pVal) ) 


#define IWscProduct3_get_AntivirusScanSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusScanSubstatus(This,peStatus) ) 

#define IWscProduct3_get_AntivirusSettingsSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusSettingsSubstatus(This,peStatus) ) 

#define IWscProduct3_get_AntivirusProtectionUpdateSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_AntivirusProtectionUpdateSubstatus(This,peStatus) ) 

#define IWscProduct3_get_FirewallDomainProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallDomainProfileSubstatus(This,peStatus) ) 

#define IWscProduct3_get_FirewallPrivateProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallPrivateProfileSubstatus(This,peStatus) ) 

#define IWscProduct3_get_FirewallPublicProfileSubstatus(This,peStatus)	\
    ( (This)->lpVtbl -> get_FirewallPublicProfileSubstatus(This,peStatus) ) 


#define IWscProduct3_get_AntivirusDaysUntilExpired(This,pdwDays)	\
    ( (This)->lpVtbl -> get_AntivirusDaysUntilExpired(This,pdwDays) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWscProduct3_INTERFACE_DEFINED__ */


#ifndef __IWSCProductList_INTERFACE_DEFINED__
#define __IWSCProductList_INTERFACE_DEFINED__

/* interface IWSCProductList */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWSCProductList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("722A338C-6E8E-4E72-AC27-1417FB0C81C2")
    IWSCProductList : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ULONG provider) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ ULONG index,
            /* [retval][out] */ __RPC__deref_out_opt IWscProduct **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSCProductListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWSCProductList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWSCProductList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWSCProductList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWSCProductList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWSCProductList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWSCProductList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSCProductList * This,
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
        
        DECLSPEC_XFGVIRT(IWSCProductList, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IWSCProductList * This,
            /* [in] */ ULONG provider);
        
        DECLSPEC_XFGVIRT(IWSCProductList, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IWSCProductList * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IWSCProductList, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IWSCProductList * This,
            /* [in] */ ULONG index,
            /* [retval][out] */ __RPC__deref_out_opt IWscProduct **pVal);
        
        END_INTERFACE
    } IWSCProductListVtbl;

    interface IWSCProductList
    {
        CONST_VTBL struct IWSCProductListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSCProductList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSCProductList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSCProductList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSCProductList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSCProductList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSCProductList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSCProductList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSCProductList_Initialize(This,provider)	\
    ( (This)->lpVtbl -> Initialize(This,provider) ) 

#define IWSCProductList_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IWSCProductList_get_Item(This,index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,index,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSCProductList_INTERFACE_DEFINED__ */


#ifndef __IWSCDefaultProduct_INTERFACE_DEFINED__
#define __IWSCDefaultProduct_INTERFACE_DEFINED__

/* interface IWSCDefaultProduct */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWSCDefaultProduct;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0476d69c-f21a-11e5-9ce9-5e5517507c66")
    IWSCDefaultProduct : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetDefaultProduct( 
            /* [in] */ SECURITY_PRODUCT_TYPE eType,
            /* [in] */ __RPC__in BSTR pGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSCDefaultProductVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWSCDefaultProduct * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWSCDefaultProduct * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWSCDefaultProduct * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWSCDefaultProduct * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWSCDefaultProduct * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWSCDefaultProduct * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWSCDefaultProduct * This,
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
        
        DECLSPEC_XFGVIRT(IWSCDefaultProduct, SetDefaultProduct)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultProduct )( 
            __RPC__in IWSCDefaultProduct * This,
            /* [in] */ SECURITY_PRODUCT_TYPE eType,
            /* [in] */ __RPC__in BSTR pGuid);
        
        END_INTERFACE
    } IWSCDefaultProductVtbl;

    interface IWSCDefaultProduct
    {
        CONST_VTBL struct IWSCDefaultProductVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSCDefaultProduct_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSCDefaultProduct_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSCDefaultProduct_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSCDefaultProduct_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWSCDefaultProduct_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWSCDefaultProduct_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWSCDefaultProduct_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWSCDefaultProduct_SetDefaultProduct(This,eType,pGuid)	\
    ( (This)->lpVtbl -> SetDefaultProduct(This,eType,pGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSCDefaultProduct_INTERFACE_DEFINED__ */



#ifndef __wscAPILib_LIBRARY_DEFINED__
#define __wscAPILib_LIBRARY_DEFINED__

/* library wscAPILib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_wscAPILib;

EXTERN_C const CLSID CLSID_WSCProductList;

#ifdef __cplusplus

class DECLSPEC_UUID("17072F7B-9ABE-4A74-A261-1EB76B55107A")
WSCProductList;
#endif

EXTERN_C const CLSID CLSID_WSCDefaultProduct;

#ifdef __cplusplus

class DECLSPEC_UUID("2981a36e-f22d-11e5-9ce9-5e5517507c66")
WSCDefaultProduct;
#endif
#endif /* __wscAPILib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_iwscapi_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iwscapi_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iwscapi_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



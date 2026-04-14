

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


#ifndef __xpsdigitalsignature_h__
#define __xpsdigitalsignature_h__

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

#ifndef __IXpsSigningOptions_FWD_DEFINED__
#define __IXpsSigningOptions_FWD_DEFINED__
typedef interface IXpsSigningOptions IXpsSigningOptions;

#endif 	/* __IXpsSigningOptions_FWD_DEFINED__ */


#ifndef __IXpsSignatureCollection_FWD_DEFINED__
#define __IXpsSignatureCollection_FWD_DEFINED__
typedef interface IXpsSignatureCollection IXpsSignatureCollection;

#endif 	/* __IXpsSignatureCollection_FWD_DEFINED__ */


#ifndef __IXpsSignature_FWD_DEFINED__
#define __IXpsSignature_FWD_DEFINED__
typedef interface IXpsSignature IXpsSignature;

#endif 	/* __IXpsSignature_FWD_DEFINED__ */


#ifndef __IXpsSignatureBlockCollection_FWD_DEFINED__
#define __IXpsSignatureBlockCollection_FWD_DEFINED__
typedef interface IXpsSignatureBlockCollection IXpsSignatureBlockCollection;

#endif 	/* __IXpsSignatureBlockCollection_FWD_DEFINED__ */


#ifndef __IXpsSignatureBlock_FWD_DEFINED__
#define __IXpsSignatureBlock_FWD_DEFINED__
typedef interface IXpsSignatureBlock IXpsSignatureBlock;

#endif 	/* __IXpsSignatureBlock_FWD_DEFINED__ */


#ifndef __IXpsSignatureRequestCollection_FWD_DEFINED__
#define __IXpsSignatureRequestCollection_FWD_DEFINED__
typedef interface IXpsSignatureRequestCollection IXpsSignatureRequestCollection;

#endif 	/* __IXpsSignatureRequestCollection_FWD_DEFINED__ */


#ifndef __IXpsSignatureRequest_FWD_DEFINED__
#define __IXpsSignatureRequest_FWD_DEFINED__
typedef interface IXpsSignatureRequest IXpsSignatureRequest;

#endif 	/* __IXpsSignatureRequest_FWD_DEFINED__ */


#ifndef __IXpsSignatureManager_FWD_DEFINED__
#define __IXpsSignatureManager_FWD_DEFINED__
typedef interface IXpsSignatureManager IXpsSignatureManager;

#endif 	/* __IXpsSignatureManager_FWD_DEFINED__ */


#ifndef __XpsSignatureManager_FWD_DEFINED__
#define __XpsSignatureManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class XpsSignatureManager XpsSignatureManager;
#else
typedef struct XpsSignatureManager XpsSignatureManager;
#endif /* __cplusplus */

#endif 	/* __XpsSignatureManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "wincrypt.h"
#include "msopc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsdigitalsignature_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_xpsdigitalsignature_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsdigitalsignature_0000_0000_v0_0_s_ifspec;


#ifndef __MSXPSSIG_LIBRARY_DEFINED__
#define __MSXPSSIG_LIBRARY_DEFINED__

/* library MSXPSSIG */
/* [uuid] */ 








typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_xpsdigitalsignature_0000_0000_0001
    {
        XPS_SIGNATURE_STATUS_INCOMPLIANT	= 1,
        XPS_SIGNATURE_STATUS_INCOMPLETE	= 2,
        XPS_SIGNATURE_STATUS_BROKEN	= 3,
        XPS_SIGNATURE_STATUS_QUESTIONABLE	= 4,
        XPS_SIGNATURE_STATUS_VALID	= 5
    } 	XPS_SIGNATURE_STATUS;

typedef /* [public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsdigitalsignature_0000_0000_0002
    {
        XPS_SIGN_POLICY_NONE	= 0,
        XPS_SIGN_POLICY_CORE_PROPERTIES	= 0x1,
        XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS	= 0x2,
        XPS_SIGN_POLICY_PRINT_TICKET	= 0x4,
        XPS_SIGN_POLICY_DISCARD_CONTROL	= 0x8,
        XPS_SIGN_POLICY_ALL	= 0xf
    } 	XPS_SIGN_POLICY;

DEFINE_ENUM_FLAG_OPERATORS(XPS_SIGN_POLICY)
typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsdigitalsignature_0000_0000_0003
    {
        XPS_SIGN_FLAGS_NONE	= 0,
        XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY	= 0x1
    } 	XPS_SIGN_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(XPS_SIGN_FLAGS)

EXTERN_C const IID LIBID_MSXPSSIG;

#ifndef __IXpsSigningOptions_INTERFACE_DEFINED__
#define __IXpsSigningOptions_INTERFACE_DEFINED__

/* interface IXpsSigningOptions */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSigningOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7718eae4-3215-49be-af5b-594fef7fcfa6")
    IXpsSigningOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSignatureId( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureId( 
            /* [string][in] */ __RPC__in_string LPCWSTR signatureId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureMethod( 
            /* [string][in] */ __RPC__in_string LPCWSTR signatureMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDigestMethod( 
            /* [string][in] */ __RPC__in_string LPCWSTR digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignaturePartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPolicy( 
            /* [retval][out] */ __RPC__out XPS_SIGN_POLICY *policy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPolicy( 
            /* [in] */ XPS_SIGN_POLICY policy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigningTimeFormat( 
            /* [retval][out] */ __RPC__out OPC_SIGNATURE_TIME_FORMAT *timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSigningTimeFormat( 
            /* [in] */ OPC_SIGNATURE_TIME_FORMAT timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomObjects( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectSet **customObjectSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomReferences( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceSet **customReferenceSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcCertificateSet **certificateSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [retval][out] */ __RPC__out XPS_SIGN_FLAGS *flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ XPS_SIGN_FLAGS flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSigningOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSigningOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSigningOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSigningOptions * This);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureId )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureId);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureId )( 
            __RPC__in IXpsSigningOptions * This,
            /* [string][in] */ __RPC__in_string LPCWSTR signatureId);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetSignatureMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureMethod )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureMethod);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetSignatureMethod)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureMethod )( 
            __RPC__in IXpsSigningOptions * This,
            /* [string][in] */ __RPC__in_string LPCWSTR signatureMethod);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *GetDigestMethod )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *SetDigestMethod )( 
            __RPC__in IXpsSigningOptions * This,
            /* [string][in] */ __RPC__in_string LPCWSTR digestMethod);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartName )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **signaturePartName);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *SetSignaturePartName )( 
            __RPC__in IXpsSigningOptions * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *signaturePartName);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetPolicy )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__out XPS_SIGN_POLICY *policy);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetPolicy )( 
            __RPC__in IXpsSigningOptions * This,
            /* [in] */ XPS_SIGN_POLICY policy);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetSigningTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetSigningTimeFormat )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__out OPC_SIGNATURE_TIME_FORMAT *timeFormat);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetSigningTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSigningTimeFormat )( 
            __RPC__in IXpsSigningOptions * This,
            /* [in] */ OPC_SIGNATURE_TIME_FORMAT timeFormat);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetCustomObjects)
        HRESULT ( STDMETHODCALLTYPE *GetCustomObjects )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectSet **customObjectSet);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetCustomReferences)
        HRESULT ( STDMETHODCALLTYPE *GetCustomReferences )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceSet **customReferenceSet);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetCertificateSet)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateSet )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcCertificateSet **certificateSet);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in IXpsSigningOptions * This,
            /* [retval][out] */ __RPC__out XPS_SIGN_FLAGS *flags);
        
        DECLSPEC_XFGVIRT(IXpsSigningOptions, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            __RPC__in IXpsSigningOptions * This,
            /* [in] */ XPS_SIGN_FLAGS flags);
        
        END_INTERFACE
    } IXpsSigningOptionsVtbl;

    interface IXpsSigningOptions
    {
        CONST_VTBL struct IXpsSigningOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSigningOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSigningOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSigningOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSigningOptions_GetSignatureId(This,signatureId)	\
    ( (This)->lpVtbl -> GetSignatureId(This,signatureId) ) 

#define IXpsSigningOptions_SetSignatureId(This,signatureId)	\
    ( (This)->lpVtbl -> SetSignatureId(This,signatureId) ) 

#define IXpsSigningOptions_GetSignatureMethod(This,signatureMethod)	\
    ( (This)->lpVtbl -> GetSignatureMethod(This,signatureMethod) ) 

#define IXpsSigningOptions_SetSignatureMethod(This,signatureMethod)	\
    ( (This)->lpVtbl -> SetSignatureMethod(This,signatureMethod) ) 

#define IXpsSigningOptions_GetDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> GetDigestMethod(This,digestMethod) ) 

#define IXpsSigningOptions_SetDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> SetDigestMethod(This,digestMethod) ) 

#define IXpsSigningOptions_GetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> GetSignaturePartName(This,signaturePartName) ) 

#define IXpsSigningOptions_SetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> SetSignaturePartName(This,signaturePartName) ) 

#define IXpsSigningOptions_GetPolicy(This,policy)	\
    ( (This)->lpVtbl -> GetPolicy(This,policy) ) 

#define IXpsSigningOptions_SetPolicy(This,policy)	\
    ( (This)->lpVtbl -> SetPolicy(This,policy) ) 

#define IXpsSigningOptions_GetSigningTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> GetSigningTimeFormat(This,timeFormat) ) 

#define IXpsSigningOptions_SetSigningTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> SetSigningTimeFormat(This,timeFormat) ) 

#define IXpsSigningOptions_GetCustomObjects(This,customObjectSet)	\
    ( (This)->lpVtbl -> GetCustomObjects(This,customObjectSet) ) 

#define IXpsSigningOptions_GetCustomReferences(This,customReferenceSet)	\
    ( (This)->lpVtbl -> GetCustomReferences(This,customReferenceSet) ) 

#define IXpsSigningOptions_GetCertificateSet(This,certificateSet)	\
    ( (This)->lpVtbl -> GetCertificateSet(This,certificateSet) ) 

#define IXpsSigningOptions_GetFlags(This,flags)	\
    ( (This)->lpVtbl -> GetFlags(This,flags) ) 

#define IXpsSigningOptions_SetFlags(This,flags)	\
    ( (This)->lpVtbl -> SetFlags(This,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSigningOptions_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureCollection_INTERFACE_DEFINED__
#define __IXpsSignatureCollection_INTERFACE_DEFINED__

/* interface IXpsSignatureCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2D1D95D-ADD2-4DFF-AB27-6B9C645FF322")
    IXpsSignatureCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignature **signature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            UINT32 index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSignatureCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSignatureCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSignatureCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsSignatureCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsSignatureCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsSignatureCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignature **signature);
        
        DECLSPEC_XFGVIRT(IXpsSignatureCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsSignatureCollection * This,
            UINT32 index);
        
        END_INTERFACE
    } IXpsSignatureCollectionVtbl;

    interface IXpsSignatureCollection
    {
        CONST_VTBL struct IXpsSignatureCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsSignatureCollection_GetAt(This,index,signature)	\
    ( (This)->lpVtbl -> GetAt(This,index,signature) ) 

#define IXpsSignatureCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsSignature_INTERFACE_DEFINED__
#define __IXpsSignature_INTERFACE_DEFINED__

/* interface IXpsSignature */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IXpsSignature;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6AE4C93E-1ADE-42FB-898B-3A5658284857")
    IXpsSignature : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSignatureId( 
            /* [retval][string][out] */ LPWSTR *sigId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureValue( 
            /* [size_is][size_is][out] */ UINT8 **signatureHashValue,
            /* [out] */ UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateEnumerator( 
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigningTime( 
            /* [retval][string][out] */ LPWSTR *sigDateTimeString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigningTimeFormat( 
            /* [retval][out] */ OPC_SIGNATURE_TIME_FORMAT *timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartName( 
            /* [retval][out] */ IOpcPartUri **signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Verify( 
            /* [in] */ const CERT_CONTEXT *x509Certificate,
            /* [retval][out] */ XPS_SIGNATURE_STATUS *sigStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPolicy( 
            /* [retval][out] */ XPS_SIGN_POLICY *policy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomObjectEnumerator( 
            /* [retval][out] */ IOpcSignatureCustomObjectEnumerator **customObjectEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomReferenceEnumerator( 
            /* [retval][out] */ IOpcSignatureReferenceEnumerator **customReferenceEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureXml( 
            /* [size_is][size_is][out] */ UINT8 **signatureXml,
            /* [out] */ UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureXml( 
            /* [size_is][in] */ const UINT8 *signatureXml,
            /* [in] */ UINT32 count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsSignature * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsSignature * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsSignature * This);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureId )( 
            IXpsSignature * This,
            /* [retval][string][out] */ LPWSTR *sigId);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSignatureValue)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureValue )( 
            IXpsSignature * This,
            /* [size_is][size_is][out] */ UINT8 **signatureHashValue,
            /* [out] */ UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetCertificateEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateEnumerator )( 
            IXpsSignature * This,
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSigningTime)
        HRESULT ( STDMETHODCALLTYPE *GetSigningTime )( 
            IXpsSignature * This,
            /* [retval][string][out] */ LPWSTR *sigDateTimeString);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSigningTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetSigningTimeFormat )( 
            IXpsSignature * This,
            /* [retval][out] */ OPC_SIGNATURE_TIME_FORMAT *timeFormat);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartName )( 
            IXpsSignature * This,
            /* [retval][out] */ IOpcPartUri **signaturePartName);
        
        DECLSPEC_XFGVIRT(IXpsSignature, Verify)
        HRESULT ( STDMETHODCALLTYPE *Verify )( 
            IXpsSignature * This,
            /* [in] */ const CERT_CONTEXT *x509Certificate,
            /* [retval][out] */ XPS_SIGNATURE_STATUS *sigStatus);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetPolicy )( 
            IXpsSignature * This,
            /* [retval][out] */ XPS_SIGN_POLICY *policy);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetCustomObjectEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCustomObjectEnumerator )( 
            IXpsSignature * This,
            /* [retval][out] */ IOpcSignatureCustomObjectEnumerator **customObjectEnumerator);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetCustomReferenceEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCustomReferenceEnumerator )( 
            IXpsSignature * This,
            /* [retval][out] */ IOpcSignatureReferenceEnumerator **customReferenceEnumerator);
        
        DECLSPEC_XFGVIRT(IXpsSignature, GetSignatureXml)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureXml )( 
            IXpsSignature * This,
            /* [size_is][size_is][out] */ UINT8 **signatureXml,
            /* [out] */ UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsSignature, SetSignatureXml)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureXml )( 
            IXpsSignature * This,
            /* [size_is][in] */ const UINT8 *signatureXml,
            /* [in] */ UINT32 count);
        
        END_INTERFACE
    } IXpsSignatureVtbl;

    interface IXpsSignature
    {
        CONST_VTBL struct IXpsSignatureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignature_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignature_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignature_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignature_GetSignatureId(This,sigId)	\
    ( (This)->lpVtbl -> GetSignatureId(This,sigId) ) 

#define IXpsSignature_GetSignatureValue(This,signatureHashValue,count)	\
    ( (This)->lpVtbl -> GetSignatureValue(This,signatureHashValue,count) ) 

#define IXpsSignature_GetCertificateEnumerator(This,certificateEnumerator)	\
    ( (This)->lpVtbl -> GetCertificateEnumerator(This,certificateEnumerator) ) 

#define IXpsSignature_GetSigningTime(This,sigDateTimeString)	\
    ( (This)->lpVtbl -> GetSigningTime(This,sigDateTimeString) ) 

#define IXpsSignature_GetSigningTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> GetSigningTimeFormat(This,timeFormat) ) 

#define IXpsSignature_GetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> GetSignaturePartName(This,signaturePartName) ) 

#define IXpsSignature_Verify(This,x509Certificate,sigStatus)	\
    ( (This)->lpVtbl -> Verify(This,x509Certificate,sigStatus) ) 

#define IXpsSignature_GetPolicy(This,policy)	\
    ( (This)->lpVtbl -> GetPolicy(This,policy) ) 

#define IXpsSignature_GetCustomObjectEnumerator(This,customObjectEnumerator)	\
    ( (This)->lpVtbl -> GetCustomObjectEnumerator(This,customObjectEnumerator) ) 

#define IXpsSignature_GetCustomReferenceEnumerator(This,customReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetCustomReferenceEnumerator(This,customReferenceEnumerator) ) 

#define IXpsSignature_GetSignatureXml(This,signatureXml,count)	\
    ( (This)->lpVtbl -> GetSignatureXml(This,signatureXml,count) ) 

#define IXpsSignature_SetSignatureXml(This,signatureXml,count)	\
    ( (This)->lpVtbl -> SetSignatureXml(This,signatureXml,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignature_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureBlockCollection_INTERFACE_DEFINED__
#define __IXpsSignatureBlockCollection_INTERFACE_DEFINED__

/* interface IXpsSignatureBlockCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureBlockCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23397050-FE99-467A-8DCE-9237F074FFE4")
    IXpsSignatureBlockCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureBlock **signatureBlock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            UINT32 index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureBlockCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSignatureBlockCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSignatureBlockCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSignatureBlockCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlockCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsSignatureBlockCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlockCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsSignatureBlockCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureBlock **signatureBlock);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlockCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsSignatureBlockCollection * This,
            UINT32 index);
        
        END_INTERFACE
    } IXpsSignatureBlockCollectionVtbl;

    interface IXpsSignatureBlockCollection
    {
        CONST_VTBL struct IXpsSignatureBlockCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureBlockCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureBlockCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureBlockCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureBlockCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsSignatureBlockCollection_GetAt(This,index,signatureBlock)	\
    ( (This)->lpVtbl -> GetAt(This,index,signatureBlock) ) 

#define IXpsSignatureBlockCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureBlockCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureBlock_INTERFACE_DEFINED__
#define __IXpsSignatureBlock_INTERFACE_DEFINED__

/* interface IXpsSignatureBlock */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureBlock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("151FAC09-0B97-4AC6-A323-5E4297D4322B")
    IXpsSignatureBlock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRequests( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequestCollection **requests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPartName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentIndex( 
            /* [retval][out] */ __RPC__out UINT32 *fixedDocumentIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **fixedDocumentName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRequest( 
            /* [string][in] */ __RPC__in_string LPCWSTR requestId,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequest **signatureRequest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureBlockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSignatureBlock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSignatureBlock * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlock, GetRequests)
        HRESULT ( STDMETHODCALLTYPE *GetRequests )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequestCollection **requests);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlock, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlock, GetDocumentIndex)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentIndex )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [retval][out] */ __RPC__out UINT32 *fixedDocumentIndex);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlock, GetDocumentName)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentName )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **fixedDocumentName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureBlock, CreateRequest)
        HRESULT ( STDMETHODCALLTYPE *CreateRequest )( 
            __RPC__in IXpsSignatureBlock * This,
            /* [string][in] */ __RPC__in_string LPCWSTR requestId,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequest **signatureRequest);
        
        END_INTERFACE
    } IXpsSignatureBlockVtbl;

    interface IXpsSignatureBlock
    {
        CONST_VTBL struct IXpsSignatureBlockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureBlock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureBlock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureBlock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureBlock_GetRequests(This,requests)	\
    ( (This)->lpVtbl -> GetRequests(This,requests) ) 

#define IXpsSignatureBlock_GetPartName(This,partName)	\
    ( (This)->lpVtbl -> GetPartName(This,partName) ) 

#define IXpsSignatureBlock_GetDocumentIndex(This,fixedDocumentIndex)	\
    ( (This)->lpVtbl -> GetDocumentIndex(This,fixedDocumentIndex) ) 

#define IXpsSignatureBlock_GetDocumentName(This,fixedDocumentName)	\
    ( (This)->lpVtbl -> GetDocumentName(This,fixedDocumentName) ) 

#define IXpsSignatureBlock_CreateRequest(This,requestId,signatureRequest)	\
    ( (This)->lpVtbl -> CreateRequest(This,requestId,signatureRequest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureBlock_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureRequestCollection_INTERFACE_DEFINED__
#define __IXpsSignatureRequestCollection_INTERFACE_DEFINED__

/* interface IXpsSignatureRequestCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureRequestCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0253E68-9F19-412E-9B4F-54D3B0AC6CD9")
    IXpsSignatureRequestCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequest **signatureRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureRequestCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSignatureRequestCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSignatureRequestCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSignatureRequestCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequestCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsSignatureRequestCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequestCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsSignatureRequestCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignatureRequest **signatureRequest);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequestCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsSignatureRequestCollection * This,
            /* [in] */ UINT32 index);
        
        END_INTERFACE
    } IXpsSignatureRequestCollectionVtbl;

    interface IXpsSignatureRequestCollection
    {
        CONST_VTBL struct IXpsSignatureRequestCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureRequestCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureRequestCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureRequestCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureRequestCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsSignatureRequestCollection_GetAt(This,index,signatureRequest)	\
    ( (This)->lpVtbl -> GetAt(This,index,signatureRequest) ) 

#define IXpsSignatureRequestCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureRequestCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureRequest_INTERFACE_DEFINED__
#define __IXpsSignatureRequest_INTERFACE_DEFINED__

/* interface IXpsSignatureRequest */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ac58950b-7208-4b2d-b2c4-951083d3b8eb")
    IXpsSignatureRequest : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIntent( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *intent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIntent( 
            /* [string][in] */ __RPC__in_string LPCWSTR intent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestedSigner( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signerName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRequestedSigner( 
            /* [string][in] */ __RPC__in_string LPCWSTR signerName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestSignByDate( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *dateString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRequestSignByDate( 
            /* [string][in] */ __RPC__in_string LPCWSTR dateString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigningLocale( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *place) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSigningLocale( 
            /* [string][in] */ __RPC__in_string LPCWSTR place) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpotLocation( 
            /* [out] */ __RPC__out INT32 *pageIndex,
            /* [out] */ __RPC__deref_out_opt IOpcPartUri **pagePartName,
            /* [out] */ __RPC__out float *x,
            /* [out] */ __RPC__out float *y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSpotLocation( 
            /* [in] */ INT32 pageIndex,
            /* [in] */ float x,
            /* [in] */ float y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestId( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *requestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignature( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignature **signature) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsSignatureRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsSignatureRequest * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetIntent)
        HRESULT ( STDMETHODCALLTYPE *GetIntent )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *intent);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, SetIntent)
        HRESULT ( STDMETHODCALLTYPE *SetIntent )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [string][in] */ __RPC__in_string LPCWSTR intent);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetRequestedSigner)
        HRESULT ( STDMETHODCALLTYPE *GetRequestedSigner )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signerName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, SetRequestedSigner)
        HRESULT ( STDMETHODCALLTYPE *SetRequestedSigner )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [string][in] */ __RPC__in_string LPCWSTR signerName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetRequestSignByDate)
        HRESULT ( STDMETHODCALLTYPE *GetRequestSignByDate )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *dateString);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, SetRequestSignByDate)
        HRESULT ( STDMETHODCALLTYPE *SetRequestSignByDate )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [string][in] */ __RPC__in_string LPCWSTR dateString);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetSigningLocale)
        HRESULT ( STDMETHODCALLTYPE *GetSigningLocale )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *place);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, SetSigningLocale)
        HRESULT ( STDMETHODCALLTYPE *SetSigningLocale )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [string][in] */ __RPC__in_string LPCWSTR place);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetSpotLocation)
        HRESULT ( STDMETHODCALLTYPE *GetSpotLocation )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [out] */ __RPC__out INT32 *pageIndex,
            /* [out] */ __RPC__deref_out_opt IOpcPartUri **pagePartName,
            /* [out] */ __RPC__out float *x,
            /* [out] */ __RPC__out float *y);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, SetSpotLocation)
        HRESULT ( STDMETHODCALLTYPE *SetSpotLocation )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [in] */ INT32 pageIndex,
            /* [in] */ float x,
            /* [in] */ float y);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetRequestId)
        HRESULT ( STDMETHODCALLTYPE *GetRequestId )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *requestId);
        
        DECLSPEC_XFGVIRT(IXpsSignatureRequest, GetSignature)
        HRESULT ( STDMETHODCALLTYPE *GetSignature )( 
            __RPC__in IXpsSignatureRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsSignature **signature);
        
        END_INTERFACE
    } IXpsSignatureRequestVtbl;

    interface IXpsSignatureRequest
    {
        CONST_VTBL struct IXpsSignatureRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureRequest_GetIntent(This,intent)	\
    ( (This)->lpVtbl -> GetIntent(This,intent) ) 

#define IXpsSignatureRequest_SetIntent(This,intent)	\
    ( (This)->lpVtbl -> SetIntent(This,intent) ) 

#define IXpsSignatureRequest_GetRequestedSigner(This,signerName)	\
    ( (This)->lpVtbl -> GetRequestedSigner(This,signerName) ) 

#define IXpsSignatureRequest_SetRequestedSigner(This,signerName)	\
    ( (This)->lpVtbl -> SetRequestedSigner(This,signerName) ) 

#define IXpsSignatureRequest_GetRequestSignByDate(This,dateString)	\
    ( (This)->lpVtbl -> GetRequestSignByDate(This,dateString) ) 

#define IXpsSignatureRequest_SetRequestSignByDate(This,dateString)	\
    ( (This)->lpVtbl -> SetRequestSignByDate(This,dateString) ) 

#define IXpsSignatureRequest_GetSigningLocale(This,place)	\
    ( (This)->lpVtbl -> GetSigningLocale(This,place) ) 

#define IXpsSignatureRequest_SetSigningLocale(This,place)	\
    ( (This)->lpVtbl -> SetSigningLocale(This,place) ) 

#define IXpsSignatureRequest_GetSpotLocation(This,pageIndex,pagePartName,x,y)	\
    ( (This)->lpVtbl -> GetSpotLocation(This,pageIndex,pagePartName,x,y) ) 

#define IXpsSignatureRequest_SetSpotLocation(This,pageIndex,x,y)	\
    ( (This)->lpVtbl -> SetSpotLocation(This,pageIndex,x,y) ) 

#define IXpsSignatureRequest_GetRequestId(This,requestId)	\
    ( (This)->lpVtbl -> GetRequestId(This,requestId) ) 

#define IXpsSignatureRequest_GetSignature(This,signature)	\
    ( (This)->lpVtbl -> GetSignature(This,signature) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureRequest_INTERFACE_DEFINED__ */


#ifndef __IXpsSignatureManager_INTERFACE_DEFINED__
#define __IXpsSignatureManager_INTERFACE_DEFINED__

/* interface IXpsSignatureManager */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IXpsSignatureManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d3e8d338-fdc4-4afc-80b5-d532a1782ee1")
    IXpsSignatureManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadPackageFile( 
            /* [string][in] */ LPCWSTR fileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadPackageStream( 
            /* [in] */ IStream *stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Sign( 
            /* [in] */ IXpsSigningOptions *signOptions,
            /* [in] */ const CERT_CONTEXT *x509Certificate,
            /* [retval][out] */ IXpsSignature **signature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureOriginPartName( 
            /* [retval][out] */ IOpcPartUri **signatureOriginPartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureOriginPartName( 
            /* [in] */ IOpcPartUri *signatureOriginPartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatures( 
            /* [retval][out] */ IXpsSignatureCollection **signatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSignatureBlock( 
            /* [in] */ IOpcPartUri *partName,
            /* [in] */ UINT32 fixedDocumentIndex,
            /* [retval][out] */ IXpsSignatureBlock **signatureBlock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureBlocks( 
            /* [retval][out] */ IXpsSignatureBlockCollection **signatureBlocks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSigningOptions( 
            /* [retval][out] */ IXpsSigningOptions **signingOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SavePackageToFile( 
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SavePackageToStream( 
            /* [in] */ IStream *stream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsSignatureManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsSignatureManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsSignatureManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsSignatureManager * This);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, LoadPackageFile)
        HRESULT ( STDMETHODCALLTYPE *LoadPackageFile )( 
            IXpsSignatureManager * This,
            /* [string][in] */ LPCWSTR fileName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, LoadPackageStream)
        HRESULT ( STDMETHODCALLTYPE *LoadPackageStream )( 
            IXpsSignatureManager * This,
            /* [in] */ IStream *stream);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, Sign)
        HRESULT ( STDMETHODCALLTYPE *Sign )( 
            IXpsSignatureManager * This,
            /* [in] */ IXpsSigningOptions *signOptions,
            /* [in] */ const CERT_CONTEXT *x509Certificate,
            /* [retval][out] */ IXpsSignature **signature);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, GetSignatureOriginPartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureOriginPartName )( 
            IXpsSignatureManager * This,
            /* [retval][out] */ IOpcPartUri **signatureOriginPartName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, SetSignatureOriginPartName)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureOriginPartName )( 
            IXpsSignatureManager * This,
            /* [in] */ IOpcPartUri *signatureOriginPartName);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, GetSignatures)
        HRESULT ( STDMETHODCALLTYPE *GetSignatures )( 
            IXpsSignatureManager * This,
            /* [retval][out] */ IXpsSignatureCollection **signatures);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, AddSignatureBlock)
        HRESULT ( STDMETHODCALLTYPE *AddSignatureBlock )( 
            IXpsSignatureManager * This,
            /* [in] */ IOpcPartUri *partName,
            /* [in] */ UINT32 fixedDocumentIndex,
            /* [retval][out] */ IXpsSignatureBlock **signatureBlock);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, GetSignatureBlocks)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureBlocks )( 
            IXpsSignatureManager * This,
            /* [retval][out] */ IXpsSignatureBlockCollection **signatureBlocks);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, CreateSigningOptions)
        HRESULT ( STDMETHODCALLTYPE *CreateSigningOptions )( 
            IXpsSignatureManager * This,
            /* [retval][out] */ IXpsSigningOptions **signingOptions);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, SavePackageToFile)
        HRESULT ( STDMETHODCALLTYPE *SavePackageToFile )( 
            IXpsSignatureManager * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes);
        
        DECLSPEC_XFGVIRT(IXpsSignatureManager, SavePackageToStream)
        HRESULT ( STDMETHODCALLTYPE *SavePackageToStream )( 
            IXpsSignatureManager * This,
            /* [in] */ IStream *stream);
        
        END_INTERFACE
    } IXpsSignatureManagerVtbl;

    interface IXpsSignatureManager
    {
        CONST_VTBL struct IXpsSignatureManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsSignatureManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsSignatureManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsSignatureManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsSignatureManager_LoadPackageFile(This,fileName)	\
    ( (This)->lpVtbl -> LoadPackageFile(This,fileName) ) 

#define IXpsSignatureManager_LoadPackageStream(This,stream)	\
    ( (This)->lpVtbl -> LoadPackageStream(This,stream) ) 

#define IXpsSignatureManager_Sign(This,signOptions,x509Certificate,signature)	\
    ( (This)->lpVtbl -> Sign(This,signOptions,x509Certificate,signature) ) 

#define IXpsSignatureManager_GetSignatureOriginPartName(This,signatureOriginPartName)	\
    ( (This)->lpVtbl -> GetSignatureOriginPartName(This,signatureOriginPartName) ) 

#define IXpsSignatureManager_SetSignatureOriginPartName(This,signatureOriginPartName)	\
    ( (This)->lpVtbl -> SetSignatureOriginPartName(This,signatureOriginPartName) ) 

#define IXpsSignatureManager_GetSignatures(This,signatures)	\
    ( (This)->lpVtbl -> GetSignatures(This,signatures) ) 

#define IXpsSignatureManager_AddSignatureBlock(This,partName,fixedDocumentIndex,signatureBlock)	\
    ( (This)->lpVtbl -> AddSignatureBlock(This,partName,fixedDocumentIndex,signatureBlock) ) 

#define IXpsSignatureManager_GetSignatureBlocks(This,signatureBlocks)	\
    ( (This)->lpVtbl -> GetSignatureBlocks(This,signatureBlocks) ) 

#define IXpsSignatureManager_CreateSigningOptions(This,signingOptions)	\
    ( (This)->lpVtbl -> CreateSigningOptions(This,signingOptions) ) 

#define IXpsSignatureManager_SavePackageToFile(This,fileName,securityAttributes,flagsAndAttributes)	\
    ( (This)->lpVtbl -> SavePackageToFile(This,fileName,securityAttributes,flagsAndAttributes) ) 

#define IXpsSignatureManager_SavePackageToStream(This,stream)	\
    ( (This)->lpVtbl -> SavePackageToStream(This,stream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsSignatureManager_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_XpsSignatureManager;

#ifdef __cplusplus

class DECLSPEC_UUID("b0c43320-2315-44a2-b70a-0943a140a8ee")
XpsSignatureManager;
#endif
#endif /* __MSXPSSIG_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_xpsdigitalsignature_0000_0001 */
/* [local] */ 

#define XPS_E_SIGREQUESTID_DUP               MAKE_HRESULT(1, FACILITY_XPS, 901) 
#define XPS_E_PACKAGE_NOT_OPENED             MAKE_HRESULT(1, FACILITY_XPS, 902) 
#define XPS_E_PACKAGE_ALREADY_OPENED         MAKE_HRESULT(1, FACILITY_XPS, 903) 
#define XPS_E_SIGNATUREID_DUP                MAKE_HRESULT(1, FACILITY_XPS, 904) 
#define XPS_E_MARKUP_COMPATIBILITY_ELEMENTS  MAKE_HRESULT(1, FACILITY_XPS, 905) 
#define XPS_E_OBJECT_DETACHED                MAKE_HRESULT(1, FACILITY_XPS, 906) 
#define XPS_E_INVALID_SIGNATUREBLOCK_MARKUP  MAKE_HRESULT(1, FACILITY_XPS, 907) 
#endif // (NTDDI >= NTDDI_WIN7)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xpsdigitalsignature_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsdigitalsignature_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



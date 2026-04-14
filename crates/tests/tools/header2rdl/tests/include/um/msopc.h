

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

#ifndef __msopc_h__
#define __msopc_h__

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

#ifndef __IOpcUri_FWD_DEFINED__
#define __IOpcUri_FWD_DEFINED__
typedef interface IOpcUri IOpcUri;

#endif 	/* __IOpcUri_FWD_DEFINED__ */


#ifndef __IOpcPartUri_FWD_DEFINED__
#define __IOpcPartUri_FWD_DEFINED__
typedef interface IOpcPartUri IOpcPartUri;

#endif 	/* __IOpcPartUri_FWD_DEFINED__ */


#ifndef __IOpcUri_FWD_DEFINED__
#define __IOpcUri_FWD_DEFINED__
typedef interface IOpcUri IOpcUri;

#endif 	/* __IOpcUri_FWD_DEFINED__ */


#ifndef __IOpcPartUri_FWD_DEFINED__
#define __IOpcPartUri_FWD_DEFINED__
typedef interface IOpcPartUri IOpcPartUri;

#endif 	/* __IOpcPartUri_FWD_DEFINED__ */


#ifndef __IOpcPackage_FWD_DEFINED__
#define __IOpcPackage_FWD_DEFINED__
typedef interface IOpcPackage IOpcPackage;

#endif 	/* __IOpcPackage_FWD_DEFINED__ */


#ifndef __IOpcPart_FWD_DEFINED__
#define __IOpcPart_FWD_DEFINED__
typedef interface IOpcPart IOpcPart;

#endif 	/* __IOpcPart_FWD_DEFINED__ */


#ifndef __IOpcRelationship_FWD_DEFINED__
#define __IOpcRelationship_FWD_DEFINED__
typedef interface IOpcRelationship IOpcRelationship;

#endif 	/* __IOpcRelationship_FWD_DEFINED__ */


#ifndef __IOpcPartSet_FWD_DEFINED__
#define __IOpcPartSet_FWD_DEFINED__
typedef interface IOpcPartSet IOpcPartSet;

#endif 	/* __IOpcPartSet_FWD_DEFINED__ */


#ifndef __IOpcRelationshipSet_FWD_DEFINED__
#define __IOpcRelationshipSet_FWD_DEFINED__
typedef interface IOpcRelationshipSet IOpcRelationshipSet;

#endif 	/* __IOpcRelationshipSet_FWD_DEFINED__ */


#ifndef __IOpcPartEnumerator_FWD_DEFINED__
#define __IOpcPartEnumerator_FWD_DEFINED__
typedef interface IOpcPartEnumerator IOpcPartEnumerator;

#endif 	/* __IOpcPartEnumerator_FWD_DEFINED__ */


#ifndef __IOpcRelationshipEnumerator_FWD_DEFINED__
#define __IOpcRelationshipEnumerator_FWD_DEFINED__
typedef interface IOpcRelationshipEnumerator IOpcRelationshipEnumerator;

#endif 	/* __IOpcRelationshipEnumerator_FWD_DEFINED__ */


#ifndef __IOpcSignaturePartReference_FWD_DEFINED__
#define __IOpcSignaturePartReference_FWD_DEFINED__
typedef interface IOpcSignaturePartReference IOpcSignaturePartReference;

#endif 	/* __IOpcSignaturePartReference_FWD_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReference_FWD_DEFINED__
#define __IOpcSignatureRelationshipReference_FWD_DEFINED__
typedef interface IOpcSignatureRelationshipReference IOpcSignatureRelationshipReference;

#endif 	/* __IOpcSignatureRelationshipReference_FWD_DEFINED__ */


#ifndef __IOpcRelationshipSelector_FWD_DEFINED__
#define __IOpcRelationshipSelector_FWD_DEFINED__
typedef interface IOpcRelationshipSelector IOpcRelationshipSelector;

#endif 	/* __IOpcRelationshipSelector_FWD_DEFINED__ */


#ifndef __IOpcSignatureReference_FWD_DEFINED__
#define __IOpcSignatureReference_FWD_DEFINED__
typedef interface IOpcSignatureReference IOpcSignatureReference;

#endif 	/* __IOpcSignatureReference_FWD_DEFINED__ */


#ifndef __IOpcSignatureCustomObject_FWD_DEFINED__
#define __IOpcSignatureCustomObject_FWD_DEFINED__
typedef interface IOpcSignatureCustomObject IOpcSignatureCustomObject;

#endif 	/* __IOpcSignatureCustomObject_FWD_DEFINED__ */


#ifndef __IOpcDigitalSignature_FWD_DEFINED__
#define __IOpcDigitalSignature_FWD_DEFINED__
typedef interface IOpcDigitalSignature IOpcDigitalSignature;

#endif 	/* __IOpcDigitalSignature_FWD_DEFINED__ */


#ifndef __IOpcSigningOptions_FWD_DEFINED__
#define __IOpcSigningOptions_FWD_DEFINED__
typedef interface IOpcSigningOptions IOpcSigningOptions;

#endif 	/* __IOpcSigningOptions_FWD_DEFINED__ */


#ifndef __IOpcDigitalSignatureManager_FWD_DEFINED__
#define __IOpcDigitalSignatureManager_FWD_DEFINED__
typedef interface IOpcDigitalSignatureManager IOpcDigitalSignatureManager;

#endif 	/* __IOpcDigitalSignatureManager_FWD_DEFINED__ */


#ifndef __IOpcSignaturePartReferenceEnumerator_FWD_DEFINED__
#define __IOpcSignaturePartReferenceEnumerator_FWD_DEFINED__
typedef interface IOpcSignaturePartReferenceEnumerator IOpcSignaturePartReferenceEnumerator;

#endif 	/* __IOpcSignaturePartReferenceEnumerator_FWD_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReferenceEnumerator_FWD_DEFINED__
#define __IOpcSignatureRelationshipReferenceEnumerator_FWD_DEFINED__
typedef interface IOpcSignatureRelationshipReferenceEnumerator IOpcSignatureRelationshipReferenceEnumerator;

#endif 	/* __IOpcSignatureRelationshipReferenceEnumerator_FWD_DEFINED__ */


#ifndef __IOpcRelationshipSelectorEnumerator_FWD_DEFINED__
#define __IOpcRelationshipSelectorEnumerator_FWD_DEFINED__
typedef interface IOpcRelationshipSelectorEnumerator IOpcRelationshipSelectorEnumerator;

#endif 	/* __IOpcRelationshipSelectorEnumerator_FWD_DEFINED__ */


#ifndef __IOpcSignatureReferenceEnumerator_FWD_DEFINED__
#define __IOpcSignatureReferenceEnumerator_FWD_DEFINED__
typedef interface IOpcSignatureReferenceEnumerator IOpcSignatureReferenceEnumerator;

#endif 	/* __IOpcSignatureReferenceEnumerator_FWD_DEFINED__ */


#ifndef __IOpcSignatureCustomObjectEnumerator_FWD_DEFINED__
#define __IOpcSignatureCustomObjectEnumerator_FWD_DEFINED__
typedef interface IOpcSignatureCustomObjectEnumerator IOpcSignatureCustomObjectEnumerator;

#endif 	/* __IOpcSignatureCustomObjectEnumerator_FWD_DEFINED__ */


#ifndef __IOpcCertificateEnumerator_FWD_DEFINED__
#define __IOpcCertificateEnumerator_FWD_DEFINED__
typedef interface IOpcCertificateEnumerator IOpcCertificateEnumerator;

#endif 	/* __IOpcCertificateEnumerator_FWD_DEFINED__ */


#ifndef __IOpcDigitalSignatureEnumerator_FWD_DEFINED__
#define __IOpcDigitalSignatureEnumerator_FWD_DEFINED__
typedef interface IOpcDigitalSignatureEnumerator IOpcDigitalSignatureEnumerator;

#endif 	/* __IOpcDigitalSignatureEnumerator_FWD_DEFINED__ */


#ifndef __IOpcSignaturePartReferenceSet_FWD_DEFINED__
#define __IOpcSignaturePartReferenceSet_FWD_DEFINED__
typedef interface IOpcSignaturePartReferenceSet IOpcSignaturePartReferenceSet;

#endif 	/* __IOpcSignaturePartReferenceSet_FWD_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReferenceSet_FWD_DEFINED__
#define __IOpcSignatureRelationshipReferenceSet_FWD_DEFINED__
typedef interface IOpcSignatureRelationshipReferenceSet IOpcSignatureRelationshipReferenceSet;

#endif 	/* __IOpcSignatureRelationshipReferenceSet_FWD_DEFINED__ */


#ifndef __IOpcRelationshipSelectorSet_FWD_DEFINED__
#define __IOpcRelationshipSelectorSet_FWD_DEFINED__
typedef interface IOpcRelationshipSelectorSet IOpcRelationshipSelectorSet;

#endif 	/* __IOpcRelationshipSelectorSet_FWD_DEFINED__ */


#ifndef __IOpcSignatureReferenceSet_FWD_DEFINED__
#define __IOpcSignatureReferenceSet_FWD_DEFINED__
typedef interface IOpcSignatureReferenceSet IOpcSignatureReferenceSet;

#endif 	/* __IOpcSignatureReferenceSet_FWD_DEFINED__ */


#ifndef __IOpcSignatureCustomObjectSet_FWD_DEFINED__
#define __IOpcSignatureCustomObjectSet_FWD_DEFINED__
typedef interface IOpcSignatureCustomObjectSet IOpcSignatureCustomObjectSet;

#endif 	/* __IOpcSignatureCustomObjectSet_FWD_DEFINED__ */


#ifndef __IOpcCertificateSet_FWD_DEFINED__
#define __IOpcCertificateSet_FWD_DEFINED__
typedef interface IOpcCertificateSet IOpcCertificateSet;

#endif 	/* __IOpcCertificateSet_FWD_DEFINED__ */


#ifndef __IOpcFactory_FWD_DEFINED__
#define __IOpcFactory_FWD_DEFINED__
typedef interface IOpcFactory IOpcFactory;

#endif 	/* __IOpcFactory_FWD_DEFINED__ */


#ifndef __OpcFactory_FWD_DEFINED__
#define __OpcFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class OpcFactory OpcFactory;
#else
typedef struct OpcFactory OpcFactory;
#endif /* __cplusplus */

#endif 	/* __OpcFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wincrypt.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msopc_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN7)
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (NTDDI_VERSION >= NTDDI_WIN7)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)




extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0000_v0_0_s_ifspec;

#ifndef __IOpcUri_INTERFACE_DEFINED__
#define __IOpcUri_INTERFACE_DEFINED__

/* interface IOpcUri */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcUri;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc9c1b9b-d62c-49eb-aef0-3b4e0b28ebed")
    IOpcUri : public IUri
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipsPartUri( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **relationshipPartUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelativeUri( 
            /* [in] */ __RPC__in_opt IOpcPartUri *targetPartUri,
            /* [retval][out] */ __RPC__deref_out_opt IUri **relativeUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CombinePartUri( 
            /* [in] */ __RPC__in_opt IUri *relativeUri,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **combinedUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcUriVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcUri * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcUri * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcUri * This);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyBSTR)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyBSTR )( 
            __RPC__in IOpcUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyLength)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyLength )( 
            __RPC__in IOpcUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pcchProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyDWORD)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyDWORD )( 
            __RPC__in IOpcUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pdwProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, HasProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HasProperty )( 
            __RPC__in IOpcUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out BOOL *pfHasProperty);
        
        DECLSPEC_XFGVIRT(IUri, GetAbsoluteUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAbsoluteUri )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAbsoluteUri);
        
        DECLSPEC_XFGVIRT(IUri, GetAuthority)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAuthority )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAuthority);
        
        DECLSPEC_XFGVIRT(IUri, GetDisplayUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayUri )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayString);
        
        DECLSPEC_XFGVIRT(IUri, GetDomain)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDomain )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDomain);
        
        DECLSPEC_XFGVIRT(IUri, GetExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetExtension )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrExtension);
        
        DECLSPEC_XFGVIRT(IUri, GetFragment)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFragment )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFragment);
        
        DECLSPEC_XFGVIRT(IUri, GetHost)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHost )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHost);
        
        DECLSPEC_XFGVIRT(IUri, GetPassword)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPassword )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPassword);
        
        DECLSPEC_XFGVIRT(IUri, GetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPath);
        
        DECLSPEC_XFGVIRT(IUri, GetPathAndQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPathAndQuery )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPathAndQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetQuery )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetRawUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRawUri )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRawUri);
        
        DECLSPEC_XFGVIRT(IUri, GetSchemeName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemeName )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSchemeName);
        
        DECLSPEC_XFGVIRT(IUri, GetUserInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserInfo )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserInfo);
        
        DECLSPEC_XFGVIRT(IUri, GetUserName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserName )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IUri, GetHostType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHostType )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__out DWORD *pdwHostType);
        
        DECLSPEC_XFGVIRT(IUri, GetPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__out DWORD *pdwPort);
        
        DECLSPEC_XFGVIRT(IUri, GetScheme)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetScheme )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__out DWORD *pdwScheme);
        
        DECLSPEC_XFGVIRT(IUri, GetZone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetZone )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__out DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IUri, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IOpcUri * This,
            /* [out] */ __RPC__out LPDWORD pdwFlags);
        
        DECLSPEC_XFGVIRT(IUri, IsEqual)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IOpcUri * This,
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(IOpcUri, GetRelationshipsPartUri)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipsPartUri )( 
            __RPC__in IOpcUri * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **relationshipPartUri);
        
        DECLSPEC_XFGVIRT(IOpcUri, GetRelativeUri)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeUri )( 
            __RPC__in IOpcUri * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *targetPartUri,
            /* [retval][out] */ __RPC__deref_out_opt IUri **relativeUri);
        
        DECLSPEC_XFGVIRT(IOpcUri, CombinePartUri)
        HRESULT ( STDMETHODCALLTYPE *CombinePartUri )( 
            __RPC__in IOpcUri * This,
            /* [in] */ __RPC__in_opt IUri *relativeUri,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **combinedUri);
        
        END_INTERFACE
    } IOpcUriVtbl;

    interface IOpcUri
    {
        CONST_VTBL struct IOpcUriVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcUri_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcUri_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcUri_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcUri_GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags) ) 

#define IOpcUri_GetPropertyLength(This,uriProp,pcchProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyLength(This,uriProp,pcchProperty,dwFlags) ) 

#define IOpcUri_GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags) ) 

#define IOpcUri_HasProperty(This,uriProp,pfHasProperty)	\
    ( (This)->lpVtbl -> HasProperty(This,uriProp,pfHasProperty) ) 

#define IOpcUri_GetAbsoluteUri(This,pbstrAbsoluteUri)	\
    ( (This)->lpVtbl -> GetAbsoluteUri(This,pbstrAbsoluteUri) ) 

#define IOpcUri_GetAuthority(This,pbstrAuthority)	\
    ( (This)->lpVtbl -> GetAuthority(This,pbstrAuthority) ) 

#define IOpcUri_GetDisplayUri(This,pbstrDisplayString)	\
    ( (This)->lpVtbl -> GetDisplayUri(This,pbstrDisplayString) ) 

#define IOpcUri_GetDomain(This,pbstrDomain)	\
    ( (This)->lpVtbl -> GetDomain(This,pbstrDomain) ) 

#define IOpcUri_GetExtension(This,pbstrExtension)	\
    ( (This)->lpVtbl -> GetExtension(This,pbstrExtension) ) 

#define IOpcUri_GetFragment(This,pbstrFragment)	\
    ( (This)->lpVtbl -> GetFragment(This,pbstrFragment) ) 

#define IOpcUri_GetHost(This,pbstrHost)	\
    ( (This)->lpVtbl -> GetHost(This,pbstrHost) ) 

#define IOpcUri_GetPassword(This,pbstrPassword)	\
    ( (This)->lpVtbl -> GetPassword(This,pbstrPassword) ) 

#define IOpcUri_GetPath(This,pbstrPath)	\
    ( (This)->lpVtbl -> GetPath(This,pbstrPath) ) 

#define IOpcUri_GetPathAndQuery(This,pbstrPathAndQuery)	\
    ( (This)->lpVtbl -> GetPathAndQuery(This,pbstrPathAndQuery) ) 

#define IOpcUri_GetQuery(This,pbstrQuery)	\
    ( (This)->lpVtbl -> GetQuery(This,pbstrQuery) ) 

#define IOpcUri_GetRawUri(This,pbstrRawUri)	\
    ( (This)->lpVtbl -> GetRawUri(This,pbstrRawUri) ) 

#define IOpcUri_GetSchemeName(This,pbstrSchemeName)	\
    ( (This)->lpVtbl -> GetSchemeName(This,pbstrSchemeName) ) 

#define IOpcUri_GetUserInfo(This,pbstrUserInfo)	\
    ( (This)->lpVtbl -> GetUserInfo(This,pbstrUserInfo) ) 

#define IOpcUri_GetUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> GetUserName(This,pbstrUserName) ) 

#define IOpcUri_GetHostType(This,pdwHostType)	\
    ( (This)->lpVtbl -> GetHostType(This,pdwHostType) ) 

#define IOpcUri_GetPort(This,pdwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pdwPort) ) 

#define IOpcUri_GetScheme(This,pdwScheme)	\
    ( (This)->lpVtbl -> GetScheme(This,pdwScheme) ) 

#define IOpcUri_GetZone(This,pdwZone)	\
    ( (This)->lpVtbl -> GetZone(This,pdwZone) ) 

#define IOpcUri_GetProperties(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetProperties(This,pdwFlags) ) 

#define IOpcUri_IsEqual(This,pUri,pfEqual)	\
    ( (This)->lpVtbl -> IsEqual(This,pUri,pfEqual) ) 


#define IOpcUri_GetRelationshipsPartUri(This,relationshipPartUri)	\
    ( (This)->lpVtbl -> GetRelationshipsPartUri(This,relationshipPartUri) ) 

#define IOpcUri_GetRelativeUri(This,targetPartUri,relativeUri)	\
    ( (This)->lpVtbl -> GetRelativeUri(This,targetPartUri,relativeUri) ) 

#define IOpcUri_CombinePartUri(This,relativeUri,combinedUri)	\
    ( (This)->lpVtbl -> CombinePartUri(This,relativeUri,combinedUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcUri_INTERFACE_DEFINED__ */


#ifndef __IOpcPartUri_INTERFACE_DEFINED__
#define __IOpcPartUri_INTERFACE_DEFINED__

/* interface IOpcPartUri */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcPartUri;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7d3babe7-88b2-46ba-85cb-4203cb016c87")
    IOpcPartUri : public IOpcUri
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ComparePartUri( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri,
            /* [retval][out] */ __RPC__out INT32 *comparisonResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceUri( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRelationshipsPartUri( 
            /* [retval][out] */ __RPC__out BOOL *isRelationshipUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcPartUriVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcPartUri * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcPartUri * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcPartUri * This);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyBSTR)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyBSTR )( 
            __RPC__in IOpcPartUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyLength)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyLength )( 
            __RPC__in IOpcPartUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pcchProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyDWORD)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyDWORD )( 
            __RPC__in IOpcPartUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pdwProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, HasProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HasProperty )( 
            __RPC__in IOpcPartUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out BOOL *pfHasProperty);
        
        DECLSPEC_XFGVIRT(IUri, GetAbsoluteUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAbsoluteUri )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAbsoluteUri);
        
        DECLSPEC_XFGVIRT(IUri, GetAuthority)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAuthority )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAuthority);
        
        DECLSPEC_XFGVIRT(IUri, GetDisplayUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayUri )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayString);
        
        DECLSPEC_XFGVIRT(IUri, GetDomain)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDomain )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDomain);
        
        DECLSPEC_XFGVIRT(IUri, GetExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetExtension )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrExtension);
        
        DECLSPEC_XFGVIRT(IUri, GetFragment)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFragment )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFragment);
        
        DECLSPEC_XFGVIRT(IUri, GetHost)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHost )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHost);
        
        DECLSPEC_XFGVIRT(IUri, GetPassword)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPassword )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPassword);
        
        DECLSPEC_XFGVIRT(IUri, GetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPath);
        
        DECLSPEC_XFGVIRT(IUri, GetPathAndQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPathAndQuery )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPathAndQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetQuery )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetRawUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRawUri )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRawUri);
        
        DECLSPEC_XFGVIRT(IUri, GetSchemeName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemeName )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSchemeName);
        
        DECLSPEC_XFGVIRT(IUri, GetUserInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserInfo )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserInfo);
        
        DECLSPEC_XFGVIRT(IUri, GetUserName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserName )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IUri, GetHostType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHostType )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__out DWORD *pdwHostType);
        
        DECLSPEC_XFGVIRT(IUri, GetPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__out DWORD *pdwPort);
        
        DECLSPEC_XFGVIRT(IUri, GetScheme)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetScheme )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__out DWORD *pdwScheme);
        
        DECLSPEC_XFGVIRT(IUri, GetZone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetZone )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__out DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IUri, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IOpcPartUri * This,
            /* [out] */ __RPC__out LPDWORD pdwFlags);
        
        DECLSPEC_XFGVIRT(IUri, IsEqual)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IOpcPartUri * This,
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(IOpcUri, GetRelationshipsPartUri)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipsPartUri )( 
            __RPC__in IOpcPartUri * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **relationshipPartUri);
        
        DECLSPEC_XFGVIRT(IOpcUri, GetRelativeUri)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeUri )( 
            __RPC__in IOpcPartUri * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *targetPartUri,
            /* [retval][out] */ __RPC__deref_out_opt IUri **relativeUri);
        
        DECLSPEC_XFGVIRT(IOpcUri, CombinePartUri)
        HRESULT ( STDMETHODCALLTYPE *CombinePartUri )( 
            __RPC__in IOpcPartUri * This,
            /* [in] */ __RPC__in_opt IUri *relativeUri,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **combinedUri);
        
        DECLSPEC_XFGVIRT(IOpcPartUri, ComparePartUri)
        HRESULT ( STDMETHODCALLTYPE *ComparePartUri )( 
            __RPC__in IOpcPartUri * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri,
            /* [retval][out] */ __RPC__out INT32 *comparisonResult);
        
        DECLSPEC_XFGVIRT(IOpcPartUri, GetSourceUri)
        HRESULT ( STDMETHODCALLTYPE *GetSourceUri )( 
            __RPC__in IOpcPartUri * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri);
        
        DECLSPEC_XFGVIRT(IOpcPartUri, IsRelationshipsPartUri)
        HRESULT ( STDMETHODCALLTYPE *IsRelationshipsPartUri )( 
            __RPC__in IOpcPartUri * This,
            /* [retval][out] */ __RPC__out BOOL *isRelationshipUri);
        
        END_INTERFACE
    } IOpcPartUriVtbl;

    interface IOpcPartUri
    {
        CONST_VTBL struct IOpcPartUriVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcPartUri_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcPartUri_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcPartUri_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcPartUri_GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags) ) 

#define IOpcPartUri_GetPropertyLength(This,uriProp,pcchProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyLength(This,uriProp,pcchProperty,dwFlags) ) 

#define IOpcPartUri_GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags) ) 

#define IOpcPartUri_HasProperty(This,uriProp,pfHasProperty)	\
    ( (This)->lpVtbl -> HasProperty(This,uriProp,pfHasProperty) ) 

#define IOpcPartUri_GetAbsoluteUri(This,pbstrAbsoluteUri)	\
    ( (This)->lpVtbl -> GetAbsoluteUri(This,pbstrAbsoluteUri) ) 

#define IOpcPartUri_GetAuthority(This,pbstrAuthority)	\
    ( (This)->lpVtbl -> GetAuthority(This,pbstrAuthority) ) 

#define IOpcPartUri_GetDisplayUri(This,pbstrDisplayString)	\
    ( (This)->lpVtbl -> GetDisplayUri(This,pbstrDisplayString) ) 

#define IOpcPartUri_GetDomain(This,pbstrDomain)	\
    ( (This)->lpVtbl -> GetDomain(This,pbstrDomain) ) 

#define IOpcPartUri_GetExtension(This,pbstrExtension)	\
    ( (This)->lpVtbl -> GetExtension(This,pbstrExtension) ) 

#define IOpcPartUri_GetFragment(This,pbstrFragment)	\
    ( (This)->lpVtbl -> GetFragment(This,pbstrFragment) ) 

#define IOpcPartUri_GetHost(This,pbstrHost)	\
    ( (This)->lpVtbl -> GetHost(This,pbstrHost) ) 

#define IOpcPartUri_GetPassword(This,pbstrPassword)	\
    ( (This)->lpVtbl -> GetPassword(This,pbstrPassword) ) 

#define IOpcPartUri_GetPath(This,pbstrPath)	\
    ( (This)->lpVtbl -> GetPath(This,pbstrPath) ) 

#define IOpcPartUri_GetPathAndQuery(This,pbstrPathAndQuery)	\
    ( (This)->lpVtbl -> GetPathAndQuery(This,pbstrPathAndQuery) ) 

#define IOpcPartUri_GetQuery(This,pbstrQuery)	\
    ( (This)->lpVtbl -> GetQuery(This,pbstrQuery) ) 

#define IOpcPartUri_GetRawUri(This,pbstrRawUri)	\
    ( (This)->lpVtbl -> GetRawUri(This,pbstrRawUri) ) 

#define IOpcPartUri_GetSchemeName(This,pbstrSchemeName)	\
    ( (This)->lpVtbl -> GetSchemeName(This,pbstrSchemeName) ) 

#define IOpcPartUri_GetUserInfo(This,pbstrUserInfo)	\
    ( (This)->lpVtbl -> GetUserInfo(This,pbstrUserInfo) ) 

#define IOpcPartUri_GetUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> GetUserName(This,pbstrUserName) ) 

#define IOpcPartUri_GetHostType(This,pdwHostType)	\
    ( (This)->lpVtbl -> GetHostType(This,pdwHostType) ) 

#define IOpcPartUri_GetPort(This,pdwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pdwPort) ) 

#define IOpcPartUri_GetScheme(This,pdwScheme)	\
    ( (This)->lpVtbl -> GetScheme(This,pdwScheme) ) 

#define IOpcPartUri_GetZone(This,pdwZone)	\
    ( (This)->lpVtbl -> GetZone(This,pdwZone) ) 

#define IOpcPartUri_GetProperties(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetProperties(This,pdwFlags) ) 

#define IOpcPartUri_IsEqual(This,pUri,pfEqual)	\
    ( (This)->lpVtbl -> IsEqual(This,pUri,pfEqual) ) 


#define IOpcPartUri_GetRelationshipsPartUri(This,relationshipPartUri)	\
    ( (This)->lpVtbl -> GetRelationshipsPartUri(This,relationshipPartUri) ) 

#define IOpcPartUri_GetRelativeUri(This,targetPartUri,relativeUri)	\
    ( (This)->lpVtbl -> GetRelativeUri(This,targetPartUri,relativeUri) ) 

#define IOpcPartUri_CombinePartUri(This,relativeUri,combinedUri)	\
    ( (This)->lpVtbl -> CombinePartUri(This,relativeUri,combinedUri) ) 


#define IOpcPartUri_ComparePartUri(This,partUri,comparisonResult)	\
    ( (This)->lpVtbl -> ComparePartUri(This,partUri,comparisonResult) ) 

#define IOpcPartUri_GetSourceUri(This,sourceUri)	\
    ( (This)->lpVtbl -> GetSourceUri(This,sourceUri) ) 

#define IOpcPartUri_IsRelationshipsPartUri(This,isRelationshipUri)	\
    ( (This)->lpVtbl -> IsRelationshipsPartUri(This,isRelationshipUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcPartUri_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msopc_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN7)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0002_v0_0_s_ifspec;


#ifndef __MSOPC_LIBRARY_DEFINED__
#define __MSOPC_LIBRARY_DEFINED__

/* library MSOPC */
/* [lcid][version][uuid] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_msopc_0000_0002_0001
    {
        OPC_URI_TARGET_MODE_INTERNAL	= 0,
        OPC_URI_TARGET_MODE_EXTERNAL	= 1
    } 	OPC_URI_TARGET_MODE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_msopc_0000_0002_0002
    {
        OPC_COMPRESSION_NONE	= -1,
        OPC_COMPRESSION_NORMAL	= 0,
        OPC_COMPRESSION_MAXIMUM	= 1,
        OPC_COMPRESSION_FAST	= 2,
        OPC_COMPRESSION_SUPERFAST	= 3
    } 	OPC_COMPRESSION_OPTIONS;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_msopc_0000_0002_0003
    {
        OPC_STREAM_IO_READ	= 1,
        OPC_STREAM_IO_WRITE	= 2
    } 	OPC_STREAM_IO_MODE;

#define IS_OPC_CONFORMANCE_ERROR(x) (((x) & 0x1FFFF000) == (0x0000+(FACILITY_OPC << 16)))
#define IS_ZIP_CONFORMANCE_ERROR(x) (((x) & 0x1FFFF000) == (0x1000+(FACILITY_OPC << 16)))
#define OPC_E_NONCONFORMING_URI MAKE_HRESULT(1, FACILITY_OPC, 0x1)
#define OPC_E_RELATIVE_URI_REQUIRED MAKE_HRESULT(1, FACILITY_OPC, 0x2)
#define OPC_E_RELATIONSHIP_URI_REQUIRED MAKE_HRESULT(1, FACILITY_OPC, 0x3)
#define OPC_E_PART_CANNOT_BE_DIRECTORY MAKE_HRESULT(1, FACILITY_OPC, 0x4)
#define OPC_E_UNEXPECTED_CONTENT_TYPE MAKE_HRESULT(1, FACILITY_OPC, 0x5)
#define OPC_E_INVALID_CONTENT_TYPE_XML MAKE_HRESULT(1, FACILITY_OPC, 0x6)
#define OPC_E_MISSING_CONTENT_TYPES MAKE_HRESULT(1, FACILITY_OPC, 0x7)
#define OPC_E_NONCONFORMING_CONTENT_TYPES_XML MAKE_HRESULT(1, FACILITY_OPC, 0x8)
#define OPC_E_NONCONFORMING_RELS_XML MAKE_HRESULT(1, FACILITY_OPC, 0x9)
#define OPC_E_INVALID_RELS_XML MAKE_HRESULT(1, FACILITY_OPC, 0xA)
#define OPC_E_DUPLICATE_PART MAKE_HRESULT(1, FACILITY_OPC, 0xB)
#define OPC_E_INVALID_OVERRIDE_PART_NAME MAKE_HRESULT(1, FACILITY_OPC, 0xC)
#define OPC_E_DUPLICATE_OVERRIDE_PART MAKE_HRESULT(1, FACILITY_OPC, 0xD)
#define OPC_E_INVALID_DEFAULT_EXTENSION MAKE_HRESULT(1, FACILITY_OPC, 0xE)
#define OPC_E_DUPLICATE_DEFAULT_EXTENSION MAKE_HRESULT(1, FACILITY_OPC, 0xF)
#define OPC_E_INVALID_RELATIONSHIP_ID MAKE_HRESULT(1, FACILITY_OPC, 0x10)
#define OPC_E_INVALID_RELATIONSHIP_TYPE MAKE_HRESULT(1, FACILITY_OPC, 0x11)
#define OPC_E_INVALID_RELATIONSHIP_TARGET MAKE_HRESULT(1, FACILITY_OPC, 0x12)
#define OPC_E_DUPLICATE_RELATIONSHIP MAKE_HRESULT(1, FACILITY_OPC, 0x13)
#define OPC_E_CONFLICTING_SETTINGS MAKE_HRESULT(1, FACILITY_OPC, 0x14)
#define OPC_E_DUPLICATE_PIECE MAKE_HRESULT(1, FACILITY_OPC, 0x15)
#define OPC_E_INVALID_PIECE MAKE_HRESULT(1, FACILITY_OPC, 0x16)
#define OPC_E_MISSING_PIECE MAKE_HRESULT(1, FACILITY_OPC, 0x17)
#define OPC_E_NO_SUCH_PART MAKE_HRESULT(1, FACILITY_OPC, 0x18)
#define OPC_E_DS_SIGNATURE_CORRUPT MAKE_HRESULT(1, FACILITY_OPC, 0x19)
#define OPC_E_DS_DIGEST_VALUE_ERROR MAKE_HRESULT(1, FACILITY_OPC, 0x1A)
#define OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP MAKE_HRESULT(1, FACILITY_OPC, 0x1B)
#define OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP MAKE_HRESULT(1, FACILITY_OPC, 0x1C)
#define OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP MAKE_HRESULT(1, FACILITY_OPC, 0x1D)
#define OPC_E_DS_EXTERNAL_SIGNATURE MAKE_HRESULT(1, FACILITY_OPC, 0x1E)
#define OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART MAKE_HRESULT(1, FACILITY_OPC, 0x1F)
#define OPC_E_DS_MISSING_SIGNATURE_PART MAKE_HRESULT(1, FACILITY_OPC, 0x20)
#define OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML MAKE_HRESULT(1, FACILITY_OPC, 0x21)
#define OPC_E_DS_INVALID_CANONICALIZATION_METHOD MAKE_HRESULT(1, FACILITY_OPC, 0x22)
#define OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION MAKE_HRESULT(1, FACILITY_OPC, 0x23)
#define OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT MAKE_HRESULT(1, FACILITY_OPC, 0x24)
#define OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED MAKE_HRESULT(1, FACILITY_OPC, 0x25)
#define OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT MAKE_HRESULT(1, FACILITY_OPC, 0x26)
#define OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT MAKE_HRESULT(1, FACILITY_OPC, 0x27)
#define OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT MAKE_HRESULT(1, FACILITY_OPC, 0x28)
#define OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY MAKE_HRESULT(1, FACILITY_OPC, 0x29)
#define OPC_E_DS_INVALID_SIGNATURE_XML MAKE_HRESULT(1, FACILITY_OPC, 0x2A)
#define OPC_E_DS_INVALID_SIGNATURE_COUNT MAKE_HRESULT(1, FACILITY_OPC, 0x2B)
#define OPC_E_DS_MISSING_SIGNATURE_ALGORITHM MAKE_HRESULT(1, FACILITY_OPC, 0x2C)
#define OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES MAKE_HRESULT(1, FACILITY_OPC, 0x2D)
#define OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE MAKE_HRESULT(1, FACILITY_OPC, 0x2E)
#define OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE MAKE_HRESULT(1, FACILITY_OPC, 0x2F)
#define OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE MAKE_HRESULT(1, FACILITY_OPC, 0x30)
#define OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS MAKE_HRESULT(1, FACILITY_OPC, 0x31)
#define OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM MAKE_HRESULT(1, FACILITY_OPC, 0x32)
#define OPC_E_MC_UNEXPECTED_ELEMENT MAKE_HRESULT(1, FACILITY_OPC, 0x33)
#define OPC_E_MC_UNEXPECTED_REQUIRES_ATTR MAKE_HRESULT(1, FACILITY_OPC, 0x34)
#define OPC_E_MC_MISSING_REQUIRES_ATTR MAKE_HRESULT(1, FACILITY_OPC, 0x35)
#define OPC_E_MC_UNEXPECTED_ATTR MAKE_HRESULT(1, FACILITY_OPC, 0x36)
#define OPC_E_MC_INVALID_PREFIX_LIST MAKE_HRESULT(1, FACILITY_OPC, 0x37)
#define OPC_E_MC_INVALID_QNAME_LIST MAKE_HRESULT(1, FACILITY_OPC, 0x38)
#define OPC_E_MC_NESTED_ALTERNATE_CONTENT MAKE_HRESULT(1, FACILITY_OPC, 0x39)
#define OPC_E_MC_UNEXPECTED_CHOICE MAKE_HRESULT(1, FACILITY_OPC, 0x3A)
#define OPC_E_MC_MISSING_CHOICE MAKE_HRESULT(1, FACILITY_OPC, 0x3B)
#define OPC_E_MC_INVALID_ENUM_TYPE MAKE_HRESULT(1, FACILITY_OPC, 0x3C)
#define OPC_E_MC_UNKNOWN_NAMESPACE MAKE_HRESULT(1, FACILITY_OPC, 0x3E)
#define OPC_E_MC_UNKNOWN_PREFIX MAKE_HRESULT(1, FACILITY_OPC, 0x3F)
#define OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT MAKE_HRESULT(1, FACILITY_OPC, 0x40)
#define OPC_E_MC_INVALID_XMLNS_ATTRIBUTE MAKE_HRESULT(1, FACILITY_OPC, 0x41)
#define OPC_E_INVALID_XML_ENCODING MAKE_HRESULT(1, FACILITY_OPC, 0x42)
#define OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI MAKE_HRESULT(1, FACILITY_OPC, 0x43)
#define OPC_E_INVALID_CONTENT_TYPE MAKE_HRESULT(1, FACILITY_OPC, 0x44)
#define OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET MAKE_HRESULT(1, FACILITY_OPC, 0x45)
#define OPC_E_DS_SIGNATURE_METHOD_NOT_SET MAKE_HRESULT(1, FACILITY_OPC, 0x46)
#define OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET MAKE_HRESULT(1, FACILITY_OPC, 0x47)
#define OPC_E_NO_SUCH_RELATIONSHIP MAKE_HRESULT(1, FACILITY_OPC, 0x48)
#define OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS MAKE_HRESULT(1, FACILITY_OPC, 0x49)
#define OPC_E_MC_INCONSISTENT_PROCESS_CONTENT MAKE_HRESULT(1, FACILITY_OPC, 0x4A)
#define OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES MAKE_HRESULT(1, FACILITY_OPC, 0x4B)
#define OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS MAKE_HRESULT(1, FACILITY_OPC, 0x4C)
#define OPC_E_INVALID_RELATIONSHIP_TARGET_MODE MAKE_HRESULT(1, FACILITY_OPC, 0x4D)
#define OPC_E_COULD_NOT_RECOVER MAKE_HRESULT(1, FACILITY_OPC, 0x4E)
#define OPC_E_UNSUPPORTED_PACKAGE MAKE_HRESULT(1, FACILITY_OPC, 0x4F)
#define OPC_E_ENUM_COLLECTION_CHANGED MAKE_HRESULT(1, FACILITY_OPC, 0x50)
#define OPC_E_ENUM_CANNOT_MOVE_NEXT MAKE_HRESULT(1, FACILITY_OPC, 0x51)
#define OPC_E_ENUM_CANNOT_MOVE_PREVIOUS MAKE_HRESULT(1, FACILITY_OPC, 0x52)
#define OPC_E_ENUM_INVALID_POSITION MAKE_HRESULT(1, FACILITY_OPC, 0x53)
#define OPC_E_DS_SIGNATURE_ORIGIN_EXISTS MAKE_HRESULT(1, FACILITY_OPC, 0x54)
#define OPC_E_DS_UNSIGNED_PACKAGE MAKE_HRESULT(1, FACILITY_OPC, 0x55)
#define OPC_E_DS_MISSING_CERTIFICATE_PART MAKE_HRESULT(1, FACILITY_OPC, 0x56)
#define OPC_E_NO_SUCH_SETTINGS MAKE_HRESULT(1, FACILITY_OPC, 0x57)
#define OPC_E_ZIP_INCORRECT_DATA_SIZE MAKE_HRESULT(1, FACILITY_OPC, 0x1001)
#define OPC_E_ZIP_CORRUPTED_ARCHIVE MAKE_HRESULT(1, FACILITY_OPC, 0x1002)
#define OPC_E_ZIP_COMPRESSION_FAILED MAKE_HRESULT(1, FACILITY_OPC, 0x1003)
#define OPC_E_ZIP_DECOMPRESSION_FAILED MAKE_HRESULT(1, FACILITY_OPC, 0x1004)
#define OPC_E_ZIP_INCONSISTENT_FILEITEM MAKE_HRESULT(1, FACILITY_OPC, 0x1005)
#define OPC_E_ZIP_INCONSISTENT_DIRECTORY MAKE_HRESULT(1, FACILITY_OPC, 0x1006)
#define OPC_E_ZIP_MISSING_DATA_DESCRIPTOR MAKE_HRESULT(1, FACILITY_OPC, 0x1007)
#define OPC_E_ZIP_UNSUPPORTEDARCHIVE MAKE_HRESULT(1, FACILITY_OPC, 0x1008)
#define OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE MAKE_HRESULT(1, FACILITY_OPC, 0x1009)
#define OPC_E_ZIP_NAME_TOO_LARGE MAKE_HRESULT(1, FACILITY_OPC, 0x100A)
#define OPC_E_ZIP_DUPLICATE_NAME MAKE_HRESULT(1, FACILITY_OPC, 0x100B)
#define OPC_E_ZIP_COMMENT_TOO_LARGE MAKE_HRESULT(1, FACILITY_OPC, 0x100C)
#define OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE MAKE_HRESULT(1, FACILITY_OPC, 0x100D)
#define OPC_E_ZIP_FILE_HEADER_TOO_LARGE MAKE_HRESULT(1, FACILITY_OPC, 0x100E)
#define OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY MAKE_HRESULT(1, FACILITY_OPC, 0x100F)
#define OPC_E_ZIP_REQUIRES_64_BIT MAKE_HRESULT(1, FACILITY_OPC, 0x1010)
#endif // (NTDDI >= NTDDI_WIN7)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (NTDDI_VERSION >= NTDDI_WIN7)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)







typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_msopc_0000_0002_0004
    {
        OPC_READ_DEFAULT	= 0,
        OPC_VALIDATE_ON_LOAD	= 0x1,
        OPC_CACHE_ON_ACCESS	= 0x2
    } 	OPC_READ_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(OPC_READ_FLAGS);
typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_msopc_0000_0002_0005
    {
        OPC_WRITE_DEFAULT	= 0,
        OPC_WRITE_FORCE_ZIP32	= 0x1
    } 	OPC_WRITE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(OPC_WRITE_FLAGS);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN7)
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (NTDDI_VERSION >= NTDDI_WIN7)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)





















typedef 
enum OPC_SIGNATURE_VALIDATION_RESULT
    {
        OPC_SIGNATURE_VALID	= 0,
        OPC_SIGNATURE_INVALID	= -1
    } 	OPC_SIGNATURE_VALIDATION_RESULT;

typedef /* [public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_msopc_0001_0076_0001
    {
        OPC_CANONICALIZATION_NONE	= 0,
        OPC_CANONICALIZATION_C14N	= 1,
        OPC_CANONICALIZATION_C14N_WITH_COMMENTS	= 2
    } 	OPC_CANONICALIZATION_METHOD;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msopc_0001_0076_0002
    {
        OPC_RELATIONSHIP_SELECT_BY_ID	= 0,
        OPC_RELATIONSHIP_SELECT_BY_TYPE	= 1
    } 	OPC_RELATIONSHIP_SELECTOR;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msopc_0001_0076_0003
    {
        OPC_RELATIONSHIP_SIGN_USING_SELECTORS	= 0,
        OPC_RELATIONSHIP_SIGN_PART	= 1
    } 	OPC_RELATIONSHIPS_SIGNING_OPTION;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msopc_0001_0076_0004
    {
        OPC_CERTIFICATE_IN_CERTIFICATE_PART	= 0,
        OPC_CERTIFICATE_IN_SIGNATURE_PART	= 1,
        OPC_CERTIFICATE_NOT_EMBEDDED	= 2
    } 	OPC_CERTIFICATE_EMBEDDING_OPTION;

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_msopc_0001_0076_0005
    {
        OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS	= 0,
        OPC_SIGNATURE_TIME_FORMAT_SECONDS	= 1,
        OPC_SIGNATURE_TIME_FORMAT_MINUTES	= 2,
        OPC_SIGNATURE_TIME_FORMAT_DAYS	= 3,
        OPC_SIGNATURE_TIME_FORMAT_MONTHS	= 4,
        OPC_SIGNATURE_TIME_FORMAT_YEARS	= 5
    } 	OPC_SIGNATURE_TIME_FORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN7)

EXTERN_C const IID LIBID_MSOPC;

#ifndef __IOpcPackage_INTERFACE_DEFINED__
#define __IOpcPackage_INTERFACE_DEFINED__

/* interface IOpcPackage */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcPackage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE70")
    IOpcPackage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPartSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartSet **partSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSet **relationshipSet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcPackageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcPackage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcPackage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcPackage * This);
        
        DECLSPEC_XFGVIRT(IOpcPackage, GetPartSet)
        HRESULT ( STDMETHODCALLTYPE *GetPartSet )( 
            __RPC__in IOpcPackage * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartSet **partSet);
        
        DECLSPEC_XFGVIRT(IOpcPackage, GetRelationshipSet)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipSet )( 
            __RPC__in IOpcPackage * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSet **relationshipSet);
        
        END_INTERFACE
    } IOpcPackageVtbl;

    interface IOpcPackage
    {
        CONST_VTBL struct IOpcPackageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcPackage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcPackage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcPackage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcPackage_GetPartSet(This,partSet)	\
    ( (This)->lpVtbl -> GetPartSet(This,partSet) ) 

#define IOpcPackage_GetRelationshipSet(This,relationshipSet)	\
    ( (This)->lpVtbl -> GetRelationshipSet(This,relationshipSet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcPackage_INTERFACE_DEFINED__ */


#ifndef __IOpcPart_INTERFACE_DEFINED__
#define __IOpcPart_INTERFACE_DEFINED__

/* interface IOpcPart */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcPart;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE71")
    IOpcPart : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSet **relationshipSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentType( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompressionOptions( 
            /* [retval][out] */ __RPC__out OPC_COMPRESSION_OPTIONS *compressionOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcPartVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcPart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcPart * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcPart * This);
        
        DECLSPEC_XFGVIRT(IOpcPart, GetRelationshipSet)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipSet )( 
            __RPC__in IOpcPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSet **relationshipSet);
        
        DECLSPEC_XFGVIRT(IOpcPart, GetContentStream)
        HRESULT ( STDMETHODCALLTYPE *GetContentStream )( 
            __RPC__in IOpcPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IOpcPart, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IOpcPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **name);
        
        DECLSPEC_XFGVIRT(IOpcPart, GetContentType)
        HRESULT ( STDMETHODCALLTYPE *GetContentType )( 
            __RPC__in IOpcPart * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType);
        
        DECLSPEC_XFGVIRT(IOpcPart, GetCompressionOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCompressionOptions )( 
            __RPC__in IOpcPart * This,
            /* [retval][out] */ __RPC__out OPC_COMPRESSION_OPTIONS *compressionOptions);
        
        END_INTERFACE
    } IOpcPartVtbl;

    interface IOpcPart
    {
        CONST_VTBL struct IOpcPartVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcPart_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcPart_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcPart_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcPart_GetRelationshipSet(This,relationshipSet)	\
    ( (This)->lpVtbl -> GetRelationshipSet(This,relationshipSet) ) 

#define IOpcPart_GetContentStream(This,stream)	\
    ( (This)->lpVtbl -> GetContentStream(This,stream) ) 

#define IOpcPart_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IOpcPart_GetContentType(This,contentType)	\
    ( (This)->lpVtbl -> GetContentType(This,contentType) ) 

#define IOpcPart_GetCompressionOptions(This,compressionOptions)	\
    ( (This)->lpVtbl -> GetCompressionOptions(This,compressionOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcPart_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationship_INTERFACE_DEFINED__
#define __IOpcRelationship_INTERFACE_DEFINED__

/* interface IOpcRelationship */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationship;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE72")
    IOpcRelationship : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetId( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *relationshipIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipType( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *relationshipType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceUri( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTargetUri( 
            /* [retval][out] */ __RPC__deref_out_opt IUri **targetUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTargetMode( 
            /* [retval][out] */ __RPC__out OPC_URI_TARGET_MODE *targetMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationship * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationship * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationship * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationship, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IOpcRelationship * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *relationshipIdentifier);
        
        DECLSPEC_XFGVIRT(IOpcRelationship, GetRelationshipType)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipType )( 
            __RPC__in IOpcRelationship * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *relationshipType);
        
        DECLSPEC_XFGVIRT(IOpcRelationship, GetSourceUri)
        HRESULT ( STDMETHODCALLTYPE *GetSourceUri )( 
            __RPC__in IOpcRelationship * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri);
        
        DECLSPEC_XFGVIRT(IOpcRelationship, GetTargetUri)
        HRESULT ( STDMETHODCALLTYPE *GetTargetUri )( 
            __RPC__in IOpcRelationship * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **targetUri);
        
        DECLSPEC_XFGVIRT(IOpcRelationship, GetTargetMode)
        HRESULT ( STDMETHODCALLTYPE *GetTargetMode )( 
            __RPC__in IOpcRelationship * This,
            /* [retval][out] */ __RPC__out OPC_URI_TARGET_MODE *targetMode);
        
        END_INTERFACE
    } IOpcRelationshipVtbl;

    interface IOpcRelationship
    {
        CONST_VTBL struct IOpcRelationshipVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationship_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationship_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationship_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationship_GetId(This,relationshipIdentifier)	\
    ( (This)->lpVtbl -> GetId(This,relationshipIdentifier) ) 

#define IOpcRelationship_GetRelationshipType(This,relationshipType)	\
    ( (This)->lpVtbl -> GetRelationshipType(This,relationshipType) ) 

#define IOpcRelationship_GetSourceUri(This,sourceUri)	\
    ( (This)->lpVtbl -> GetSourceUri(This,sourceUri) ) 

#define IOpcRelationship_GetTargetUri(This,targetUri)	\
    ( (This)->lpVtbl -> GetTargetUri(This,targetUri) ) 

#define IOpcRelationship_GetTargetMode(This,targetMode)	\
    ( (This)->lpVtbl -> GetTargetMode(This,targetMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationship_INTERFACE_DEFINED__ */


#ifndef __IOpcPartSet_INTERFACE_DEFINED__
#define __IOpcPartSet_INTERFACE_DEFINED__

/* interface IOpcPartSet */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcPartSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE73")
    IOpcPartSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPart( 
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePart( 
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [string][in] */ __RPC__in_string LPCWSTR contentType,
            /* [in] */ OPC_COMPRESSION_OPTIONS compressionOptions,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePart( 
            /* [in] */ __RPC__in_opt IOpcPartUri *name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PartExists( 
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [retval][out] */ __RPC__out BOOL *partExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartEnumerator **partEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcPartSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcPartSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcPartSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcPartSet * This);
        
        DECLSPEC_XFGVIRT(IOpcPartSet, GetPart)
        HRESULT ( STDMETHODCALLTYPE *GetPart )( 
            __RPC__in IOpcPartSet * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part);
        
        DECLSPEC_XFGVIRT(IOpcPartSet, CreatePart)
        HRESULT ( STDMETHODCALLTYPE *CreatePart )( 
            __RPC__in IOpcPartSet * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [string][in] */ __RPC__in_string LPCWSTR contentType,
            /* [in] */ OPC_COMPRESSION_OPTIONS compressionOptions,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part);
        
        DECLSPEC_XFGVIRT(IOpcPartSet, DeletePart)
        HRESULT ( STDMETHODCALLTYPE *DeletePart )( 
            __RPC__in IOpcPartSet * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *name);
        
        DECLSPEC_XFGVIRT(IOpcPartSet, PartExists)
        HRESULT ( STDMETHODCALLTYPE *PartExists )( 
            __RPC__in IOpcPartSet * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *name,
            /* [retval][out] */ __RPC__out BOOL *partExists);
        
        DECLSPEC_XFGVIRT(IOpcPartSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcPartSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartEnumerator **partEnumerator);
        
        END_INTERFACE
    } IOpcPartSetVtbl;

    interface IOpcPartSet
    {
        CONST_VTBL struct IOpcPartSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcPartSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcPartSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcPartSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcPartSet_GetPart(This,name,part)	\
    ( (This)->lpVtbl -> GetPart(This,name,part) ) 

#define IOpcPartSet_CreatePart(This,name,contentType,compressionOptions,part)	\
    ( (This)->lpVtbl -> CreatePart(This,name,contentType,compressionOptions,part) ) 

#define IOpcPartSet_DeletePart(This,name)	\
    ( (This)->lpVtbl -> DeletePart(This,name) ) 

#define IOpcPartSet_PartExists(This,name,partExists)	\
    ( (This)->lpVtbl -> PartExists(This,name,partExists) ) 

#define IOpcPartSet_GetEnumerator(This,partEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,partEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcPartSet_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationshipSet_INTERFACE_DEFINED__
#define __IOpcRelationshipSet_INTERFACE_DEFINED__

/* interface IOpcRelationshipSet */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationshipSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE74")
    IOpcRelationshipSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRelationship( 
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRelationship( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR relationshipIdentifier,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipType,
            /* [in] */ __RPC__in_opt IUri *targetUri,
            /* [in] */ OPC_URI_TARGET_MODE targetMode,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRelationship( 
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RelationshipExists( 
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier,
            /* [retval][out] */ __RPC__out BOOL *relationshipExists) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **relationshipEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumeratorForType( 
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipType,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **relationshipEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipsContentStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **contents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationshipSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationshipSet * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, GetRelationship)
        HRESULT ( STDMETHODCALLTYPE *GetRelationship )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, CreateRelationship)
        HRESULT ( STDMETHODCALLTYPE *CreateRelationship )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR relationshipIdentifier,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipType,
            /* [in] */ __RPC__in_opt IUri *targetUri,
            /* [in] */ OPC_URI_TARGET_MODE targetMode,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, DeleteRelationship)
        HRESULT ( STDMETHODCALLTYPE *DeleteRelationship )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, RelationshipExists)
        HRESULT ( STDMETHODCALLTYPE *RelationshipExists )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipIdentifier,
            /* [retval][out] */ __RPC__out BOOL *relationshipExists);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **relationshipEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, GetEnumeratorForType)
        HRESULT ( STDMETHODCALLTYPE *GetEnumeratorForType )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [string][in] */ __RPC__in_string LPCWSTR relationshipType,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **relationshipEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSet, GetRelationshipsContentStream)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipsContentStream )( 
            __RPC__in IOpcRelationshipSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **contents);
        
        END_INTERFACE
    } IOpcRelationshipSetVtbl;

    interface IOpcRelationshipSet
    {
        CONST_VTBL struct IOpcRelationshipSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationshipSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationshipSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationshipSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationshipSet_GetRelationship(This,relationshipIdentifier,relationship)	\
    ( (This)->lpVtbl -> GetRelationship(This,relationshipIdentifier,relationship) ) 

#define IOpcRelationshipSet_CreateRelationship(This,relationshipIdentifier,relationshipType,targetUri,targetMode,relationship)	\
    ( (This)->lpVtbl -> CreateRelationship(This,relationshipIdentifier,relationshipType,targetUri,targetMode,relationship) ) 

#define IOpcRelationshipSet_DeleteRelationship(This,relationshipIdentifier)	\
    ( (This)->lpVtbl -> DeleteRelationship(This,relationshipIdentifier) ) 

#define IOpcRelationshipSet_RelationshipExists(This,relationshipIdentifier,relationshipExists)	\
    ( (This)->lpVtbl -> RelationshipExists(This,relationshipIdentifier,relationshipExists) ) 

#define IOpcRelationshipSet_GetEnumerator(This,relationshipEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,relationshipEnumerator) ) 

#define IOpcRelationshipSet_GetEnumeratorForType(This,relationshipType,relationshipEnumerator)	\
    ( (This)->lpVtbl -> GetEnumeratorForType(This,relationshipType,relationshipEnumerator) ) 

#define IOpcRelationshipSet_GetRelationshipsContentStream(This,contents)	\
    ( (This)->lpVtbl -> GetRelationshipsContentStream(This,contents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationshipSet_INTERFACE_DEFINED__ */


#ifndef __IOpcPartEnumerator_INTERFACE_DEFINED__
#define __IOpcPartEnumerator_INTERFACE_DEFINED__

/* interface IOpcPartEnumerator */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcPartEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE75")
    IOpcPartEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcPartEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcPartEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcPartEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcPartEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcPartEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcPartEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcPartEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcPartEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcPartEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcPartEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPart **part);
        
        DECLSPEC_XFGVIRT(IOpcPartEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcPartEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartEnumerator **copy);
        
        END_INTERFACE
    } IOpcPartEnumeratorVtbl;

    interface IOpcPartEnumerator
    {
        CONST_VTBL struct IOpcPartEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcPartEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcPartEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcPartEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcPartEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcPartEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcPartEnumerator_GetCurrent(This,part)	\
    ( (This)->lpVtbl -> GetCurrent(This,part) ) 

#define IOpcPartEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcPartEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationshipEnumerator_INTERFACE_DEFINED__
#define __IOpcRelationshipEnumerator_INTERFACE_DEFINED__

/* interface IOpcRelationshipEnumerator */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationshipEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42195949-3B79-4fc8-89C6-FC7FB979EE76")
    IOpcRelationshipEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationshipEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationshipEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationshipEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcRelationshipEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcRelationshipEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcRelationshipEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationship **relationship);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcRelationshipEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipEnumerator **copy);
        
        END_INTERFACE
    } IOpcRelationshipEnumeratorVtbl;

    interface IOpcRelationshipEnumerator
    {
        CONST_VTBL struct IOpcRelationshipEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationshipEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationshipEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationshipEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationshipEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcRelationshipEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcRelationshipEnumerator_GetCurrent(This,relationship)	\
    ( (This)->lpVtbl -> GetCurrent(This,relationship) ) 

#define IOpcRelationshipEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationshipEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcSignaturePartReference_INTERFACE_DEFINED__
#define __IOpcSignaturePartReference_INTERFACE_DEFINED__

/* interface IOpcSignaturePartReference */
/* [unique][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignaturePartReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e24231ca-59f4-484e-b64b-36eeda36072c")
    IOpcSignaturePartReference : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPartName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentType( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestValue( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformMethod( 
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignaturePartReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignaturePartReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignaturePartReference * This);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReference, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partName);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReference, GetContentType)
        HRESULT ( STDMETHODCALLTYPE *GetContentType )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReference, GetDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *GetDigestMethod )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReference, GetDigestValue)
        HRESULT ( STDMETHODCALLTYPE *GetDigestValue )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReference, GetTransformMethod)
        HRESULT ( STDMETHODCALLTYPE *GetTransformMethod )( 
            __RPC__in IOpcSignaturePartReference * This,
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod);
        
        END_INTERFACE
    } IOpcSignaturePartReferenceVtbl;

    interface IOpcSignaturePartReference
    {
        CONST_VTBL struct IOpcSignaturePartReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignaturePartReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignaturePartReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignaturePartReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignaturePartReference_GetPartName(This,partName)	\
    ( (This)->lpVtbl -> GetPartName(This,partName) ) 

#define IOpcSignaturePartReference_GetContentType(This,contentType)	\
    ( (This)->lpVtbl -> GetContentType(This,contentType) ) 

#define IOpcSignaturePartReference_GetDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> GetDigestMethod(This,digestMethod) ) 

#define IOpcSignaturePartReference_GetDigestValue(This,digestValue,count)	\
    ( (This)->lpVtbl -> GetDigestValue(This,digestValue,count) ) 

#define IOpcSignaturePartReference_GetTransformMethod(This,transformMethod)	\
    ( (This)->lpVtbl -> GetTransformMethod(This,transformMethod) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignaturePartReference_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReference_INTERFACE_DEFINED__
#define __IOpcSignatureRelationshipReference_INTERFACE_DEFINED__

/* interface IOpcSignatureRelationshipReference */
/* [unique][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureRelationshipReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57babac6-9d4a-4e50-8b86-e5d4051eae7c")
    IOpcSignatureRelationshipReference : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSourceUri( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestValue( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformMethod( 
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipSigningOption( 
            /* [retval][out] */ __RPC__out OPC_RELATIONSHIPS_SIGNING_OPTION *relationshipSigningOption) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationshipSelectorEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **selectorEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureRelationshipReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureRelationshipReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureRelationshipReference * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetSourceUri)
        HRESULT ( STDMETHODCALLTYPE *GetSourceUri )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **sourceUri);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *GetDigestMethod )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetDigestValue)
        HRESULT ( STDMETHODCALLTYPE *GetDigestValue )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetTransformMethod)
        HRESULT ( STDMETHODCALLTYPE *GetTransformMethod )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetRelationshipSigningOption)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipSigningOption )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [retval][out] */ __RPC__out OPC_RELATIONSHIPS_SIGNING_OPTION *relationshipSigningOption);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReference, GetRelationshipSelectorEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetRelationshipSelectorEnumerator )( 
            __RPC__in IOpcSignatureRelationshipReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **selectorEnumerator);
        
        END_INTERFACE
    } IOpcSignatureRelationshipReferenceVtbl;

    interface IOpcSignatureRelationshipReference
    {
        CONST_VTBL struct IOpcSignatureRelationshipReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureRelationshipReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureRelationshipReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureRelationshipReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureRelationshipReference_GetSourceUri(This,sourceUri)	\
    ( (This)->lpVtbl -> GetSourceUri(This,sourceUri) ) 

#define IOpcSignatureRelationshipReference_GetDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> GetDigestMethod(This,digestMethod) ) 

#define IOpcSignatureRelationshipReference_GetDigestValue(This,digestValue,count)	\
    ( (This)->lpVtbl -> GetDigestValue(This,digestValue,count) ) 

#define IOpcSignatureRelationshipReference_GetTransformMethod(This,transformMethod)	\
    ( (This)->lpVtbl -> GetTransformMethod(This,transformMethod) ) 

#define IOpcSignatureRelationshipReference_GetRelationshipSigningOption(This,relationshipSigningOption)	\
    ( (This)->lpVtbl -> GetRelationshipSigningOption(This,relationshipSigningOption) ) 

#define IOpcSignatureRelationshipReference_GetRelationshipSelectorEnumerator(This,selectorEnumerator)	\
    ( (This)->lpVtbl -> GetRelationshipSelectorEnumerator(This,selectorEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureRelationshipReference_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationshipSelector_INTERFACE_DEFINED__
#define __IOpcRelationshipSelector_INTERFACE_DEFINED__

/* interface IOpcRelationshipSelector */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationshipSelector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f8f26c7f-b28f-4899-84c8-5d5639ede75f")
    IOpcRelationshipSelector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSelectorType( 
            /* [retval][out] */ __RPC__out OPC_RELATIONSHIP_SELECTOR *selector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectionCriterion( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *selectionCriterion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipSelectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationshipSelector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationshipSelector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationshipSelector * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelector, GetSelectorType)
        HRESULT ( STDMETHODCALLTYPE *GetSelectorType )( 
            __RPC__in IOpcRelationshipSelector * This,
            /* [retval][out] */ __RPC__out OPC_RELATIONSHIP_SELECTOR *selector);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelector, GetSelectionCriterion)
        HRESULT ( STDMETHODCALLTYPE *GetSelectionCriterion )( 
            __RPC__in IOpcRelationshipSelector * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *selectionCriterion);
        
        END_INTERFACE
    } IOpcRelationshipSelectorVtbl;

    interface IOpcRelationshipSelector
    {
        CONST_VTBL struct IOpcRelationshipSelectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationshipSelector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationshipSelector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationshipSelector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationshipSelector_GetSelectorType(This,selector)	\
    ( (This)->lpVtbl -> GetSelectorType(This,selector) ) 

#define IOpcRelationshipSelector_GetSelectionCriterion(This,selectionCriterion)	\
    ( (This)->lpVtbl -> GetSelectionCriterion(This,selectionCriterion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationshipSelector_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureReference_INTERFACE_DEFINED__
#define __IOpcSignatureReference_INTERFACE_DEFINED__

/* interface IOpcSignatureReference */
/* [unique][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1b47005e-3011-4edc-be6f-0f65e5ab0342")
    IOpcSignatureReference : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetId( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *referenceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUri( 
            /* [retval][out] */ __RPC__deref_out_opt IUri **referenceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformMethod( 
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDigestValue( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureReference * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IOpcSignatureReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *referenceId);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            __RPC__in IOpcSignatureReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **referenceUri);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IOpcSignatureReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *type);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetTransformMethod)
        HRESULT ( STDMETHODCALLTYPE *GetTransformMethod )( 
            __RPC__in IOpcSignatureReference * This,
            /* [retval][out] */ __RPC__out OPC_CANONICALIZATION_METHOD *transformMethod);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *GetDigestMethod )( 
            __RPC__in IOpcSignatureReference * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReference, GetDigestValue)
        HRESULT ( STDMETHODCALLTYPE *GetDigestValue )( 
            __RPC__in IOpcSignatureReference * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **digestValue,
            /* [out] */ __RPC__out UINT32 *count);
        
        END_INTERFACE
    } IOpcSignatureReferenceVtbl;

    interface IOpcSignatureReference
    {
        CONST_VTBL struct IOpcSignatureReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureReference_GetId(This,referenceId)	\
    ( (This)->lpVtbl -> GetId(This,referenceId) ) 

#define IOpcSignatureReference_GetUri(This,referenceUri)	\
    ( (This)->lpVtbl -> GetUri(This,referenceUri) ) 

#define IOpcSignatureReference_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 

#define IOpcSignatureReference_GetTransformMethod(This,transformMethod)	\
    ( (This)->lpVtbl -> GetTransformMethod(This,transformMethod) ) 

#define IOpcSignatureReference_GetDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> GetDigestMethod(This,digestMethod) ) 

#define IOpcSignatureReference_GetDigestValue(This,digestValue,count)	\
    ( (This)->lpVtbl -> GetDigestValue(This,digestValue,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureReference_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureCustomObject_INTERFACE_DEFINED__
#define __IOpcSignatureCustomObject_INTERFACE_DEFINED__

/* interface IOpcSignatureCustomObject */
/* [unique][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureCustomObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5d77a19e-62c1-44e7-becd-45da5ae51a56")
    IOpcSignatureCustomObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetXml( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **xmlMarkup,
            /* [out] */ __RPC__out UINT32 *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureCustomObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureCustomObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureCustomObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureCustomObject * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObject, GetXml)
        HRESULT ( STDMETHODCALLTYPE *GetXml )( 
            __RPC__in IOpcSignatureCustomObject * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) UINT8 **xmlMarkup,
            /* [out] */ __RPC__out UINT32 *count);
        
        END_INTERFACE
    } IOpcSignatureCustomObjectVtbl;

    interface IOpcSignatureCustomObject
    {
        CONST_VTBL struct IOpcSignatureCustomObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureCustomObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureCustomObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureCustomObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureCustomObject_GetXml(This,xmlMarkup,count)	\
    ( (This)->lpVtbl -> GetXml(This,xmlMarkup,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureCustomObject_INTERFACE_DEFINED__ */


#ifndef __IOpcDigitalSignature_INTERFACE_DEFINED__
#define __IOpcDigitalSignature_INTERFACE_DEFINED__

/* interface IOpcDigitalSignature */
/* [unique][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IOpcDigitalSignature;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52ab21dd-1cd0-4949-bc80-0c1232d00cb4")
    IOpcDigitalSignature : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNamespaces( 
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  LPWSTR **prefixes,
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  LPWSTR **namespaces,
            /* [out] */ UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureId( 
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signatureId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartName( 
            /* [retval][out] */ IOpcPartUri **signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureMethod( 
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signatureMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCanonicalizationMethod( 
            /* [retval][out] */ OPC_CANONICALIZATION_METHOD *canonicalizationMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureValue( 
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  UINT8 **signatureValue,
            /* [out] */ UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartReferenceEnumerator( 
            /* [retval][out] */ IOpcSignaturePartReferenceEnumerator **partReferenceEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureRelationshipReferenceEnumerator( 
            /* [retval][out] */ IOpcSignatureRelationshipReferenceEnumerator **relationshipReferenceEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigningTime( 
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signingTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeFormat( 
            /* [retval][out] */ OPC_SIGNATURE_TIME_FORMAT *timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPackageObjectReference( 
            /* [retval][out] */ IOpcSignatureReference **packageObjectReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateEnumerator( 
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomReferenceEnumerator( 
            /* [retval][out] */ IOpcSignatureReferenceEnumerator **customReferenceEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomObjectEnumerator( 
            /* [retval][out] */ IOpcSignatureCustomObjectEnumerator **customObjectEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureXml( 
            /* [size_is][size_is][out] */ UINT8 **signatureXml,
            /* [out] */ UINT32 *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcDigitalSignatureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOpcDigitalSignature * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOpcDigitalSignature * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOpcDigitalSignature * This);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetNamespaces)
        HRESULT ( STDMETHODCALLTYPE *GetNamespaces )( 
            IOpcDigitalSignature * This,
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  LPWSTR **prefixes,
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  LPWSTR **namespaces,
            /* [out] */ UINT32 *count);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureId )( 
            IOpcDigitalSignature * This,
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signatureId);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartName )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcPartUri **signaturePartName);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignatureMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureMethod )( 
            IOpcDigitalSignature * This,
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signatureMethod);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetCanonicalizationMethod)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalizationMethod )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ OPC_CANONICALIZATION_METHOD *canonicalizationMethod);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignatureValue)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureValue )( 
            IOpcDigitalSignature * This,
            /* [annotation][size_is][size_is][out] */ 
            __RPC__deref_out_ecount_full_opt(*count)  UINT8 **signatureValue,
            /* [out] */ UINT32 *count);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignaturePartReferenceEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartReferenceEnumerator )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcSignaturePartReferenceEnumerator **partReferenceEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignatureRelationshipReferenceEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureRelationshipReferenceEnumerator )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcSignatureRelationshipReferenceEnumerator **relationshipReferenceEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSigningTime)
        HRESULT ( STDMETHODCALLTYPE *GetSigningTime )( 
            IOpcDigitalSignature * This,
            /* [annotation][retval][string][out] */ 
            __RPC__deref_out_opt_string  LPWSTR *signingTime);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetTimeFormat )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ OPC_SIGNATURE_TIME_FORMAT *timeFormat);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetPackageObjectReference)
        HRESULT ( STDMETHODCALLTYPE *GetPackageObjectReference )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcSignatureReference **packageObjectReference);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetCertificateEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateEnumerator )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetCustomReferenceEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCustomReferenceEnumerator )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcSignatureReferenceEnumerator **customReferenceEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetCustomObjectEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetCustomObjectEnumerator )( 
            IOpcDigitalSignature * This,
            /* [retval][out] */ IOpcSignatureCustomObjectEnumerator **customObjectEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignature, GetSignatureXml)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureXml )( 
            IOpcDigitalSignature * This,
            /* [size_is][size_is][out] */ UINT8 **signatureXml,
            /* [out] */ UINT32 *count);
        
        END_INTERFACE
    } IOpcDigitalSignatureVtbl;

    interface IOpcDigitalSignature
    {
        CONST_VTBL struct IOpcDigitalSignatureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcDigitalSignature_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcDigitalSignature_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcDigitalSignature_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcDigitalSignature_GetNamespaces(This,prefixes,namespaces,count)	\
    ( (This)->lpVtbl -> GetNamespaces(This,prefixes,namespaces,count) ) 

#define IOpcDigitalSignature_GetSignatureId(This,signatureId)	\
    ( (This)->lpVtbl -> GetSignatureId(This,signatureId) ) 

#define IOpcDigitalSignature_GetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> GetSignaturePartName(This,signaturePartName) ) 

#define IOpcDigitalSignature_GetSignatureMethod(This,signatureMethod)	\
    ( (This)->lpVtbl -> GetSignatureMethod(This,signatureMethod) ) 

#define IOpcDigitalSignature_GetCanonicalizationMethod(This,canonicalizationMethod)	\
    ( (This)->lpVtbl -> GetCanonicalizationMethod(This,canonicalizationMethod) ) 

#define IOpcDigitalSignature_GetSignatureValue(This,signatureValue,count)	\
    ( (This)->lpVtbl -> GetSignatureValue(This,signatureValue,count) ) 

#define IOpcDigitalSignature_GetSignaturePartReferenceEnumerator(This,partReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetSignaturePartReferenceEnumerator(This,partReferenceEnumerator) ) 

#define IOpcDigitalSignature_GetSignatureRelationshipReferenceEnumerator(This,relationshipReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetSignatureRelationshipReferenceEnumerator(This,relationshipReferenceEnumerator) ) 

#define IOpcDigitalSignature_GetSigningTime(This,signingTime)	\
    ( (This)->lpVtbl -> GetSigningTime(This,signingTime) ) 

#define IOpcDigitalSignature_GetTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> GetTimeFormat(This,timeFormat) ) 

#define IOpcDigitalSignature_GetPackageObjectReference(This,packageObjectReference)	\
    ( (This)->lpVtbl -> GetPackageObjectReference(This,packageObjectReference) ) 

#define IOpcDigitalSignature_GetCertificateEnumerator(This,certificateEnumerator)	\
    ( (This)->lpVtbl -> GetCertificateEnumerator(This,certificateEnumerator) ) 

#define IOpcDigitalSignature_GetCustomReferenceEnumerator(This,customReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetCustomReferenceEnumerator(This,customReferenceEnumerator) ) 

#define IOpcDigitalSignature_GetCustomObjectEnumerator(This,customObjectEnumerator)	\
    ( (This)->lpVtbl -> GetCustomObjectEnumerator(This,customObjectEnumerator) ) 

#define IOpcDigitalSignature_GetSignatureXml(This,signatureXml,count)	\
    ( (This)->lpVtbl -> GetSignatureXml(This,signatureXml,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcDigitalSignature_INTERFACE_DEFINED__ */


#ifndef __IOpcSigningOptions_INTERFACE_DEFINED__
#define __IOpcSigningOptions_INTERFACE_DEFINED__

/* interface IOpcSigningOptions */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSigningOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50d2d6a5-7aeb-46c0-b241-43ab0e9b407e")
    IOpcSigningOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSignatureId( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureId( 
            /* [in] */ __RPC__in LPCWSTR signatureId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureMethod( 
            /* [in] */ __RPC__in LPCWSTR signatureMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultDigestMethod( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultDigestMethod( 
            /* [in] */ __RPC__in LPCWSTR digestMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateEmbeddingOption( 
            /* [retval][out] */ __RPC__out OPC_CERTIFICATE_EMBEDDING_OPTION *embeddingOption) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCertificateEmbeddingOption( 
            /* [in] */ OPC_CERTIFICATE_EMBEDDING_OPTION embeddingOption) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeFormat( 
            /* [retval][out] */ __RPC__out OPC_SIGNATURE_TIME_FORMAT *timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimeFormat( 
            /* [in] */ OPC_SIGNATURE_TIME_FORMAT timeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartReferenceSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceSet **partReferenceSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureRelationshipReferenceSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceSet **relationshipReferenceSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomObjectSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectSet **customObjectSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomReferenceSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceSet **customReferenceSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificateSet( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcCertificateSet **certificateSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignaturePartName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignaturePartName( 
            /* [unique][in] */ __RPC__in_opt IOpcPartUri *signaturePartName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSigningOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSigningOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSigningOptions * This);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureId )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureId);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetSignatureId)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureId )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ __RPC__in LPCWSTR signatureId);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetSignatureMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureMethod )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *signatureMethod);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetSignatureMethod)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureMethod )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ __RPC__in LPCWSTR signatureMethod);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetDefaultDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultDigestMethod )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *digestMethod);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetDefaultDigestMethod)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultDigestMethod )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ __RPC__in LPCWSTR digestMethod);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetCertificateEmbeddingOption)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateEmbeddingOption )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__out OPC_CERTIFICATE_EMBEDDING_OPTION *embeddingOption);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetCertificateEmbeddingOption)
        HRESULT ( STDMETHODCALLTYPE *SetCertificateEmbeddingOption )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ OPC_CERTIFICATE_EMBEDDING_OPTION embeddingOption);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetTimeFormat )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__out OPC_SIGNATURE_TIME_FORMAT *timeFormat);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *SetTimeFormat )( 
            __RPC__in IOpcSigningOptions * This,
            /* [in] */ OPC_SIGNATURE_TIME_FORMAT timeFormat);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetSignaturePartReferenceSet)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartReferenceSet )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceSet **partReferenceSet);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetSignatureRelationshipReferenceSet)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureRelationshipReferenceSet )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceSet **relationshipReferenceSet);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetCustomObjectSet)
        HRESULT ( STDMETHODCALLTYPE *GetCustomObjectSet )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectSet **customObjectSet);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetCustomReferenceSet)
        HRESULT ( STDMETHODCALLTYPE *GetCustomReferenceSet )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceSet **customReferenceSet);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetCertificateSet)
        HRESULT ( STDMETHODCALLTYPE *GetCertificateSet )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcCertificateSet **certificateSet);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, GetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignaturePartName )( 
            __RPC__in IOpcSigningOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **signaturePartName);
        
        DECLSPEC_XFGVIRT(IOpcSigningOptions, SetSignaturePartName)
        HRESULT ( STDMETHODCALLTYPE *SetSignaturePartName )( 
            __RPC__in IOpcSigningOptions * This,
            /* [unique][in] */ __RPC__in_opt IOpcPartUri *signaturePartName);
        
        END_INTERFACE
    } IOpcSigningOptionsVtbl;

    interface IOpcSigningOptions
    {
        CONST_VTBL struct IOpcSigningOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSigningOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSigningOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSigningOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSigningOptions_GetSignatureId(This,signatureId)	\
    ( (This)->lpVtbl -> GetSignatureId(This,signatureId) ) 

#define IOpcSigningOptions_SetSignatureId(This,signatureId)	\
    ( (This)->lpVtbl -> SetSignatureId(This,signatureId) ) 

#define IOpcSigningOptions_GetSignatureMethod(This,signatureMethod)	\
    ( (This)->lpVtbl -> GetSignatureMethod(This,signatureMethod) ) 

#define IOpcSigningOptions_SetSignatureMethod(This,signatureMethod)	\
    ( (This)->lpVtbl -> SetSignatureMethod(This,signatureMethod) ) 

#define IOpcSigningOptions_GetDefaultDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> GetDefaultDigestMethod(This,digestMethod) ) 

#define IOpcSigningOptions_SetDefaultDigestMethod(This,digestMethod)	\
    ( (This)->lpVtbl -> SetDefaultDigestMethod(This,digestMethod) ) 

#define IOpcSigningOptions_GetCertificateEmbeddingOption(This,embeddingOption)	\
    ( (This)->lpVtbl -> GetCertificateEmbeddingOption(This,embeddingOption) ) 

#define IOpcSigningOptions_SetCertificateEmbeddingOption(This,embeddingOption)	\
    ( (This)->lpVtbl -> SetCertificateEmbeddingOption(This,embeddingOption) ) 

#define IOpcSigningOptions_GetTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> GetTimeFormat(This,timeFormat) ) 

#define IOpcSigningOptions_SetTimeFormat(This,timeFormat)	\
    ( (This)->lpVtbl -> SetTimeFormat(This,timeFormat) ) 

#define IOpcSigningOptions_GetSignaturePartReferenceSet(This,partReferenceSet)	\
    ( (This)->lpVtbl -> GetSignaturePartReferenceSet(This,partReferenceSet) ) 

#define IOpcSigningOptions_GetSignatureRelationshipReferenceSet(This,relationshipReferenceSet)	\
    ( (This)->lpVtbl -> GetSignatureRelationshipReferenceSet(This,relationshipReferenceSet) ) 

#define IOpcSigningOptions_GetCustomObjectSet(This,customObjectSet)	\
    ( (This)->lpVtbl -> GetCustomObjectSet(This,customObjectSet) ) 

#define IOpcSigningOptions_GetCustomReferenceSet(This,customReferenceSet)	\
    ( (This)->lpVtbl -> GetCustomReferenceSet(This,customReferenceSet) ) 

#define IOpcSigningOptions_GetCertificateSet(This,certificateSet)	\
    ( (This)->lpVtbl -> GetCertificateSet(This,certificateSet) ) 

#define IOpcSigningOptions_GetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> GetSignaturePartName(This,signaturePartName) ) 

#define IOpcSigningOptions_SetSignaturePartName(This,signaturePartName)	\
    ( (This)->lpVtbl -> SetSignaturePartName(This,signaturePartName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSigningOptions_INTERFACE_DEFINED__ */


#ifndef __IOpcDigitalSignatureManager_INTERFACE_DEFINED__
#define __IOpcDigitalSignatureManager_INTERFACE_DEFINED__

/* interface IOpcDigitalSignatureManager */
/* [ref][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IOpcDigitalSignatureManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5e62a0b-696d-462f-94df-72e33cef2659")
    IOpcDigitalSignatureManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSignatureOriginPartName( 
            /* [retval][out] */ IOpcPartUri **signatureOriginPartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignatureOriginPartName( 
            /* [unique][in] */ IOpcPartUri *signatureOriginPartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureEnumerator( 
            /* [retval][out] */ IOpcDigitalSignatureEnumerator **signatureEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveSignature( 
            /* [in] */ IOpcPartUri *signaturePartName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSigningOptions( 
            /* [retval][out] */ IOpcSigningOptions **signingOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Validate( 
            /* [in] */ IOpcDigitalSignature *signature,
            /* [in] */ const CERT_CONTEXT *certificate,
            /* [retval][out] */ OPC_SIGNATURE_VALIDATION_RESULT *validationResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Sign( 
            /* [in] */ const CERT_CONTEXT *certificate,
            /* [in] */ IOpcSigningOptions *signingOptions,
            /* [retval][out] */ IOpcDigitalSignature **digitalSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReplaceSignatureXml( 
            /* [in] */ IOpcPartUri *signaturePartName,
            /* [size_is][in] */ const UINT8 *newSignatureXml,
            /* [in] */ UINT32 count,
            /* [retval][out] */ IOpcDigitalSignature **digitalSignature) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcDigitalSignatureManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOpcDigitalSignatureManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOpcDigitalSignatureManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOpcDigitalSignatureManager * This);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, GetSignatureOriginPartName)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureOriginPartName )( 
            IOpcDigitalSignatureManager * This,
            /* [retval][out] */ IOpcPartUri **signatureOriginPartName);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, SetSignatureOriginPartName)
        HRESULT ( STDMETHODCALLTYPE *SetSignatureOriginPartName )( 
            IOpcDigitalSignatureManager * This,
            /* [unique][in] */ IOpcPartUri *signatureOriginPartName);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, GetSignatureEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureEnumerator )( 
            IOpcDigitalSignatureManager * This,
            /* [retval][out] */ IOpcDigitalSignatureEnumerator **signatureEnumerator);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, RemoveSignature)
        HRESULT ( STDMETHODCALLTYPE *RemoveSignature )( 
            IOpcDigitalSignatureManager * This,
            /* [in] */ IOpcPartUri *signaturePartName);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, CreateSigningOptions)
        HRESULT ( STDMETHODCALLTYPE *CreateSigningOptions )( 
            IOpcDigitalSignatureManager * This,
            /* [retval][out] */ IOpcSigningOptions **signingOptions);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, Validate)
        HRESULT ( STDMETHODCALLTYPE *Validate )( 
            IOpcDigitalSignatureManager * This,
            /* [in] */ IOpcDigitalSignature *signature,
            /* [in] */ const CERT_CONTEXT *certificate,
            /* [retval][out] */ OPC_SIGNATURE_VALIDATION_RESULT *validationResult);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, Sign)
        HRESULT ( STDMETHODCALLTYPE *Sign )( 
            IOpcDigitalSignatureManager * This,
            /* [in] */ const CERT_CONTEXT *certificate,
            /* [in] */ IOpcSigningOptions *signingOptions,
            /* [retval][out] */ IOpcDigitalSignature **digitalSignature);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureManager, ReplaceSignatureXml)
        HRESULT ( STDMETHODCALLTYPE *ReplaceSignatureXml )( 
            IOpcDigitalSignatureManager * This,
            /* [in] */ IOpcPartUri *signaturePartName,
            /* [size_is][in] */ const UINT8 *newSignatureXml,
            /* [in] */ UINT32 count,
            /* [retval][out] */ IOpcDigitalSignature **digitalSignature);
        
        END_INTERFACE
    } IOpcDigitalSignatureManagerVtbl;

    interface IOpcDigitalSignatureManager
    {
        CONST_VTBL struct IOpcDigitalSignatureManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcDigitalSignatureManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcDigitalSignatureManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcDigitalSignatureManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcDigitalSignatureManager_GetSignatureOriginPartName(This,signatureOriginPartName)	\
    ( (This)->lpVtbl -> GetSignatureOriginPartName(This,signatureOriginPartName) ) 

#define IOpcDigitalSignatureManager_SetSignatureOriginPartName(This,signatureOriginPartName)	\
    ( (This)->lpVtbl -> SetSignatureOriginPartName(This,signatureOriginPartName) ) 

#define IOpcDigitalSignatureManager_GetSignatureEnumerator(This,signatureEnumerator)	\
    ( (This)->lpVtbl -> GetSignatureEnumerator(This,signatureEnumerator) ) 

#define IOpcDigitalSignatureManager_RemoveSignature(This,signaturePartName)	\
    ( (This)->lpVtbl -> RemoveSignature(This,signaturePartName) ) 

#define IOpcDigitalSignatureManager_CreateSigningOptions(This,signingOptions)	\
    ( (This)->lpVtbl -> CreateSigningOptions(This,signingOptions) ) 

#define IOpcDigitalSignatureManager_Validate(This,signature,certificate,validationResult)	\
    ( (This)->lpVtbl -> Validate(This,signature,certificate,validationResult) ) 

#define IOpcDigitalSignatureManager_Sign(This,certificate,signingOptions,digitalSignature)	\
    ( (This)->lpVtbl -> Sign(This,certificate,signingOptions,digitalSignature) ) 

#define IOpcDigitalSignatureManager_ReplaceSignatureXml(This,signaturePartName,newSignatureXml,count,digitalSignature)	\
    ( (This)->lpVtbl -> ReplaceSignatureXml(This,signaturePartName,newSignatureXml,count,digitalSignature) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcDigitalSignatureManager_INTERFACE_DEFINED__ */


#ifndef __IOpcSignaturePartReferenceEnumerator_INTERFACE_DEFINED__
#define __IOpcSignaturePartReferenceEnumerator_INTERFACE_DEFINED__

/* interface IOpcSignaturePartReferenceEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignaturePartReferenceEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80eb1561-8c77-49cf-8266-459b356ee99a")
    IOpcSignaturePartReferenceEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReference **partReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignaturePartReferenceEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReference **partReference);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcSignaturePartReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceEnumerator **copy);
        
        END_INTERFACE
    } IOpcSignaturePartReferenceEnumeratorVtbl;

    interface IOpcSignaturePartReferenceEnumerator
    {
        CONST_VTBL struct IOpcSignaturePartReferenceEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignaturePartReferenceEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignaturePartReferenceEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignaturePartReferenceEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignaturePartReferenceEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcSignaturePartReferenceEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcSignaturePartReferenceEnumerator_GetCurrent(This,partReference)	\
    ( (This)->lpVtbl -> GetCurrent(This,partReference) ) 

#define IOpcSignaturePartReferenceEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignaturePartReferenceEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReferenceEnumerator_INTERFACE_DEFINED__
#define __IOpcSignatureRelationshipReferenceEnumerator_INTERFACE_DEFINED__

/* interface IOpcSignatureRelationshipReferenceEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureRelationshipReferenceEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("773ba3e4-f021-48e4-aa04-9816db5d3495")
    IOpcSignatureRelationshipReferenceEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReference **relationshipReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureRelationshipReferenceEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReference **relationshipReference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcSignatureRelationshipReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceEnumerator **copy);
        
        END_INTERFACE
    } IOpcSignatureRelationshipReferenceEnumeratorVtbl;

    interface IOpcSignatureRelationshipReferenceEnumerator
    {
        CONST_VTBL struct IOpcSignatureRelationshipReferenceEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureRelationshipReferenceEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureRelationshipReferenceEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureRelationshipReferenceEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureRelationshipReferenceEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcSignatureRelationshipReferenceEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcSignatureRelationshipReferenceEnumerator_GetCurrent(This,relationshipReference)	\
    ( (This)->lpVtbl -> GetCurrent(This,relationshipReference) ) 

#define IOpcSignatureRelationshipReferenceEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureRelationshipReferenceEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationshipSelectorEnumerator_INTERFACE_DEFINED__
#define __IOpcRelationshipSelectorEnumerator_INTERFACE_DEFINED__

/* interface IOpcRelationshipSelectorEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationshipSelectorEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e50a181-a91b-48ac-88d2-bca3d8f8c0b1")
    IOpcRelationshipSelectorEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelector **relationshipSelector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipSelectorEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelector **relationshipSelector);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcRelationshipSelectorEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **copy);
        
        END_INTERFACE
    } IOpcRelationshipSelectorEnumeratorVtbl;

    interface IOpcRelationshipSelectorEnumerator
    {
        CONST_VTBL struct IOpcRelationshipSelectorEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationshipSelectorEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationshipSelectorEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationshipSelectorEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationshipSelectorEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcRelationshipSelectorEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcRelationshipSelectorEnumerator_GetCurrent(This,relationshipSelector)	\
    ( (This)->lpVtbl -> GetCurrent(This,relationshipSelector) ) 

#define IOpcRelationshipSelectorEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationshipSelectorEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureReferenceEnumerator_INTERFACE_DEFINED__
#define __IOpcSignatureReferenceEnumerator_INTERFACE_DEFINED__

/* interface IOpcSignatureReferenceEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureReferenceEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cfa59a45-28b1-4868-969e-fa8097fdc12a")
    IOpcSignatureReferenceEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReference **reference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureReferenceEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReference **reference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcSignatureReferenceEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceEnumerator **copy);
        
        END_INTERFACE
    } IOpcSignatureReferenceEnumeratorVtbl;

    interface IOpcSignatureReferenceEnumerator
    {
        CONST_VTBL struct IOpcSignatureReferenceEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureReferenceEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureReferenceEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureReferenceEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureReferenceEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcSignatureReferenceEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcSignatureReferenceEnumerator_GetCurrent(This,reference)	\
    ( (This)->lpVtbl -> GetCurrent(This,reference) ) 

#define IOpcSignatureReferenceEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureReferenceEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureCustomObjectEnumerator_INTERFACE_DEFINED__
#define __IOpcSignatureCustomObjectEnumerator_INTERFACE_DEFINED__

/* interface IOpcSignatureCustomObjectEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureCustomObjectEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5ee4fe1d-e1b0-4683-8079-7ea0fcf80b4c")
    IOpcSignatureCustomObjectEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObject **customObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureCustomObjectEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObject **customObject);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcSignatureCustomObjectEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectEnumerator **copy);
        
        END_INTERFACE
    } IOpcSignatureCustomObjectEnumeratorVtbl;

    interface IOpcSignatureCustomObjectEnumerator
    {
        CONST_VTBL struct IOpcSignatureCustomObjectEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureCustomObjectEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureCustomObjectEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureCustomObjectEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureCustomObjectEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcSignatureCustomObjectEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcSignatureCustomObjectEnumerator_GetCurrent(This,customObject)	\
    ( (This)->lpVtbl -> GetCurrent(This,customObject) ) 

#define IOpcSignatureCustomObjectEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureCustomObjectEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcCertificateEnumerator_INTERFACE_DEFINED__
#define __IOpcCertificateEnumerator_INTERFACE_DEFINED__

/* interface IOpcCertificateEnumerator */
/* [ref][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IOpcCertificateEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85131937-8f24-421f-b439-59ab24d140b8")
    IOpcCertificateEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ const CERT_CONTEXT **certificate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IOpcCertificateEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcCertificateEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOpcCertificateEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOpcCertificateEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOpcCertificateEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcCertificateEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            IOpcCertificateEnumerator * This,
            /* [retval][out] */ BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcCertificateEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            IOpcCertificateEnumerator * This,
            /* [retval][out] */ BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcCertificateEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            IOpcCertificateEnumerator * This,
            /* [retval][out] */ const CERT_CONTEXT **certificate);
        
        DECLSPEC_XFGVIRT(IOpcCertificateEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IOpcCertificateEnumerator * This,
            /* [retval][out] */ IOpcCertificateEnumerator **copy);
        
        END_INTERFACE
    } IOpcCertificateEnumeratorVtbl;

    interface IOpcCertificateEnumerator
    {
        CONST_VTBL struct IOpcCertificateEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcCertificateEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcCertificateEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcCertificateEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcCertificateEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcCertificateEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcCertificateEnumerator_GetCurrent(This,certificate)	\
    ( (This)->lpVtbl -> GetCurrent(This,certificate) ) 

#define IOpcCertificateEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcCertificateEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcDigitalSignatureEnumerator_INTERFACE_DEFINED__
#define __IOpcDigitalSignatureEnumerator_INTERFACE_DEFINED__

/* interface IOpcDigitalSignatureEnumerator */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcDigitalSignatureEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("967b6882-0ba3-4358-b9e7-b64c75063c5e")
    IOpcDigitalSignatureEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ __RPC__out BOOL *hasNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MovePrevious( 
            /* [retval][out] */ __RPC__out BOOL *hasPrevious) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignature **digitalSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignatureEnumerator **copy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcDigitalSignatureEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureEnumerator, MoveNext)
        HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasNext);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureEnumerator, MovePrevious)
        HRESULT ( STDMETHODCALLTYPE *MovePrevious )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This,
            /* [retval][out] */ __RPC__out BOOL *hasPrevious);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureEnumerator, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignature **digitalSignature);
        
        DECLSPEC_XFGVIRT(IOpcDigitalSignatureEnumerator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOpcDigitalSignatureEnumerator * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignatureEnumerator **copy);
        
        END_INTERFACE
    } IOpcDigitalSignatureEnumeratorVtbl;

    interface IOpcDigitalSignatureEnumerator
    {
        CONST_VTBL struct IOpcDigitalSignatureEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcDigitalSignatureEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcDigitalSignatureEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcDigitalSignatureEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcDigitalSignatureEnumerator_MoveNext(This,hasNext)	\
    ( (This)->lpVtbl -> MoveNext(This,hasNext) ) 

#define IOpcDigitalSignatureEnumerator_MovePrevious(This,hasPrevious)	\
    ( (This)->lpVtbl -> MovePrevious(This,hasPrevious) ) 

#define IOpcDigitalSignatureEnumerator_GetCurrent(This,digitalSignature)	\
    ( (This)->lpVtbl -> GetCurrent(This,digitalSignature) ) 

#define IOpcDigitalSignatureEnumerator_Clone(This,copy)	\
    ( (This)->lpVtbl -> Clone(This,copy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcDigitalSignatureEnumerator_INTERFACE_DEFINED__ */


#ifndef __IOpcSignaturePartReferenceSet_INTERFACE_DEFINED__
#define __IOpcSignaturePartReferenceSet_INTERFACE_DEFINED__

/* interface IOpcSignaturePartReferenceSet */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignaturePartReferenceSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6c9fe28c-ecd9-4b22-9d36-7fdde670fec0")
    IOpcSignaturePartReferenceSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReference **partReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in_opt IOpcSignaturePartReference *partReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceEnumerator **partReferenceEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignaturePartReferenceSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignaturePartReferenceSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignaturePartReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignaturePartReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceSet, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IOpcSignaturePartReferenceSet * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReference **partReference);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IOpcSignaturePartReferenceSet * This,
            /* [in] */ __RPC__in_opt IOpcSignaturePartReference *partReference);
        
        DECLSPEC_XFGVIRT(IOpcSignaturePartReferenceSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcSignaturePartReferenceSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignaturePartReferenceEnumerator **partReferenceEnumerator);
        
        END_INTERFACE
    } IOpcSignaturePartReferenceSetVtbl;

    interface IOpcSignaturePartReferenceSet
    {
        CONST_VTBL struct IOpcSignaturePartReferenceSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignaturePartReferenceSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignaturePartReferenceSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignaturePartReferenceSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignaturePartReferenceSet_Create(This,partUri,digestMethod,transformMethod,partReference)	\
    ( (This)->lpVtbl -> Create(This,partUri,digestMethod,transformMethod,partReference) ) 

#define IOpcSignaturePartReferenceSet_Delete(This,partReference)	\
    ( (This)->lpVtbl -> Delete(This,partReference) ) 

#define IOpcSignaturePartReferenceSet_GetEnumerator(This,partReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,partReferenceEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignaturePartReferenceSet_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureRelationshipReferenceSet_INTERFACE_DEFINED__
#define __IOpcSignatureRelationshipReferenceSet_INTERFACE_DEFINED__

/* interface IOpcSignatureRelationshipReferenceSet */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureRelationshipReferenceSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9f863ca5-3631-404c-828d-807e0715069b")
    IOpcSignatureRelationshipReferenceSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in_opt IOpcUri *sourceUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_RELATIONSHIPS_SIGNING_OPTION relationshipSigningOption,
            /* [unique][in] */ __RPC__in_opt IOpcRelationshipSelectorSet *selectorSet,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReference **relationshipReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRelationshipSelectorSet( 
            /* [out] */ __RPC__deref_out_opt IOpcRelationshipSelectorSet **selectorSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in_opt IOpcSignatureRelationshipReference *relationshipReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceEnumerator **relationshipReferenceEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureRelationshipReferenceSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceSet, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This,
            /* [in] */ __RPC__in_opt IOpcUri *sourceUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_RELATIONSHIPS_SIGNING_OPTION relationshipSigningOption,
            /* [unique][in] */ __RPC__in_opt IOpcRelationshipSelectorSet *selectorSet,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReference **relationshipReference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceSet, CreateRelationshipSelectorSet)
        HRESULT ( STDMETHODCALLTYPE *CreateRelationshipSelectorSet )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This,
            /* [out] */ __RPC__deref_out_opt IOpcRelationshipSelectorSet **selectorSet);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This,
            /* [in] */ __RPC__in_opt IOpcSignatureRelationshipReference *relationshipReference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureRelationshipReferenceSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcSignatureRelationshipReferenceSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureRelationshipReferenceEnumerator **relationshipReferenceEnumerator);
        
        END_INTERFACE
    } IOpcSignatureRelationshipReferenceSetVtbl;

    interface IOpcSignatureRelationshipReferenceSet
    {
        CONST_VTBL struct IOpcSignatureRelationshipReferenceSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureRelationshipReferenceSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureRelationshipReferenceSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureRelationshipReferenceSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureRelationshipReferenceSet_Create(This,sourceUri,digestMethod,relationshipSigningOption,selectorSet,transformMethod,relationshipReference)	\
    ( (This)->lpVtbl -> Create(This,sourceUri,digestMethod,relationshipSigningOption,selectorSet,transformMethod,relationshipReference) ) 

#define IOpcSignatureRelationshipReferenceSet_CreateRelationshipSelectorSet(This,selectorSet)	\
    ( (This)->lpVtbl -> CreateRelationshipSelectorSet(This,selectorSet) ) 

#define IOpcSignatureRelationshipReferenceSet_Delete(This,relationshipReference)	\
    ( (This)->lpVtbl -> Delete(This,relationshipReference) ) 

#define IOpcSignatureRelationshipReferenceSet_GetEnumerator(This,relationshipReferenceEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,relationshipReferenceEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureRelationshipReferenceSet_INTERFACE_DEFINED__ */


#ifndef __IOpcRelationshipSelectorSet_INTERFACE_DEFINED__
#define __IOpcRelationshipSelectorSet_INTERFACE_DEFINED__

/* interface IOpcRelationshipSelectorSet */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcRelationshipSelectorSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6e34c269-a4d3-47c0-b5c4-87ff2b3b6136")
    IOpcRelationshipSelectorSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ OPC_RELATIONSHIP_SELECTOR selector,
            /* [in] */ __RPC__in LPCWSTR selectionCriterion,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelector **relationshipSelector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in_opt IOpcRelationshipSelector *relationshipSelector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **relationshipSelectorEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcRelationshipSelectorSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcRelationshipSelectorSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcRelationshipSelectorSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcRelationshipSelectorSet * This);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorSet, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IOpcRelationshipSelectorSet * This,
            /* [in] */ OPC_RELATIONSHIP_SELECTOR selector,
            /* [in] */ __RPC__in LPCWSTR selectionCriterion,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelector **relationshipSelector);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IOpcRelationshipSelectorSet * This,
            /* [in] */ __RPC__in_opt IOpcRelationshipSelector *relationshipSelector);
        
        DECLSPEC_XFGVIRT(IOpcRelationshipSelectorSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcRelationshipSelectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcRelationshipSelectorEnumerator **relationshipSelectorEnumerator);
        
        END_INTERFACE
    } IOpcRelationshipSelectorSetVtbl;

    interface IOpcRelationshipSelectorSet
    {
        CONST_VTBL struct IOpcRelationshipSelectorSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcRelationshipSelectorSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcRelationshipSelectorSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcRelationshipSelectorSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcRelationshipSelectorSet_Create(This,selector,selectionCriterion,relationshipSelector)	\
    ( (This)->lpVtbl -> Create(This,selector,selectionCriterion,relationshipSelector) ) 

#define IOpcRelationshipSelectorSet_Delete(This,relationshipSelector)	\
    ( (This)->lpVtbl -> Delete(This,relationshipSelector) ) 

#define IOpcRelationshipSelectorSet_GetEnumerator(This,relationshipSelectorEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,relationshipSelectorEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcRelationshipSelectorSet_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureReferenceSet_INTERFACE_DEFINED__
#define __IOpcSignatureReferenceSet_INTERFACE_DEFINED__

/* interface IOpcSignatureReferenceSet */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureReferenceSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f3b02d31-ab12-42dd-9e2f-2b16761c3c1e")
    IOpcSignatureReferenceSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in_opt IUri *referenceUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR referenceId,
            /* [unique][in] */ __RPC__in_opt LPCWSTR type,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReference **reference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in_opt IOpcSignatureReference *reference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceEnumerator **referenceEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureReferenceSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureReferenceSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureReferenceSet * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceSet, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IOpcSignatureReferenceSet * This,
            /* [in] */ __RPC__in_opt IUri *referenceUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR referenceId,
            /* [unique][in] */ __RPC__in_opt LPCWSTR type,
            /* [unique][in] */ __RPC__in_opt LPCWSTR digestMethod,
            /* [in] */ OPC_CANONICALIZATION_METHOD transformMethod,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReference **reference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IOpcSignatureReferenceSet * This,
            /* [in] */ __RPC__in_opt IOpcSignatureReference *reference);
        
        DECLSPEC_XFGVIRT(IOpcSignatureReferenceSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcSignatureReferenceSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureReferenceEnumerator **referenceEnumerator);
        
        END_INTERFACE
    } IOpcSignatureReferenceSetVtbl;

    interface IOpcSignatureReferenceSet
    {
        CONST_VTBL struct IOpcSignatureReferenceSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureReferenceSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureReferenceSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureReferenceSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureReferenceSet_Create(This,referenceUri,referenceId,type,digestMethod,transformMethod,reference)	\
    ( (This)->lpVtbl -> Create(This,referenceUri,referenceId,type,digestMethod,transformMethod,reference) ) 

#define IOpcSignatureReferenceSet_Delete(This,reference)	\
    ( (This)->lpVtbl -> Delete(This,reference) ) 

#define IOpcSignatureReferenceSet_GetEnumerator(This,referenceEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,referenceEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureReferenceSet_INTERFACE_DEFINED__ */


#ifndef __IOpcSignatureCustomObjectSet_INTERFACE_DEFINED__
#define __IOpcSignatureCustomObjectSet_INTERFACE_DEFINED__

/* interface IOpcSignatureCustomObjectSet */
/* [ref][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IOpcSignatureCustomObjectSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f792ac5-7947-4e11-bc3d-2659ff046ae1")
    IOpcSignatureCustomObjectSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [size_is][in] */ __RPC__in_ecount_full(count) const UINT8 *xmlMarkup,
            /* [in] */ UINT32 count,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObject **customObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in_opt IOpcSignatureCustomObject *customObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectEnumerator **customObjectEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcSignatureCustomObjectSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcSignatureCustomObjectSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcSignatureCustomObjectSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcSignatureCustomObjectSet * This);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectSet, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IOpcSignatureCustomObjectSet * This,
            /* [size_is][in] */ __RPC__in_ecount_full(count) const UINT8 *xmlMarkup,
            /* [in] */ UINT32 count,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObject **customObject);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IOpcSignatureCustomObjectSet * This,
            /* [in] */ __RPC__in_opt IOpcSignatureCustomObject *customObject);
        
        DECLSPEC_XFGVIRT(IOpcSignatureCustomObjectSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            __RPC__in IOpcSignatureCustomObjectSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcSignatureCustomObjectEnumerator **customObjectEnumerator);
        
        END_INTERFACE
    } IOpcSignatureCustomObjectSetVtbl;

    interface IOpcSignatureCustomObjectSet
    {
        CONST_VTBL struct IOpcSignatureCustomObjectSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcSignatureCustomObjectSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcSignatureCustomObjectSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcSignatureCustomObjectSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcSignatureCustomObjectSet_Create(This,xmlMarkup,count,customObject)	\
    ( (This)->lpVtbl -> Create(This,xmlMarkup,count,customObject) ) 

#define IOpcSignatureCustomObjectSet_Delete(This,customObject)	\
    ( (This)->lpVtbl -> Delete(This,customObject) ) 

#define IOpcSignatureCustomObjectSet_GetEnumerator(This,customObjectEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,customObjectEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcSignatureCustomObjectSet_INTERFACE_DEFINED__ */


#ifndef __IOpcCertificateSet_INTERFACE_DEFINED__
#define __IOpcCertificateSet_INTERFACE_DEFINED__

/* interface IOpcCertificateSet */
/* [ref][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IOpcCertificateSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56ea4325-8e2d-4167-b1a4-e486d24c8fa7")
    IOpcCertificateSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ const CERT_CONTEXT *certificate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ const CERT_CONTEXT *certificate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcCertificateSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOpcCertificateSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOpcCertificateSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOpcCertificateSet * This);
        
        DECLSPEC_XFGVIRT(IOpcCertificateSet, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            IOpcCertificateSet * This,
            /* [in] */ const CERT_CONTEXT *certificate);
        
        DECLSPEC_XFGVIRT(IOpcCertificateSet, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IOpcCertificateSet * This,
            /* [in] */ const CERT_CONTEXT *certificate);
        
        DECLSPEC_XFGVIRT(IOpcCertificateSet, GetEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            IOpcCertificateSet * This,
            /* [retval][out] */ IOpcCertificateEnumerator **certificateEnumerator);
        
        END_INTERFACE
    } IOpcCertificateSetVtbl;

    interface IOpcCertificateSet
    {
        CONST_VTBL struct IOpcCertificateSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcCertificateSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcCertificateSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcCertificateSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcCertificateSet_Add(This,certificate)	\
    ( (This)->lpVtbl -> Add(This,certificate) ) 

#define IOpcCertificateSet_Remove(This,certificate)	\
    ( (This)->lpVtbl -> Remove(This,certificate) ) 

#define IOpcCertificateSet_GetEnumerator(This,certificateEnumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,certificateEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcCertificateSet_INTERFACE_DEFINED__ */


#ifndef __IOpcFactory_INTERFACE_DEFINED__
#define __IOpcFactory_INTERFACE_DEFINED__

/* interface IOpcFactory */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IOpcFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6d0b4446-cd73-4ab3-94f4-8ccdf6116154")
    IOpcFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePackageRootUri( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **rootUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePartUri( 
            /* [string][in] */ __RPC__in_string LPCWSTR pwzUri,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateStreamOnFile( 
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ OPC_STREAM_IO_MODE ioMode,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD dwFlagsAndAttributes,
            /* [retval][out] */ IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackage( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadPackageFromStream( 
            /* [in] */ __RPC__in_opt IStream *stream,
            /* [in] */ OPC_READ_FLAGS flags,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WritePackageToStream( 
            /* [in] */ __RPC__in_opt IOpcPackage *package,
            /* [in] */ OPC_WRITE_FLAGS flags,
            /* [in] */ __RPC__in_opt IStream *stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDigitalSignatureManager( 
            /* [in] */ __RPC__in_opt IOpcPackage *package,
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignatureManager **signatureManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpcFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpcFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpcFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpcFactory * This);
        
        DECLSPEC_XFGVIRT(IOpcFactory, CreatePackageRootUri)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageRootUri )( 
            __RPC__in IOpcFactory * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcUri **rootUri);
        
        DECLSPEC_XFGVIRT(IOpcFactory, CreatePartUri)
        HRESULT ( STDMETHODCALLTYPE *CreatePartUri )( 
            __RPC__in IOpcFactory * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwzUri,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IOpcFactory, CreateStreamOnFile)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateStreamOnFile )( 
            IOpcFactory * This,
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ OPC_STREAM_IO_MODE ioMode,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD dwFlagsAndAttributes,
            /* [retval][out] */ IStream **stream);
        
        DECLSPEC_XFGVIRT(IOpcFactory, CreatePackage)
        HRESULT ( STDMETHODCALLTYPE *CreatePackage )( 
            __RPC__in IOpcFactory * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPackage **package);
        
        DECLSPEC_XFGVIRT(IOpcFactory, ReadPackageFromStream)
        HRESULT ( STDMETHODCALLTYPE *ReadPackageFromStream )( 
            __RPC__in IOpcFactory * This,
            /* [in] */ __RPC__in_opt IStream *stream,
            /* [in] */ OPC_READ_FLAGS flags,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPackage **package);
        
        DECLSPEC_XFGVIRT(IOpcFactory, WritePackageToStream)
        HRESULT ( STDMETHODCALLTYPE *WritePackageToStream )( 
            __RPC__in IOpcFactory * This,
            /* [in] */ __RPC__in_opt IOpcPackage *package,
            /* [in] */ OPC_WRITE_FLAGS flags,
            /* [in] */ __RPC__in_opt IStream *stream);
        
        DECLSPEC_XFGVIRT(IOpcFactory, CreateDigitalSignatureManager)
        HRESULT ( STDMETHODCALLTYPE *CreateDigitalSignatureManager )( 
            __RPC__in IOpcFactory * This,
            /* [in] */ __RPC__in_opt IOpcPackage *package,
            /* [retval][out] */ __RPC__deref_out_opt IOpcDigitalSignatureManager **signatureManager);
        
        END_INTERFACE
    } IOpcFactoryVtbl;

    interface IOpcFactory
    {
        CONST_VTBL struct IOpcFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpcFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpcFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpcFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpcFactory_CreatePackageRootUri(This,rootUri)	\
    ( (This)->lpVtbl -> CreatePackageRootUri(This,rootUri) ) 

#define IOpcFactory_CreatePartUri(This,pwzUri,partUri)	\
    ( (This)->lpVtbl -> CreatePartUri(This,pwzUri,partUri) ) 

#define IOpcFactory_CreateStreamOnFile(This,filename,ioMode,securityAttributes,dwFlagsAndAttributes,stream)	\
    ( (This)->lpVtbl -> CreateStreamOnFile(This,filename,ioMode,securityAttributes,dwFlagsAndAttributes,stream) ) 

#define IOpcFactory_CreatePackage(This,package)	\
    ( (This)->lpVtbl -> CreatePackage(This,package) ) 

#define IOpcFactory_ReadPackageFromStream(This,stream,flags,package)	\
    ( (This)->lpVtbl -> ReadPackageFromStream(This,stream,flags,package) ) 

#define IOpcFactory_WritePackageToStream(This,package,flags,stream)	\
    ( (This)->lpVtbl -> WritePackageToStream(This,package,flags,stream) ) 

#define IOpcFactory_CreateDigitalSignatureManager(This,package,signatureManager)	\
    ( (This)->lpVtbl -> CreateDigitalSignatureManager(This,package,signatureManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpcFactory_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_OpcFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("6B2D6BA0-9F3E-4f27-920B-313CC426A39E")
OpcFactory;
#endif
#endif /* __MSOPC_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msopc_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msopc_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



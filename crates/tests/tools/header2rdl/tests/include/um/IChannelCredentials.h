//-----------------------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//-----------------------------------------------------------------------------
//=--------------------------------------------------------------------------=
// IChannelCredentials.h
//=--------------------------------------------------------------------------=
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


/* File created by MIDL compiler version 6.00.0366 */
//@@MIDL_FILE_HEADING(  )

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __ichannelcredentials_h__
#define __ichannelcredentials_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

/* Forward Declarations */ 

#ifndef __IChannelCredentials_FWD_DEFINED__
#define __IChannelCredentials_FWD_DEFINED__
typedef interface IChannelCredentials IChannelCredentials;
#endif 	/* __IChannelCredentials_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 

void * __RPC_USER MIDL_user_allocate(size_t);
void __RPC_USER MIDL_user_free( void * ); 


#ifndef __IChannelCredentials_INTERFACE_DEFINED__
#define __IChannelCredentials_INTERFACE_DEFINED__

/* interface IChannelCredentials */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IChannelCredentials;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("181b448c-c17c-4b17-ac6d-06699b93198f")
    IChannelCredentials : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetWindowsCredential( 
            /* [in] */ BSTR domain,
            /* [in] */ BSTR username,
            /* [in] */ BSTR password,
            /* [in] */ int impersonationLevel,
            /* [in] */ BOOL allowNtlm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUserNameCredential( 
            /* [in] */ BSTR username,
            /* [in] */ BSTR password) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificateFromStore( 
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName,
            /* [in] */ BSTR findYype,
            /* [in] */ VARIANT findValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificateFromStoreByName( 
            /* [in] */ BSTR subjectName,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificateFromFile( 
            /* [in] */ BSTR filename,
            /* [in] */ BSTR password,
            /* [in] */ BSTR keystorageFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultServiceCertificateFromStore( 
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName,
            /* [in] */ BSTR findType,
            /* [in] */ VARIANT findValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultServiceCertificateFromStoreByName( 
            /* [in] */ BSTR subjectName,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultServiceCertificateFromFile( 
            /* [in] */ BSTR filename,
            /* [in] */ BSTR password,
            /* [in] */ BSTR keystorageFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetServiceCertificateAuthentication( 
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR revocationMode,
            /* [in] */ BSTR certificateValidationMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIssuedToken( 
            /* [in] */ BSTR localIssuerAddres,
            /* [in] */ BSTR localIssuerBindingType,
            /* [in] */ BSTR localIssuerBinding) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IChannelCredentialsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IChannelCredentials * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IChannelCredentials * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IChannelCredentials * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IChannelCredentials * This,
            /* [out] */ UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IChannelCredentials * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IChannelCredentials * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IChannelCredentials * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        HRESULT ( STDMETHODCALLTYPE *SetWindowsCredential )( 
            IChannelCredentials * This,
            /* [in] */ BSTR domain,
            /* [in] */ BSTR username,
            /* [in] */ BSTR password,
            /* [in] */ int impersonationLevel,
            /* [in] */ BOOL allowNtlm);
        
        HRESULT ( STDMETHODCALLTYPE *SetUserNameCredential )( 
            IChannelCredentials * This,
            /* [in] */ BSTR username,
            /* [in] */ BSTR password);
        
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateFromStore )( 
            IChannelCredentials * This,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName,
            /* [in] */ BSTR findYype,
            /* [in] */ VARIANT findValue);
        
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateFromStoreByName )( 
            IChannelCredentials * This,
            /* [in] */ BSTR subjectName,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName);
        
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateFromFile )( 
            IChannelCredentials * This,
            /* [in] */ BSTR filename,
            /* [in] */ BSTR password,
            /* [in] */ BSTR keystorageFlags);
        
        HRESULT ( STDMETHODCALLTYPE *SetDefaultServiceCertificateFromStore )( 
            IChannelCredentials * This,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName,
            /* [in] */ BSTR findType,
            /* [in] */ VARIANT findValue);
        
        HRESULT ( STDMETHODCALLTYPE *SetDefaultServiceCertificateFromStoreByName )( 
            IChannelCredentials * This,
            /* [in] */ BSTR subjectName,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR storeName);
        
        HRESULT ( STDMETHODCALLTYPE *SetDefaultServiceCertificateFromFile )( 
            IChannelCredentials * This,
            /* [in] */ BSTR filename,
            /* [in] */ BSTR password,
            /* [in] */ BSTR keystorageFlags);
        
        HRESULT ( STDMETHODCALLTYPE *SetServiceCertificateAuthentication )( 
            IChannelCredentials * This,
            /* [in] */ BSTR storeLocation,
            /* [in] */ BSTR revocationMode,
            /* [in] */ BSTR certificateValidationMode);
        
        HRESULT ( STDMETHODCALLTYPE *SetIssuedToken )( 
            IChannelCredentials * This,
            /* [in] */ BSTR localIssuerAddres,
            /* [in] */ BSTR localIssuerBindingType,
            /* [in] */ BSTR localIssuerBinding);
        
        END_INTERFACE
    } IChannelCredentialsVtbl;

    interface IChannelCredentials
    {
        CONST_VTBL struct IChannelCredentialsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChannelCredentials_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IChannelCredentials_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IChannelCredentials_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IChannelCredentials_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IChannelCredentials_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IChannelCredentials_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IChannelCredentials_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IChannelCredentials_SetWindowsCredential(This,domain,username,password,impersonationLevel,allowNtlm)	\
    (This)->lpVtbl -> SetWindowsCredential(This,domain,username,password,impersonationLevel,allowNtlm)

#define IChannelCredentials_SetUserNameCredential(This,username,password)	\
    (This)->lpVtbl -> SetUserNameCredential(This,username,password)

#define IChannelCredentials_SetClientCertificateFromStore(This,storeLocation,storeName,findYype,findValue)	\
    (This)->lpVtbl -> SetClientCertificateFromStore(This,storeLocation,storeName,findYype,findValue)

#define IChannelCredentials_SetClientCertificateFromStoreByName(This,subjectName,storeLocation,storeName)	\
    (This)->lpVtbl -> SetClientCertificateFromStoreByName(This,subjectName,storeLocation,storeName)

#define IChannelCredentials_SetClientCertificateFromFile(This,filename,password,keystorageFlags)	\
    (This)->lpVtbl -> SetClientCertificateFromFile(This,filename,password,keystorageFlags)

#define IChannelCredentials_SetDefaultServiceCertificateFromStore(This,storeLocation,storeName,findType,findValue)	\
    (This)->lpVtbl -> SetDefaultServiceCertificateFromStore(This,storeLocation,storeName,findType,findValue)

#define IChannelCredentials_SetDefaultServiceCertificateFromStoreByName(This,subjectName,storeLocation,storeName)	\
    (This)->lpVtbl -> SetDefaultServiceCertificateFromStoreByName(This,subjectName,storeLocation,storeName)

#define IChannelCredentials_SetDefaultServiceCertificateFromFile(This,filename,password,keystorageFlags)	\
    (This)->lpVtbl -> SetDefaultServiceCertificateFromFile(This,filename,password,keystorageFlags)

#define IChannelCredentials_SetServiceCertificateAuthentication(This,storeLocation,revocationMode,certificateValidationMode)	\
    (This)->lpVtbl -> SetServiceCertificateAuthentication(This,storeLocation,revocationMode,certificateValidationMode)

#define IChannelCredentials_SetIssuedToken(This,localIssuerAddres,localIssuerBindingType,localIssuerBinding)	\
    (This)->lpVtbl -> SetIssuedToken(This,localIssuerAddres,localIssuerBindingType,localIssuerBinding)

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IChannelCredentials_INTERFACE_DEFINED__ */


#ifdef __cplusplus
}
#endif

#endif



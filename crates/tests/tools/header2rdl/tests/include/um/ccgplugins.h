

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

#ifndef __ccgplugins_h__
#define __ccgplugins_h__

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

#ifndef __ICcgDomainAuthCredentials_FWD_DEFINED__
#define __ICcgDomainAuthCredentials_FWD_DEFINED__
typedef interface ICcgDomainAuthCredentials ICcgDomainAuthCredentials;

#endif 	/* __ICcgDomainAuthCredentials_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __ICcgDomainAuthCredentials_INTERFACE_DEFINED__
#define __ICcgDomainAuthCredentials_INTERFACE_DEFINED__

/* interface ICcgDomainAuthCredentials */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICcgDomainAuthCredentials;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ecda518-2010-4437-8bc3-46e752b7b172")
    ICcgDomainAuthCredentials : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPasswordCredentials( 
            /* [in] */ __RPC__in LPCWSTR pluginInput,
            /* [out] */ __RPC__deref_out_opt LPWSTR *domainName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *username,
            /* [out] */ __RPC__deref_out_opt LPWSTR *password) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICcgDomainAuthCredentialsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICcgDomainAuthCredentials * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICcgDomainAuthCredentials * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICcgDomainAuthCredentials * This);
        
        DECLSPEC_XFGVIRT(ICcgDomainAuthCredentials, GetPasswordCredentials)
        HRESULT ( STDMETHODCALLTYPE *GetPasswordCredentials )( 
            __RPC__in ICcgDomainAuthCredentials * This,
            /* [in] */ __RPC__in LPCWSTR pluginInput,
            /* [out] */ __RPC__deref_out_opt LPWSTR *domainName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *username,
            /* [out] */ __RPC__deref_out_opt LPWSTR *password);
        
        END_INTERFACE
    } ICcgDomainAuthCredentialsVtbl;

    interface ICcgDomainAuthCredentials
    {
        CONST_VTBL struct ICcgDomainAuthCredentialsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICcgDomainAuthCredentials_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICcgDomainAuthCredentials_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICcgDomainAuthCredentials_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICcgDomainAuthCredentials_GetPasswordCredentials(This,pluginInput,domainName,username,password)	\
    ( (This)->lpVtbl -> GetPasswordCredentials(This,pluginInput,domainName,username,password) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICcgDomainAuthCredentials_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif





/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __bits2_5_h__
#define __bits2_5_h__

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

#ifndef __IBackgroundCopyJobHttpOptions_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions IBackgroundCopyJobHttpOptions;

#endif 	/* __IBackgroundCopyJobHttpOptions_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager2_5_FWD_DEFINED__
#define __BackgroundCopyManager2_5_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager2_5 BackgroundCopyManager2_5;
#else
typedef struct BackgroundCopyManager2_5 BackgroundCopyManager2_5;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager2_5_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJobHttpOptions_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions IBackgroundCopyJobHttpOptions;

#endif 	/* __IBackgroundCopyJobHttpOptions_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits2_5_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits2_5_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits2_5_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyJobHttpOptions_INTERFACE_DEFINED__
#define __IBackgroundCopyJobHttpOptions_INTERFACE_DEFINED__

/* interface IBackgroundCopyJobHttpOptions */
/* [object][helpstring][uuid] */ 

typedef 
enum BG_CERT_STORE_LOCATION
    {
        BG_CERT_STORE_LOCATION_CURRENT_USER	= 0,
        BG_CERT_STORE_LOCATION_LOCAL_MACHINE	= ( BG_CERT_STORE_LOCATION_CURRENT_USER + 1 ) ,
        BG_CERT_STORE_LOCATION_CURRENT_SERVICE	= ( BG_CERT_STORE_LOCATION_LOCAL_MACHINE + 1 ) ,
        BG_CERT_STORE_LOCATION_SERVICES	= ( BG_CERT_STORE_LOCATION_CURRENT_SERVICE + 1 ) ,
        BG_CERT_STORE_LOCATION_USERS	= ( BG_CERT_STORE_LOCATION_SERVICES + 1 ) ,
        BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY	= ( BG_CERT_STORE_LOCATION_USERS + 1 ) ,
        BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY	= ( BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY + 1 ) ,
        BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE	= ( BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY + 1 ) 
    } 	BG_CERT_STORE_LOCATION;


EXTERN_C const IID IID_IBackgroundCopyJobHttpOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f1bd1079-9f01-4bdc-8036-f09b70095066")
    IBackgroundCopyJobHttpOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificateByID( 
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(20) byte *pCertHashBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificateByName( 
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [in] */ __RPC__in LPCWSTR SubjectName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveClientCertificate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClientCertificate( 
            /* [ref][out] */ __RPC__out BG_CERT_STORE_LOCATION *pStoreLocation,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pStoreName,
            /* [ref][size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(20) byte **ppCertHashBlob,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pSubjectName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustomHeaders( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR RequestHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomHeaders( 
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pRequestHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSecurityFlags( 
            /* [in] */ ULONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecurityFlags( 
            /* [ref][out] */ __RPC__out ULONG *pFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJobHttpOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByID)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByID )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(20) byte *pCertHashBlob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByName)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByName )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [in] */ __RPC__in LPCWSTR SubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, RemoveClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *RemoveClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [ref][out] */ __RPC__out BG_CERT_STORE_LOCATION *pStoreLocation,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pStoreName,
            /* [ref][size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(20) byte **ppCertHashBlob,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pSubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *SetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR RequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pRequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *SetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [in] */ ULONG Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions * This,
            /* [ref][out] */ __RPC__out ULONG *pFlags);
        
        END_INTERFACE
    } IBackgroundCopyJobHttpOptionsVtbl;

    interface IBackgroundCopyJobHttpOptions
    {
        CONST_VTBL struct IBackgroundCopyJobHttpOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJobHttpOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJobHttpOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJobHttpOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJobHttpOptions_SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob)	\
    ( (This)->lpVtbl -> SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob) ) 

#define IBackgroundCopyJobHttpOptions_SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName)	\
    ( (This)->lpVtbl -> SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName) ) 

#define IBackgroundCopyJobHttpOptions_RemoveClientCertificate(This)	\
    ( (This)->lpVtbl -> RemoveClientCertificate(This) ) 

#define IBackgroundCopyJobHttpOptions_GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName)	\
    ( (This)->lpVtbl -> GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName) ) 

#define IBackgroundCopyJobHttpOptions_SetCustomHeaders(This,RequestHeaders)	\
    ( (This)->lpVtbl -> SetCustomHeaders(This,RequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions_GetCustomHeaders(This,pRequestHeaders)	\
    ( (This)->lpVtbl -> GetCustomHeaders(This,pRequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions_SetSecurityFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetSecurityFlags(This,Flags) ) 

#define IBackgroundCopyJobHttpOptions_GetSecurityFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetSecurityFlags(This,pFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJobHttpOptions_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager2_5_LIBRARY_DEFINED__
#define __BackgroundCopyManager2_5_LIBRARY_DEFINED__

/* library BackgroundCopyManager2_5 */
/* [version][lcid][helpstring][uuid] */ 






EXTERN_C const IID LIBID_BackgroundCopyManager2_5;

EXTERN_C const CLSID CLSID_BackgroundCopyManager2_5;

#ifdef __cplusplus

class DECLSPEC_UUID("03ca98d6-ff5d-49b8-abc6-03dd84127020")
BackgroundCopyManager2_5;
#endif
#endif /* __BackgroundCopyManager2_5_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits2_5_0000_0002 */
/* [local] */ 

#define   BG_SSL_ENABLE_CRL_CHECK               0x0001
#define   BG_SSL_IGNORE_CERT_CN_INVALID         0x0002
#define   BG_SSL_IGNORE_CERT_DATE_INVALID       0x0004
#define   BG_SSL_IGNORE_UNKNOWN_CA              0x0008
#define   BG_SSL_IGNORE_CERT_WRONG_USAGE        0x0010
#define   BG_HTTP_REDIRECT_POLICY_MASK          0x0700
#define   BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT  0x0000
#define   BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT  0x0100
#define   BG_HTTP_REDIRECT_POLICY_DISALLOW      0x0200
#define   BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP  0x0800
#include "bits3_0.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits2_5_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits2_5_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



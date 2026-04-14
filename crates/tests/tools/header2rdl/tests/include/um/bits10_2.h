

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

#ifndef __bits10_2_h__
#define __bits10_2_h__

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

#ifndef __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions2 IBackgroundCopyJobHttpOptions2;

#endif 	/* __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager10_2_FWD_DEFINED__
#define __BackgroundCopyManager10_2_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager10_2 BackgroundCopyManager10_2;
#else
typedef struct BackgroundCopyManager10_2 BackgroundCopyManager10_2;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager10_2_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions2 IBackgroundCopyJobHttpOptions2;

#endif 	/* __IBackgroundCopyJobHttpOptions2_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"
#include "bits2_5.h"
#include "bits3_0.h"
#include "bits4_0.h"
#include "bits5_0.h"
#include "bits10_1.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits10_2_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits10_2_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_2_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyJobHttpOptions2_INTERFACE_DEFINED__
#define __IBackgroundCopyJobHttpOptions2_INTERFACE_DEFINED__

/* interface IBackgroundCopyJobHttpOptions2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyJobHttpOptions2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B591A192-A405-4FC3-8323-4C5C542578FC")
    IBackgroundCopyJobHttpOptions2 : public IBackgroundCopyJobHttpOptions
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetHttpMethod( 
            /* [in] */ __RPC__in LPCWSTR method) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHttpMethod( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *method) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJobHttpOptions2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByID)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByID )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(20) byte *pCertHashBlob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByName)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByName )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [in] */ __RPC__in LPCWSTR SubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, RemoveClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *RemoveClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [ref][out] */ __RPC__out BG_CERT_STORE_LOCATION *pStoreLocation,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pStoreName,
            /* [ref][size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(20) byte **ppCertHashBlob,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pSubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *SetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR RequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pRequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *SetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [in] */ ULONG Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [ref][out] */ __RPC__out ULONG *pFlags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions2, SetHttpMethod)
        HRESULT ( STDMETHODCALLTYPE *SetHttpMethod )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [in] */ __RPC__in LPCWSTR method);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions2, GetHttpMethod)
        HRESULT ( STDMETHODCALLTYPE *GetHttpMethod )( 
            __RPC__in IBackgroundCopyJobHttpOptions2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *method);
        
        END_INTERFACE
    } IBackgroundCopyJobHttpOptions2Vtbl;

    interface IBackgroundCopyJobHttpOptions2
    {
        CONST_VTBL struct IBackgroundCopyJobHttpOptions2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJobHttpOptions2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJobHttpOptions2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJobHttpOptions2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJobHttpOptions2_SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob)	\
    ( (This)->lpVtbl -> SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob) ) 

#define IBackgroundCopyJobHttpOptions2_SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName)	\
    ( (This)->lpVtbl -> SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName) ) 

#define IBackgroundCopyJobHttpOptions2_RemoveClientCertificate(This)	\
    ( (This)->lpVtbl -> RemoveClientCertificate(This) ) 

#define IBackgroundCopyJobHttpOptions2_GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName)	\
    ( (This)->lpVtbl -> GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName) ) 

#define IBackgroundCopyJobHttpOptions2_SetCustomHeaders(This,RequestHeaders)	\
    ( (This)->lpVtbl -> SetCustomHeaders(This,RequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions2_GetCustomHeaders(This,pRequestHeaders)	\
    ( (This)->lpVtbl -> GetCustomHeaders(This,pRequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions2_SetSecurityFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetSecurityFlags(This,Flags) ) 

#define IBackgroundCopyJobHttpOptions2_GetSecurityFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetSecurityFlags(This,pFlags) ) 


#define IBackgroundCopyJobHttpOptions2_SetHttpMethod(This,method)	\
    ( (This)->lpVtbl -> SetHttpMethod(This,method) ) 

#define IBackgroundCopyJobHttpOptions2_GetHttpMethod(This,method)	\
    ( (This)->lpVtbl -> GetHttpMethod(This,method) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJobHttpOptions2_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager10_2_LIBRARY_DEFINED__
#define __BackgroundCopyManager10_2_LIBRARY_DEFINED__

/* library BackgroundCopyManager10_2 */
/* [version][lcid][uuid] */ 












EXTERN_C const IID LIBID_BackgroundCopyManager10_2;

EXTERN_C const CLSID CLSID_BackgroundCopyManager10_2;

#ifdef __cplusplus

class DECLSPEC_UUID("4575438F-A6C8-4976-B0FE-2F26B80D959E")
BackgroundCopyManager10_2;
#endif
#endif /* __BackgroundCopyManager10_2_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits10_2_0000_0002 */
/* [local] */ 

#include "bits10_3.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS5 */


extern RPC_IF_HANDLE __MIDL_itf_bits10_2_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_2_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



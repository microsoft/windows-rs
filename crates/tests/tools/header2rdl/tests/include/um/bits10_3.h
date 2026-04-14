

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

#ifndef __bits10_3_h__
#define __bits10_3_h__

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

#ifndef __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__
#define __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__
typedef interface IBackgroundCopyServerCertificateValidationCallback IBackgroundCopyServerCertificateValidationCallback;

#endif 	/* __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions3 IBackgroundCopyJobHttpOptions3;

#endif 	/* __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager10_3_FWD_DEFINED__
#define __BackgroundCopyManager10_3_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager10_3 BackgroundCopyManager10_3;
#else
typedef struct BackgroundCopyManager10_3 BackgroundCopyManager10_3;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager10_3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__
#define __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__
typedef interface IBackgroundCopyJobHttpOptions3 IBackgroundCopyJobHttpOptions3;

#endif 	/* __IBackgroundCopyJobHttpOptions3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__
#define __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__
typedef interface IBackgroundCopyServerCertificateValidationCallback IBackgroundCopyServerCertificateValidationCallback;

#endif 	/* __IBackgroundCopyServerCertificateValidationCallback_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"
#include "bits2_5.h"
#include "bits3_0.h"
#include "bits4_0.h"
#include "bits5_0.h"
#include "bits10_1.h"
#include "bits10_2.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits10_3_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits10_3_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_3_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyServerCertificateValidationCallback_INTERFACE_DEFINED__
#define __IBackgroundCopyServerCertificateValidationCallback_INTERFACE_DEFINED__

/* interface IBackgroundCopyServerCertificateValidationCallback */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyServerCertificateValidationCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4CEC0D02-DEF7-4158-813A-C32A46945FF7")
    IBackgroundCopyServerCertificateValidationCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ValidateServerCertificate( 
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *job,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *file,
            /* [in] */ DWORD certLength,
            /* [size_is][in] */ __RPC__in_ecount_full(certLength) const BYTE certData[  ],
            /* [in] */ DWORD certEncodingType,
            /* [in] */ DWORD certStoreLength,
            /* [size_is][in] */ __RPC__in_ecount_full(certStoreLength) const BYTE certStoreData[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyServerCertificateValidationCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyServerCertificateValidationCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyServerCertificateValidationCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyServerCertificateValidationCallback * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyServerCertificateValidationCallback, ValidateServerCertificate)
        HRESULT ( STDMETHODCALLTYPE *ValidateServerCertificate )( 
            __RPC__in IBackgroundCopyServerCertificateValidationCallback * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *job,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *file,
            /* [in] */ DWORD certLength,
            /* [size_is][in] */ __RPC__in_ecount_full(certLength) const BYTE certData[  ],
            /* [in] */ DWORD certEncodingType,
            /* [in] */ DWORD certStoreLength,
            /* [size_is][in] */ __RPC__in_ecount_full(certStoreLength) const BYTE certStoreData[  ]);
        
        END_INTERFACE
    } IBackgroundCopyServerCertificateValidationCallbackVtbl;

    interface IBackgroundCopyServerCertificateValidationCallback
    {
        CONST_VTBL struct IBackgroundCopyServerCertificateValidationCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyServerCertificateValidationCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyServerCertificateValidationCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyServerCertificateValidationCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyServerCertificateValidationCallback_ValidateServerCertificate(This,job,file,certLength,certData,certEncodingType,certStoreLength,certStoreData)	\
    ( (This)->lpVtbl -> ValidateServerCertificate(This,job,file,certLength,certData,certEncodingType,certStoreLength,certStoreData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyServerCertificateValidationCallback_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyJobHttpOptions3_INTERFACE_DEFINED__
#define __IBackgroundCopyJobHttpOptions3_INTERFACE_DEFINED__

/* interface IBackgroundCopyJobHttpOptions3 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyJobHttpOptions3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A9263D3-FD4C-4EDA-9B28-30132A4D4E3C")
    IBackgroundCopyJobHttpOptions3 : public IBackgroundCopyJobHttpOptions2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetServerCertificateValidationInterface( 
            /* [in] */ __RPC__in_opt IUnknown *certValidationCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeCustomHeadersWriteOnly( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJobHttpOptions3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByID)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByID )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(20) byte *pCertHashBlob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetClientCertificateByName)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificateByName )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ BG_CERT_STORE_LOCATION StoreLocation,
            /* [in] */ __RPC__in LPCWSTR StoreName,
            /* [in] */ __RPC__in LPCWSTR SubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, RemoveClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *RemoveClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetClientCertificate )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [ref][out] */ __RPC__out BG_CERT_STORE_LOCATION *pStoreLocation,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pStoreName,
            /* [ref][size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(20) byte **ppCertHashBlob,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pSubjectName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *SetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR RequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetCustomHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetCustomHeaders )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pRequestHeaders);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, SetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *SetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ ULONG Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions, GetSecurityFlags)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityFlags )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [ref][out] */ __RPC__out ULONG *pFlags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions2, SetHttpMethod)
        HRESULT ( STDMETHODCALLTYPE *SetHttpMethod )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ __RPC__in LPCWSTR method);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions2, GetHttpMethod)
        HRESULT ( STDMETHODCALLTYPE *GetHttpMethod )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *method);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions3, SetServerCertificateValidationInterface)
        HRESULT ( STDMETHODCALLTYPE *SetServerCertificateValidationInterface )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This,
            /* [in] */ __RPC__in_opt IUnknown *certValidationCallback);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJobHttpOptions3, MakeCustomHeadersWriteOnly)
        HRESULT ( STDMETHODCALLTYPE *MakeCustomHeadersWriteOnly )( 
            __RPC__in IBackgroundCopyJobHttpOptions3 * This);
        
        END_INTERFACE
    } IBackgroundCopyJobHttpOptions3Vtbl;

    interface IBackgroundCopyJobHttpOptions3
    {
        CONST_VTBL struct IBackgroundCopyJobHttpOptions3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJobHttpOptions3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJobHttpOptions3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJobHttpOptions3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJobHttpOptions3_SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob)	\
    ( (This)->lpVtbl -> SetClientCertificateByID(This,StoreLocation,StoreName,pCertHashBlob) ) 

#define IBackgroundCopyJobHttpOptions3_SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName)	\
    ( (This)->lpVtbl -> SetClientCertificateByName(This,StoreLocation,StoreName,SubjectName) ) 

#define IBackgroundCopyJobHttpOptions3_RemoveClientCertificate(This)	\
    ( (This)->lpVtbl -> RemoveClientCertificate(This) ) 

#define IBackgroundCopyJobHttpOptions3_GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName)	\
    ( (This)->lpVtbl -> GetClientCertificate(This,pStoreLocation,pStoreName,ppCertHashBlob,pSubjectName) ) 

#define IBackgroundCopyJobHttpOptions3_SetCustomHeaders(This,RequestHeaders)	\
    ( (This)->lpVtbl -> SetCustomHeaders(This,RequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions3_GetCustomHeaders(This,pRequestHeaders)	\
    ( (This)->lpVtbl -> GetCustomHeaders(This,pRequestHeaders) ) 

#define IBackgroundCopyJobHttpOptions3_SetSecurityFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetSecurityFlags(This,Flags) ) 

#define IBackgroundCopyJobHttpOptions3_GetSecurityFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetSecurityFlags(This,pFlags) ) 


#define IBackgroundCopyJobHttpOptions3_SetHttpMethod(This,method)	\
    ( (This)->lpVtbl -> SetHttpMethod(This,method) ) 

#define IBackgroundCopyJobHttpOptions3_GetHttpMethod(This,method)	\
    ( (This)->lpVtbl -> GetHttpMethod(This,method) ) 


#define IBackgroundCopyJobHttpOptions3_SetServerCertificateValidationInterface(This,certValidationCallback)	\
    ( (This)->lpVtbl -> SetServerCertificateValidationInterface(This,certValidationCallback) ) 

#define IBackgroundCopyJobHttpOptions3_MakeCustomHeadersWriteOnly(This)	\
    ( (This)->lpVtbl -> MakeCustomHeadersWriteOnly(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJobHttpOptions3_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager10_3_LIBRARY_DEFINED__
#define __BackgroundCopyManager10_3_LIBRARY_DEFINED__

/* library BackgroundCopyManager10_3 */
/* [version][lcid][uuid] */ 













EXTERN_C const IID LIBID_BackgroundCopyManager10_3;

EXTERN_C const CLSID CLSID_BackgroundCopyManager10_3;

#ifdef __cplusplus

class DECLSPEC_UUID("5FD42AD5-C04E-4D36-ADC7-E08FF15737AD")
BackgroundCopyManager10_3;
#endif
#endif /* __BackgroundCopyManager10_3_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits10_3_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits10_3_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_3_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif





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

#ifndef __proofofpossessioncookieinfo_h__
#define __proofofpossessioncookieinfo_h__

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

#ifndef __IProofOfPossessionCookieInfoManager_FWD_DEFINED__
#define __IProofOfPossessionCookieInfoManager_FWD_DEFINED__
typedef interface IProofOfPossessionCookieInfoManager IProofOfPossessionCookieInfoManager;

#endif 	/* __IProofOfPossessionCookieInfoManager_FWD_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager2_FWD_DEFINED__
#define __IProofOfPossessionCookieInfoManager2_FWD_DEFINED__
typedef interface IProofOfPossessionCookieInfoManager2 IProofOfPossessionCookieInfoManager2;

#endif 	/* __IProofOfPossessionCookieInfoManager2_FWD_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager3_FWD_DEFINED__
#define __IProofOfPossessionCookieInfoManager3_FWD_DEFINED__
typedef interface IProofOfPossessionCookieInfoManager3 IProofOfPossessionCookieInfoManager3;

#endif 	/* __IProofOfPossessionCookieInfoManager3_FWD_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager4_FWD_DEFINED__
#define __IProofOfPossessionCookieInfoManager4_FWD_DEFINED__
typedef interface IProofOfPossessionCookieInfoManager4 IProofOfPossessionCookieInfoManager4;

#endif 	/* __IProofOfPossessionCookieInfoManager4_FWD_DEFINED__ */


#ifndef __ProofOfPossessionCookieInfoManager_FWD_DEFINED__
#define __ProofOfPossessionCookieInfoManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class ProofOfPossessionCookieInfoManager ProofOfPossessionCookieInfoManager;
#else
typedef struct ProofOfPossessionCookieInfoManager ProofOfPossessionCookieInfoManager;
#endif /* __cplusplus */

#endif 	/* __ProofOfPossessionCookieInfoManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "Inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_proofofpossessioncookieinfo_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct ProofOfPossessionCookieInfo
    {
    LPWSTR name;
    LPWSTR data;
    DWORD flags;
    LPWSTR p3pHeader;
    } 	ProofOfPossessionCookieInfo;

__inline void FreeProofOfPossessionCookieInfoArray(_In_reads_(cookieInfoCount) ProofOfPossessionCookieInfo* cookieInfo, DWORD cookieInfoCount)
{                                                
     DWORD i;                                    
     for (i = 0; i < cookieInfoCount; i++)       
     {                                           
         CoTaskMemFree(cookieInfo[i].name);      
         CoTaskMemFree(cookieInfo[i].data);      
         CoTaskMemFree(cookieInfo[i].p3pHeader); 
     }                                           
     CoTaskMemFree(cookieInfo);                  
}                                                
// Flags that can be passed into GetCookieInfoForUriWithOptions
// to control the behavior
typedef enum _PROOF_OF_POSSESSION_FLAGS : DWORD
{
    PROOF_OF_POSSESSION_DEFAULT                      = 0x0000000000000000,
    PROOF_OF_POSSESSION_ALLOW_SILENT_REQUESTS        = 0x0000000000000001,
} PROOF_OF_POSSESSION_FLAGS;


extern RPC_IF_HANDLE __MIDL_itf_proofofpossessioncookieinfo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_proofofpossessioncookieinfo_0000_0000_v0_0_s_ifspec;

#ifndef __IProofOfPossessionCookieInfoManager_INTERFACE_DEFINED__
#define __IProofOfPossessionCookieInfoManager_INTERFACE_DEFINED__

/* interface IProofOfPossessionCookieInfoManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProofOfPossessionCookieInfoManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CDAECE56-4EDF-43DF-B113-88E4556FA1BB")
    IProofOfPossessionCookieInfoManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCookieInfoForUri( 
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProofOfPossessionCookieInfoManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProofOfPossessionCookieInfoManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProofOfPossessionCookieInfoManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProofOfPossessionCookieInfoManager * This);
        
        DECLSPEC_XFGVIRT(IProofOfPossessionCookieInfoManager, GetCookieInfoForUri)
        HRESULT ( STDMETHODCALLTYPE *GetCookieInfoForUri )( 
            __RPC__in IProofOfPossessionCookieInfoManager * This,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo);
        
        END_INTERFACE
    } IProofOfPossessionCookieInfoManagerVtbl;

    interface IProofOfPossessionCookieInfoManager
    {
        CONST_VTBL struct IProofOfPossessionCookieInfoManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProofOfPossessionCookieInfoManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProofOfPossessionCookieInfoManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProofOfPossessionCookieInfoManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProofOfPossessionCookieInfoManager_GetCookieInfoForUri(This,uri,cookieInfoCount,cookieInfo)	\
    ( (This)->lpVtbl -> GetCookieInfoForUri(This,uri,cookieInfoCount,cookieInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProofOfPossessionCookieInfoManager_INTERFACE_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager2_INTERFACE_DEFINED__
#define __IProofOfPossessionCookieInfoManager2_INTERFACE_DEFINED__

/* interface IProofOfPossessionCookieInfoManager2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProofOfPossessionCookieInfoManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("15E41407-B42F-4AE7-9966-34A087B2D713")
    IProofOfPossessionCookieInfoManager2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCookieInfoWithUriForAccount( 
            /* [in] */ __RPC__in_opt IInspectable *webAccount,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProofOfPossessionCookieInfoManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProofOfPossessionCookieInfoManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProofOfPossessionCookieInfoManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProofOfPossessionCookieInfoManager2 * This);
        
        DECLSPEC_XFGVIRT(IProofOfPossessionCookieInfoManager2, GetCookieInfoWithUriForAccount)
        HRESULT ( STDMETHODCALLTYPE *GetCookieInfoWithUriForAccount )( 
            __RPC__in IProofOfPossessionCookieInfoManager2 * This,
            /* [in] */ __RPC__in_opt IInspectable *webAccount,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo);
        
        END_INTERFACE
    } IProofOfPossessionCookieInfoManager2Vtbl;

    interface IProofOfPossessionCookieInfoManager2
    {
        CONST_VTBL struct IProofOfPossessionCookieInfoManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProofOfPossessionCookieInfoManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProofOfPossessionCookieInfoManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProofOfPossessionCookieInfoManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProofOfPossessionCookieInfoManager2_GetCookieInfoWithUriForAccount(This,webAccount,uri,cookieInfoCount,cookieInfo)	\
    ( (This)->lpVtbl -> GetCookieInfoWithUriForAccount(This,webAccount,uri,cookieInfoCount,cookieInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProofOfPossessionCookieInfoManager2_INTERFACE_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager3_INTERFACE_DEFINED__
#define __IProofOfPossessionCookieInfoManager3_INTERFACE_DEFINED__

/* interface IProofOfPossessionCookieInfoManager3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProofOfPossessionCookieInfoManager3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8891744-32bd-4a77-b92c-0e79a2823b96")
    IProofOfPossessionCookieInfoManager3 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCookieInfoForUriWithOptions( 
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ DWORD options,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProofOfPossessionCookieInfoManager3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProofOfPossessionCookieInfoManager3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProofOfPossessionCookieInfoManager3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProofOfPossessionCookieInfoManager3 * This);
        
        DECLSPEC_XFGVIRT(IProofOfPossessionCookieInfoManager3, GetCookieInfoForUriWithOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCookieInfoForUriWithOptions )( 
            __RPC__in IProofOfPossessionCookieInfoManager3 * This,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ DWORD options,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo);
        
        END_INTERFACE
    } IProofOfPossessionCookieInfoManager3Vtbl;

    interface IProofOfPossessionCookieInfoManager3
    {
        CONST_VTBL struct IProofOfPossessionCookieInfoManager3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProofOfPossessionCookieInfoManager3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProofOfPossessionCookieInfoManager3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProofOfPossessionCookieInfoManager3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProofOfPossessionCookieInfoManager3_GetCookieInfoForUriWithOptions(This,uri,options,cookieInfoCount,cookieInfo)	\
    ( (This)->lpVtbl -> GetCookieInfoForUriWithOptions(This,uri,options,cookieInfoCount,cookieInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProofOfPossessionCookieInfoManager3_INTERFACE_DEFINED__ */


#ifndef __IProofOfPossessionCookieInfoManager4_INTERFACE_DEFINED__
#define __IProofOfPossessionCookieInfoManager4_INTERFACE_DEFINED__

/* interface IProofOfPossessionCookieInfoManager4 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProofOfPossessionCookieInfoManager4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3B74C75B-6E3F-494E-95EC-13174E12A89F")
    IProofOfPossessionCookieInfoManager4 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCookieInfoForUriWithUserAgentId( 
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ __RPC__in LPCWSTR uaClientId,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCookieInfoWithUriAndUserAgentIdForAccount( 
            /* [in] */ __RPC__in_opt IInspectable *webAccount,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ __RPC__in LPCWSTR uaClientId,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProofOfPossessionCookieInfoManager4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProofOfPossessionCookieInfoManager4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProofOfPossessionCookieInfoManager4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProofOfPossessionCookieInfoManager4 * This);
        
        DECLSPEC_XFGVIRT(IProofOfPossessionCookieInfoManager4, GetCookieInfoForUriWithUserAgentId)
        HRESULT ( STDMETHODCALLTYPE *GetCookieInfoForUriWithUserAgentId )( 
            __RPC__in IProofOfPossessionCookieInfoManager4 * This,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ __RPC__in LPCWSTR uaClientId,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo);
        
        DECLSPEC_XFGVIRT(IProofOfPossessionCookieInfoManager4, GetCookieInfoWithUriAndUserAgentIdForAccount)
        HRESULT ( STDMETHODCALLTYPE *GetCookieInfoWithUriAndUserAgentIdForAccount )( 
            __RPC__in IProofOfPossessionCookieInfoManager4 * This,
            /* [in] */ __RPC__in_opt IInspectable *webAccount,
            /* [in] */ __RPC__in LPCWSTR uri,
            /* [in] */ __RPC__in LPCWSTR uaClientId,
            /* [out] */ __RPC__out DWORD *cookieInfoCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*cookieInfoCount) ProofOfPossessionCookieInfo **cookieInfo);
        
        END_INTERFACE
    } IProofOfPossessionCookieInfoManager4Vtbl;

    interface IProofOfPossessionCookieInfoManager4
    {
        CONST_VTBL struct IProofOfPossessionCookieInfoManager4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProofOfPossessionCookieInfoManager4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProofOfPossessionCookieInfoManager4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProofOfPossessionCookieInfoManager4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProofOfPossessionCookieInfoManager4_GetCookieInfoForUriWithUserAgentId(This,uri,uaClientId,cookieInfoCount,cookieInfo)	\
    ( (This)->lpVtbl -> GetCookieInfoForUriWithUserAgentId(This,uri,uaClientId,cookieInfoCount,cookieInfo) ) 

#define IProofOfPossessionCookieInfoManager4_GetCookieInfoWithUriAndUserAgentIdForAccount(This,webAccount,uri,uaClientId,cookieInfoCount,cookieInfo)	\
    ( (This)->lpVtbl -> GetCookieInfoWithUriAndUserAgentIdForAccount(This,webAccount,uri,uaClientId,cookieInfoCount,cookieInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProofOfPossessionCookieInfoManager4_INTERFACE_DEFINED__ */



#ifndef __ProofOfPossessionCookieInfoManagerLib_LIBRARY_DEFINED__
#define __ProofOfPossessionCookieInfoManagerLib_LIBRARY_DEFINED__

/* library ProofOfPossessionCookieInfoManagerLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_ProofOfPossessionCookieInfoManagerLib;

EXTERN_C const CLSID CLSID_ProofOfPossessionCookieInfoManager;

#ifdef __cplusplus

class DECLSPEC_UUID("A9927F85-A304-4390-8B23-A75F1C668600")
ProofOfPossessionCookieInfoManager;
#endif
#endif /* __ProofOfPossessionCookieInfoManagerLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_proofofpossessioncookieinfo_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_proofofpossessioncookieinfo_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_proofofpossessioncookieinfo_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



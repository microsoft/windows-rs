

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

#ifndef __bits4_0_h__
#define __bits4_0_h__

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

#ifndef __IBitsTokenOptions_FWD_DEFINED__
#define __IBitsTokenOptions_FWD_DEFINED__
typedef interface IBitsTokenOptions IBitsTokenOptions;

#endif 	/* __IBitsTokenOptions_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile4_FWD_DEFINED__
#define __IBackgroundCopyFile4_FWD_DEFINED__
typedef interface IBackgroundCopyFile4 IBackgroundCopyFile4;

#endif 	/* __IBackgroundCopyFile4_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager4_0_FWD_DEFINED__
#define __BackgroundCopyManager4_0_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager4_0 BackgroundCopyManager4_0;
#else
typedef struct BackgroundCopyManager4_0 BackgroundCopyManager4_0;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager4_0_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile4_FWD_DEFINED__
#define __IBackgroundCopyFile4_FWD_DEFINED__
typedef interface IBackgroundCopyFile4 IBackgroundCopyFile4;

#endif 	/* __IBackgroundCopyFile4_FWD_DEFINED__ */


#ifndef __IBitsTokenOptions_FWD_DEFINED__
#define __IBitsTokenOptions_FWD_DEFINED__
typedef interface IBitsTokenOptions IBitsTokenOptions;

#endif 	/* __IBitsTokenOptions_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"
#include "bits2_5.h"
#include "bits3_0.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits4_0_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits4_0_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits4_0_0000_0000_v0_0_s_ifspec;

#ifndef __IBitsTokenOptions_INTERFACE_DEFINED__
#define __IBitsTokenOptions_INTERFACE_DEFINED__

/* interface IBitsTokenOptions */
/* [object][helpstring][uuid] */ 

#define BG_TOKEN_LOCAL_FILE      0x0001
#define BG_TOKEN_NETWORK         0x0002

EXTERN_C const IID IID_IBitsTokenOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9a2584c3-f7d2-457a-9a5e-22b67bffc7d2")
    IBitsTokenOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetHelperTokenFlags( 
            DWORD UsageFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelperTokenFlags( 
            /* [out] */ __RPC__out DWORD *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelperToken( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearHelperToken( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelperTokenSid( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *pSid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBitsTokenOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBitsTokenOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBitsTokenOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBitsTokenOptions * This);
        
        DECLSPEC_XFGVIRT(IBitsTokenOptions, SetHelperTokenFlags)
        HRESULT ( STDMETHODCALLTYPE *SetHelperTokenFlags )( 
            __RPC__in IBitsTokenOptions * This,
            DWORD UsageFlags);
        
        DECLSPEC_XFGVIRT(IBitsTokenOptions, GetHelperTokenFlags)
        HRESULT ( STDMETHODCALLTYPE *GetHelperTokenFlags )( 
            __RPC__in IBitsTokenOptions * This,
            /* [out] */ __RPC__out DWORD *pFlags);
        
        DECLSPEC_XFGVIRT(IBitsTokenOptions, SetHelperToken)
        HRESULT ( STDMETHODCALLTYPE *SetHelperToken )( 
            __RPC__in IBitsTokenOptions * This);
        
        DECLSPEC_XFGVIRT(IBitsTokenOptions, ClearHelperToken)
        HRESULT ( STDMETHODCALLTYPE *ClearHelperToken )( 
            __RPC__in IBitsTokenOptions * This);
        
        DECLSPEC_XFGVIRT(IBitsTokenOptions, GetHelperTokenSid)
        HRESULT ( STDMETHODCALLTYPE *GetHelperTokenSid )( 
            __RPC__in IBitsTokenOptions * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pSid);
        
        END_INTERFACE
    } IBitsTokenOptionsVtbl;

    interface IBitsTokenOptions
    {
        CONST_VTBL struct IBitsTokenOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBitsTokenOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBitsTokenOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBitsTokenOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBitsTokenOptions_SetHelperTokenFlags(This,UsageFlags)	\
    ( (This)->lpVtbl -> SetHelperTokenFlags(This,UsageFlags) ) 

#define IBitsTokenOptions_GetHelperTokenFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetHelperTokenFlags(This,pFlags) ) 

#define IBitsTokenOptions_SetHelperToken(This)	\
    ( (This)->lpVtbl -> SetHelperToken(This) ) 

#define IBitsTokenOptions_ClearHelperToken(This)	\
    ( (This)->lpVtbl -> ClearHelperToken(This) ) 

#define IBitsTokenOptions_GetHelperTokenSid(This,pSid)	\
    ( (This)->lpVtbl -> GetHelperTokenSid(This,pSid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBitsTokenOptions_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyFile4_INTERFACE_DEFINED__
#define __IBackgroundCopyFile4_INTERFACE_DEFINED__

/* interface IBackgroundCopyFile4 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyFile4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ef7e0655-7888-4960-b0e5-730846e03492")
    IBackgroundCopyFile4 : public IBackgroundCopyFile3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPeerDownloadStats( 
            /* [out] */ __RPC__out PUINT64 pFromOrigin,
            /* [out] */ __RPC__out PUINT64 pFromPeers) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyFile4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyFile4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyFile4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteName )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetLocalName)
        HRESULT ( STDMETHODCALLTYPE *GetLocalName )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__out BG_FILE_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, GetFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFileRanges )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *RangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*RangeCount) BG_FILE_RANGE **Ranges);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, SetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteName )( 
            __RPC__in IBackgroundCopyFile4 * This,
            __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetTemporaryName)
        HRESULT ( STDMETHODCALLTYPE *GetTemporaryName )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pFilename);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, SetValidationState)
        HRESULT ( STDMETHODCALLTYPE *SetValidationState )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [in] */ BOOL state);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetValidationState)
        HRESULT ( STDMETHODCALLTYPE *GetValidationState )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__out BOOL *pState);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, IsDownloadedFromPeer)
        HRESULT ( STDMETHODCALLTYPE *IsDownloadedFromPeer )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile4, GetPeerDownloadStats)
        HRESULT ( STDMETHODCALLTYPE *GetPeerDownloadStats )( 
            __RPC__in IBackgroundCopyFile4 * This,
            /* [out] */ __RPC__out PUINT64 pFromOrigin,
            /* [out] */ __RPC__out PUINT64 pFromPeers);
        
        END_INTERFACE
    } IBackgroundCopyFile4Vtbl;

    interface IBackgroundCopyFile4
    {
        CONST_VTBL struct IBackgroundCopyFile4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyFile4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyFile4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyFile4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyFile4_GetRemoteName(This,pVal)	\
    ( (This)->lpVtbl -> GetRemoteName(This,pVal) ) 

#define IBackgroundCopyFile4_GetLocalName(This,pVal)	\
    ( (This)->lpVtbl -> GetLocalName(This,pVal) ) 

#define IBackgroundCopyFile4_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 


#define IBackgroundCopyFile4_GetFileRanges(This,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> GetFileRanges(This,RangeCount,Ranges) ) 

#define IBackgroundCopyFile4_SetRemoteName(This,Val)	\
    ( (This)->lpVtbl -> SetRemoteName(This,Val) ) 


#define IBackgroundCopyFile4_GetTemporaryName(This,pFilename)	\
    ( (This)->lpVtbl -> GetTemporaryName(This,pFilename) ) 

#define IBackgroundCopyFile4_SetValidationState(This,state)	\
    ( (This)->lpVtbl -> SetValidationState(This,state) ) 

#define IBackgroundCopyFile4_GetValidationState(This,pState)	\
    ( (This)->lpVtbl -> GetValidationState(This,pState) ) 

#define IBackgroundCopyFile4_IsDownloadedFromPeer(This,pVal)	\
    ( (This)->lpVtbl -> IsDownloadedFromPeer(This,pVal) ) 


#define IBackgroundCopyFile4_GetPeerDownloadStats(This,pFromOrigin,pFromPeers)	\
    ( (This)->lpVtbl -> GetPeerDownloadStats(This,pFromOrigin,pFromPeers) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyFile4_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager4_0_LIBRARY_DEFINED__
#define __BackgroundCopyManager4_0_LIBRARY_DEFINED__

/* library BackgroundCopyManager4_0 */
/* [version][lcid][helpstring][uuid] */ 












EXTERN_C const IID LIBID_BackgroundCopyManager4_0;

EXTERN_C const CLSID CLSID_BackgroundCopyManager4_0;

#ifdef __cplusplus

class DECLSPEC_UUID("bb6df56b-cace-11dc-9992-0019b93a3a84")
BackgroundCopyManager4_0;
#endif
#endif /* __BackgroundCopyManager4_0_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits4_0_0000_0003 */
/* [local] */ 

#include "bits5_0.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits4_0_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits4_0_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


